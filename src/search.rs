/*
 * Copyright (C) 2020 Zixiao Han
 */

use crate::{
    def,
    eval,
    hashtable::{AlwaysReplaceHashTable, DepthPreferredHashTable, LookupResult, HASH_TYPE_ALPHA, HASH_TYPE_BETA},
    mov_table::MoveTable,
    state::State,
    util,
};

use std::u64;

const PV_TRACK_LENGTH: usize = 64;
const KILLER_TABLE_LENGTH: usize = 256;
const MAX_HISTORY_SCORE: u64 = u64::MAX >> 1;
const WINDOW_SIZE: i32 = 50;

const MIN_NM_DEPTH: u8 = 5;
const NM_DEPTH_REDUCTION: u8 = 3;
const NM_PV_TABLE: [u32; PV_TRACK_LENGTH] = [0; PV_TRACK_LENGTH];

const ENDGAME_PIECE_COUNT: u32 = 6;
const PROMO_HORIZON_RANK: usize = 1;

pub enum SearchMovResult {
    Return(i32),
    RaiseAlpha(i32),
    Noop,
}

use SearchMovResult::*;
use LookupResult::*;
use std::time::Instant;

pub struct SearchEngine {
    mov_table: MoveTable,
    depth_preferred_hash_table: DepthPreferredHashTable,
    always_replace_hash_table: AlwaysReplaceHashTable,
    killer_table: [((i32, u32), (i32, u32)); KILLER_TABLE_LENGTH],
    history_table: [[u64; def::BOARD_SIZE]; def::BOARD_SIZE],
    master_pv_table: [u32; PV_TRACK_LENGTH],
    root_node_mov_list: Vec<(u8, i32, u32)>,
    time_tracker: Instant,

    abort: bool,
    max_time_millis: u128,
}

impl SearchEngine {
    pub fn new(hash_size: usize) -> Self {
        SearchEngine {
            mov_table: MoveTable::new(),
            depth_preferred_hash_table: DepthPreferredHashTable::new(hash_size >> 1),
            always_replace_hash_table: AlwaysReplaceHashTable::new(hash_size >> 1),
            killer_table: [((0, 0), (0, 0)); KILLER_TABLE_LENGTH],
            history_table: [[0; def::BOARD_SIZE]; def::BOARD_SIZE],
            master_pv_table: [0; PV_TRACK_LENGTH],
            root_node_mov_list: Vec::new(),
            time_tracker: Instant::now(),

            abort: false,
            max_time_millis: 0,
        }
    }

    pub fn set_hash_size(&mut self, hash_size: usize) {
        self.depth_preferred_hash_table = DepthPreferredHashTable::new(hash_size >> 1);
        self.always_replace_hash_table = AlwaysReplaceHashTable::new(hash_size >> 1);
    }

    pub fn reset(&mut self) {
        self.depth_preferred_hash_table.clear();
        self.always_replace_hash_table.clear();
    }

    pub fn stop(&mut self) {
        self.abort = true;
    }

    pub fn perft(&self, state: &mut State, depth: u8) -> usize {
        if self.mov_table.is_in_check(state, def::get_opposite_player(state.player)) {
            return 0
        }

        if depth == 0 {
            return 1
        }

        let mut node_count = 0;

        let mut cap_list = [0; def::MAX_CAP_COUNT];
        let mut non_cap_list = [0; def::MAX_MOV_COUNT];
        let mut cas_list = [0; def::MAX_CAS_COUNT];

        self.mov_table.gen_reg_mov_list(state, &mut cap_list, &mut non_cap_list);
        self.mov_table.gen_castle_mov_list(state, &mut cas_list);

        for cap_index in 0..def::MAX_CAP_COUNT {
            let cap = cap_list[cap_index];

            if cap == 0 {
                break
            }

            let (from, to, tp, promo) = util::decode_u32_mov(cap);

            if def::is_k(state.squares[to]) {
                return 0
            }

            state.do_mov(from, to, tp, promo);
            node_count += self.perft(state, depth - 1);
            state.undo_mov(from, to, tp);
        }

        for non_cap_index in 0..def::MAX_MOV_COUNT {
            let non_cap = non_cap_list[non_cap_index];

            if non_cap == 0 {
                break
            }

            let (from, to, tp, promo) = util::decode_u32_mov(non_cap);
            state.do_mov(from, to, tp, promo);
            node_count += self.perft(state, depth - 1);
            state.undo_mov(from, to, tp);
        }

        for cas_index in 0..def::MAX_CAS_COUNT {
            let cas_mov = cas_list[cas_index];

            if cas_mov == 0 {
                break
            }

            let (from, to, tp, promo) = util::decode_u32_mov(cas_mov);
            state.do_mov(from, to, tp, promo);
            node_count += self.perft(state, depth - 1);
            state.undo_mov(from, to, tp);
        }

        node_count
    }

