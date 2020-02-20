use crate::{
    def,
    eval,
    hashtable::{AlwaysReplaceHashTable, DepthPreferredHashTable, LookupResult, HASH_TYPE_ALPHA, HASH_TYPE_BETA},
    mov_gen::MoveGenerator,
    state::State,
    util,
};

use std::u64;

const PV_TRACK_LENGTH: usize = 32;
const MAX_DEPTH: u8 = 32;
const KILLER_TABLE_LENGTH: usize = 256;
const MIN_BRANCHING_FACTOR: u64 = 2;
const MAX_HISTORY_SCORE: u64 = u64::MAX >> 1;
const WINDOW_SIZE: i32 = 50;

pub enum SearchMovResult {
    Beta(i32),
    Alpha(i32),
    Noop,
}

use SearchMovResult::*;
use LookupResult::*;
use std::time::Instant;

pub struct SearchEngine {
    mov_generator: MoveGenerator,
    depth_preferred_hash_table: DepthPreferredHashTable,
    always_replace_hash_table: AlwaysReplaceHashTable,
    killer_table: [((i32, u32), (i32, u32)); KILLER_TABLE_LENGTH],
    w_history_table: [[u64; def::BOARD_SIZE]; def::BOARD_SIZE],
    b_history_table: [[u64; def::BOARD_SIZE]; def::BOARD_SIZE],
    master_pv_table: Vec<u32>,
    root_node_mov_list: Vec<(i32, u32)>,
    time_tracker: Instant,

    abort: bool,
    max_time_millis: u128,
}

