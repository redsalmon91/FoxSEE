/*
 * Copyright (C) 2020 Zixiao Han
 */

use crate::{
    def,
    eval,
    hashtable::{DepthPreferredHashTable, LookupResult, HASH_TYPE_ALPHA, HASH_TYPE_BETA, HASH_TYPE_EXACT},
    mov_table,
    state::State,
    time_control::TimeCapacity,
    util,
    zob_keys,
};

const PV_TRACK_LENGTH: usize = 128;
const PV_PRINT_LENGTH: usize = 16;

const SORTING_CAP_BASE_VAL: i32 = 10000000;
const SORTING_Q_VAL: i32 = 1000;
const SORTING_HALF_PAWN_VAL: i32 = 50;

const WD_SIZE: i32 = 10;
const MAX_WD_EXTENSION_COUNT: i32 = 10;

const NM_DEPTH: u8 = 6;
const NM_R: u8 = 2;

const MAX_DEPTH: u8 = 128;

const DELTA_MARGIN: i32 = 200;

const TIME_CHECK_INTEVAL: u64 = 4095;

static mut NODE_COUNT: u64 = 0;
static mut SEL_DEPTH: u8 = 0;

pub static mut ABORT_SEARCH: bool = false;

use std::time::Instant;
use LookupResult::*;

pub struct SearchEngine {
    depth_preferred_hash_table: DepthPreferredHashTable,
    primary_killer_table: [(u32, i32, u8); PV_TRACK_LENGTH],
    secondary_killer_table: [(u32, i32, u8); PV_TRACK_LENGTH],
    history_table: [[[i32; def::BOARD_SIZE]; def::BOARD_SIZE]; 2],
    butterfly_table: [[[i32; def::BOARD_SIZE]; def::BOARD_SIZE]; 2],
    time_tracker: Instant,
    max_time_millis: u128,
}

impl SearchEngine {
    pub fn new(hash_size: usize) -> Self {
        SearchEngine {
            depth_preferred_hash_table: DepthPreferredHashTable::new(hash_size >> 1),
            primary_killer_table: [(0, 0, 0); PV_TRACK_LENGTH],
            secondary_killer_table: [(0, 0, 0); PV_TRACK_LENGTH],
            history_table: [[[0; def::BOARD_SIZE]; def::BOARD_SIZE]; 2],
            butterfly_table: [[[1; def::BOARD_SIZE]; def::BOARD_SIZE]; 2],
            time_tracker: Instant::now(),
            max_time_millis: 0,
        }
    }

    pub fn reset(&mut self) {
        self.depth_preferred_hash_table.clear();
    }