    pub fn search(&mut self, state: &mut State, max_time_millis: u128, max_depth: u8) -> u32 {
        self.time_tracker = Instant::now();
        self.max_time_millis = max_time_millis;
        self.abort = false;
        self.master_pv_table = [0; PV_TRACK_LENGTH];
        self.root_node_mov_list = Vec::new();

        let mut alpha = -eval::MATE_VAL;
        let mut beta = eval::MATE_VAL;

        let mut depth = 1;
        let mut best_mov = 0;

        loop {
            self.killer_table = [((0, 0), (0, 0)); KILLER_TABLE_LENGTH];
            self.history_table = [[0; def::BOARD_SIZE]; def::BOARD_SIZE];

            let mut node_count = 0;
            let mut seldepth = 0;

            let mut pv_table = [0; PV_TRACK_LENGTH];
            let score = self.root_search(state, &mut pv_table, alpha, beta, depth, 0, &mut node_count, &mut seldepth);

            if self.abort {
                break
            }

            self.validate_pv(state, &mut pv_table, 0);

            let checkmate = score > eval::TERM_VAL;

            if !pv_table.is_empty() && pv_table[0] != 0 {
                let time_taken_millis = self.time_tracker.elapsed().as_millis();
                let nps = node_count as u128 / (time_taken_millis / 1000).max(1);

                if checkmate {
                    println!("info score mate {} depth {} seldepth {} nodes {} nps {} time {} pv {}", (eval::MATE_VAL - score.abs() + 1) / 2, depth, seldepth, node_count, nps, time_taken_millis, util::format_pv(&pv_table));
                } else {
                    println!("info score cp {} depth {} seldepth {} nodes {} nps {} time {} pv {}", score, depth, seldepth, node_count, nps, time_taken_millis, util::format_pv(&pv_table));
                }
            }

            if score <= alpha {
                alpha = -eval::MATE_VAL;
                continue
            }

            if score >= beta {
                beta = eval::MATE_VAL;
                continue
            }

            let pv_changed = best_mov != pv_table[0];

            best_mov = pv_table[0];

            if checkmate {
                break
            }

            let current_time_millis = self.time_tracker.elapsed().as_millis();

            if !pv_changed {
                if current_time_millis > max_time_millis >> 1 {
                    break
                }
            }

            depth += 1;

            if depth > max_depth {
                break
            }

            alpha = score - WINDOW_SIZE;
            beta = score + WINDOW_SIZE;

            self.master_pv_table.copy_from_slice(&pv_table);
        }

        best_mov
    }

    pub fn validate_pv(&self, state: &mut State, pv_table: &mut [u32], mov_index: usize) {
        if mov_index == PV_TRACK_LENGTH - 1 {
            return
        }

        let mov = pv_table[mov_index];
        if mov == 0 {
            return
        }

        let player = state.player;

        let (from, to, tp, promo) = util::decode_u32_mov(mov);
        state.do_mov(from, to, tp, promo);

        if self.mov_table.is_in_check(state, player) {
            pv_table[mov_index] = 0;
            state.undo_mov(from, to, tp);
            return
        }

        self.validate_pv(state, pv_table, mov_index + 1);

        state.undo_mov(from, to, tp);
    }

