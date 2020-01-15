use crate::{
    def,
    eval,
    mov_gen::MoveGenerator,
    state::State,
    util,
};

use std::u64;

const NULL_MOV_MIN_DEPTH: u8 = 5;
const NULL_MOV_REDUCTION: u8 = 3;
const PV_TRACK_LENGTH: usize = 16;
const REFUTATION_TABLE_SIZE: usize = 128;
const MAX_HISTORY_SCORE: u64 = u64::MAX;

const WINDOW_SIZE: i32 = 10;
const MIN_BRANCHING_FACTOR: u64 = 2;
const MIN_REDUCTION_DEPTH: u8 = 3;

pub enum SearchMovResult {
    Beta(i32),
    Alpha(i32),
    Noop,
}

use SearchMovResult::*;
use std::time::Instant;

pub struct SearchEngine {
    mov_generator: MoveGenerator,
    history_table: [[u64; def::BOARD_SIZE]; def::BOARD_SIZE],
    refutation_table: [(i32, u32); REFUTATION_TABLE_SIZE],
    master_pv_table: Vec<u32>,
    time_tracker: Instant,

    abort: bool,
    max_time_millis: u128,
}

impl SearchEngine {
    pub fn new() -> SearchEngine {
        SearchEngine {
            mov_generator: MoveGenerator::new(),
            history_table: [[0; def::BOARD_SIZE]; def::BOARD_SIZE],
            refutation_table: [(0, 0); REFUTATION_TABLE_SIZE],
            master_pv_table: Vec::new(),
            time_tracker: Instant::now(),

            abort: false,
            max_time_millis: 0,
        }
    }

    pub fn search(&mut self, state: &mut State, max_time_millis: u128) -> u32 {
        self.time_tracker = Instant::now();
        self.max_time_millis = max_time_millis;
        self.abort = false;
        self.history_table = [[0; def::BOARD_SIZE]; def::BOARD_SIZE];
        self.refutation_table = [(0, 0); REFUTATION_TABLE_SIZE];
        self.master_pv_table = Vec::new();

        let player_sign = if state.player == def::PLAYER_W {
            1
        } else {
            -1
        };

        let mut beta = player_sign * eval::K_VAL;
        let mut alpha = -beta;

        let mut depth = 1;
        let mut best_mov = 0;
        let mut previous_node_count = 1;
        let mut time_after_previous_iter = self.time_tracker.elapsed().as_millis();

        loop {
            let mut node_count = 0;
            let mut seldepth = 0;

            let mut pv_table = [0; PV_TRACK_LENGTH];
            let score = self.ab_search(state, true, &mut pv_table, alpha, beta, depth, false, 0, 0, &mut node_count, &mut seldepth);

            if self.abort {
                break
            }

            if score * player_sign <= alpha * player_sign {
                alpha = -eval::K_VAL * player_sign;
                continue
            }

            if score * player_sign >= beta * player_sign {
                beta = eval::K_VAL * player_sign;
                continue
            }
    
            best_mov = pv_table[0];
            let time_taken_millis = self.time_tracker.elapsed().as_millis();
            let nps = (time_taken_millis / 1000).max(1) / node_count as u128;

            if score * player_sign > eval::TERM_VAL {
                println!("info score mate {} depth {} seldepth {} nodes {} nps {} time {} pv {}",
                    (eval::K_VAL - score.abs() + 1) / 2, depth, seldepth, node_count, nps, time_taken_millis, util::format_pv(&pv_table));
                break
            }

            println!("info score cp {} depth {} seldepth {} nodes {} nps {} time {} pv {}",
                score * player_sign, depth, seldepth, node_count, nps, time_taken_millis, util::format_pv(&pv_table));

            depth += 1;

            alpha = score - player_sign * WINDOW_SIZE;
            beta = score + player_sign * WINDOW_SIZE;

            let current_time_millis = self.time_tracker.elapsed().as_millis();
            let estimated_time_for_next_iter = (node_count / previous_node_count).max(MIN_BRANCHING_FACTOR) as u128 * (current_time_millis - time_after_previous_iter);

            if current_time_millis + estimated_time_for_next_iter > max_time_millis {
                break
            }

            previous_node_count = node_count;
            time_after_previous_iter = current_time_millis;

            self.master_pv_table = vec![0; pv_table.len()];
            self.master_pv_table.copy_from_slice(&pv_table);
        }

        best_mov
    }

