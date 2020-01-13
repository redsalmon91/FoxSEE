mod def;
mod eval;
mod mov_gen;
mod mov_tbl;
mod search;
mod state;
mod uci;
mod util;

use state::State;
use search::SearchEngine;
use uci::{UciProcessResult, Rawmov};

use std::io::{self, prelude::*};

const FEN_START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const OVERHEAD_TIME: u128 = 50;

fn main() {
    let mut search_engine = SearchEngine::new();
    let mut state = State::new(FEN_START_POS);

    loop {
        let input_cmd = read_gui_input();
        let uci_cmd_process_result = uci::process_uci_cmd(input_cmd.trim());
        match uci_cmd_process_result {
            UciProcessResult::Position(mov_list) => {
                state = State::new(FEN_START_POS);

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
                        }

                        if (from as isize - to as isize).abs() == 32 {
                            state.do_mov(from, to, def::MOV_CR_ENP, 0);
                            continue
                        }
                    }

                    state.do_mov(from, to, def::MOV_REG, 0);
                }
            },
            UciProcessResult::StartSearchWithTime(time_millis) => {
                let best_mov = search_engine.search(&mut state, time_millis - OVERHEAD_TIME);
                println!("bestmove {}", util::format_mov(best_mov));
                io::stdout().flush().ok();

                let (from, to, tp, promo) = util::decode_u32_mov(best_mov);
                state.do_mov(from, to, tp, promo);
            },
            UciProcessResult::StartSearchWithComplextTimeControl(time_info) => {
                let time_millis = if state.player == def::PLAYER_W {
                    time_info.white_millis
                } else {
                    time_info.black_millis
                };

                let best_mov = search_engine.search(&mut state, time_millis - OVERHEAD_TIME);
                println!("bestmove {}", util::format_mov(best_mov));
                io::stdout().flush().ok();

                let (from, to, tp, promo) = util::decode_u32_mov(best_mov);
                state.do_mov(from, to, tp, promo);
            },
            UciProcessResult::Ready => {},
            UciProcessResult::Stop => {},
            UciProcessResult::Noop => {},
            UciProcessResult::Reset => {},
            UciProcessResult::Quit => {
                println!("quit");
                std::process::exit(0);
            }
        }
    }
}

fn read_gui_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(error) => panic!("uable to read input {}", error),
    }
}