    pub fn root_search(&mut self, state: &mut State, pv_table: &mut [u32], mut alpha: i32, beta: i32, depth: u8, ply: u8, node_count: &mut u64, seldepth: &mut u8) -> i32 {
        if self.root_node_mov_list.is_empty() {
            let mut cap_list = [0; def::MAX_CAP_COUNT];
            let mut non_cap_list = [0; def::MAX_MOV_COUNT];

            self.mov_table.gen_reg_mov_list(state, &mut cap_list, &mut non_cap_list);

            let player = state.player;
            let squares = state.squares;

            for cap_index in 0..def::MAX_CAP_COUNT {
                let cap = cap_list[cap_index];

                if cap == 0 {
                    break
                }

                let (from, to, _tp, promo) = util::decode_u32_mov(cap);
                self.root_node_mov_list.push((depth, eval::val_of(squares[to]) - eval::val_of(squares[from]) + eval::val_of(promo), cap));
            }

            for non_cap_index in 0..def::MAX_MOV_COUNT {
                let non_cap = non_cap_list[non_cap_index];

                if non_cap == 0  {
                    break
                }

                let (_from, _to, _tp, promo) = util::decode_u32_mov(non_cap);
                self.root_node_mov_list.push((depth, eval::val_of(promo), non_cap));
            }

            if (player == def::PLAYER_W && (state.cas_rights & 0b1100 != 0)) || (player == def::PLAYER_B && (state.cas_rights & 0b0011 != 0)) {
                let mut cas_list = [0; def::MAX_CAS_COUNT];
                self.mov_table.gen_castle_mov_list(state, &mut cas_list);

                for cas_index in 0..def::MAX_CAS_COUNT {
                    let cas_mov = cas_list[cas_index];

                    if cas_mov == 0  {
                        break
                    }

                    self.root_node_mov_list.push((depth, eval::TERM_VAL, cas_mov));
                }
            }
        }

        self.root_node_mov_list.sort_by(|(depth_a, score_a, _), (depth_b, score_b, _)| {
            if depth_a == depth_b {
                score_b.partial_cmp(&score_a).unwrap()
            } else {
                depth_b.partial_cmp(&depth_a).unwrap()
            }
        });

        for mov_index in 0..self.root_node_mov_list.len() {
            let (_depth, _score, mov) = self.root_node_mov_list[mov_index];
            let (from, to, tp, promo) = util::decode_u32_mov(mov);

            let mut next_pv_table = [0; PV_TRACK_LENGTH];

            state.do_mov(from, to, tp, promo);

            let gives_check = self.mov_table.is_in_check(state, state.player);

            let depth = if gives_check {
                depth + 1
            } else {
                depth
            };

            let score = if mov == self.master_pv_table[0] {
                -self.ab_search(state, true, gives_check, false, &mut next_pv_table, -beta, -alpha, depth - 1, ply + 1, node_count, seldepth)
            } else {
                let score = -self.ab_search(state, false, gives_check, true, &mut next_pv_table, -alpha - 1, -alpha, depth - 1, ply + 1, node_count, seldepth);

                if score > alpha && score < beta {
                    -self.ab_search(state, false, gives_check, false, &mut next_pv_table, -beta, -alpha, depth - 1, ply + 1, node_count, seldepth)
                } else {
                    score
                }
            };

            state.undo_mov(from, to, tp);

            if score >= beta {
                pv_table[0] = mov;
                pv_table[1..PV_TRACK_LENGTH].copy_from_slice(&next_pv_table[0..PV_TRACK_LENGTH-1]);

                return score
            }

            if score > alpha {
                alpha = score;

                pv_table[0] = mov;
                pv_table[1..PV_TRACK_LENGTH].copy_from_slice(&next_pv_table[0..PV_TRACK_LENGTH-1]);
            }

            self.root_node_mov_list[mov_index] = (depth, score, mov);
        }

        alpha
    }

