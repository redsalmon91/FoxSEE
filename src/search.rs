/*
 * Copyright (C) 2020 Zixiao Han
 */

use crate::{
    def,
    eval,
    hashtable::{AlwaysReplaceHashTable, DepthPreferredHashTable, LookupResult, HASH_TYPE_ALPHA, HASH_TYPE_BETA, HASH_TYPE_EXACT},
    mov_table,
    state::State,
    time_control::TimeCapacity,
    util,
};

const PV_TRACK_LENGTH: usize = 128;
const PV_PRINT_LENGTH: usize = 16;

const SORTING_CAP_BASE_VAL: i32 = 10000000;
const SORTING_HALF_PAWN_VAL: i32 = 50;
const SORTING_Q_VAL: i32 = 1000;

const WINDOW_SIZE: i32 = 10;
const EXTENDED_WINDOW_SIZE: i32 = 50;

const NM_DEPTH: u8 = 6;
const NM_R: u8 = 2;

const MAX_DEPTH: u8 = 128;

const IID_DEPTH: u8 = 7;
const IID_R: u8 = 2;

const DELTA_MARGIN: i32 = 200;

const TIME_CHECK_INTEVAL: u64 = 4095;

static mut NODE_COUNT: u64 = 0;
static mut SEL_DEPTH: u8 = 0;

pub static mut ABORT_SEARCH: bool = false;

use std::time::Instant;
use LookupResult::*;

pub struct SearchEngine {
    depth_preferred_hash_table: DepthPreferredHashTable,
    always_replace_hash_table: AlwaysReplaceHashTable,
    primary_killer_table: [u32; PV_TRACK_LENGTH],
    secondary_killer_table: [u32; PV_TRACK_LENGTH],
    history_table: [[[i32; def::BOARD_SIZE]; def::BOARD_SIZE]; 2],
    butterfly_table: [[[i32; def::BOARD_SIZE]; def::BOARD_SIZE]; 2],
    time_tracker: Instant,
    max_time_millis: u128,
}

impl SearchEngine {
    pub fn new(hash_size: usize) -> Self {
        SearchEngine {
            depth_preferred_hash_table: DepthPreferredHashTable::new(hash_size >> 1),
            always_replace_hash_table: AlwaysReplaceHashTable::new(hash_size >> 1),
            primary_killer_table: [0; PV_TRACK_LENGTH],
            secondary_killer_table: [0; PV_TRACK_LENGTH],
            history_table: [[[0; def::BOARD_SIZE]; def::BOARD_SIZE]; 2],
            butterfly_table: [[[1; def::BOARD_SIZE]; def::BOARD_SIZE]; 2],
            time_tracker: Instant::now(),
            max_time_millis: 0,
        }
    }

    pub fn reset(&mut self) {
        self.depth_preferred_hash_table.clear();
        self.always_replace_hash_table.clear();
    }

    pub fn set_hash_size(&mut self, hash_size: usize) {
        self.depth_preferred_hash_table = DepthPreferredHashTable::new(hash_size >> 1);
        self.always_replace_hash_table = AlwaysReplaceHashTable::new(hash_size >> 1);
    }

    pub fn perft(&self, state: &mut State, depth: u8) -> usize {
        if mov_table::is_in_check(state, def::get_opposite_player(state.player)) {
            return 0
        }

        if depth == 0 {
            return 1
        }

        let mut node_count = 0;

        let mut mov_list = [0; def::MAX_MOV_COUNT];

        mov_table::gen_reg_mov_list(state, &mut mov_list);

        for mov_index in 0..def::MAX_MOV_COUNT {
            let mov = mov_list[mov_index];

            if mov == 0 {
                break
            }

            let (from, to, tp, promo) = util::decode_u32_mov(mov);

            if def::is_k(state.squares[to]) {
                return 0
            }

            state.do_mov(from, to, tp, promo);
            node_count += self.perft(state, depth - 1);
            state.undo_mov(from, to, tp);
        }

        node_count
    }

