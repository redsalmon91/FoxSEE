/*
 * Copyright (C) 2020 Zixiao Han
 */

use crate::{
    def,
    util,
};

use std::io::{self, prelude::*};

const DEFAULT_MOVS_TO_GO: u128 = 50;
const DEFAULT_MOVS_TO_GO_NO_INCREMENT: u128 = 60;

pub const FEN_START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug)]
pub struct Rawmov {
    pub from: usize,
    pub to: usize,
    pub promo: String,
    pub origin_mov_str: String,
}

#[derive(Debug)]
pub struct TimeInfo {
    pub all_time_millis: u128,
    pub moves_to_go: u128,
    pub increment_millis: u128,
}

#[derive(Debug)]
pub enum UciCommand {
    Noop,
    Reset,
    IgnoredOption,
    Perft(u8),
    SetHashSize(usize),
    Position(String, Vec<Rawmov>),
    PrintDebugInfo,
    StartSearchWithTime(u128),
    StartSearchWithComplextTimeControl((TimeInfo, TimeInfo)),
    StartSearchToDepth(u8),
    StartSearchInfinite,
}

pub fn process_uci_cmd(uci_cmd: &str) -> UciCommand {
    let mut cmd_seq: Vec<&str> = uci_cmd.split(' ').collect();
    match cmd_seq[0] {
        "uci" => {
            println!("id name {} {}", def::ENGINE_NAME, def::VERSION);
            println!("id author {}", def::AUTHOR);
            println!("option name Hash type spin default {} min {} max {}", def::DEFAULT_HASH_SIZE_MB, def::MIN_HASH_SIZE_MB, def::MAX_HASH_SIZE_MB);
            println!("uciok");
            io::stdout().flush().ok();
            UciCommand::Noop
        },
        "printdebug" => UciCommand::PrintDebugInfo,
        "isready" => {
            println!("readyok");
            io::stdout().flush().ok();
            UciCommand::Noop
        },
        "setoption" => {
            match cmd_seq[2] {
                "Hash" => {
                    let hash_size_mb = cmd_seq[4].parse::<usize>().unwrap();
                    let hash_ratio = hash_size_mb / def::MIN_HASH_SIZE_MB;

                    if hash_ratio == 0 || hash_ratio & (hash_ratio - 1) != 0 {
                        println!("hash size {} is not supported", hash_size_mb);
                        return UciCommand::IgnoredOption
                    }

                    UciCommand::SetHashSize(hash_ratio * def::MIN_HASH_SIZE_UNIT)
                },
                _ => UciCommand::IgnoredOption,
            }
        },
        "register" => UciCommand::Noop,
        "ucinewgame" => UciCommand::Reset,
        "position" => match cmd_seq[1] {
            "startpos" => {
                if cmd_seq.len() > 3 {
                    process_position_with_mov_list(FEN_START_POS, cmd_seq.split_off(3))
                } else {
                    process_position(FEN_START_POS)
                }
            },
            "fen" => {
                if cmd_seq.contains(&"moves") {
                    let mut fen_str_vec = Vec::new();

                    let mut token_index = 2;
                    loop {
                        let next_token = cmd_seq[token_index];

                        if next_token == "moves" {
                            break;
                        }

                        fen_str_vec.push(next_token);
                        token_index += 1;
                    }

                    process_position_with_mov_list(&fen_str_vec.join(" "), cmd_seq.split_off(token_index+1))
                } else {
                    let len = cmd_seq.len();
                    let mut fen_str_vec = vec![""; len-2];
                    fen_str_vec.copy_from_slice(&cmd_seq[2..len]);
                    process_position(&fen_str_vec.join(" "))
                }
            },
            _ => UciCommand::Noop
        },
        "go" => process_go_cmd(&mut cmd_seq.split_off(0)),
        "perft" => UciCommand::Perft(cmd_seq[1].parse::<u8>().unwrap()),
        "ponderhit" => UciCommand::Noop,
        _ => {
            eprintln!("unknown uci command {}", cmd_seq[0]);
            UciCommand::Noop
        }
    }
}

fn process_go_cmd(go_cmd_seq: &mut Vec<&str>) -> UciCommand {
    match go_cmd_seq[1] {
        "movestogo" => process_time_control(go_cmd_seq.split_off(1)),
        "wtime" => process_time_control(go_cmd_seq.split_off(1)),
        "btime" => process_time_control(go_cmd_seq.split_off(1)),
        "movetime" => UciCommand::StartSearchWithTime(go_cmd_seq[2].parse::<u128>().unwrap()),
        "depth" => UciCommand::StartSearchToDepth(go_cmd_seq[2].parse::<u8>().unwrap()),
        "infinite" => UciCommand::StartSearchInfinite,
        "ponder" => UciCommand::Noop,
        sub_cmd => panic!("unsupported sub command {}", sub_cmd),
    }
}

fn process_time_control(go_cmd_seq: Vec<&str>) -> UciCommand {
    let mut wtime = 0;
    let mut btime = 0;
    let mut winc = 0;
    let mut binc = 0;
    let mut movestogo = 0;

    let mut tc_cmd_seq = go_cmd_seq.into_iter();

    loop {
        let cmd_key = tc_cmd_seq.next();

        if cmd_key.is_none() {
            break;
        }

        let cmd_key = cmd_key.unwrap();

        match cmd_key {
            "wtime" => {
                wtime = tc_cmd_seq.next().unwrap().parse::<u128>().unwrap();
            },
            "btime" => {
                btime = tc_cmd_seq.next().unwrap().parse::<u128>().unwrap();
            },
            "winc" => {
                winc = tc_cmd_seq.next().unwrap().parse::<u128>().unwrap();
            },
            "binc" => {
                binc = tc_cmd_seq.next().unwrap().parse::<u128>().unwrap();
            },
            "movestogo" => {
                movestogo = tc_cmd_seq.next().unwrap().parse::<u128>().unwrap();
            },
            _ => {}
        }
    }

    if movestogo == 0 {
        if winc == 0 && binc == 0 {
            movestogo = DEFAULT_MOVS_TO_GO_NO_INCREMENT;
        } else {
            movestogo = DEFAULT_MOVS_TO_GO;
        }
    }

    UciCommand::StartSearchWithComplextTimeControl((
        TimeInfo{
            all_time_millis: wtime,
            moves_to_go: movestogo,
            increment_millis: winc,
        },
        TimeInfo{
            all_time_millis: btime,
            moves_to_go: movestogo,
            increment_millis: binc,
        }
    ))
}

fn process_position(fen_str: &str) -> UciCommand {
    UciCommand::Position(fen_str.to_owned(), vec![])
}

fn process_position_with_mov_list(fen_str: &str, mov_str_list: Vec<&str>) -> UciCommand {
    let mut mov_list = Vec::new();
    for mov_str in mov_str_list {
        mov_list.push(parse_mov_str(mov_str));
    }

    UciCommand::Position(fen_str.to_owned(), mov_list)
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