    pub fn ab_search(&mut self, state: &mut State, on_pv: bool, pv_table: &mut [u32], mut alpha: i32, beta: i32, mut depth: u8, depth_reduced: bool, mut depth_extend_count: u8, ply: u8, node_count: &mut u64, seldepth: &mut u8) -> i32 {
        if self.abort {
            return 0
        }

        if self.time_tracker.elapsed().as_millis() > self.max_time_millis {
            self.abort = true;
            return 0
        }

        *node_count += 1;

        if ply > 0 && state.is_draw() {
            return 0
        }

        let player_sign = if state.player == def::PLAYER_W {
            1
        } else {
            -1
        };

        let in_check = self.mov_generator.is_in_check(state);

        if in_check && (ply < 2 || depth_extend_count * 2 < ply) {
            depth += 1;
            depth_extend_count += 1;
        }

        if depth == 0 {
            return self.q_search(state, alpha, beta, ply, seldepth)
        }

        let mut pv_mov = 0;
        if on_pv && self.master_pv_table.len() > ply as usize {
            pv_mov = self.master_pv_table[ply as usize];

            if pv_mov != 0 {
                match self.search_mov(state, false, pv_table, pv_mov, true, alpha, beta, depth, depth_reduced, depth_extend_count, ply, player_sign, node_count, seldepth) {
                    Beta(score) => return score,
                    Alpha(score) => {
                        alpha = score;
                    },
                    Noop => (),
                }
            }
        }

        if ply > 0 && !on_pv && !in_check && (!depth_reduced || depth_extend_count > 1) && depth >= NULL_MOV_MIN_DEPTH {
            let mut next_pv_table = [0; PV_TRACK_LENGTH];

            state.do_null_mov();
            let score = self.ab_search(state, false, &mut next_pv_table, beta, beta - player_sign, depth - NULL_MOV_REDUCTION - 1, true, depth_extend_count, ply + 1, node_count, seldepth);
            state.undo_null_mov();

            if score * player_sign >= beta * player_sign {
                return beta
            }
        }

        let (cap_list, non_cap_list) = self.mov_generator.gen_reg_mov_list(state, false);

        let mut scored_capture_list = Vec::new();
        let squares = state.squares;

        for cap in cap_list {
            if cap == pv_mov {
                continue
            }

            let (from, to, _tp, promo) = util::decode_u32_mov(cap);

            let exchange_score = eval::val_of(squares[to]) - eval::val_of(squares[from]) + eval::val_of(promo);

            if exchange_score > eval::EQUAL_EXCHANGE_VAL || depth == 1 {
                scored_capture_list.push((exchange_score, cap));
            } else {
                let see_score = self.see(state, to, squares[from]) * player_sign + eval::val_of(promo);
                scored_capture_list.push((see_score, cap));
            }
        }

        scored_capture_list.sort_by(|(score_a, _), (score_b, _)| {
            score_b.partial_cmp(&score_a).unwrap()
        });

        for (score, cap) in scored_capture_list {
            let (depth, depth_reduced) = if score < eval::LOSING_EXCHANGE_VAL && !on_pv && !in_check && depth >= MIN_REDUCTION_DEPTH {
                (depth -1, true)
            } else {
                (depth, depth_reduced)
            };

            match self.search_mov(state, false, pv_table, cap, true, alpha, beta, depth, depth_reduced, depth_extend_count, ply, player_sign, node_count, seldepth) {
                Beta(score) => return score,
                Alpha(score) => {
                    alpha = score;
                },
                Noop => (),
            }
        }

        let mut refutation_mov = 0;
        if ply > 0 && !depth_reduced {
            let (_refutation_score, saved_refutation_mov) = self.refutation_table[ply as usize];
            refutation_mov = saved_refutation_mov;

            if refutation_mov != 0 && non_cap_list.contains(&refutation_mov) {
                match self.search_mov(state, false, pv_table, refutation_mov, false, alpha, beta, depth, depth_reduced, depth_extend_count, ply, player_sign, node_count, seldepth) {
                    Beta(score) => return score,
                    Alpha(score) => {
                        alpha = score;
                    },
                    Noop => (),
                }
            }
        }

        let mut scored_non_cap_list = Vec::new();

        for non_cap in non_cap_list {
            if non_cap == refutation_mov || non_cap == pv_mov {
                continue
            }

            let (from, to, _tp, promo) = util::decode_u32_mov(non_cap);
            if promo != 0 {
                scored_non_cap_list.push((MAX_HISTORY_SCORE, non_cap));
                continue
            }

            let history_score = self.history_table[from][to];
            scored_non_cap_list.push((history_score, non_cap));
        }

        scored_non_cap_list.sort_by(|(score_a, _), (score_b, _)| {
            score_b.partial_cmp(&score_a).unwrap()
        });

        for (score, non_cap) in scored_non_cap_list {
            let (depth, depth_reduced) = if score == 0 && !in_check && !on_pv && depth >= MIN_REDUCTION_DEPTH {
                (depth -1, true)
            } else {
                (depth, depth_reduced)
            };

            match self.search_mov(state, false, pv_table, non_cap, false, alpha, beta, depth, depth_reduced, depth_extend_count, ply, player_sign, node_count, seldepth) {
                Beta(score) => return score,
                Alpha(score) => {
                    alpha = score;
                },
                Noop => (),
            }
        }

        let castle_list = self.mov_generator.gen_castle_mov_list(state);
        for cas_mov in castle_list {
            if cas_mov == refutation_mov || cas_mov == pv_mov {
                continue
            }

            match self.search_mov(state, false, pv_table, cas_mov, false, alpha, beta, depth, depth_reduced, depth_extend_count, ply, player_sign, node_count, seldepth) {
                Beta(score) => return score,
                Alpha(score) => {
                    alpha = score;
                },
                Noop => (),
            }
        }

        let signed_score = alpha * player_sign;

        if signed_score < -eval::TERM_VAL {
            if ply > 0 && pv_table[1] == 0 {
                pv_table[0] = 0;
            }

            if !in_check && self.in_stale_mate(state) {
                return 0
            }
        }

        alpha
    }

