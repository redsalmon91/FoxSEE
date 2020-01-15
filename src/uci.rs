use crate::{
    util,
};

use std::io::{self, prelude::*};

const DEFAULT_MOVS_TO_GO: u128 = 30;
const OVERHEAD_TIME: u128 = 10;

pub struct Rawmov {
    pub from: usize,
    pub to: usize,
    pub promo: String,
    pub origin_mov_str: String,
}

pub struct TimeInfo {
    pub white_millis: u128,
    pub black_millis: u128,
}

pub enum UciProcessResult {
    Noop,
    Ready,
    Reset,
    Position(Vec<Rawmov>),
    StartSearchWithTime(u128),
    StartSearchWithComplextTimeControl(TimeInfo),
    Stop,
    Quit,
}

pub fn process_uci_cmd(uci_cmd: &str) -> UciProcessResult {
    let mut cmd_seq: Vec<&str> = uci_cmd.split(' ').collect();
    match cmd_seq[0] {
        "uci" => {
            println!("id name FoxSEE 0.2.0");
            println!("id author Zixiao Han");
            println!("uciok");
            io::stdout().flush().ok();
            UciProcessResult::Ready
        }
        "debug" => UciProcessResult::Noop,
        "isready" => {
            println!("readyok");
            io::stdout().flush().ok();
            UciProcessResult::Noop
        }
        "setoption" => UciProcessResult::Noop,
        "register" => UciProcessResult::Noop,
        "ucinewgame" => UciProcessResult::Reset,
        "position" => match cmd_seq[1] {
            "startpos" => {
                if cmd_seq.len() > 3 {
                    process_position_startpos_with_mov_list(cmd_seq.split_off(3))
                } else {
                    process_position_startpos()
                }
            }
            _ => {
                eprintln!("only support startpos with mov list");
                UciProcessResult::Noop
            },
        },
        "go" => process_go_cmd(cmd_seq.split_off(0)),
        "stop" => UciProcessResult::Stop,
        "ponderhit" => UciProcessResult::Noop,
        "quit" => UciProcessResult::Quit,
        _ => {
            eprintln!("unknown uci command {}", cmd_seq[0]);
            UciProcessResult::Noop
        }
    }
}

fn process_go_cmd(go_cmd_seq: Vec<&str>) -> UciProcessResult {
    match go_cmd_seq[1] {
        "ponder" => UciProcessResult::Noop,
        "movetime" => UciProcessResult::StartSearchWithTime(go_cmd_seq[2].parse::<u128>().unwrap()),
        "wtime" => process_time_control(go_cmd_seq),
        sub_cmd => panic!("unsupported sub command {}", sub_cmd),
    }
}

fn process_time_control(go_cmd_seq: Vec<&str>) -> UciProcessResult {
    assert!(go_cmd_seq[1] == "wtime");
    let wtime = go_cmd_seq[2].parse::<u128>().unwrap();

    assert!(go_cmd_seq[3] == "btime");
    let btime = go_cmd_seq[4].parse::<u128>().unwrap();

    let movs_to_go = if go_cmd_seq.len() > 5 && go_cmd_seq[5] == "movestogo" {
        go_cmd_seq[6].parse::<u128>().unwrap()
    } else if go_cmd_seq.len() > 9 && go_cmd_seq[9] == "movestogo" {
        go_cmd_seq[10].parse::<u128>().unwrap()
    } else {
        eprintln!("no movestogo specified, use default");
        DEFAULT_MOVS_TO_GO
    };

    let wtime = if wtime > OVERHEAD_TIME {
        wtime - OVERHEAD_TIME
    } else {
        wtime
    };

    let btime = if btime > OVERHEAD_TIME {
        btime - OVERHEAD_TIME
    } else {
        btime
    };

    UciProcessResult::StartSearchWithComplextTimeControl(TimeInfo{
        white_millis: wtime / movs_to_go,
        black_millis: btime / movs_to_go,
    })
}

fn process_position_startpos() -> UciProcessResult {
    UciProcessResult::Position(vec![])
}

fn process_position_startpos_with_mov_list(mov_str_list: Vec<&str>) -> UciProcessResult {
    let mut mov_list = Vec::new();
    for mov_str in mov_str_list {
        mov_list.push(parse_mov_str(mov_str));
    }

    UciProcessResult::Position(mov_list)
}

fn parse_mov_str(mov_str: &str) -> Rawmov {
    let from_str = &mov_str[0..2];
    let to_str = &mov_str[2..4];

    let mut promotion_piece = String::new();
    if mov_str.len() == 5 {
        promotion_piece.push_str(&mov_str[4..]);
    }

    Rawmov {
        from: util::map_sqr_notation_to_index(from_str),
        to: util::map_sqr_notation_to_index(to_str),
        promo: promotion_piece,
        origin_mov_str: mov_str.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        util,
    };

    #[test]
    fn test_parse_mov_str() {
        let raw_mov = parse_mov_str("e1g1");
        assert_eq!(util::map_sqr_notation_to_index("e1"), raw_mov.from);
        assert_eq!(util::map_sqr_notation_to_index("g1"), raw_mov.to);
        assert_eq!("".to_owned(), raw_mov.promo);

        let raw_mov = parse_mov_str("a7b8q");
        assert_eq!(util::map_sqr_notation_to_index("a7"), raw_mov.from);
        assert_eq!(util::map_sqr_notation_to_index("b8"), raw_mov.to);
        assert_eq!("q".to_owned(), raw_mov.promo);
    }
}