impl SearchEngine {
    pub fn new(hash_size: usize) -> Self {
        SearchEngine {
            mov_generator: MoveGenerator::new(),
            depth_preferred_hash_table: DepthPreferredHashTable::new(hash_size >> 1),
            always_replace_hash_table: AlwaysReplaceHashTable::new(hash_size >> 1),
            killer_table: [((0, 0), (0, 0)); KILLER_TABLE_LENGTH],
            w_history_table: [[0; def::BOARD_SIZE]; def::BOARD_SIZE],
            b_history_table: [[0; def::BOARD_SIZE]; def::BOARD_SIZE],
            master_pv_table: Vec::new(),
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

    pub fn search(&mut self, state: &mut State, max_time_millis: u128) -> u32 {
        self.time_tracker = Instant::now();
        self.max_time_millis = max_time_millis;
        self.abort = false;
        self.master_pv_table = Vec::new();
        self.root_node_mov_list = Vec::new();
        self.killer_table = [((0, 0), (0, 0)); KILLER_TABLE_LENGTH];
        self.w_history_table = [[0; def::BOARD_SIZE]; def::BOARD_SIZE];
        self.b_history_table = [[0; def::BOARD_SIZE]; def::BOARD_SIZE];

        let player_sign = if state.player == def::PLAYER_W {
            1
        } else {
            -1
        };

        let max_score = eval::MATE_VAL * player_sign;
        let mut alpha = -max_score;
        let mut beta = max_score;

        let mut depth = 1;
        let mut best_mov = 0;
        let mut previous_node_count = 1;
        let mut time_after_previous_iter = self.time_tracker.elapsed().as_millis();

        loop {
            let mut node_count = 0;
            let mut seldepth = 0;

            let mut pv_table = [0; PV_TRACK_LENGTH];
            let score = self.root_search(state, &mut pv_table, alpha, beta, depth, 0, &mut node_count, &mut seldepth);

            if self.abort {
                break
            }

            let checkmate = score * player_sign > eval::TERM_VAL;

            if !pv_table.is_empty() && pv_table[0] != 0 {
                let time_taken_millis = self.time_tracker.elapsed().as_millis();
                let nps = node_count as u128 / (time_taken_millis / 1000).max(1);

                if checkmate {
                    println!("info score mate {} depth {} seldepth {} nodes {} nps {} time {} pv {}",
                        (eval::MATE_VAL - score.abs() + 1) / 2, depth, seldepth, node_count, nps, time_taken_millis, util::format_pv(&pv_table));
                } else {
                    println!("info score cp {} depth {} seldepth {} nodes {} nps {} time {} pv {}",
                    score * player_sign, depth, seldepth, node_count, nps, time_taken_millis, util::format_pv(&pv_table));
                }
            }

            if score * player_sign <= alpha * player_sign {
                alpha = -max_score;
                continue
            }

            if score * player_sign >= beta * player_sign {
                beta = max_score;
                continue
            }

            let pv_changed = best_mov != pv_table[0];

            best_mov = pv_table[0];

            if checkmate {
                break
            }

            let current_time_millis = self.time_tracker.elapsed().as_millis();

            if !pv_changed {
                let estimated_time_for_next_iter = (node_count / previous_node_count).max(MIN_BRANCHING_FACTOR) as u128 * (current_time_millis - time_after_previous_iter);

                if current_time_millis + estimated_time_for_next_iter > max_time_millis || current_time_millis > max_time_millis >> 1 {
                    break
                }
            }

            depth += 1;

            if depth > MAX_DEPTH {
                break
            }

            previous_node_count = node_count;
            time_after_previous_iter = current_time_millis;

            alpha = score - WINDOW_SIZE * player_sign;
            beta = score + WINDOW_SIZE * player_sign;

            self.master_pv_table = vec![0; pv_table.len()];
            self.master_pv_table.copy_from_slice(&pv_table);
        }

        best_mov
    }

    pub fn root_search(&mut self, state: &mut State, pv_table: &mut [u32], mut alpha: i32, beta: i32, depth: u8, ply: u8, node_count: &mut u64, seldepth: &mut u8) -> i32 {
        let player_sign = if state.player == def::PLAYER_W {
            1
        } else {
            -1
        };

        if self.root_node_mov_list.is_empty() {
            let (cap_list, non_cap_list) = self.mov_generator.gen_reg_mov_list(state);

            let squares = state.squares;

            for cap in cap_list {
                let (from, to, _tp, promo) = util::decode_u32_mov(cap);

                let initial_attacker = if promo == 0 {
                    squares[from]
                } else {
                    promo
                };

                let see_score = self.see(state, to, initial_attacker) * player_sign + eval::val_of(promo);
                self.root_node_mov_list.push((see_score, cap));
            }

            for non_cap in non_cap_list {
                let (_from, _to, _tp, promo) = util::decode_u32_mov(non_cap);
                self.root_node_mov_list.push((eval::val_of(promo), non_cap));
            }

            if (player_sign > 0 && (state.cas_rights & 0b1100 != 0)) || (player_sign < 0 && (state.cas_rights & 0b0011 != 0)) {
                let cas_mov_list = self.mov_generator.gen_castle_mov_list(state);
                for cas_mov in cas_mov_list {
                    self.root_node_mov_list.push((0, cas_mov));
                }
            }
        }

        self.root_node_mov_list.sort_by(|(score_a, _), (score_b, _)| {
            score_b.partial_cmp(&score_a).unwrap()
        });

        for mov_index in 0..self.root_node_mov_list.len() {
            let (_score, mov) = self.root_node_mov_list[mov_index];
            let (from, to, tp, promo) = util::decode_u32_mov(mov);

            let mut next_pv_table = [0; PV_TRACK_LENGTH];

            state.do_mov(from, to, tp, promo);
            let score = self.ab_search(state, mov_index == 0, &mut next_pv_table, beta, alpha, depth - 1, false, ply + 1, node_count, seldepth);
            state.undo_mov(from, to, tp);

            if score * player_sign >= beta * player_sign {
                return score
            }

            if score * player_sign > alpha * player_sign {
                alpha = score;

                pv_table[0] = mov;
                pv_table[1..PV_TRACK_LENGTH].copy_from_slice(&next_pv_table[0..PV_TRACK_LENGTH-1]);

                self.set_hash(state.hash_key, state.bitboard.w_all | state.bitboard.b_all, state.player, depth, state.cas_rights, state.enp_square, HASH_TYPE_ALPHA, score, mov);
            }

            self.root_node_mov_list[mov_index] = (score * player_sign, mov);
        }

        alpha
    }

    pub fn ab_search(&mut self, state: &mut State, on_pv: bool, pv_table: &mut [u32], mut alpha: i32, beta: i32, mut depth: u8, depth_reduced: bool, ply: u8, node_count: &mut u64, seldepth: &mut u8) -> i32 {
        if self.abort {
            return 0
        }

        *node_count += 1;

        if (*node_count & 1023 == 0) && self.time_tracker.elapsed().as_millis() > self.max_time_millis {
            self.abort = true;
            return 0
        }

        if state.is_draw() {
            return 0
        }

        let player_sign = if state.player == def::PLAYER_W {
            1
        } else {
            -1
        };

        let in_check = self.mov_generator.is_in_check(state);

        if in_check {
            depth += 1;

            if depth_reduced {
                depth += 1;
            }
        }

        if depth == 0 {
            return self.q_search(state, alpha, beta, ply, seldepth)
        }

        let mut mov_count = 0;

        let mut best_score = -eval::MATE_VAL * player_sign;

        let mut pv_mov = 0;
        if on_pv && self.master_pv_table.len() > ply as usize {
            pv_mov = self.master_pv_table[ply as usize];
        }

        if on_pv && pv_mov == 0 {
            depth += 1;
        }

        if pv_mov != 0 {
            let (_from, to, _tp, _promo) = util::decode_u32_mov(pv_mov);

            match self.search_mov(state, true, in_check, pv_table, pv_mov, &mut mov_count, state.squares[to] != 0, &mut best_score, alpha, beta, depth, ply, player_sign, node_count, seldepth) {
                Beta(score) => return score,
                Alpha(score) => {
                    alpha = score;
                },
                Noop => (),
            }
        }

        let mut hash_mov = 0;
        match self.get_hash(state.hash_key, state.bitboard.w_all | state.bitboard.b_all, state.player, depth, state.cas_rights, state.enp_square) {
            Match(flag, score, mov) => {
                hash_mov = mov;

                if !on_pv {
                    match flag {
                        HASH_TYPE_ALPHA => {
                            if score * player_sign <= alpha * player_sign {
                                return alpha
                            }
                        },
                        HASH_TYPE_BETA => {
                            if score * player_sign >= beta * player_sign {
                                return beta
                            }
                        },
                        _ => (),
                    }
                }
            },
            MovOnly(mov) => {
                hash_mov = mov;
            },
            _ => (),
        }

        let squares = state.squares;

        if hash_mov != 0 && hash_mov != pv_mov {
            let (from, to, _tp, _promo) = util::decode_u32_mov(hash_mov);

            if def::on_same_side(state.player, squares[from]) && !def::on_same_side(state.player, squares[to]) {
                match self.search_mov(state, false, in_check, pv_table, hash_mov, &mut mov_count, state.squares[to] != 0, &mut best_score, alpha, beta, depth, ply, player_sign, node_count, seldepth) {
                    Beta(score) => return score,
                    Alpha(score) => {
                        alpha = score;
                    },
                    Noop => (),
                }
            }
        }

        let (cap_list, non_cap_list) = self.mov_generator.gen_reg_mov_list(state);

        let mut scored_cap_list = Vec::new();

        let (last_to, last_captured) = *(state.history_mov_stack.last().unwrap());

        for cap in cap_list {
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
            if exchange_score > 0 || depth < 3 {
                scored_cap_list.push((exchange_score, cap));
            } else {
                let initial_attacker = if promo == 0 {
                    squares[from]
                } else {
                    promo
                };

                let see_score = self.see(state, to, initial_attacker) * player_sign + eval::val_of(promo);
                scored_cap_list.push((see_score, cap));
            }
        }

        scored_cap_list.sort_by(|(score_a, _), (score_b, _)| {
            score_b.partial_cmp(&score_a).unwrap()
        });

        for (_score, cap) in scored_cap_list {
            match self.search_mov(state, false, in_check, pv_table, cap, &mut mov_count, true, &mut best_score, alpha, beta, depth, ply, player_sign, node_count, seldepth) {
                Beta(score) => return score,
                Alpha(score) => {
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
            }  else if ply > 2 {
                let (_killer_score, saved_killer_mov) = self.killer_table[ply as usize - 2].0;
                if saved_killer_mov != 0 && saved_killer_mov != pv_mov && saved_killer_mov != hash_mov && non_cap_list.contains(&saved_killer_mov) {
                    killer_mov = saved_killer_mov;
                } else {
                    let (_killer_score, saved_killer_mov) = self.killer_table[ply as usize - 2].1;
                    if saved_killer_mov != 0 && saved_killer_mov != pv_mov && saved_killer_mov != hash_mov && non_cap_list.contains(&saved_killer_mov) {
                        killer_mov = saved_killer_mov;
                    }
                }
            }
        }

        if killer_mov != 0 {
            let (_from, to, _tp, _promo) = util::decode_u32_mov(killer_mov);

            match self.search_mov(state, false, in_check, pv_table, killer_mov, &mut mov_count, state.squares[to] != 0, &mut best_score, alpha, beta, depth, ply, player_sign, node_count, seldepth) {
                Beta(score) => return score,
                Alpha(score) => {
                    alpha = score;
                },
                Noop => (),
            }
        }

        if (player_sign > 0 && (state.cas_rights & 0b1100 != 0)) || (player_sign < 0 && (state.cas_rights & 0b0011 != 0)) {
            let castle_list = self.mov_generator.gen_castle_mov_list(state);
            for cas_mov in castle_list {
                if cas_mov == pv_mov || cas_mov == hash_mov && cas_mov != killer_mov {
                    continue
                }

                match self.search_mov(state, false, in_check, pv_table, cas_mov, &mut mov_count, false, &mut best_score, alpha, beta, depth, ply, player_sign, node_count, seldepth) {
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
            if non_cap == pv_mov || non_cap == hash_mov || non_cap == killer_mov {
                continue
            }

            let (from, to, _tp, promo) = util::decode_u32_mov(non_cap);
            if promo != 0 && def::is_q(promo) {
                scored_non_cap_list.push((MAX_HISTORY_SCORE, non_cap));
                continue
            }

            let history_score = if player_sign > 0 {
                self.w_history_table[from][to]
            } else {
                self.b_history_table[from][to]
            };

            scored_non_cap_list.push((history_score, non_cap));
        }

        scored_non_cap_list.sort_by(|(score_a, _), (score_b, _)| {
            score_b.partial_cmp(&score_a).unwrap()
        });

        for (_score, non_cap) in scored_non_cap_list {
            match self.search_mov(state, false, in_check, pv_table, non_cap, &mut mov_count, false, &mut best_score, alpha, beta, depth, ply, player_sign, node_count, seldepth) {
                Beta(score) => return score,
                Alpha(score) => {
                    alpha = score;
                },
                Noop => (),
            }
        }

        if best_score * player_sign < -eval::TERM_VAL {
            if pv_table[1] == 0 {
                pv_table[0] = 0;
            }

            if !in_check && self.in_stale_mate(state) {
                return 0
            }
        }

        alpha
    }

    #[inline]
    fn search_mov(&mut self, state: &mut State, on_pv: bool, in_check: bool, pv_table: &mut [u32], mov: u32, mov_count: &mut usize, is_capture: bool, best_score: &mut i32, alpha: i32, beta: i32, depth: u8, ply: u8, player_sign: i32, node_count: &mut u64, seldepth: &mut u8) -> SearchMovResult {
        if self.abort {
            return Beta(0)
        }

        *mov_count += 1;

        let (from, to, tp, promo) = util::decode_u32_mov(mov);

        if is_capture {
            let captured_piece = state.squares[to];
            if def::is_k(captured_piece) {
                pv_table[0] = 0;
                return Beta(player_sign * (eval::MATE_VAL - ply as i32))
            }
        }

        let mut next_pv_table = [0; PV_TRACK_LENGTH];

        state.do_mov(from, to, tp, promo);

        let score = if !in_check && !on_pv && depth > 1 && *mov_count > 2 {
            let score = self.ab_search(state, on_pv, &mut next_pv_table, beta, alpha, depth - 2, true, ply + 1, node_count, seldepth);
            if score * player_sign > alpha * player_sign {
                self.ab_search(state, on_pv, &mut next_pv_table, beta, alpha, depth - 1, false, ply + 1, node_count, seldepth)
            } else {
                score
            }
        } else {
            self.ab_search(state, on_pv, &mut next_pv_table, beta, alpha, depth - 1, false, ply + 1, node_count, seldepth)
        };

        state.undo_mov(from, to, tp);

        let signed_score = score * player_sign;

        if signed_score >= beta * player_sign {
            if !is_capture {
                let (killer_score, _killer_mov) = self.killer_table[ply as usize].0;
                if score * player_sign > killer_score * player_sign || killer_score == 0 {
                    self.killer_table[ply as usize].1 = self.killer_table[ply as usize].0;
                    self.killer_table[ply as usize].0 = (score, mov);
                } else {
                    self.killer_table[ply as usize].1 = (score, mov);
                }

                if player_sign > 0 {
                    let current_history_score = self.w_history_table[from][to];
                    if current_history_score < MAX_HISTORY_SCORE {
                        self.w_history_table[from][to] = current_history_score + (depth * depth) as u64;
                    }
                } else {
                    let current_history_score = self.b_history_table[from][to];
                    if current_history_score < MAX_HISTORY_SCORE {
                        self.b_history_table[from][to] = current_history_score + (depth * depth) as u64;
                    }
                }
            }

            self.set_hash(state.hash_key, state.bitboard.w_all | state.bitboard.b_all, state.player, depth, state.cas_rights, state.enp_square, HASH_TYPE_BETA, score, mov);

            return Beta(score)
        }

        if signed_score > *best_score * player_sign {
            *best_score = score;
        }

        if signed_score > alpha * player_sign {
            pv_table[0] = mov;
            pv_table[1..PV_TRACK_LENGTH].copy_from_slice(&next_pv_table[0..PV_TRACK_LENGTH-1]);

            if !is_capture {
                if player_sign > 0 {
                    let current_history_score = self.w_history_table[from][to];
                    if current_history_score < MAX_HISTORY_SCORE {
                        self.w_history_table[from][to] = current_history_score + depth as u64;
                    }
                } else {
                    let current_history_score = self.b_history_table[from][to];
                    if current_history_score < MAX_HISTORY_SCORE {
                        self.b_history_table[from][to] = current_history_score + depth as u64;
                    }
                }
            }

            self.set_hash(state.hash_key, state.bitboard.w_all | state.bitboard.b_all, state.player, depth, state.cas_rights, state.enp_square, HASH_TYPE_ALPHA, score, mov);

            return Alpha(score)
        }

        Noop
    }

    #[inline]
    pub fn get_hash(&self, hash_key: u64, bitmask: u64, player: u8, depth: u8, cas_rights: u8, enp_sqr: usize) -> LookupResult {        
        match self.depth_preferred_hash_table.get(hash_key, bitmask, player, depth, cas_rights, enp_sqr) {
            NoMatch => {
                self.always_replace_hash_table.get(hash_key, bitmask, player, depth, cas_rights, enp_sqr)
            },
            matched => matched
        }
    }

    #[inline]
    pub fn set_hash(&mut self, hash_key: u64, bitmask: u64, player: u8, depth: u8, cas_rights: u8, enp_sqr: usize, hash_flag: u8, score: i32, mov: u32) {
        if !self.depth_preferred_hash_table.set(hash_key, bitmask, player, depth, cas_rights, enp_sqr, hash_flag, score, mov) {
            self.always_replace_hash_table.set(hash_key, bitmask, player, depth, cas_rights, enp_sqr, hash_flag, score, mov);
        }
    }

    #[inline]
    pub fn in_stale_mate(&self, state: &mut State) -> bool {
        let (cap_list, non_cap_list) = self.mov_generator.gen_reg_mov_list(state);

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

    pub fn q_search(&mut self, state: &mut State, mut alpha: i32, beta: i32, ply: u8, seldepth: &mut u8) -> i32 {
        if self.abort {
            return 0
        }

        let player_sign = if state.player == def::PLAYER_W {
            1
        } else {
            -1
        };

        if ply > *seldepth {
            *seldepth = ply;
        }

        let score = eval::eval_state(state, &self.mov_generator);

        if score * player_sign >= beta * player_sign {
            return score
        }

        if score * player_sign > alpha * player_sign {
            alpha = score;
        }

        let cap_list = self.mov_generator.gen_capture_list(state);

        if cap_list.is_empty() {
            return alpha
        }

        let squares = state.squares;
        let mut scored_cap_list = Vec::new();

        for cap in cap_list {
            let (from, to, _tp, promo) = util::decode_u32_mov(cap);
            scored_cap_list.push((eval::val_of(squares[to]) - eval::val_of(squares[from]) + eval::val_of(promo), cap));
        }

        scored_cap_list.sort_by(|(score_a, _), (score_b, _)| {
            score_b.partial_cmp(&score_a).unwrap()
        });

        for (_score, cap) in scored_cap_list {
            let (from, to, tp, promo) = util::decode_u32_mov(cap);

            let captured_piece = state.squares[to];
            if def::is_k(captured_piece) {
                return player_sign * (eval::MATE_VAL - ply as i32)
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
    fn test_see_1() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("4q1kr/ppn1rp1p/n1p1PB2/5P2/2B1Q2P/2N3p1/PPP1b1P1/4R2K b - - 1 1", &zob_keys, &bitmask);
        let search_engine = SearchEngine::new(65536);

        assert_eq!(145, search_engine.see(&state, util::map_sqr_notation_to_index("e6"), def::BN));
        assert_eq!(-100, search_engine.see(&state, util::map_sqr_notation_to_index("e6"), def::BP));
        assert_eq!(325, search_engine.see(&state, util::map_sqr_notation_to_index("e6"), def::BR));
    }

    #[test]
    fn test_see_2() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("r5kr/1b1pR1p1/ppq1N2p/5P1n/3Q4/B6B/P5PP/5RK1 w - - 1 1", &zob_keys, &bitmask);
        let search_engine = SearchEngine::new(65536);

        assert_eq!(-555, search_engine.see(&state, util::map_sqr_notation_to_index("g7"), def::WQ));
        assert_eq!(100, search_engine.see(&state, util::map_sqr_notation_to_index("g7"), def::WN));
        assert_eq!(-80, search_engine.see(&state, util::map_sqr_notation_to_index("g7"), def::WR));
    }

    #[test]
    fn test_see_3() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("r2q1kn1/p2b1rb1/1p1p1pp1/2pPp3/1PP1Pn2/PRNBB1K1/3QNPPP/5R2 w - - 0 1", &zob_keys, &bitmask);
        let search_engine = SearchEngine::new(65536);

        assert_eq!(100, search_engine.see(&state, util::map_sqr_notation_to_index("f4"), def::WN));
        assert_eq!(95, search_engine.see(&state, util::map_sqr_notation_to_index("f4"), def::WB));
        assert_eq!(-19555, search_engine.see(&state, util::map_sqr_notation_to_index("f4"), def::WK));
    }

    #[test]
    fn test_see_4() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("r4kn1/p2bprb1/Bp1p1ppP/2pP4/1PP1Pn2/PRNB2K1/2QN1PPq/5R2 w - - 0 1", &zob_keys, &bitmask);
        let search_engine = SearchEngine::new(65536);

        assert_eq!(-19655, search_engine.see(&state, util::map_sqr_notation_to_index("f4"), def::WK));
    }

    #[test]
    fn test_see_5() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("rn1qkbnr/pppbpppp/8/3p4/4P3/5Q2/PPPP1PPP/RNB1KBNR w KQkq - 2 3", &zob_keys, &bitmask);
        let search_engine = SearchEngine::new(65536);

        assert_eq!(100, search_engine.see(&state, util::map_sqr_notation_to_index("d5"), def::WP));
    }

    #[test]
    fn test_q_search_1() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("r5kr/1b1pR1p1/p1q1N2p/5P1n/3Q4/B7/P5PP/5RK1 w - - 1 1", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        assert_eq!(169, search_engine.q_search(&mut state, -20000, 20000, 0, &mut 0));
    }

    #[test]
    fn test_q_search_2() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("2k2r2/pp2br2/1np1p2q/2NpP2p/2PP2p1/1P1N4/P3Q1PP/3R1R1K b - - 8 27", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        assert_eq!(14, search_engine.q_search(&mut state, 20000, -20000, 0, &mut 0));
    }

    #[test]
    fn test_q_search_3() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        assert_eq!(0, search_engine.q_search(&mut state, -20000, 20000, 0, &mut 0));
    }

    #[test]
    fn test_q_search_4() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("2k5/pp2b3/1np1p3/2NpP2p/3P2p1/2PN4/PP4PP/5q1K w - - 8 27", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        assert_eq!(-971, search_engine.q_search(&mut state, -20000, 20000, 0, &mut 0));
    }

    #[test]
    fn test_q_search_5() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("2r4k/1R5p/8/p1p5/P1Pp1p2/3R3P/KP3r2/8 w - - 0 40", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        assert_eq!(-250, search_engine.q_search(&mut state, -20000, 20000, 0, &mut 0));
    }

    #[test]
    fn test_search_puzzle_01() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("2k2r2/pp2br2/1np1p2q/2NpP2p/2PP2p1/1P1N4/P3Q1PP/3R1R1K b - - 8 27", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 5500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("h6"));
        assert_eq!(to, util::map_sqr_notation_to_index("d2"));
    }

    #[test]
    fn test_search_puzzle_2() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("r3r1k1/ppqb1ppp/8/4p1NQ/8/2P5/PP3PPP/R3R1K1 b - - 0 1", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 5500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("d7"));
        assert_eq!(to, util::map_sqr_notation_to_index("f5"));
    }

    #[test]
    fn test_search_puzzle_3() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("8/1k3ppp/8/5PPP/8/8/1K6/8 w - - 9 83", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 5500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("g5"));
        assert_eq!(to, util::map_sqr_notation_to_index("g6"));
    }