    pub fn search(&mut self, state: &mut State, time_capacity: TimeCapacity, max_depth: u8) -> u32 {
        self.time_tracker = Instant::now();
        self.max_time_millis = time_capacity.main_time_millis;

        self.primary_killer_table = [0; PV_TRACK_LENGTH];
        self.secondary_killer_table = [0; PV_TRACK_LENGTH];
        self.history_table = [[[0; def::BOARD_SIZE]; def::BOARD_SIZE]; 2];
        self.butterfly_table = [[[1; def::BOARD_SIZE]; def::BOARD_SIZE]; 2];
        self.depth_preferred_hash_table.clear();

        unsafe {
            ABORT_SEARCH = false;
        }

        let in_check = mov_table::is_in_check(state, state.player);

        let mut alpha = -eval::MATE_VAL;
        let mut beta = eval::MATE_VAL;

        let mut depth = 1;
        let mut best_mov = 0;
        let mut accumulated_time_taken = 0;
        let mut window_extended = false;

        loop {
            unsafe {
                NODE_COUNT = 0;
                SEL_DEPTH = 0;
            }

            let score = self.ab_search(state, in_check, false, alpha, beta, depth, 0);

            unsafe {
                if ABORT_SEARCH {
                    break
                }
            }

            if score <= alpha {
                if !window_extended {
                    alpha = score - EXTENDED_WINDOW_SIZE;
                    window_extended = true;
                } else {
                    alpha = -eval::MATE_VAL;
                }

                continue
            }

            if score >= beta {
               if !window_extended {
                   beta = score + EXTENDED_WINDOW_SIZE;
                   window_extended = true;
               } else {
                   beta = eval::MATE_VAL;
               }

                continue
            }

            let mut pv_table = [0; PV_TRACK_LENGTH];
            self.retrieve_pv(state, &mut pv_table, 0);

            let checkmate = score.abs() > eval::TERM_VAL;

            let total_time_taken = self.time_tracker.elapsed().as_millis();

            if pv_table[0] != 0 {
                best_mov = pv_table[0];

                unsafe {
                    let iter_time_taken_millis = total_time_taken - accumulated_time_taken;
                    let nps = NODE_COUNT as u128 / (iter_time_taken_millis / 1000).max(1);

                    if checkmate {
                        let mate_score = if score > 0 {
                            (eval::MATE_VAL - score + 1) / 2
                        } else {
                            (-eval::MATE_VAL - score - 1) / 2
                        };

                        println!("info score mate {} depth {} seldepth {} nodes {} nps {} time {} pv {}", mate_score, depth, SEL_DEPTH, NODE_COUNT, nps, total_time_taken, util::format_pv(&pv_table));
                    } else {
                        println!("info score cp {} depth {} seldepth {} nodes {} nps {} time {} pv {}", score, depth, SEL_DEPTH, NODE_COUNT, nps, total_time_taken, util::format_pv(&pv_table));
                    }
                }

                if checkmate {
                    break
                }
    
                if total_time_taken - accumulated_time_taken > self.max_time_millis / 2 {
                    break
                }
            }

            depth += 1;
            accumulated_time_taken = total_time_taken;

            if depth > max_depth {
                break
            }

            alpha = score - WINDOW_SIZE;
            beta = score + WINDOW_SIZE;
            window_extended = false;
        }

        best_mov
    }