    #[inline]
    fn search_mov(&mut self, state: &mut State, on_pv: bool, pv_table: &mut [u32], mov: u32, is_capture: bool, alpha: i32, beta: i32, depth: u8, depth_reduced: bool, depth_extend_count: u8, ply: u8, player_sign: i32, node_count: &mut u64, seldepth: &mut u8) -> SearchMovResult {
        let (from, to, tp, promo) = util::decode_u32_mov(mov);

        if is_capture {
            let captured_piece = state.squares[to];
            if def::is_k(captured_piece) {
                return Beta(player_sign * (eval::K_VAL - ply as i32))
            }
        }

        let mut next_pv_table = [0; PV_TRACK_LENGTH];

        state.do_mov(from, to, tp, promo);
        let score = self.ab_search(state, on_pv, &mut next_pv_table, beta, alpha, depth - 1, depth_reduced, depth_extend_count, ply + 1, node_count, seldepth);
        state.undo_mov(from, to, tp);

        let history_improvement = depth as u64;

        if score * player_sign >= beta * player_sign {
            if !is_capture && !depth_reduced {
                let (refutation_score, refutation_mov) = self.refutation_table[ply as usize];
                if refutation_mov == 0 || score * player_sign > refutation_score * player_sign {
                    self.refutation_table[ply as usize] = (score, mov);
                }
            }

            if !is_capture {
                self.history_table[from][to] = self.history_table[from][to] + history_improvement * history_improvement;
            }

            return Beta(score)
        }

        if score * player_sign > alpha * player_sign {
            pv_table[0] = mov;
            pv_table[1..PV_TRACK_LENGTH].copy_from_slice(&next_pv_table[0..PV_TRACK_LENGTH-1]);

            if !is_capture {
                self.history_table[from][to] = self.history_table[from][to] + history_improvement;
            }

            return Alpha(score)
        }

        Noop
    }

    #[inline]
    fn in_stale_mate(&self, state: &mut State) -> bool {
        let (cap_list, non_cap_list) = self.mov_generator.gen_reg_mov_list(state, false);

        for cap in cap_list {
            let (from, to, tp, promo) = util::decode_u32_mov(cap);
            state.do_mov(from, to, tp, promo);
            state.player = def::get_opposite_player(state.player);

            if !self.mov_generator.is_in_check(state) {
                state.player = def::get_opposite_player(state.player);
                state.undo_mov(from, to, tp);
                return false
            }

            state.player = def::get_opposite_player(state.player);
            state.undo_mov(from, to, tp);
        }

        for non_cap in non_cap_list {
            let (from, to, tp, promo) = util::decode_u32_mov(non_cap);
            state.do_mov(from, to, tp, promo);
            state.player = def::get_opposite_player(state.player);

            if !self.mov_generator.is_in_check(state) {
                state.player = def::get_opposite_player(state.player);
                state.undo_mov(from, to, tp);
                return false
            }

            state.player = def::get_opposite_player(state.player);
            state.undo_mov(from, to, tp);
        }

        true
    }