    pub fn ab_search(&mut self, state: &mut State, on_pv: bool, in_check: bool, on_scout: bool, pv_table: &mut [u32], mut alpha: i32, beta: i32, mut depth: u8, ply: u8, node_count: &mut u64, seldepth: &mut u8) -> i32 {
        if self.abort {
            return 0
        }

        *node_count += 1;

        if (*node_count & 1023 == 0) && self.time_tracker.elapsed().as_millis() > self.max_time_millis {
            self.abort = true;
            return 0
        }

        if state.is_draw(ply) {
            return 0
        }

        if depth == 0 {
            return self.q_search(state, alpha, beta, ply, seldepth)
        }

        if self.mov_table.is_in_check(state, def::get_opposite_player(state.player)) {
            return eval::MATE_VAL - ply as i32
        }

        if !on_pv && !in_check && depth >= MIN_NM_DEPTH {
            state.do_null_mov();

            let gives_check = self.mov_table.is_in_check(state, state.player);

            let scout_score = -self.ab_search(state, false, gives_check, true, &mut NM_PV_TABLE, -beta, -beta+1, depth - NM_DEPTH_REDUCTION - 1, ply + 1, node_count, seldepth);
            state.undo_null_mov();

            if scout_score >= beta {
                return beta
            }
        }

        let mut mov_count = 0;

        let mut best_score = -eval::MATE_VAL;

        let mut pv_mov = 0;
        if on_pv && self.master_pv_table.len() > ply as usize {
            pv_mov = self.master_pv_table[ply as usize];
        }

        if pv_mov != 0 {
            let (_from, to, _tp, _promo) = util::decode_u32_mov(pv_mov);

            match self.search_mov(state, true, in_check, on_scout, pv_table, pv_mov, &mut mov_count, state.squares[to] != 0, &mut best_score, alpha, beta, depth, ply, node_count, seldepth) {
                Return(score) => return score,
                RaiseAlpha(score) => {
                    alpha = score;
                },
                Noop => (),
            }
        }

        if on_pv && pv_mov == 0 {
            depth += 1;
        }

        let mut hash_mov = 0;
        if !on_pv {
            match self.get_hash(state, depth) {
                Match(flag, score, mov) => {
                    let (from, to, _tp, _promo) = util::decode_u32_mov(mov);

                    if self.mov_table.is_mov_valid(state, from, to) {
                        hash_mov = mov;

                        if score != 0 && !eval::is_term_val(score) {
                            match flag {
                                HASH_TYPE_ALPHA => {
                                    if score <= alpha {
                                        return alpha
                                    } else {
                                        alpha = score;
                                    }
                                },
                                HASH_TYPE_BETA => {
                                    if score >= beta {
                                        return beta
                                    } else {
                                        alpha = score;
                                    }
                                },
                                _ => (),
                            }
                        }
                    }
                },
                MovOnly(mov) => {
                    let (from, to, _tp, _promo) = util::decode_u32_mov(mov);

                    if self.mov_table.is_mov_valid(state, from, to) {
                        hash_mov = mov;
                    }
                },
                _ => (),
            }
        }

        let player = state.player;
        let squares = state.squares;

        if hash_mov != 0 {
            let (_from, to, _tp, _promo) = util::decode_u32_mov(hash_mov);

            match self.search_mov(state, false, in_check, on_scout, pv_table, hash_mov, &mut mov_count, state.squares[to] != 0, &mut best_score, alpha, beta, depth, ply, node_count, seldepth) {
                Return(score) => return score,
                RaiseAlpha(score) => {
                    alpha = score;
                },
                Noop => (),
            }
        }

        let mut cap_list = [0; def::MAX_CAP_COUNT];
        let mut non_cap_list = [0; def::MAX_MOV_COUNT];

        self.mov_table.gen_reg_mov_list(state, &mut cap_list, &mut non_cap_list);

        let mut scored_cap_list = Vec::new();

        let (last_to, last_captured) = *(state.history_mov_stack.last().unwrap());

        for cap_index in 0..def::MAX_CAP_COUNT {
            let cap = cap_list[cap_index];

            if cap == 0 {
                break
            }

            if cap == pv_mov || cap == hash_mov {
                continue
            }

            let (from, to, _tp, promo) = util::decode_u32_mov(cap);

            let reply_score = if last_captured != 0 && to == last_to {
                eval::TERM_VAL - eval::val_of(squares[from])
            } else {
                0
            };

            if reply_score != 0 {
                scored_cap_list.push((reply_score, cap));
                continue
            }

            let exchange_score = eval::val_of(squares[to]) - eval::val_of(squares[from]) + eval::val_of(promo);
            scored_cap_list.push((exchange_score, cap));
        }

        scored_cap_list.sort_by(|(score_a, _), (score_b, _)| {
            score_b.partial_cmp(&score_a).unwrap()
        });

        for (_score, cap) in scored_cap_list {
            match self.search_mov(state, false, in_check, on_scout, pv_table, cap, &mut mov_count, true, &mut best_score, alpha, beta, depth, ply, node_count, seldepth) {
                Return(score) => return score,
                RaiseAlpha(score) => {
                    alpha = score;
                },
                Noop => (),
            }
        }

        let mut killer_mov = 0;
        let (_killer_score, saved_killer_mov) = self.killer_table[ply as usize].0;
        if saved_killer_mov != 0 && saved_killer_mov != pv_mov && saved_killer_mov != hash_mov && non_cap_list.contains(&saved_killer_mov) {
            killer_mov = saved_killer_mov;
        } else {
            let (_killer_score, saved_killer_mov) = self.killer_table[ply as usize].1;
            if saved_killer_mov != 0 && saved_killer_mov != pv_mov && saved_killer_mov != hash_mov && non_cap_list.contains(&saved_killer_mov) {
                killer_mov = saved_killer_mov;
            }
        }

        if killer_mov != 0 {
            let (_from, to, _tp, _promo) = util::decode_u32_mov(killer_mov);

            match self.search_mov(state, false, in_check, on_scout, pv_table, killer_mov, &mut mov_count, state.squares[to] != 0, &mut best_score, alpha, beta, depth, ply, node_count, seldepth) {
                Return(score) => return score,
                RaiseAlpha(score) => {
                    alpha = score;
                },
                Noop => (),
            }
        }

        if (player == def::PLAYER_W && (state.cas_rights & 0b1100 != 0)) || (player == def::PLAYER_B && (state.cas_rights & 0b0011 != 0)) {
            let mut cas_list = [0; def::MAX_CAS_COUNT];
            self.mov_table.gen_castle_mov_list(state, &mut cas_list);

            for cas_index in 0..def::MAX_CAS_COUNT {
                let cas_mov = cas_list[cas_index];

                if cas_mov == 0 {
                    break
                }

                if cas_mov == pv_mov || cas_mov == hash_mov || cas_mov == killer_mov {
                    continue
                }

                match self.search_mov(state, false, in_check, on_scout, pv_table, cas_mov, &mut mov_count, false, &mut best_score, alpha, beta, depth, ply, node_count, seldepth) {
                    Return(score) => return score,
                    RaiseAlpha(score) => {
                        alpha = score;
                    },
                    Noop => (),
                }
            }
        }

        let mut scored_non_cap_list = Vec::new();

        for non_cap_index in 0..def::MAX_MOV_COUNT {
            let non_cap = non_cap_list[non_cap_index];

            if non_cap == 0 {
                break
            }

            if non_cap == pv_mov || non_cap == hash_mov || non_cap == killer_mov {
                continue
            }

            let (from, to, _tp, promo) = util::decode_u32_mov(non_cap);
            if def::is_q(promo) {
                scored_non_cap_list.push((MAX_HISTORY_SCORE, non_cap));
                continue
            }

            scored_non_cap_list.push((self.history_table[from][to], non_cap));
        }

        scored_non_cap_list.sort_by(|(score_a, _), (score_b, _)| {
            score_b.partial_cmp(&score_a).unwrap()
        });

        for (_score, non_cap) in scored_non_cap_list {
            match self.search_mov(state, false, in_check, on_scout,pv_table, non_cap, &mut mov_count, false, &mut best_score, alpha, beta, depth, ply, node_count, seldepth) {
                Return(score) => return score,
                RaiseAlpha(score) => {
                    alpha = score;
                },
                Noop => (),
            }
        }

        if best_score < -eval::TERM_VAL {
            if !in_check && self.in_stale_mate(state) {
                return 0
            }
        }

        alpha
    }