    pub fn set_hash_size(&mut self, hash_size: usize) {
        self.depth_preferred_hash_table = DepthPreferredHashTable::new(hash_size >> 1);
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

        self.primary_killer_table = [(0, 0, 0); PV_TRACK_LENGTH];
        self.secondary_killer_table = [(0, 0, 0); PV_TRACK_LENGTH];
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
        let mut window_extended_count = 0;
        let mut accumulated_time_taken = 0;

        loop {
            unsafe {
                NODE_COUNT = 0;
                SEL_DEPTH = 0;
            }

            let score = self.ab_search(state, in_check, false, false, alpha, beta, depth, 0);

            unsafe {
                if ABORT_SEARCH {
                    break
                }
            }

            let mut pv_table = [0; PV_TRACK_LENGTH];
            self.retrieve_pv(state, &mut pv_table, 0);

            let checkmate = score.abs() > eval::TERM_VAL;

            let total_time_taken = self.time_tracker.elapsed().as_millis();

            if pv_table[0] != 0 {
                if score > alpha && score < beta {
                    best_mov = pv_table[0];
                }

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

            if score <= alpha {
                if window_extended_count <= MAX_WD_EXTENSION_COUNT {
                    alpha = score - WD_SIZE * window_extended_count * window_extended_count;
                    window_extended_count += 1;
                } else {
                    alpha = -eval::MATE_VAL;
                }

                continue;
            }

            if score >= beta {
                if window_extended_count <= MAX_WD_EXTENSION_COUNT {
                    beta = score + WD_SIZE * window_extended_count * window_extended_count;
                    window_extended_count += 1;
                } else {
                    beta = eval::MATE_VAL;
                }

                continue;
            }

            depth += 1;
            accumulated_time_taken = total_time_taken;

            if depth > max_depth {
                break
            }

            alpha = score - WD_SIZE;
            beta = score + WD_SIZE;
            window_extended_count = 0;
        }

        best_mov
    }

    fn ab_search(&mut self, state: &mut State, in_check: bool, on_extend: bool, on_cut: bool, mut alpha: i32, mut beta: i32, depth: u8, ply: u8) -> i32 {
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
        let mut is_singular_mov = false;

        match self.get_hash(state, depth) {
            Match(flag, score, mov) => {
                pv_mov = mov;

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

                            if beta - alpha > 1 && pv_mov != 0 {
                                is_singular_mov = true;
                            }
                        }
                    },
                    _ => (),
                }
            },
            MovOnly(mov) => {
                pv_mov = mov;
            },
            _ => (),
        }

        if depth == 0 {
            return self.q_search(state, alpha, beta, ply);
        }

        let on_pv = beta - alpha > 1;

        let in_endgame = eval::is_in_endgame(state);

        if !on_pv && !on_extend && !on_cut && !in_check && !in_endgame && depth >= NM_DEPTH {
            let depth_reduction = if depth > NM_DEPTH {
                NM_R + 1
            } else {
                NM_R
            };

            state.do_null_mov();
            let scout_score = -self.ab_search(state, false, false, true, -beta, -beta+1, depth - depth_reduction - 1, ply + 1);
            state.undo_null_mov();

            unsafe {
                if ABORT_SEARCH {
                    return alpha
                }
            }

            if scout_score >= beta && scout_score != 0 {
                return beta
            }
        }

        let mut mov_count = 0;
        let mut best_score = -eval::MATE_VAL;
        let mut best_mov = pv_mov;
        let mut best_mov_depth = depth;
        let mut pv_found = false;
        let mut pv_extended = false;

        if pv_mov != 0 {
            mov_count += 1;

            let (from, to, tp, promo) = util::decode_u32_mov(pv_mov);

            let is_capture = state.squares[to] != 0;

            state.do_mov(from, to, tp, promo);

            let gives_check = mov_table::is_in_check(state, state.player);

            let mut depth = depth;

            if gives_check && !in_endgame {
                depth += 1;
                pv_extended = true;
            }

            let score = -self.ab_search(state, gives_check, pv_extended, false, -beta, -alpha, depth - 1, ply + 1);

            state.undo_mov(from, to, tp);

            unsafe {
                if ABORT_SEARCH {
                    return alpha
                }
            }

            if score >= beta {
                if !is_capture {
                    self.update_history_table(state.player, depth, from, to);
                    self.update_killer_table(ply, pv_mov, score, depth);
                }

                self.set_hash(state, depth, state.full_mov_count, HASH_TYPE_BETA, score, pv_mov);

                return score
            }

            if score > best_score {
                best_score = score;
                best_mov = pv_mov;
                best_mov_depth = depth;
            }

            if score > alpha {
                alpha = score;
                pv_found = true;
            } else {
                is_singular_mov = false;
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

            state.undo_mov(from, to, tp);

            if state.squares[to] != 0 {
                if gives_check {
                    ordered_mov_list.push((SORTING_CAP_BASE_VAL + SORTING_HALF_PAWN_VAL + see(state, from, to, tp, promo), gives_check, mov));
                } else {
                    ordered_mov_list.push((SORTING_CAP_BASE_VAL + see(state, from, to, tp, promo), gives_check, mov));
                }
            } else if mov == primary_killer {
                ordered_mov_list.push((SORTING_CAP_BASE_VAL - SORTING_Q_VAL, gives_check, mov));
            } else if mov == secondary_killer {
                ordered_mov_list.push((SORTING_CAP_BASE_VAL - SORTING_Q_VAL - SORTING_HALF_PAWN_VAL, gives_check, mov));
            } else {
                let history_score = self.history_table[state.player as usize - 1][from][to];
                let butterfly_score = self.butterfly_table[state.player as usize - 1][from][to];
                ordered_mov_list.push((history_score / butterfly_score, gives_check, mov));
            }
        }

        ordered_mov_list.sort_by(|(score_a, _, _), (score_b, _, _)| {
            score_b.partial_cmp(&score_a).unwrap()
        });

        unsafe {
            if ABORT_SEARCH {
                return alpha
            }
        }

        for (sort_score, gives_check, mov) in &ordered_mov_list {
            mov_count += 1;

            let (from, to, tp, promo) = util::decode_u32_mov(*mov);

            let is_capture = state.squares[to] != 0;
            let is_winning_capture = is_capture && *sort_score >= SORTING_CAP_BASE_VAL;

            state.do_mov(from, to, tp, promo);

            let original_depth = depth;
            let mut depth = depth;
            let mut extended = false;

            if *gives_check && !in_endgame {
                depth += 1;
                extended = true;
            }

            let mut reduced = false;

            if depth > 1 && mov_count > 1 && !extended && !is_winning_capture {
               depth -= ((mov_count as f64 + depth as f64).sqrt() as u8).min(depth - 1);
               reduced = true;
            }

            let mut score = if pv_found {
                -self.ab_search(state, *gives_check, extended, true, -alpha - 1, -alpha, depth - 1, ply + 1)
            } else {
                -self.ab_search(state, *gives_check, extended, reduced, -beta, -alpha, depth - 1, ply + 1)
            };

            if score > alpha {
                if reduced {
                    depth = original_depth;

                    if pv_found {
                        score = -self.ab_search(state, *gives_check, extended, true, -alpha-1, -alpha, depth - 1, ply + 1);

                        if score > alpha && score < beta {
                            pv_found = false;
                            score = -self.ab_search(state, *gives_check, extended, false, -beta, -alpha, depth - 1, ply + 1);
                        }
                    } else {
                        score = -self.ab_search(state, *gives_check, extended, false, -beta, -alpha, depth - 1, ply + 1);
                    }
                }  else if pv_found && score < beta {
                    pv_found = false;
                    score = -self.ab_search(state, *gives_check, extended, false, -beta, -alpha, depth - 1, ply + 1);
                }
            }

            state.undo_mov(from, to, tp);

            unsafe {
                if ABORT_SEARCH {
                    return alpha
                }
            }

            if score >= beta {
                if !is_capture {
                    self.update_history_table(state.player, depth, from, to);
                    self.update_killer_table(ply, *mov, score, depth);

                    if pv_mov != 0 {
                        let (from, to, _tp, _promo) = util::decode_u32_mov(pv_mov);

                        let is_capture = state.squares[to] != 0;

                        if !is_capture {
                            self.update_butterfly_table(state.player, from, to);
                        }

                        for index in 0..mov_count-1 {
                            let (_score, _gives_check, mov) = ordered_mov_list[index];
                            let (from, to, _tp, _promo) = util::decode_u32_mov(mov);

                            let is_capture = state.squares[to] != 0;
    
                            if !is_capture {
                                self.update_butterfly_table(state.player, from, to);
                            }
                        }
                    } else {
                        for index in 0..mov_count {
                            let (_score, _gives_check, mov) = ordered_mov_list[index];
                            let (from, to, _tp, _promo) = util::decode_u32_mov(mov);

                            let is_capture = state.squares[to] != 0;
    
                            if !is_capture {
                                self.update_butterfly_table(state.player, from, to);
                            }
                        }
                    }
                }

                self.set_hash(state, depth, state.full_mov_count, HASH_TYPE_BETA, score, *mov);

                return score
            }

            if score > best_score {
                best_score = score;
                best_mov = *mov;
                best_mov_depth = depth;
            }

            if score > alpha {
                alpha = score;
                pv_found = true;
                is_singular_mov = false;
            }
        }

        if is_singular_mov && !pv_extended {
            let (from, to, tp, promo) = util::decode_u32_mov(pv_mov);

            let is_capture = state.squares[to] != 0;

            state.do_mov(from, to, tp, promo);

            let gives_check = mov_table::is_in_check(state, state.player);

            let depth = depth + 1;

            let score = -self.ab_search(state, gives_check, true, false, -beta, -alpha, depth - 1, ply + 1);

            state.undo_mov(from, to, tp);

            unsafe {
                if ABORT_SEARCH {
                    return alpha
                }
            }

            if score >= beta {
                if !is_capture {
                    self.update_history_table(state.player, depth, from, to);
                    self.update_killer_table(ply, pv_mov, score, depth);
                }

                self.set_hash(state, depth, state.full_mov_count, HASH_TYPE_BETA, score, pv_mov);

                return score
            }

            if !is_capture {
                self.update_butterfly_table(state.player, from, to);
            }

            if score > best_score {
                best_score = score;
                best_mov = pv_mov;
                best_mov_depth = depth;
            }

            if score > alpha {
                alpha = score;
            }
        }

        if alpha > original_alpha {
            self.set_hash(state, best_mov_depth, state.full_mov_count, HASH_TYPE_EXACT, alpha, best_mov);
        } else {
            if !in_check && self.in_stale_mate(state) {
                self.set_hash(state, MAX_DEPTH, state.full_mov_count, HASH_TYPE_EXACT, 0, 0);
                return 0
            }

            self.set_hash(state, best_mov_depth, state.full_mov_count, HASH_TYPE_ALPHA, best_score, best_mov);
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

        let mut scored_cap_list = Vec::new();

        for mov_index in 0..def::MAX_CAP_COUNT {
            let cap = cap_list[mov_index];

            if cap == 0 {
                break
            }

            let (from, to, tp, promo) = util::decode_u32_mov(cap);

            let gain = eval::val_of(state.squares[to]) + eval::val_of(promo);

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
        self.depth_preferred_hash_table.get(get_hash_key(state), depth)
    }

    #[inline]
    fn set_hash(&mut self, state: &State, depth: u8, age: u16, hash_flag: u8, score: i32, mov: u32) {
        self.depth_preferred_hash_table.set(get_hash_key(state), depth, age, hash_flag, score, mov);
    }

    #[inline]
    fn update_killer_table(&mut self, ply: u8, mov: u32, score: i32, depth: u8) {
        let ply_index = ply as usize;

        if ply_index >= PV_TRACK_LENGTH {
            return
        }

        let (primary_killer_mov, killer_score, killer_depth) = self.primary_killer_table[ply_index];
        if primary_killer_mov != 0 {
            if score >= killer_score && depth >= killer_depth {
                self.primary_killer_table[ply_index] = (mov, score, depth);
                self.secondary_killer_table[ply_index] = (primary_killer_mov, killer_score, killer_depth);
            } else {
                self.secondary_killer_table[ply_index] = (mov, score, depth);
            }
        } else {
            self.secondary_killer_table[ply_index] = (mov, score, depth);
        }
    }

    #[inline]
    fn get_killer_mov(&self, ply: u8) -> (u32, u32) {
        let ply_index = ply as usize;

        if ply_index >= PV_TRACK_LENGTH {
            return (0, 0)
        }

        let (primary_killer, _score, _depth) = self.primary_killer_table[ply_index];

        if primary_killer == 0 && ply > 2 {
            let (primary_killer, _score, _depth) = self.primary_killer_table[ply_index - 2];
            let (secondary_killer, _score, _depth) = self.secondary_killer_table[ply_index - 2];

            return (primary_killer, secondary_killer)
        }

        let (secondary_killer, _score, _depth) = self.secondary_killer_table[ply_index];

        if secondary_killer == 0 && ply > 2 {
            let (secondary_killer, _score, _depth) = self.primary_killer_table[ply_index - 2];

            return (primary_killer, secondary_killer)
        }

        (primary_killer, secondary_killer)
    }

    #[inline]
    fn update_history_table(&mut self, player: u8, depth: u8, from: usize, to: usize) {
        let history_score = self.history_table[player as usize - 1][from][to];
        let increment = depth as i32;
        self.history_table[player as usize - 1][from][to] = history_score + increment * increment * increment;
    }

    #[inline]
    fn update_butterfly_table(&mut self, player: u8, from: usize, to: usize) {
        let butterfly_score = self.butterfly_table[player as usize - 1][from][to];
        self.butterfly_table[player as usize - 1][from][to] = butterfly_score + 1;
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
fn get_hash_key(state: &State) -> u64 {
    let mut key = state.hash_key
    ^ zob_keys::get_enp_sqr_zob_key(state.enp_square as usize)
    ^ zob_keys::get_cas_rights_zob_key(state.cas_rights);

    if state.player == def::PLAYER_B {
        key ^= zob_keys::get_b_player_zob_key();
    }

    key
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
        bitmask,
        state::State,
        zob_keys,
    };

    #[test]
    fn test_perft_1() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let search_engine = SearchEngine::new(131072);

        assert_eq!(20, search_engine.perft(&mut state, 1));
        assert_eq!(400, search_engine.perft(&mut state, 2));
        assert_eq!(8902, search_engine.perft(&mut state, 3));
    }

    #[test]
    fn test_perft_2() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
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
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
        let search_engine = SearchEngine::new(131072);

        assert_eq!(44, search_engine.perft(&mut state, 1));
        assert_eq!(1486, search_engine.perft(&mut state, 2));
        assert_eq!(62379, search_engine.perft(&mut state, 3));
        assert_eq!(2103487, search_engine.perft(&mut state, 4));
        assert_eq!(89941194, search_engine.perft(&mut state, 5));
    }

    #[test]
    fn test_perft_4() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10");
        let search_engine = SearchEngine::new(131072);

        assert_eq!(46, search_engine.perft(&mut state, 1));
        assert_eq!(2079, search_engine.perft(&mut state, 2));
        assert_eq!(89890, search_engine.perft(&mut state, 3));
        assert_eq!(3894594, search_engine.perft(&mut state, 4));
        assert_eq!(164075551, search_engine.perft(&mut state, 5));
    }

    #[test]
    fn test_perft_5() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1");
        let search_engine = SearchEngine::new(131072);

        assert_eq!(6, search_engine.perft(&mut state, 1));
        assert_eq!(264, search_engine.perft(&mut state, 2));
        assert_eq!(9467, search_engine.perft(&mut state, 3));
        assert_eq!(422333, search_engine.perft(&mut state, 4));
        assert_eq!(15833292, search_engine.perft(&mut state, 5));
    }

    #[test]
    fn test_perft_6() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("8/P1k5/K7/8/8/8/8/8 w - - 0 1");
        let search_engine = SearchEngine::new(131072);

        assert_eq!(92683, search_engine.perft(&mut state, 6));
    }

    #[test]
    fn test_search_1() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("4r2k/1q3pp1/5n1p/1pr2P2/p2R3P/P2B4/1P2QP2/4R1K1 w - - 2 51");
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

    #[test]
    fn test_search_2() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("6k1/p3q2p/1nr3pB/8/3Q1P2/6P1/PP5P/3R2K1 b - - 0 1");
        let mut search_engine = SearchEngine::new(131072);

        let time_capacity = TimeCapacity {
            main_time_millis: 25500,
            extra_time_millis: 15500,
        };

        let best_mov = search_engine.search(&mut state, time_capacity, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("c6"));
        assert_eq!(to, util::map_sqr_notation_to_index("d6"));
    }

    #[test]
    fn test_search_3() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("5nk1/nbb2pr1/p3p1p1/1p1r3q/2P5/PP1PP1P1/N3RP1P/BQN1RBK1 b - - 0 1");
        let mut search_engine = SearchEngine::new(131072);

        let time_capacity = TimeCapacity {
            main_time_millis: 15500,
            extra_time_millis: 15500,
        };

        let best_mov = search_engine.search(&mut state, time_capacity, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("h5"));
        assert_eq!(to, util::map_sqr_notation_to_index("h2"));
    }

    #[test]
    fn test_search_4() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("3rr1k1/pp3pp1/1qn2np1/8/3p4/PP1R1P2/2P1NQPP/R1B3K1 b - - 0 1");
        let mut search_engine = SearchEngine::new(131072);

        let time_capacity = TimeCapacity {
            main_time_millis: 15500,
            extra_time_millis: 5500,
        };

        let best_mov = search_engine.search(&mut state, time_capacity, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("c6"));
        assert_eq!(to, util::map_sqr_notation_to_index("e5"));
    }

    #[test]
    fn test_search_5() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("q4rk1/1n1Qbppp/2p5/1p2p3/1P2P3/2P4P/6P1/2B1NRK1 b - - 0 1");
        let mut search_engine = SearchEngine::new(131072);

        let time_capacity = TimeCapacity {
            main_time_millis: 25500,
            extra_time_millis: 5500,
        };

        let best_mov = search_engine.search(&mut state, time_capacity, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("a8"));
        assert_eq!(to, util::map_sqr_notation_to_index("c8"));
    }

    #[test]
    fn test_search_6() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("4r1k1/1bq2ppp/8/r4p1Q/pPpP1P2/N3p1PP/PP3P2/3RR1K1 b - - 0 30");
        let mut search_engine = SearchEngine::new(131072);

        let time_capacity = TimeCapacity {
            main_time_millis: 15500,
            extra_time_millis: 5500,
        };

        let best_mov = search_engine.search(&mut state, time_capacity, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("c7"));
        assert_eq!(to, util::map_sqr_notation_to_index("c6"));
    }

    #[test]
    fn test_search_7() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("8/8/p1p5/1p5p/1P5p/8/PPP2K1p/4R1rk w - - 0 1");
        let mut search_engine = SearchEngine::new(131072);

        let time_capacity = TimeCapacity {
            main_time_millis: 15500,
            extra_time_millis: 5500,
        };

        let best_mov = search_engine.search(&mut state, time_capacity, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("e1"));
        assert_eq!(to, util::map_sqr_notation_to_index("f1"));
    }

    #[test]
    fn test_search_8() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("8/4p3/p2p4/2pP4/2P1P3/1P4k1/1P1K4/8 w - - 0 1");
        let mut search_engine = SearchEngine::new(131072);

        let time_capacity = TimeCapacity {
            main_time_millis: 15500,
            extra_time_millis: 5500,
        };

        let best_mov = search_engine.search(&mut state, time_capacity, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("b3"));
        assert_eq!(to, util::map_sqr_notation_to_index("b4"));
    }

    #[test]
    fn test_search_9() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("r2q1rk1/p3bppp/b1n1p3/2Nn4/8/5NP1/PPQBPPBP/R3K2R b KQ - 0 12");
        let mut search_engine = SearchEngine::new(131072);

        let time_capacity = TimeCapacity {
            main_time_millis: 15500,
            extra_time_millis: 5500,
        };

        let best_mov = search_engine.search(&mut state, time_capacity, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("c6"));
        assert_eq!(to, util::map_sqr_notation_to_index("b4"));
    }
}