    pub fn q_search(&self, state: &mut State, mut alpha: i32, beta: i32, ply: u8, seldepth: &mut u8) -> i32 {
        if ply > *seldepth {
            *seldepth = ply;
        }

        let player_sign = if state.player == def::PLAYER_W {
            1
        } else {
            -1
        };

        let mut score = eval::eval_state(state);

        if score > eval::ADVANCE_VAL {
            score -= state.non_cap_mov_count.min(50) as i32;
        } else if score < -eval::ADVANCE_VAL {
            score += state.non_cap_mov_count.min(50) as i32;
        }

        if score * player_sign >= beta * player_sign {
            return score
        }

        if score * player_sign > alpha * player_sign {
            alpha = score;
        }

        let (cap_list, _) = self.mov_generator.gen_reg_mov_list(state, true);

        if cap_list.is_empty() {
            return score
        }

        let squares = state.squares;
        let mut scored_cap_list = Vec::new();

        for cap in cap_list {
            let (from, to, _tp, promo) = util::decode_u32_mov(cap);

            let exchange_score = eval::val_of(squares[to]) - eval::val_of(squares[from]) + eval::val_of(promo);
            scored_cap_list.push((exchange_score, cap));
        }

        scored_cap_list.sort_by(|(score_a, _), (score_b, _)| {
            score_b.partial_cmp(&score_a).unwrap()
        });

        for (_score, cap) in scored_cap_list {
            let (from, to, tp, promo) = util::decode_u32_mov(cap);

            let captured_piece = state.squares[to];
            if def::is_k(captured_piece) {
                return player_sign * (eval::K_VAL - ply as i32)
            }

            state.do_mov(from, to, tp, promo);
            let score = self.q_search(state, beta, alpha, ply + 1, seldepth);
            state.undo_mov(from, to, tp);

            if score * player_sign >= beta * player_sign {
                return score
            }

            if score * player_sign > alpha * player_sign {
                alpha = score;
            }
        }

        alpha
    }

    pub fn see(&self, state: &State, index: usize, initial_attacker: u8) -> i32 {
        let (generated_w_attacker_list, generated_b_attacker_list) =
            self.mov_generator.find_attacker_list(state, index);

        let player_sign = if state.player == def::PLAYER_W {
            1
        } else {
            -1
        };

        let w_attacker_list = if player_sign > 0 {
            let mut attacker_list = Vec::new();
            let mut found_init_attacker = false;
            for attacker in &generated_w_attacker_list {
                let attacker = *attacker;
                if attacker == initial_attacker {
                    found_init_attacker = true;
                    continue
                }
    
                if found_init_attacker || attacker != initial_attacker {
                    attacker_list.push(attacker);
                }
            }

            attacker_list
        } else {
            generated_w_attacker_list
        };

        let b_attacker_list = if player_sign < 0 {
            let mut attacker_list = Vec::new();
            let mut found_init_attacker = false;
            for attacker in &generated_b_attacker_list {
                let attacker = *attacker;
                if attacker == initial_attacker {
                    found_init_attacker = true;
                    continue
                }

                if found_init_attacker || attacker != initial_attacker {
                    attacker_list.push(attacker);
                }
            }

            attacker_list
        } else {
            generated_b_attacker_list
        };

        eval::val_of(state.squares[index]) * player_sign + self.simulate_exchange(player_sign * -1, w_attacker_list, b_attacker_list, 0, 0, initial_attacker)
    }