    fn search_mov(&mut self, state: &mut State, on_pv: bool, in_check: bool, on_scout: bool, pv_table: &mut [u32], mov: u32, mov_count: &mut usize, is_capture: bool, best_score: &mut i32, alpha: i32, beta: i32, mut depth: u8, ply: u8, node_count: &mut u64, seldepth: &mut u8) -> SearchMovResult {
        if self.abort {
            return Return(0)
        }

        *mov_count += 1;

        let mut next_pv_table = [0; PV_TRACK_LENGTH];

        let (from, to, tp, promo) = util::decode_u32_mov(mov);

        state.do_mov(from, to, tp, promo);

        let gives_check = self.mov_table.is_in_check(state, state.player);

        if def::near_horizon(depth) {
            if gives_check || def::is_q(promo) {
                depth += 1;
            } else if def::is_p(state.squares[to]) {
                let bitboard = state.bitboard;
                if ((bitboard.w_all | bitboard.b_all) ^ (bitboard.w_pawn | bitboard.b_pawn)).count_ones() <= ENDGAME_PIECE_COUNT {
                    depth += 1;

                    if def::get_rank(state.player, to) == PROMO_HORIZON_RANK {
                        depth += 1;
                    }
                }
            }
        }

        let score = if depth > 1 && *mov_count > 2 && !in_check && !gives_check && !on_pv && !is_capture && !def::is_q(promo) {
            let score = -self.ab_search(state, on_pv, gives_check, true, &mut next_pv_table, -beta, -alpha, depth - 2, ply + 1, node_count, seldepth);
            if score > alpha {
                -self.ab_search(state, on_pv, gives_check, on_scout, &mut next_pv_table, -beta, -alpha, depth - 1, ply + 1, node_count, seldepth)
            } else {
                score
            }
        } else {
            -self.ab_search(state, on_pv, gives_check, on_scout, &mut next_pv_table, -beta, -alpha, depth - 1, ply + 1, node_count, seldepth)
        };

        state.undo_mov(from, to, tp);

        if score >= beta {
            if !is_capture {
                let (killer_score, _killer_mov) = self.killer_table[ply as usize].0;
                if score > killer_score || killer_score == 0 {
                    self.killer_table[ply as usize].1 = self.killer_table[ply as usize].0;
                    self.killer_table[ply as usize].0 = (score, mov);
                } else {
                    self.killer_table[ply as usize].1 = (score, mov);
                }

                let history_score_increment = depth as u64;
                let current_history_score = self.history_table[from][to];
                if current_history_score < MAX_HISTORY_SCORE {
                    self.history_table[from][to] = current_history_score + history_score_increment * history_score_increment;
                }
            }

            self.set_hash(state, depth, HASH_TYPE_BETA, score, mov);

            return Return(score)
        }

        if score > *best_score {
            *best_score = score;
        }

        if score > alpha {
            if !on_scout {
                pv_table[0] = mov;
                pv_table[1..PV_TRACK_LENGTH].copy_from_slice(&next_pv_table[0..PV_TRACK_LENGTH-1]);
            }

            self.set_hash(state, depth, HASH_TYPE_ALPHA, score, mov);

            return RaiseAlpha(score)
        }

        Noop
    }