    #[test]
    fn test_search_puzzle_4() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("8/8/1r2b2p/8/8/2p5/2kR4/K7 b - - 3 56", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 5500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("c2"));
        assert_eq!(to, util::map_sqr_notation_to_index("b3"));
    }

    #[test]
    fn test_search_puzzle_5() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("4r1k1/pp1Q1ppp/3B4/q2p4/5P1P/P3PbPK/1P1r4/2R5 b - - 3 5", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 5500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("d2"));
        assert_eq!(to, util::map_sqr_notation_to_index("h2"));
    }

    #[test]
    fn test_search_puzzle_6() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("r5rk/2p1Nppp/3p3P/pp2p1P1/4P3/2qnPQK1/8/R6R w - - 1 0", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 5500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("h6"));
        assert_eq!(to, util::map_sqr_notation_to_index("g7"));
    }

    #[test]
    fn test_search_puzzle_7() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("3r2k1/ppq2pp1/4p2p/3n3P/3N2P1/2P5/PP2QP2/K2R4 b - - 0 1", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 5500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("d5"));
        assert_eq!(to, util::map_sqr_notation_to_index("c3"));
    }

    #[test]
    fn test_search_puzzle_8() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("1r2k1r1/pbppnp1p/1b3P2/8/Q7/B1PB1q2/P4PPP/3R2K1 w - - 1 0", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 5500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("a4"));
        assert_eq!(to, util::map_sqr_notation_to_index("d7"));
    }

    #[test]
    fn test_search_puzzle_9() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("8/8/8/5p1p/3k1P1P/5K2/8/8 b - - 1 59", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 5500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("d4"));
        assert_eq!(to, util::map_sqr_notation_to_index("d3"));
    }

    #[test]
    fn test_search_puzzle_10() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("2r3k1/pppR1pp1/4p3/4P1P1/5P2/1P4K1/P1P5/8 w - - 0 1", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 7500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("g5"));
        assert_eq!(to, util::map_sqr_notation_to_index("g6"));
    }

    #[test]
    fn test_search_puzzle_11() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("3rn2k/ppb2rpp/2ppqp2/5N2/2P1P3/1P5Q/PB3PPP/3RR1K1 w - - 0 1", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 7500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("f5"));
        assert_eq!(to, util::map_sqr_notation_to_index("h6"));
    }

    #[test]
    fn test_search_puzzle_12() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("r1bqkb1r/4npp1/p1p4p/1p1pP1B1/8/1B6/PPPN1PPP/R2Q1RK1 w kq - 0 1", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 15500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("d2"));
        assert_eq!(to, util::map_sqr_notation_to_index("e4"));
    }

    #[test]
    fn test_search_puzzle_13() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("1nk1r1r1/pp2n1pp/4p3/q2pPp1N/b1pP1P2/B1P2R2/2P1B1PP/R2Q2K1 w - - 0 1", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 15500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("h5"));
        assert_eq!(to, util::map_sqr_notation_to_index("f6"));
    }

    #[test]
    fn test_search_puzzle_14() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("1r1q1rk1/p1p2pbp/2pp1np1/6B1/4P3/2NQ4/PPP2PPP/3R1RK1 w - - 0 1", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 15500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("e4"));
        assert_eq!(to, util::map_sqr_notation_to_index("e5"));
    }

    #[test]
    fn test_search_puzzle_15() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("2r2rk1/1bqnbpp1/1p1ppn1p/pP6/N1P1P3/P2B1N1P/1B2QPP1/R2R2K1 b - - 0 1", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 15500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("b7"));
        assert_eq!(to, util::map_sqr_notation_to_index("e4"));
    }

    #[test]
    fn test_search_puzzle_16() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("q7/2R1prk1/p2p1p1p/1p3Bp1/r3P3/4B3/P1Q5/6KR w - - 9 33", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 5500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("h1"));
        assert_eq!(to, util::map_sqr_notation_to_index("h6"));
    }

    #[test]
    fn test_search_endgame_1() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("8/2k5/2pR4/1pPp4/p7/P1PK1P2/1P6/8 w - - 5 52", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 5500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("f3"));
        assert_eq!(to, util::map_sqr_notation_to_index("f4"));
    }

    #[test]
    fn test_search_endgame_2() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("5rk1/2PQpppp/5b2/p7/8/4P1P1/2q2P1P/3R2K1 w - - 0 1", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 5500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("d7"));
        assert_eq!(to, util::map_sqr_notation_to_index("d8"));
    }

    #[test]
    fn test_search_endgame_3() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("8/k1K5/8/1P6/8/8/8/8 w - - 0 1", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 5500);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("b5"));
        assert_eq!(to, util::map_sqr_notation_to_index("b6"));
    }
}
