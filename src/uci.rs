/*
 * Copyright (C) 2020 Zixiao Han
 */

use crate::{
    def,
    util,
};

use std::io::{self, prelude::*};

const DEFAULT_MOVS_TO_GO: u128 = 20;
const OVERHEAD_TIME: u128 = 100;

pub const FEN_START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

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
    Reset,
    IgnoredOption,
    SetHashSize(usize),
    Perft(u8),
    Position(String, Vec<Rawmov>),
    StartSearchWithTime(u128),
    StartSearchWithComplextTimeControl(TimeInfo),
    StartSearchToDepth(u8),
    StartSearchInfinite,
    Stop,
    Quit,
}

pub fn process_uci_cmd(uci_cmd: &str) -> UciProcessResult {
    let mut cmd_seq: Vec<&str> = uci_cmd.split(' ').collect();
    match cmd_seq[0] {
        "uci" => {
            println!("id name {} {}", def::ENGINE_NAME, def::VERSION);
            println!("id author {}", def::AUTHOR);
            println!("option name Hash type spin default {} min {} max {}", def::DEFAULT_HASH_SIZE_MB, def::MIN_HASH_SIZE_MB, def::MAX_HASH_SIZE_MB);
            println!("uciok");
            io::stdout().flush().ok();
            UciProcessResult::Noop
        }
        "debug" => UciProcessResult::Noop,
        "isready" => {
            println!("readyok");
            io::stdout().flush().ok();
            UciProcessResult::Noop
        }
        "setoption" => {
            match cmd_seq[2] {
                "Hash" => {
                    let hash_size_mb = cmd_seq[4].parse::<usize>().unwrap();
                    let hash_ratio = hash_size_mb / def::MIN_HASH_SIZE_MB;

                    if hash_ratio == 0 || hash_ratio & (hash_ratio - 1) != 0 {
                        println!("hash size {} is not supported", hash_size_mb);
                        return UciProcessResult::IgnoredOption
                    }

                    UciProcessResult::SetHashSize(hash_ratio * def::MIN_HASH_SIZE_UNIT)
                },
                _ => UciProcessResult::IgnoredOption,
            }
        },
        "register" => UciProcessResult::Noop,
        "ucinewgame" => UciProcessResult::Reset,
        "position" => match cmd_seq[1] {
            "startpos" => {
                if cmd_seq.len() > 3 {
                    process_position_with_mov_list(FEN_START_POS, cmd_seq.split_off(3))
                } else {
                    process_position(FEN_START_POS)
                }
            },
            "fen" => {
                let len = cmd_seq.len();
                let mut fen_str_vec = vec![""; len-2];
                fen_str_vec.copy_from_slice(&cmd_seq[2..len]);
                process_position(&fen_str_vec.join(" "))
            },
            _ => {
                eprintln!("only support fen or startpos with mov list");
                UciProcessResult::Noop
            }
        },
        "go" => process_go_cmd(cmd_seq.split_off(0)),
        "perft" => process_perft_cmd(cmd_seq[1]),
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
        "wtime" => process_time_control(go_cmd_seq),
        "movetime" => UciProcessResult::StartSearchWithTime(go_cmd_seq[2].parse::<u128>().unwrap()),
        "depth" => UciProcessResult::StartSearchToDepth(go_cmd_seq[2].parse::<u8>().unwrap()),
        "infinite" => UciProcessResult::StartSearchInfinite,
        "ponder" => UciProcessResult::Noop,
        sub_cmd => panic!("unsupported sub command {}", sub_cmd),
    }
}

fn process_time_control(go_cmd_seq: Vec<&str>) -> UciProcessResult {
    assert!(go_cmd_seq[1] == "wtime");
    let wtime = go_cmd_seq[2].parse::<u128>().unwrap();

    assert!(go_cmd_seq[3] == "btime");
    let btime = go_cmd_seq[4].parse::<u128>().unwrap();

    let movs_to_go;
    let mut winc = 0;
    let mut binc = 0;

    if go_cmd_seq.len() > 5 && go_cmd_seq[5] == "movestogo" {
        movs_to_go = go_cmd_seq[6].parse::<u128>().unwrap();
    } else if go_cmd_seq.len() > 9 && go_cmd_seq[9] == "movestogo" {
        if go_cmd_seq[5] == "winc" {
            winc = go_cmd_seq[6].parse::<u128>().unwrap();
        }

        if go_cmd_seq[7] == "binc" {
            binc = go_cmd_seq[8].parse::<u128>().unwrap()
        }

        movs_to_go = go_cmd_seq[10].parse::<u128>().unwrap();
    } else if go_cmd_seq.len() > 5 {
        if go_cmd_seq[5] == "winc" {
            winc = go_cmd_seq[6].parse::<u128>().unwrap();
        }

        if go_cmd_seq[7] == "binc" {
            binc = go_cmd_seq[8].parse::<u128>().unwrap()
        }

        movs_to_go = DEFAULT_MOVS_TO_GO;
    } else {
        movs_to_go = DEFAULT_MOVS_TO_GO;
    };

    let wtime = ((wtime + movs_to_go * winc) / movs_to_go).min(wtime);
    let btime = ((btime + movs_to_go * binc) / movs_to_go).min(btime);

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
        white_millis: wtime,
        black_millis: btime,
    })
}

fn process_position(fen_str: &str) -> UciProcessResult {
    UciProcessResult::Position(fen_str.to_owned(), vec![])
}

fn process_position_with_mov_list(fen_str: &str, mov_str_list: Vec<&str>) -> UciProcessResult {
    let mut mov_list = Vec::new();
    for mov_str in mov_str_list {
        mov_list.push(parse_mov_str(mov_str));
    }

    UciProcessResult::Position(fen_str.to_owned(), mov_list)
}

fn process_perft_cmd(depth: &str) -> UciProcessResult {
    UciProcessResult::Perft(depth.parse::<u8>().unwrap())
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