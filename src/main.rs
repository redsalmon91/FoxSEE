/*
* Copyright (C) 2020 Zixiao Han
*/

mod bitboard;
mod bitmask;
mod def;
mod eval;
mod eval_params;
mod hashtable;
mod mov_table;
mod prng;
mod search;
mod search_params;
mod simple_rnd;
mod state;
mod time_control;
mod uci;
mod util;
mod zob_keys;

use prng::XorshiftPrng;
use state::State;
use search::SearchEngine;
use time_control::TimeCapacity;
use uci::{UciCommand, Rawmov};

use std::io::{self, prelude::*};
use std::thread;
use std::sync::mpsc;
use std::time;
use std::u128;
use std::env;

const DEFAULT_MAX_TIME: TimeCapacity = TimeCapacity {
    main_time_millis: u128::MAX,
    extra_time_millis: 0,
};

const DEFAULT_MAX_DEPTH: u8 = 128;

fn main() {
    if 1u8 != 0b01 {
        println!("only litte-endian systems are supported");
        std::process::exit(0);
    }

    zob_keys::init();
    bitmask::init();

    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let mut search_engine = SearchEngine::new(def::DEFAULT_HASH_SIZE_UNIT);

        let args: Vec<String> = env::args().collect();

        if args.len() > 1 {
            let params_file: String = args[1].parse().unwrap();
            search_engine.set_params(&params_file);
        }

        let mut state = State::new(uci::FEN_START_POS);

        loop {
            let command: String = receiver.recv().unwrap();
            let uci_cmd_process_result = uci::process_uci_cmd(command.trim());
            match uci_cmd_process_result {
                UciCommand::SetHashSize(hash_size) => {
                    search_engine.set_hash_size(hash_size);
                },
                UciCommand::Position(fen_str, mov_list) => {
                    state = State::new(&fen_str);

                    if mov_list.is_empty() {
                        continue
                    }

                    for Rawmov { from, to, promo, origin_mov_str } in mov_list {
                        if !promo.is_empty() {
                            let promo_piece_code = if state.player == def::PLAYER_W {
                                match promo.as_str() {
                                    "q" => def::WQ,
                                    "r" => def::WR,
                                    "b" => def::WB,
                                    "n" => def::WN,
                                    _ => panic!("invalid promo piece"),
                                }
                            } else {
                                match promo.as_str() {
                                    "q" => def::BQ,
                                    "r" => def::BR,
                                    "b" => def::BB,
                                    "n" => def::BN,
                                    _ => panic!("invalid promo piece"),
                                }
                            };

                            state.do_mov(from, to, def::MOV_PROMO, promo_piece_code);
                            continue
                        }

                        let moving_piece = state.squares[from];
                        if def::is_k(moving_piece) && ["e1g1", "e1c1", "e8g8", "e8c8"].contains(&origin_mov_str.as_str()) {
                            state.do_mov(from, to, def::MOV_CAS, 0);
                            continue
                        }

                        if def::is_p(moving_piece) {
                            if to == state.enp_square {
                                state.do_mov(from, to, def::MOV_ENP, 0);
                                continue
                            } else if (from as isize - to as isize).abs() == 16 {
                                state.do_mov(from, to, def::MOV_CR_ENP, 0);
                                continue
                            }
                        }

                        state.do_mov(from, to, def::MOV_REG, 0);
                    }
                },
                UciCommand::StartSearchWithTime(time_millis) => {
                    let best_mov = search_engine.search(&mut state, time_control::calculate_time_capacity(time_millis, 1, 0), DEFAULT_MAX_DEPTH);
                    print_best_mov(best_mov);
                },
                UciCommand::StartSearchWithComplextTimeControl((w_time_info, b_time_info)) => {
                    let time_capacity = if state.player == def::PLAYER_W {
                        time_control::calculate_time_capacity(w_time_info.all_time_millis, w_time_info.moves_to_go, w_time_info.increment_millis)
                    } else {
                        time_control::calculate_time_capacity(b_time_info.all_time_millis, b_time_info.moves_to_go, b_time_info.increment_millis)
                    };

                    let best_mov = search_engine.search(&mut state, time_capacity, DEFAULT_MAX_DEPTH);
                    print_best_mov(best_mov);
                },
                UciCommand::StartSearchToDepth(depth) => {
                    let best_mov = search_engine.search(&mut state, DEFAULT_MAX_TIME, depth);
                    print_best_mov(best_mov);
                },
                UciCommand::StartSearchInfinite => {
                    let best_mov = search_engine.search(&mut state, DEFAULT_MAX_TIME, DEFAULT_MAX_DEPTH);
                    print_best_mov(best_mov);
                },
                UciCommand::Perft(depth) => {
                    let start_time = time::Instant::now();
                    let perft_val = search_engine.perft(&mut state, depth);

                    println!("depth {} perft {} time {} milliseconds", depth, perft_val, start_time.elapsed().as_millis());
                },
                UciCommand::PrintDebugInfo => {
                    println!("{}", &state);
                },
                UciCommand::Reset => {
                    search_engine.reset();
                },
                UciCommand::IgnoredOption => {},
                UciCommand::Noop => {},
            }
        }
    });

    loop {
        let mut input = String::new();
        match io::stdin().lock().read_line(&mut input) {
            Ok(_) => {},
            Err(error) => panic!("uable to read input {}", error),
        }

        match input.trim() {
            "stop" => {
                unsafe {
                    search::ABORT_SEARCH = true;
                }
            },
            "quit" => {
                std::process::exit(0);
            },
            _ => {
                sender.send(input).unwrap();
            }
        }
    }
}

fn print_best_mov(best_mov: u32) {
    println!("bestmove {}", util::format_mov(best_mov));
    io::stdout().flush().ok();
}