    fn simulate_exchange(&self, player_sign: i32, w_attacker_list: Vec<u8>, b_attacker_list: Vec<u8>, mut w_attacker_index: usize, mut b_attacker_index: usize, last_attacker: u8) -> i32 {
        if (player_sign > 0 && w_attacker_index == w_attacker_list.len()) || (player_sign < 0 && b_attacker_index == b_attacker_list.len()) {
            return 0
        }

        let next_attacker;
        
        if player_sign > 0 {
            next_attacker = w_attacker_list[w_attacker_index];
            w_attacker_index += 1;
        } else {
            next_attacker = b_attacker_list[b_attacker_index];
            b_attacker_index += 1;
        };

        let sim_score =  player_sign * eval::val_of(last_attacker)
            + self.simulate_exchange(player_sign * -1, w_attacker_list, b_attacker_list, w_attacker_index, b_attacker_index, next_attacker);

        if sim_score * player_sign > 0 {
            sim_score
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        state::State,
        util,
    };

    #[test]
    fn test_see_1() {
        let state = State::new("4q1kr/ppn1rp1p/n1p1PB2/5P2/2B1Q2P/2N3p1/PPP1b1P1/4R2K b - - 1 1");
        let search_engine = SearchEngine::new();

        assert_eq!(120, search_engine.see(&state, util::map_sqr_notation_to_index("e6"), def::BN));
        assert_eq!(-100, search_engine.see(&state, util::map_sqr_notation_to_index("e6"), def::BP));
        assert_eq!(300, search_engine.see(&state, util::map_sqr_notation_to_index("e6"), def::BR));
    }

    #[test]
    fn test_see_2() {
        let state = State::new("r5kr/1b1pR1p1/ppq1N2p/5P1n/3Q4/B6B/P5PP/5RK1 w - - 1 1");
        let search_engine = SearchEngine::new();

        assert_eq!(-530, search_engine.see(&state, util::map_sqr_notation_to_index("g7"), def::WQ));
        assert_eq!(100, search_engine.see(&state, util::map_sqr_notation_to_index("g7"), def::WN));
        assert_eq!(-80, search_engine.see(&state, util::map_sqr_notation_to_index("g7"), def::WR));
    }

    #[test]
    fn test_see_3() {
        let state = State::new("r2q1kn1/p2b1rb1/1p1p1pp1/2pPp3/1PP1Pn2/PRNBB1K1/3QNPPP/5R2 w - - 0 1");
        let search_engine = SearchEngine::new();

        assert_eq!(100, search_engine.see(&state, util::map_sqr_notation_to_index("f4"), def::WN));
        assert_eq!(90, search_engine.see(&state, util::map_sqr_notation_to_index("f4"), def::WB));
        assert_eq!(-19580, search_engine.see(&state, util::map_sqr_notation_to_index("f4"), def::WK));
    }

    #[test]
    fn test_see_4() {
        let state = State::new("r4kn1/p2bprb1/Bp1p1ppP/2pP4/1PP1Pn2/PRNB2K1/2QN1PPq/5R2 w - - 0 1");
        let search_engine = SearchEngine::new();

        assert_eq!(-19680, search_engine.see(&state, util::map_sqr_notation_to_index("f4"), def::WK));
    }

    #[test]
    fn test_see_5() {
        let state = State::new("rn1qkbnr/pppbpppp/8/3p4/4P3/5Q2/PPPP1PPP/RNB1KBNR w KQkq - 2 3");
        let search_engine = SearchEngine::new();

        assert_eq!(100, search_engine.see(&state, util::map_sqr_notation_to_index("d5"), def::WP));
    }

    #[test]
    fn test_q_search_1() {
        let mut state = State::new("r5kr/1b1pR1p1/p1q1N2p/5P1n/3Q4/B7/P5PP/5RK1 w - - 1 1");
        let search_engine = SearchEngine::new();

        assert_eq!(70, search_engine.q_search(&mut state, -20000, 20000, 0, &mut 0));
    }

    #[test]
    fn test_q_search_2() {
        let mut state = State::new("2k2r2/pp2br2/1np1p2q/2NpP2p/2PP2p1/1P1N4/P3Q1PP/3R1R1K b - - 8 27");
        let search_engine = SearchEngine::new();

        assert_eq!(15, search_engine.q_search(&mut state, 20000, -20000, 0, &mut 0));
    }

    #[test]
    fn test_q_search_3() {
        let mut state = State::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let search_engine = SearchEngine::new();

        assert_eq!(0, search_engine.q_search(&mut state, -20000, 20000, 0, &mut 0));
    }

    #[test]
    fn test_q_search_4() {
        let mut state = State::new("2k5/pp2b3/1np1p3/2NpP2p/3P2p1/2PN4/PP4PP/5q1K w - - 8 27");
        let search_engine = SearchEngine::new();

        assert_eq!(-985, search_engine.q_search(&mut state, -20000, 20000, 0, &mut 0));
    }

    #[test]
    fn test_search_puzzle_1() {
        let mut state = State::new("2k2r2/pp2br2/1np1p2q/2NpP2p/2PP2p1/1P1N4/P3Q1PP/3R1R1K b - - 8 27");
        let mut search_engine = SearchEngine::new();

        let best_mov = search_engine.search(&mut state, 5500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("h6"));
        assert_eq!(to, util::map_sqr_notation_to_index("d2"));
    }

    #[test]
    fn test_search_puzzle_2() {
        let mut state = State::new("r3r1k1/ppqb1ppp/8/4p1NQ/8/2P5/PP3PPP/R3R1K1 b - - 0 1");
        let mut search_engine = SearchEngine::new();

        let best_mov = search_engine.search(&mut state, 5500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("d7"));
        assert_eq!(to, util::map_sqr_notation_to_index("f5"));
    }

    #[test]
    fn test_search_puzzle_3() {
        let mut state = State::new("3rr3/2pq2pk/p2p1pnp/8/2QBPP2/1P6/P5PP/4RRK1 b - - 0 1");
        let mut search_engine = SearchEngine::new();

        let best_mov = search_engine.search(&mut state, 15500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("e8"));
        assert_eq!(to, util::map_sqr_notation_to_index("e4"));
    }

    #[test]
    fn test_search_puzzle_4() {
        let mut state = State::new("8/8/1r2b2p/8/8/2p5/2kR4/K7 b - - 3 56");
        let mut search_engine = SearchEngine::new();

        let best_mov = search_engine.search(&mut state, 5500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("c2"));
        assert_eq!(to, util::map_sqr_notation_to_index("b3"));
    }

    #[test]
    fn test_search_puzzle_5() {
        let mut state = State::new("4r1k1/pp1Q1ppp/3B4/q2p4/5P1P/P3PbPK/1P1r4/2R5 b - - 3 5");
        let mut search_engine = SearchEngine::new();

        let best_mov = search_engine.search(&mut state, 5500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("d2"));
        assert_eq!(to, util::map_sqr_notation_to_index("h2"));
    }

    #[test]
    fn test_search_puzzle_6() {
        let mut state = State::new("r5rk/2p1Nppp/3p3P/pp2p1P1/4P3/2qnPQK1/8/R6R w - - 1 0");
        let mut search_engine = SearchEngine::new();

        let best_mov = search_engine.search(&mut state, 10000);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("h6"));
        assert_eq!(to, util::map_sqr_notation_to_index("g7"));
    }