    fn ab_search(&mut self, state: &mut State, in_check: bool, on_extend: bool, mut alpha: i32, mut beta: i32, depth: u8, ply: u8) -> i32 {
        unsafe {
            if ABORT_SEARCH {
                return alpha
            }
        }

        unsafe {
            NODE_COUNT += 1;

            if (NODE_COUNT & TIME_CHECK_INTEVAL == 0) && self.time_tracker.elapsed().as_millis() > self.max_time_millis {
                ABORT_SEARCH = true;
                return alpha
            }
        }

        if mov_table::is_in_check(state, def::get_opposite_player(state.player)) {
            return eval::MATE_VAL - ply as i32
        }

        if ply > 0 && state.is_draw() {
            return 0
        }

        let on_pv = beta - alpha > 1;

        let mating_val = eval::MATE_VAL - ply as i32;
        if mating_val < beta {
            if alpha >= mating_val {
                return mating_val;
            }

            beta = mating_val;
        }

        let mated_val = -eval::MATE_VAL + ply as i32;
        if mated_val > alpha {
            if beta <= mated_val {
                return mated_val;
            }

            alpha = mated_val;
        }

        let original_alpha = alpha;

        let mut pv_mov = 0;

        match self.get_hash(state, depth) {
            Match(flag, score, mov) => {
                pv_mov = mov;

                if ply > 1 {
                    match flag {
                        HASH_TYPE_EXACT => {
                            return score;
                        },
                        HASH_TYPE_ALPHA => {
                            if score <= alpha {
                                return alpha
                            }
    
                            if score < beta {
                                beta = score;
                            }
                        },
                        HASH_TYPE_BETA => {
                            if score >= beta {
                                return beta
                            }
    
                            if score > alpha {
                                alpha = score;
                            }
                        },
                        _ => (),
                    }
                }
            },
            MovOnly(mov) => {
                pv_mov = mov;
            },
            _ => (),
        }

        if depth == 0 {
            let score = self.q_search(state, alpha, beta, ply);

            if score >= beta {
                self.set_hash(state, depth, HASH_TYPE_BETA, score, 0);
            } else if score > alpha {
                self.set_hash(state, depth, HASH_TYPE_EXACT, score, 0);
            } else {
                self.set_hash(state, depth, HASH_TYPE_ALPHA, score, 0);
            }

            return score
        }

        if ply > 0 && !on_extend && !in_check && depth >= NM_DEPTH {
            let depth_reduction = if depth > NM_DEPTH {
                NM_R + 1
            } else {
                NM_R
            };

            state.do_null_mov();
            let scout_score = -self.ab_search(state, false, false, -beta, -beta+1, depth - depth_reduction - 1, ply + 1);
            state.undo_null_mov();

            unsafe {
                if ABORT_SEARCH {
                    return alpha
                }
            }

            if scout_score != 0 && scout_score >= beta {
                return beta
            }
        }

        if on_pv && pv_mov == 0 && depth >= IID_DEPTH {
            self.ab_search(state, in_check, on_extend, alpha, beta, depth - IID_R, ply);

            unsafe {
                if ABORT_SEARCH {
                    return alpha
                }
            }

            match self.get_hash(state, depth) {
                MovOnly(hash_mov) => {
                    pv_mov = hash_mov;
                },
                _ => (),
            }
        }

        let mut mov_count = 0;
        let mut best_score = -eval::MATE_VAL;
        let mut best_mov = pv_mov;
        let mut pv_found = false;

        if pv_mov != 0 {
            mov_count += 1;

            let (from, to, tp, promo) = util::decode_u32_mov(pv_mov);

            let is_capture = state.squares[to] != 0;

            state.do_mov(from, to, tp, promo);

            let gives_check = mov_table::is_in_check(state, state.player);

            let is_passer = is_passed_pawn(state, state.squares[to], to);

            let mut depth = depth;
            let mut extended = false;

            if gives_check || is_passer {
                depth += 1;
                extended = true;
            }

            let score = -self.ab_search(state, gives_check, extended, -beta, -alpha, depth - 1, ply + 1);

            state.undo_mov(from, to, tp);

            unsafe {
                if ABORT_SEARCH {
                    return alpha
                }
            }

            if score >= beta {
                if !is_capture {
                    self.update_history_table(state.player, depth, from, to);
                    self.update_killer_table(ply, pv_mov);
                }

                self.set_hash(state, depth, HASH_TYPE_BETA, score, pv_mov);

                return score
            } else {
                if !is_capture {
                    self.update_butterfly_table(state.player, from, to);
                }
            }

            if score > best_score {
                best_score = score;
                best_mov = pv_mov;
            }

            if score > alpha {
                alpha = score;
                pv_found = true;
            }
        }

        let mut mov_list = [0; def::MAX_MOV_COUNT];

        mov_table::gen_reg_mov_list(state, &mut mov_list);

        let (primary_killer, secondary_killer) = self.get_killer_mov(ply);

        let mut ordered_mov_list = Vec::new();

        for mov_index in 0..def::MAX_MOV_COUNT {
            let mov = mov_list[mov_index];

            if mov == 0 {
                break
            }

            if mov == pv_mov {
                continue
            }

            let (from, to, tp, promo) = util::decode_u32_mov(mov);

            state.do_mov(from, to, tp, promo);
            let gives_check = mov_table::is_in_check(state, state.player);
            let is_passer = is_passed_pawn(state, state.squares[to], to);
            state.undo_mov(from, to, tp);

            if state.squares[to] != 0 {
                ordered_mov_list.push((SORTING_CAP_BASE_VAL + see(state, from, to, tp, promo), gives_check, is_passer, mov));
            } else if mov == primary_killer {
                ordered_mov_list.push((SORTING_CAP_BASE_VAL - SORTING_HALF_PAWN_VAL, gives_check, is_passer, mov));
            } else if mov == secondary_killer {
                ordered_mov_list.push((SORTING_CAP_BASE_VAL - SORTING_Q_VAL, gives_check, is_passer, mov));
            } else {
                let history_score = self.history_table[state.player as usize - 1][from][to];
                let butterfly_score = self.butterfly_table[state.player as usize - 1][from][to];
                ordered_mov_list.push((history_score / butterfly_score, gives_check, is_passer, mov));
            }
        }

        ordered_mov_list.sort_by(|(score_a, _, _, _), (score_b, _, _, _)| {
            score_b.partial_cmp(&score_a).unwrap()
        });

        unsafe {
            if ABORT_SEARCH {
                return alpha
            }
        }

        for (_sort_score, gives_check, is_passer, mov) in ordered_mov_list {
            mov_count += 1;

            let (from, to, tp, promo) = util::decode_u32_mov(mov);

            let is_capture = state.squares[to] != 0;

            state.do_mov(from, to, tp, promo);

            let mut depth = depth;
            let mut extended = false;

            if gives_check || is_passer {
                depth += 1;
                extended = true;
            }

            let score = if depth > 2 && mov_count > 1 && !gives_check && !is_passer {
                let score = -self.ab_search(state, gives_check, extended, -alpha - 1, -alpha, depth - 2, ply + 1);
                if score > alpha {
                    if pv_found {
                        let score = -self.ab_search(state, gives_check, extended, -alpha-1, -alpha, depth - 1, ply + 1);

                        if score > alpha && score < beta {
                            -self.ab_search(state, gives_check, extended, -beta, -alpha, depth - 1, ply + 1)
                        } else {
                            score
                        }
                    } else {
                        -self.ab_search(state, gives_check, extended, -beta, -alpha, depth - 1, ply + 1)
                    }
                } else {
                    score
                }
            } else {
                if pv_found {
                    let score = -self.ab_search(state, gives_check, extended, -alpha-1, -alpha, depth - 1, ply + 1);

                    if score > alpha && score < beta {
                        -self.ab_search(state, gives_check, extended, -beta, -alpha, depth - 1, ply + 1)
                    } else {
                        score
                    }
                } else {
                    -self.ab_search(state, gives_check, extended, -beta, -alpha, depth - 1, ply + 1)
                }
            };

            state.undo_mov(from, to, tp);

            unsafe {
                if ABORT_SEARCH {
                    return alpha
                }
            }

            if score >= beta {
                if !is_capture {
                    self.update_history_table(state.player, depth, from, to);
                    self.update_killer_table(ply, mov);
                }

                self.set_hash(state, depth, HASH_TYPE_BETA, score, mov);

                return score
            } else {
                if !is_capture {
                    self.update_butterfly_table(state.player, from, to);
                }
            }

            if score > best_score {
                best_score = score;
                best_mov = mov;
            }

            if score > alpha {
                alpha = score;
                pv_found = true;
            }
        }

        if alpha > original_alpha {
            self.set_hash(state, depth, HASH_TYPE_EXACT, alpha, best_mov);
        } else {
            self.set_hash(state, depth, HASH_TYPE_ALPHA, best_score, best_mov);
        }

        if best_score < -eval::TERM_VAL {
            if !in_check && self.in_stale_mate(state) {
                self.set_hash(state, MAX_DEPTH, HASH_TYPE_EXACT, 0, 0);
                return 0
            }
        }

        alpha
    }