    pub fn q_search(&mut self, state: &mut State, mut alpha: i32, beta: i32, ply: u8, seldepth: &mut u8) -> i32 {
        if self.abort {
            return 0
        }

        if self.mov_table.is_in_check(state, def::get_opposite_player(state.player)) {
            return eval::MATE_VAL - ply as i32
        }

        if ply > *seldepth {
            *seldepth = ply;
        }

        let score = eval::eval_state(state, &self.mov_table);

        if score >= beta {
            return score
        }

        if score > alpha {
            alpha = score;
        }

        let mut cap_list = [0; def::MAX_CAP_COUNT];
        self.mov_table.gen_capture_list(state, &mut cap_list);

        if cap_list[0] == 0 {
            return alpha
        }

        let squares = state.squares;
        let mut scored_cap_list = Vec::new();

        for mov_index in 0..256 {
            let cap = cap_list[mov_index];
            if cap == 0 {
                break
            }

            let (from, to, _tp, promo) = util::decode_u32_mov(cap);
            scored_cap_list.push((eval::val_of(squares[to]) + eval::val_of(promo) - eval::val_of(squares[from]), cap));
        }

        scored_cap_list.sort_by(|(score_a, _), (score_b, _)| {
            score_b.partial_cmp(&score_a).unwrap()
        });

        for (_score, cap) in scored_cap_list {
            let (from, to, tp, promo) = util::decode_u32_mov(cap);

            state.do_mov(from, to, tp, promo);
            let score = -self.q_search(state, -beta, -alpha, ply + 1, seldepth);
            state.undo_mov(from, to, tp);

            if score >= beta {
                return score
            }

            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }

    #[inline]
    pub fn get_hash(&self, state: &State, depth: u8) -> LookupResult {
        match self.depth_preferred_hash_table.get(state.hash_key, state.bitboard.w_all | state.bitboard.b_all, state.player, depth, state.cas_rights, state.enp_square) {
            NoMatch => {
                self.always_replace_hash_table.get(state.hash_key, state.bitboard.w_all | state.bitboard.b_all, state.player, depth, state.cas_rights, state.enp_square)
            },
            matched => matched
        }
    }

    #[inline]
    pub fn set_hash(&mut self, state: &State, depth: u8, hash_flag: u8, score: i32, mov: u32) {
        if !self.depth_preferred_hash_table.set(state.hash_key, state.bitboard.w_all | state.bitboard.b_all, state.player, depth, state.cas_rights, state.enp_square, hash_flag, score, mov) {
            self.always_replace_hash_table.set(state.hash_key, state.bitboard.w_all | state.bitboard.b_all, state.player, depth, state.cas_rights, state.enp_square, hash_flag, score, mov);
        }
    }

    #[inline]
    pub fn in_stale_mate(&self, state: &mut State) -> bool {
        let player = state.player;

        let mut cap_list = [0; def::MAX_CAP_COUNT];
        let mut non_cap_list = [0; def::MAX_MOV_COUNT];

        self.mov_table.gen_reg_mov_list(state, &mut cap_list, &mut non_cap_list);

        for cap_index in 0..def::MAX_CAP_COUNT {
            let cap = cap_list[cap_index];

            if cap == 0 {
                break
            }

            let (from, to, tp, promo) = util::decode_u32_mov(cap);
            state.do_mov(from, to, tp, promo);

            if !self.mov_table.is_in_check(state, player) {
                state.undo_mov(from, to, tp);
                return false
            }

            state.undo_mov(from, to, tp);
        }

        for non_cap_index in 0..def::MAX_MOV_COUNT {
            let non_cap = non_cap_list[non_cap_index];

            if non_cap == 0 {
                break
            }

            let (from, to, tp, promo) = util::decode_u32_mov(non_cap);
            state.do_mov(from, to, tp, promo);

            if !self.mov_table.is_in_check(state, player) {
                state.undo_mov(from, to, tp);
                return false
            }

            state.undo_mov(from, to, tp);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bitboard::BitMask,
        state::State,
        prng::XorshiftPrng,
        util,
    };

    #[test]
    fn test_in_stale_mate_1() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("1k6/1P6/1K6/8/8/8/8/8 b - - 0 1", &zob_keys, &bitmask);
        let search_engine = SearchEngine::new(65536);

        assert!(search_engine.in_stale_mate(&mut state));
    }

    #[test]
    fn test_in_stale_mate_2() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("8/8/8/2r5/8/k7/1p6/1K6 w - - 0 1", &zob_keys, &bitmask);
        let search_engine = SearchEngine::new(65536);

        assert!(search_engine.in_stale_mate(&mut state));
    }