    #[test]
    fn test_search_puzzle_7() {
        let mut state = State::new("r1b3kr/3pR1p1/ppq4p/5P2/4Q3/B7/P5PP/5RK1 w - - 1 0");
        let mut search_engine = SearchEngine::new();

        let best_mov = search_engine.search(&mut state, 5500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("e7"));
        assert_eq!(to, util::map_sqr_notation_to_index("g7"));
    }

    #[test]
    fn test_search_puzzle_8() {
        let mut state = State::new("1r2k1r1/pbppnp1p/1b3P2/8/Q7/B1PB1q2/P4PPP/3R2K1 w - - 1 0");
        let mut search_engine = SearchEngine::new();

        let best_mov = search_engine.search(&mut state, 5500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("a4"));
        assert_eq!(to, util::map_sqr_notation_to_index("d7"));
    }

    #[test]
    fn test_search_puzzle_9() {
        let mut state = State::new("8/8/8/5p1p/3k1P1P/5K2/8/8 b - - 1 59");
        let mut search_engine = SearchEngine::new();

        let best_mov = search_engine.search(&mut state, 5500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("d4"));
        assert_eq!(to, util::map_sqr_notation_to_index("d3"));
    }

    #[test]
    fn test_search_endgame_1() {
        let mut state = State::new("8/2k5/2pR4/1pPp4/p7/P1P2P2/1P6/5K2 w - - 5 52");
        let mut search_engine = SearchEngine::new();

        let best_mov = search_engine.search(&mut state, 5500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("f3"));
        assert_eq!(to, util::map_sqr_notation_to_index("f4"));
    }

    #[test]
    fn test_search_endgame_2() {
        let mut state = State::new("7r/8/7P/4k3/4B3/4K2R/8/8 w - - 7 76");
        let mut search_engine = SearchEngine::new();

        let best_mov = search_engine.search(&mut state, 5500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("h3"));
        assert_eq!(to, util::map_sqr_notation_to_index("h5"));
    }

    #[test]
    fn test_search_endgame_3() {
        let mut state = State::new("8/1K6/8/3k4/8/8/5p2/8 b - - 9 83");
        let mut search_engine = SearchEngine::new();

        let best_mov = search_engine.search(&mut state, 5500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("f2"));
        assert_eq!(to, util::map_sqr_notation_to_index("f1"));
    }
}