    fn q_search(&mut self, state: &mut State, mut alpha: i32, beta: i32, ply: u8) -> i32 {
        unsafe {
            if ABORT_SEARCH {
                return alpha
            }
        }

        unsafe {
            NODE_COUNT += 1;
        }

        if mov_table::is_in_check(state, def::get_opposite_player(state.player)) {
            return eval::MATE_VAL - ply as i32
        }

        unsafe {
            if ply > SEL_DEPTH {
                SEL_DEPTH = ply;
            }
        }

        let (material_score, is_draw) = eval::eval_materials(state);

        if is_draw {
            return 0
        }

        if material_score - DELTA_MARGIN >= beta {
            return beta
        }

        let score = eval::eval_state(state, material_score);

        if score >= beta {
            return score
        }

        if score > alpha {
            alpha = score;
        }

        let delta = alpha - score - DELTA_MARGIN;

        let mut cap_list = [0; def::MAX_CAP_COUNT];
        mov_table::gen_capture_list(state, &mut cap_list);

        if cap_list[0] == 0 {
            return alpha
        }

        let squares = state.squares;

        let mut scored_cap_list = Vec::new();

        for mov_index in 0..def::MAX_CAP_COUNT {
            let cap = cap_list[mov_index];

            if cap == 0 {
                break
            }

            let (from, to, tp, promo) = util::decode_u32_mov(cap);

            let gain = eval::val_of(squares[to]) + eval::val_of(promo);

            if gain < delta {
                continue
            }

            scored_cap_list.push((see(state, from, to, tp, promo), cap));
        }

        if scored_cap_list.is_empty() {
            return alpha
        }

        scored_cap_list.sort_by(|(score_a, _), (score_b, _)| {
            score_b.partial_cmp(&score_a).unwrap()
        });

        for (_score, cap) in scored_cap_list {
            let (from, to, tp, promo) = util::decode_u32_mov(cap);

            state.do_mov(from, to, tp, promo);
            let score = -self.q_search(state, -beta, -alpha, ply + 1);
            state.undo_mov(from, to, tp);

            unsafe {
                if ABORT_SEARCH {
                    return alpha
                }
            }

            if score >= beta {
                return score
            }

            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }

    fn retrieve_pv(&self, state: &mut State, pv_table: &mut [u32], mov_index: usize) {
        if mov_index == PV_PRINT_LENGTH {
            return
        }

        let mut mov = 0;

        match self.get_hash(state, MAX_DEPTH) {
            Match(_flag, _score, hash_mov) => {
                mov = hash_mov;
            },
            MovOnly(hash_mov) => {
                mov = hash_mov;
            },
            _ => (),
        }

        if mov == 0 {
            return
        }

        let player = state.player;

        let (from, to, tp, promo) = util::decode_u32_mov(mov);
        state.do_mov(from, to, tp, promo);

        if mov_table::is_in_check(state, player) {
            state.undo_mov(from, to, tp);
            return
        }

        pv_table[mov_index] = mov;

        self.retrieve_pv(state, pv_table, mov_index + 1);

        state.undo_mov(from, to, tp);
    }

    #[inline]
    fn get_hash(&self, state: &State, depth: u8) -> LookupResult {
        match self.depth_preferred_hash_table.get(state.hash_key, state.player, depth, state.cas_rights, state.enp_square) {
            NoMatch => {
                self.always_replace_hash_table.get(state.hash_key, state.player, depth, state.cas_rights, state.enp_square)
            },
            matched => matched
        }
    }

    #[inline]
    fn set_hash(&mut self, state: &State, depth: u8, hash_flag: u8, score: i32, mov: u32) {
        if !self.depth_preferred_hash_table.set(state.hash_key, state.player, depth, state.cas_rights, state.enp_square, hash_flag, score, mov) {
            self.always_replace_hash_table.set(state.hash_key, state.player, depth, state.cas_rights, state.enp_square, hash_flag, score, mov);
        }
    }

    #[inline]
    fn update_killer_table(&mut self, ply: u8, mov: u32) {
        let ply_index = ply as usize;

        if ply_index >= PV_TRACK_LENGTH {
            return
        }

        let primary_killer_mov = self.primary_killer_table[ply_index];
        if primary_killer_mov != 0 {
            self.primary_killer_table[ply_index] = mov;
            self.secondary_killer_table[ply_index] = primary_killer_mov;
        } else {
            self.secondary_killer_table[ply_index] = mov;
        }
    }

    #[inline]
    fn get_killer_mov(&self, ply: u8) -> (u32, u32) {
        let ply_index = ply as usize;

        if ply_index >= PV_TRACK_LENGTH {
            return (0, 0)
        }

        let primary_killer = self.primary_killer_table[ply_index];

        if primary_killer == 0 && ply > 2 {
            let primary_killer = self.primary_killer_table[ply_index - 2];
            let secondary_killer = self.secondary_killer_table[ply_index - 2];

            return (primary_killer, secondary_killer)
        }

        let secondary_killer = self.secondary_killer_table[ply_index];

        if secondary_killer == 0 && ply > 2 {
            let secondary_killer = self.primary_killer_table[ply_index - 2];

            return (primary_killer, secondary_killer)
        }

        (primary_killer, secondary_killer)
    }

    #[inline]
    fn update_history_table(&mut self, player: u8, depth: u8, from: usize, to: usize) {
        let history_score = self.history_table[player as usize - 1][from][to];
        self.history_table[player as usize - 1][from][to] = history_score + depth as i32 * depth as i32;
    }

    #[inline]
    fn update_butterfly_table(&mut self, player: u8, from: usize, to: usize) {
        let history_score = self.butterfly_table[player as usize - 1][from][to];
        self.butterfly_table[player as usize - 1][from][to] = history_score + 1;
    }

    #[inline]
    fn in_stale_mate(&self, state: &mut State) -> bool {
        let player = state.player;

        let mut mov_list = [0; def::MAX_MOV_COUNT];

        mov_table::gen_reg_mov_list(state, &mut mov_list);

        for mov_index in 0..def::MAX_CAP_COUNT {
            let mov = mov_list[mov_index];

            if mov == 0 {
                break
            }

            let (from, to, tp, promo) = util::decode_u32_mov(mov);
            state.do_mov(from, to, tp, promo);

            if !mov_table::is_in_check(state, player) {
                state.undo_mov(from, to, tp);
                return false
            }

            state.undo_mov(from, to, tp);
        }

        true
    }
}

#[inline]
fn is_passed_pawn(state: &State, moving_piece: u8, to_index: usize) -> bool {
    match moving_piece {
        def::WP => {
            state.bitmask.wp_forward_masks[to_index] & state.bitboard.b_pawn == 0
            && state.bitmask.file_masks[to_index] & state.bitboard.b_all == 0
        },
        def::BP => {
            state.bitmask.bp_forward_masks[to_index] & state.bitboard.w_pawn == 0
            && state.bitmask.file_masks[to_index] & state.bitboard.w_all == 0
        },
        _ => false
    }
}

fn see(state: &mut State, from: usize, to: usize, tp: u8, promo: u8) -> i32 {
    let initial_gain = eval::val_of(state.squares[to]) + eval::val_of(promo);

    state.do_mov(from, to, tp, promo);

    let score = initial_gain - see_exchange(state, to, state.squares[to]);

    state.undo_mov(from, to, tp);

    score
}

fn see_exchange(state: &mut State, to: usize, last_attacker: u8) -> i32 {
    let (attacker, tp, promo, attack_from) = mov_table::get_smallest_attacker_index(state, to);

    if attacker == 0 {
        return 0
    }

    state.do_mov(attack_from, to, tp, promo);

    let score = (eval::val_of(last_attacker) + eval::val_of(promo) - see_exchange(state, to, attacker)).max(0);

    state.undo_mov(attack_from, to, tp);

    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bitboard::BitMask,
        state::State,
        prng::XorshiftPrng,
    };