    #[test]
    fn test_search_01() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("2k2r2/pp2br2/1np1p2q/2NpP2p/2PP2p1/1P1N4/P3Q1PP/3R1R1K b - - 8 27", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 5500, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("h6"));
        assert_eq!(to, util::map_sqr_notation_to_index("d2"));
    }

    #[test]
    fn test_search_2() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("r3r1k1/ppqb1ppp/8/4p1NQ/8/2P5/PP3PPP/R3R1K1 b - - 0 1", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 5500, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("d7"));
        assert_eq!(to, util::map_sqr_notation_to_index("f5"));
    }

    #[test]
    fn test_search_3() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("8/1k3ppp/8/5PPP/8/8/1K6/8 w - - 9 83", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 5500, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("g5"));
        assert_eq!(to, util::map_sqr_notation_to_index("g6"));
    }

    #[test]
    fn test_search_4() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("8/8/1r2b2p/8/8/2p5/2kR4/K7 b - - 3 56", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 500, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("c2"));
        assert_eq!(to, util::map_sqr_notation_to_index("b3"));
    }

    #[test]
    fn test_search_5() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("4r1k1/pp1Q1ppp/3B4/q2p4/5P1P/P3PbPK/1P1r4/2R5 b - - 3 5", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 500, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("d2"));
        assert_eq!(to, util::map_sqr_notation_to_index("h2"));
    }

    #[test]
    fn test_search_6() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("r5rk/2p1Nppp/3p3P/pp2p1P1/4P3/2qnPQK1/8/R6R w - - 1 0", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 500, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("h6"));
        assert_eq!(to, util::map_sqr_notation_to_index("g7"));
    }

    #[test]
    fn test_search_7() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("3r2k1/ppq2pp1/4p2p/3n3P/3N2P1/2P5/PP2QP2/K2R4 b - - 0 1", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 5500, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("d5"));
        assert_eq!(to, util::map_sqr_notation_to_index("c3"));
    }

    #[test]
    fn test_search_8() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("1r2k1r1/pbppnp1p/1b3P2/8/Q7/B1PB1q2/P4PPP/3R2K1 w - - 1 0", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 500, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("a4"));
        assert_eq!(to, util::map_sqr_notation_to_index("d7"));
    }

    #[test]
    fn test_search_9() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("8/8/8/5p1p/3k1P1P/5K2/8/8 b - - 1 59", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 5500, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("d4"));
        assert_eq!(to, util::map_sqr_notation_to_index("d3"));
    }

    #[test]
    fn test_search_10() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("1r4k1/7p/5np1/3p3n/8/2NB4/7P/3N1RK1 w - - 0 1", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 7500, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("c3"));
        assert_eq!(to, util::map_sqr_notation_to_index("d5"));
    }

    #[test]
    fn test_search_11() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("3rn2k/ppb2rpp/2ppqp2/5N2/2P1P3/1P5Q/PB3PPP/3RR1K1 w - - 0 1", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 7500, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("f5"));
        assert_eq!(to, util::map_sqr_notation_to_index("h6"));
    }

    #[test]
    fn test_search_12() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("r1bqkb1r/4npp1/p1p4p/1p1pP1B1/8/1B6/PPPN1PPP/R2Q1RK1 w kq - 0 1", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 15500, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("d2"));
        assert_eq!(to, util::map_sqr_notation_to_index("e4"));
    }

    #[test]
    fn test_search_13() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("3rr1k1/pp3pp1/1qn2np1/8/3p4/PP1R1P2/2P1NQPP/R1B3K1 b - - 0 1", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 25500, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("c6"));
        assert_eq!(to, util::map_sqr_notation_to_index("e5"));
    }

    #[test]
    fn test_search_14() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("1r1q1rk1/p1p2pbp/2pp1np1/6B1/4P3/2NQ4/PPP2PPP/3R1RK1 w - - 0 1", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 15500, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("e4"));
        assert_eq!(to, util::map_sqr_notation_to_index("e5"));
    }

    #[test]
    fn test_search_15() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("2r2rk1/1bqnbpp1/1p1ppn1p/pP6/N1P1P3/P2B1N1P/1B2QPP1/R2R2K1 b - - 0 1", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 15500, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("b7"));
        assert_eq!(to, util::map_sqr_notation_to_index("e4"));
    }

    #[test]
    fn test_search_16() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("8/1k6/3R4/2K5/8/8/8/8 w - - 0 1", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 15500, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("d6"));
        assert_eq!(to, util::map_sqr_notation_to_index("d7"));
    }

    #[test]
    fn test_search_perft_1() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &zob_keys, &bitmask);
        let search_engine = SearchEngine::new(65536);

        assert_eq!(20, search_engine.perft(&mut state, 1));
        assert_eq!(400, search_engine.perft(&mut state, 2));
        assert_eq!(8902, search_engine.perft(&mut state, 3));
    }

    #[test]
    fn test_search_perft_2() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", &zob_keys, &bitmask);
        let search_engine = SearchEngine::new(65536);

        assert_eq!(14, search_engine.perft(&mut state, 1));
        assert_eq!(191, search_engine.perft(&mut state, 2));
        assert_eq!(2812, search_engine.perft(&mut state, 3));
        assert_eq!(43238, search_engine.perft(&mut state, 4));
        assert_eq!(674624, search_engine.perft(&mut state, 5));
        assert_eq!(11030083, search_engine.perft(&mut state, 6));
        assert_eq!(178633661, search_engine.perft(&mut state, 7));
    }

    #[test]
    fn test_search_perft_3() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", &zob_keys, &bitmask);
        let search_engine = SearchEngine::new(65536);

        assert_eq!(44, search_engine.perft(&mut state, 1));
        assert_eq!(1486, search_engine.perft(&mut state, 2));
        assert_eq!(62379, search_engine.perft(&mut state, 3));
        assert_eq!(2103487, search_engine.perft(&mut state, 4));
        assert_eq!(89941194, search_engine.perft(&mut state, 5));
    }

    #[test]
    fn test_search_perft_4() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", &zob_keys, &bitmask);
        let search_engine = SearchEngine::new(65536);

        assert_eq!(46, search_engine.perft(&mut state, 1));
        assert_eq!(2079, search_engine.perft(&mut state, 2));
        assert_eq!(89890, search_engine.perft(&mut state, 3));
        assert_eq!(3894594, search_engine.perft(&mut state, 4));
        assert_eq!(164075551, search_engine.perft(&mut state, 5));
    }

    #[test]
    fn test_search_perft_5() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1", &zob_keys, &bitmask);
        let search_engine = SearchEngine::new(65536);

        assert_eq!(6, search_engine.perft(&mut state, 1));
        assert_eq!(264, search_engine.perft(&mut state, 2));
        assert_eq!(9467, search_engine.perft(&mut state, 3));
        assert_eq!(422333, search_engine.perft(&mut state, 4));
        assert_eq!(15833292, search_engine.perft(&mut state, 5));
    }

    #[test]
    fn test_search_perft_6() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("8/P1k5/K7/8/8/8/8/8 w - - 0 1", &zob_keys, &bitmask);
        let search_engine = SearchEngine::new(65536);

        assert_eq!(92683, search_engine.perft(&mut state, 6));
    }
}