    #[test]
    fn test_perft_1() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &zob_keys, &bitmask);
        let search_engine = SearchEngine::new(131072);

        assert_eq!(20, search_engine.perft(&mut state, 1));
        assert_eq!(400, search_engine.perft(&mut state, 2));
        assert_eq!(8902, search_engine.perft(&mut state, 3));
    }

    #[test]
    fn test_perft_2() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", &zob_keys, &bitmask);
        let search_engine = SearchEngine::new(131072);

        assert_eq!(14, search_engine.perft(&mut state, 1));
        assert_eq!(191, search_engine.perft(&mut state, 2));
        assert_eq!(2812, search_engine.perft(&mut state, 3));
        assert_eq!(43238, search_engine.perft(&mut state, 4));
        assert_eq!(674624, search_engine.perft(&mut state, 5));
        assert_eq!(11030083, search_engine.perft(&mut state, 6));
        assert_eq!(178633661, search_engine.perft(&mut state, 7));
    }

    #[test]
    fn test_perft_3() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", &zob_keys, &bitmask);
        let search_engine = SearchEngine::new(131072);

        assert_eq!(44, search_engine.perft(&mut state, 1));
        assert_eq!(1486, search_engine.perft(&mut state, 2));
        assert_eq!(62379, search_engine.perft(&mut state, 3));
        assert_eq!(2103487, search_engine.perft(&mut state, 4));
        assert_eq!(89941194, search_engine.perft(&mut state, 5));
    }

    #[test]
    fn test_perft_4() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", &zob_keys, &bitmask);
        let search_engine = SearchEngine::new(131072);

        assert_eq!(46, search_engine.perft(&mut state, 1));
        assert_eq!(2079, search_engine.perft(&mut state, 2));
        assert_eq!(89890, search_engine.perft(&mut state, 3));
        assert_eq!(3894594, search_engine.perft(&mut state, 4));
        assert_eq!(164075551, search_engine.perft(&mut state, 5));
    }

    #[test]
    fn test_perft_5() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1", &zob_keys, &bitmask);
        let search_engine = SearchEngine::new(131072);

        assert_eq!(6, search_engine.perft(&mut state, 1));
        assert_eq!(264, search_engine.perft(&mut state, 2));
        assert_eq!(9467, search_engine.perft(&mut state, 3));
        assert_eq!(422333, search_engine.perft(&mut state, 4));
        assert_eq!(15833292, search_engine.perft(&mut state, 5));
    }

    #[test]
    fn test_perft_6() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("8/P1k5/K7/8/8/8/8/8 w - - 0 1", &zob_keys, &bitmask);
        let search_engine = SearchEngine::new(131072);

        assert_eq!(92683, search_engine.perft(&mut state, 6));
    }

    #[test]
    fn test_search() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("4r2k/1q3pp1/5n1p/1pr2P2/p2R3P/P2B4/1P2QP2/4R1K1 w - - 2 51", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(131072);

        let time_capacity = TimeCapacity {
            main_time_millis: 15500,
            extra_time_millis: 15500,
        };

        let best_mov = search_engine.search(&mut state, time_capacity, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("e2"));
        assert_eq!(to, util::map_sqr_notation_to_index("e8"));
    }
}