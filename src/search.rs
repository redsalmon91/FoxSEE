/*
 * Copyright (C) 2020 Zixiao Han
 */

use crate::{
    bitboard,
    def,
    eval,
    hashtable::{AlwaysReplaceHashTable, DepthPreferredHashTable, LookupResult, HASH_TYPE_ALPHA, HASH_TYPE_BETA, HASH_TYPE_EXACT},
    mov_table,
    state::State,
    util,
};

const PV_TRACK_LENGTH: usize = 128;

const MAX_HISTORY_SCORE: i32 = 10000;
const MAX_NON_CAP_SCORE: i32 = 20000;

const WINDOW_SIZE: i32 = 50;

const NM_DEPTH: u8 = 6;
const NM_R: u8 = 2;

const MULTICUT_DEPTH: u8 = 6;
const MULTICUT_R: u8 = 2;
const MULTICUT_MOV_COUNT: u8 = 3;
const MULTICUT_CUT_COUNT: u8 = 2;

const IID_R: u8 = 2;

const TIME_CHECK_INTEVAL: u64 = 4095;

const EMPTY_HISTORY_PV_TABLE: [u32; PV_TRACK_LENGTH] = [0; PV_TRACK_LENGTH];
const EMPTY_PV_TABLE: [u32; PV_TRACK_LENGTH] = [0; PV_TRACK_LENGTH];

use std::time::Instant;
use LookupResult::*;

pub struct SearchEngine {
    depth_preferred_hash_table: DepthPreferredHashTable,
    always_replace_hash_table: AlwaysReplaceHashTable,
    primary_killer_table: [(i32, u32); PV_TRACK_LENGTH],
    secondary_killer_table: [(i32, u32); PV_TRACK_LENGTH],
    index_history_table: [[i32; def::BOARD_SIZE]; def::BOARD_SIZE],
    piece_history_table: [[i32; def::BOARD_SIZE]; def::PIECE_CODE_RANGE],
    root_node_mov_list: Vec<(u8, i32, u32, [u32; PV_TRACK_LENGTH])>,
    time_tracker: Instant,

    abort: bool,
    max_time_millis: u128,
    recent_search_score: i32,
}

impl SearchEngine {
    pub fn new(hash_size: usize) -> Self {
        SearchEngine {
            depth_preferred_hash_table: DepthPreferredHashTable::new(hash_size >> 1),
            always_replace_hash_table: AlwaysReplaceHashTable::new(hash_size >> 1),
            primary_killer_table: [(0, 0); PV_TRACK_LENGTH],
            secondary_killer_table: [(0, 0); PV_TRACK_LENGTH],
            index_history_table: [[0; def::BOARD_SIZE]; def::BOARD_SIZE],
            piece_history_table: [[0; def::BOARD_SIZE]; def::PIECE_CODE_RANGE],
            root_node_mov_list: Vec::new(),
            time_tracker: Instant::now(),

            abort: false,
            max_time_millis: 0,
            recent_search_score: 0,
        }
    }

    pub fn stop(&mut self) {
        self.abort = true;
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

    pub fn search(&mut self, state: &mut State, max_time_millis: u128, max_depth: u8) -> u32 {
        self.time_tracker = Instant::now();
        self.max_time_millis = max_time_millis;
        self.abort = false;
        self.root_node_mov_list.clear();

        self.primary_killer_table = [(0, 0); PV_TRACK_LENGTH];
        self.secondary_killer_table = [(0, 0); PV_TRACK_LENGTH];
        self.index_history_table = [[0; def::BOARD_SIZE]; def::BOARD_SIZE];
        self.piece_history_table = [[0; def::BOARD_SIZE]; def::PIECE_CODE_RANGE];

        let mut alpha = self.recent_search_score - WINDOW_SIZE;
        let mut beta = self.recent_search_score + WINDOW_SIZE;

        let mut depth = 1;
        let mut best_mov = 0;
        let mut accumulated_time_taken = 0;

        loop {
            let mut node_count = 0;
            let mut seldepth = 0;

            let mut pv_table = [0; PV_TRACK_LENGTH];
            let score = self.root_search(state, &mut pv_table, alpha, beta, depth, 0, &mut node_count, &mut seldepth);

            if self.abort {
                break
            }

            if score <= alpha {
                alpha = -eval::MATE_VAL;

                continue
            }

            if score >= beta {
                beta = eval::MATE_VAL;

                continue
            }

            self.validate_pv(state, &mut pv_table, 0);

            let checkmate = score > eval::TERM_VAL;

            let total_time_taken = self.time_tracker.elapsed().as_millis();

            if !pv_table.is_empty() && pv_table[0] != 0 {
                let iter_time_taken_millis = total_time_taken - accumulated_time_taken;
                let nps = node_count as u128 / (iter_time_taken_millis / 1000).max(1);

                if checkmate {
                    println!("info score mate {} depth {} seldepth {} nodes {} nps {} time {} pv {}", (eval::MATE_VAL - score.abs() + 1) / 2, depth, seldepth, node_count, nps, total_time_taken, util::format_pv(&pv_table));
                } else {
                    println!("info score cp {} depth {} seldepth {} nodes {} nps {} time {} pv {}", score, depth, seldepth, node_count, nps, total_time_taken, util::format_pv(&pv_table));
                }
            }

            let pv_changed = best_mov != pv_table[0];

            if pv_table[0] != 0 {
                best_mov = pv_table[0];
            }

            if checkmate {
                break
            }

            if !pv_changed {
                if total_time_taken - accumulated_time_taken > max_time_millis >> 1 {
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
        }

        best_mov
    }

    fn root_search(&mut self, state: &mut State, pv_table: &mut [u32], mut alpha: i32, beta: i32, depth: u8, ply: u8, node_count: &mut u64, seldepth: &mut u8) -> i32 {
        if self.root_node_mov_list.is_empty() {
            let mut mov_list = [0; def::MAX_MOV_COUNT];

            mov_table::gen_reg_mov_list(state, &mut mov_list);

            for mov_index in 0..def::MAX_CAP_COUNT {
                let mov = mov_list[mov_index];

                if mov == 0 {
                    break
                }

                let (from, to, tp, promo) = util::decode_u32_mov(mov);

                if state.squares[to] != 0 {
                    self.root_node_mov_list.push((depth, see(state, from, to, tp, promo), mov, [0; PV_TRACK_LENGTH]));
                } else {
                    self.root_node_mov_list.push((depth, -eval::val_of(state.squares[from]), mov, [0; PV_TRACK_LENGTH]));
                }
            }
        }

        self.root_node_mov_list.sort_by(|(depth_a, score_a, _, _), (depth_b, score_b, _, _)| {
            if depth_a == depth_b {
                score_b.partial_cmp(&score_a).unwrap()
            } else {
                depth_b.partial_cmp(&depth_a).unwrap()
            }
        });

        let mut pv_found = false;

        for mov_index in 0..self.root_node_mov_list.len() {
            if self.abort {
                return alpha
            }

            let (_depth, _score, mov, mut history_pv_table) = self.root_node_mov_list[mov_index];
            let (from, to, tp, promo) = util::decode_u32_mov(mov);

            let mut next_pv_table = [0; PV_TRACK_LENGTH];

            state.do_mov(from, to, tp, promo);

            let gives_check = mov_table::is_in_check(state, state.player);

            let score = if !pv_found  {
                -self.ab_search(state, true, gives_check, false, &mut history_pv_table, &mut next_pv_table, -beta, -alpha, depth - 1, ply + 1, node_count, seldepth)
            } else {
                let score = -self.ab_search(state, true, gives_check, false, &mut history_pv_table, &mut next_pv_table, -alpha - 1, -alpha, depth - 1, ply + 1, node_count, seldepth);

                if score > alpha && score < beta {
                    -self.ab_search(state, true, gives_check, false, &mut history_pv_table, &mut next_pv_table, -beta, -alpha, depth - 1, ply + 1, node_count, seldepth)
                } else {
                    score
                }
            };

            state.undo_mov(from, to, tp);

            if self.abort {
                break
            }

            let mut updated_history_pv_table = [0; PV_TRACK_LENGTH];
            updated_history_pv_table[0] = mov;
            updated_history_pv_table[1..PV_TRACK_LENGTH].copy_from_slice(&next_pv_table[0..PV_TRACK_LENGTH-1]);
            self.root_node_mov_list[mov_index] = (depth, score, mov, updated_history_pv_table);

            if score >= beta {
                return score
            }

            if score > alpha {
                alpha = score;
                pv_found = true;

                pv_table[0] = mov;
                pv_table[1..PV_TRACK_LENGTH].copy_from_slice(&next_pv_table[0..PV_TRACK_LENGTH-1]);
            }
        }

        alpha
    }

    fn ab_search(&mut self, state: &mut State, on_pv: bool, in_check: bool, on_cut: bool, history_pv_table: &mut [u32], pv_table: &mut [u32], mut alpha: i32, mut beta: i32, depth: u8, ply: u8, node_count: &mut u64, seldepth: &mut u8) -> i32 {
        if self.abort {
            return alpha
        }

        *node_count += 1;

        if (*node_count & TIME_CHECK_INTEVAL == 0) && self.time_tracker.elapsed().as_millis() > self.max_time_millis {
            self.abort = true;
            return alpha
        }

        if mov_table::is_in_check(state, def::get_opposite_player(state.player)) {
            return eval::MATE_VAL - ply as i32
        }

        if state.is_draw(ply) {
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
        if on_pv && (ply as usize) < PV_TRACK_LENGTH {
            pv_mov = history_pv_table[ply as usize];
        }

        let mut has_potential_cut = false;

        if pv_mov == 0 {
            match self.get_hash(state, depth) {
                Match(flag, score, mov) => {
                    let (from, to, _tp, _promo) = util::decode_u32_mov(mov);

                    if mov_table::is_mov_valid(state, from, to) {
                        pv_mov = mov;

                        if !on_pv {
                            match flag {
                                HASH_TYPE_EXACT => {
                                    return score
                                },
                                HASH_TYPE_ALPHA => {
                                    if score <= alpha {
                                        return alpha
                                    }
                                },
                                HASH_TYPE_BETA => {
                                    if score >= beta {
                                        return beta
                                    }

                                    has_potential_cut = true;
                                },
                                _ => (),
                            }
                        }
                    }
                },
                MovOnly(mov) => {
                    let (from, to, _tp, _promo) = util::decode_u32_mov(mov);

                    if mov_table::is_mov_valid(state, from, to) {
                        pv_mov = mov;
                    }
                },
                _ => (),
            }
        }

        if depth == 0 {
            return self.q_search(state, alpha, beta, ply, node_count, seldepth)
        }

        let mut under_threat = false;

        if beta - alpha == 1 && !on_cut && !in_check {
            let in_extreme_endgame = state.bitboard.w_pawn == 0 || state.bitboard.b_pawn == 0;

            if !in_extreme_endgame && depth >= NM_DEPTH {
                let depth_reduction = if depth > NM_DEPTH {
                    NM_R + 1
                } else {
                    NM_R
                };

                state.do_null_mov();
                let scout_score = -self.ab_search(state, false, false, true, &mut EMPTY_HISTORY_PV_TABLE, &mut EMPTY_PV_TABLE, -beta, -beta+1, depth - depth_reduction - 1, ply + 1, node_count, seldepth);
                state.undo_null_mov();

                if self.abort {
                    return alpha
                }

                if scout_score >= beta {
                    return beta
                } else if scout_score < -eval::TERM_VAL {
                    under_threat = true;
                }
            }

            let opponent_has_promoting_pawn =
                (state.player == def::PLAYER_W && state.bitboard.b_pawn & bitboard::BP_PROMO_PAWNS_MASK != 0)
                || (state.player == def::PLAYER_B && state.bitboard.w_pawn & bitboard::WP_PROMO_PAWNS_MASK != 0);

            if !in_extreme_endgame && !opponent_has_promoting_pawn && !under_threat && depth <= 6 {
                if eval::eval_materials(state) - get_futility_margin(depth) >= beta {
                    return beta
                }
            }
        }

        if on_pv && pv_mov == 0 && depth >= 5 {
            let mut next_pv_table = [0; PV_TRACK_LENGTH];
            self.ab_search(state, true, in_check, false, history_pv_table, &mut next_pv_table, alpha, beta, depth - IID_R, ply, node_count, seldepth);

            if self.abort {
                return alpha
            }

            pv_mov = next_pv_table[0];

            if (ply as usize) < PV_TRACK_LENGTH {
                history_pv_table[ply as usize..PV_TRACK_LENGTH].copy_from_slice(&next_pv_table[0..PV_TRACK_LENGTH - ply as usize]);
            }
        }

        let mut mov_count = 0;
        let mut best_score = -eval::MATE_VAL;
        let mut best_mov = 0;
        let mut pv_found = false;

        if pv_mov != 0 {
            mov_count += 1;

            let (from, to, tp, promo) = util::decode_u32_mov(pv_mov);

            let mut next_pv_table = [0; PV_TRACK_LENGTH];

            let is_threating_pawn_mov = is_threating_pawn(state, from, to);

            state.do_mov(from, to, tp, promo);

            let gives_check = mov_table::is_in_check(state, state.player);

            let mut depth = depth;
            if under_threat || gives_check || is_threating_pawn_mov {
                depth += 1;
            }

            let score = -self.ab_search(state, on_pv, gives_check, false, history_pv_table, &mut next_pv_table, -beta, -alpha, depth - 1, ply + 1, node_count, seldepth);

            state.undo_mov(from, to, tp);

            if self.abort {
                return alpha
            }

            if score >= beta {
                pv_table[0] = pv_mov;
                pv_table[1..PV_TRACK_LENGTH].copy_from_slice(&next_pv_table[0..PV_TRACK_LENGTH-1]);

                self.set_hash(state, depth, HASH_TYPE_BETA, score, pv_mov);

                return score
            }

            if score > alpha {
                alpha = score;
                best_mov = pv_mov;
                pv_found = true;

                pv_table[0] = pv_mov;
                pv_table[1..PV_TRACK_LENGTH].copy_from_slice(&next_pv_table[0..PV_TRACK_LENGTH-1]);
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

            if state.squares[to] != 0 {
                ordered_mov_list.push((MAX_NON_CAP_SCORE + see(state, from, to, tp, promo), mov));
            } else if promo != 0 {
                ordered_mov_list.push((MAX_NON_CAP_SCORE + eval::val_of(promo), mov));
            } else if mov == primary_killer {
                ordered_mov_list.push((MAX_NON_CAP_SCORE - 1, mov));
            } else if mov == secondary_killer {
                ordered_mov_list.push((MAX_NON_CAP_SCORE - 2, mov));
            } else {
                ordered_mov_list.push((self.index_history_table[from][to] + self.piece_history_table[state.squares[from] as usize][to], mov));
            }
        }

        ordered_mov_list.sort_by(|(score_a, _), (score_b, _)| {
            score_b.partial_cmp(&score_a).unwrap()
        });

        if (pv_mov == 0 || has_potential_cut) && beta - alpha == 1 && !on_cut && !in_check && depth >= MULTICUT_DEPTH && self.multicut_search(state, &ordered_mov_list, beta, depth, ply, node_count, seldepth) {
            return beta
        }

        if self.abort {
            return alpha
        }

        for (score, mov) in ordered_mov_list {
            mov_count += 1;

            let mut next_pv_table = [0; PV_TRACK_LENGTH];

            let (from, to, tp, promo) = util::decode_u32_mov(mov);

            let is_capture = state.squares[to] != 0;
            let is_win_capture = is_capture && score >= MAX_NON_CAP_SCORE;
            let is_threating_pawn_mov = is_threating_pawn(state, from, to);

            state.do_mov(from, to, tp, promo);

            let gives_check = mov_table::is_in_check(state, state.player);

            let mut depth = depth;
            if under_threat {
                depth += 1;
            } else if gives_check && (!is_capture || is_win_capture) {
                depth += 1;
            } else if is_threating_pawn_mov && depth < 5 {
                depth += 1;
            }

            let score = if depth > 1 && mov_count > 1 && !in_check && !gives_check && !under_threat && !is_win_capture && !is_threating_pawn_mov {
                let score = -self.ab_search(state, false, gives_check, false, history_pv_table, &mut next_pv_table, -alpha-1, -alpha, depth - 2, ply + 1, node_count, seldepth);
                if score > alpha {
                    if pv_found {
                        let score = -self.ab_search(state, false, gives_check, false, history_pv_table, &mut next_pv_table, -alpha-1, -alpha, depth - 1, ply + 1, node_count, seldepth);

                        if score > alpha && score < beta {
                            -self.ab_search(state, false, gives_check, false, history_pv_table, &mut next_pv_table, -beta, -alpha, depth - 1, ply + 1, node_count, seldepth)
                        } else {
                            score
                        }
                    } else {
                        -self.ab_search(state, false, gives_check, false, history_pv_table, &mut next_pv_table, -beta, -alpha, depth - 1, ply + 1, node_count, seldepth)
                    }
                } else {
                    score
                }
            } else {
                if pv_found {
                    let score = -self.ab_search(state, false, gives_check, false, history_pv_table, &mut next_pv_table, -alpha-1, -alpha, depth - 1, ply + 1, node_count, seldepth);

                    if score > alpha && score < beta {
                        -self.ab_search(state, false, gives_check, false, history_pv_table, &mut next_pv_table, -beta, -alpha, depth - 1, ply + 1, node_count, seldepth)
                    } else {
                        score
                    }
                } else {
                    -self.ab_search(state, false, gives_check, false, history_pv_table, &mut next_pv_table, -beta, -alpha, depth - 1, ply + 1, node_count, seldepth)
                }
            };

            state.undo_mov(from, to, tp);

            if self.abort {
                return alpha
            }

            if score >= beta {
                if !is_capture {
                    self.update_history_table(depth, from, to, state.squares[from]);
                    self.update_killer_table(score, ply, mov);
                }

                self.set_hash(state, depth, HASH_TYPE_BETA, score, mov);

                return score
            }

            if score > best_score {
                best_score = score;
            }

            if score > alpha {
                pv_table[0] = mov;
                pv_table[1..PV_TRACK_LENGTH].copy_from_slice(&next_pv_table[0..PV_TRACK_LENGTH-1]);

                alpha = score;
                best_mov = mov;
                pv_found = true;
            }
        }

        if alpha > original_alpha {
            self.set_hash(state, depth, HASH_TYPE_EXACT, alpha, best_mov);
        } else {
            self.set_hash(state, depth, HASH_TYPE_ALPHA, alpha, best_mov);
        }

        if best_score < -eval::TERM_VAL {
            if !in_check && self.in_stale_mate(state) {
                return 0
            }
        }

        alpha
    }

    fn multicut_search(&mut self, state: &mut State, mov_list: &Vec<(i32, u32)>, beta: i32, depth: u8, ply: u8, node_count: &mut u64, seldepth: &mut u8) -> bool {
        let mut mov_count = 0;
        let mut cut_count = 0;

        let depth_reduction = if depth > MULTICUT_DEPTH {
            MULTICUT_R + 1
        } else {
            MULTICUT_R
        };

        for (_score, mov) in mov_list {
            mov_count += 1;

            if mov_count > MULTICUT_MOV_COUNT {
                break
            }

            let mov = *mov;
            let (from, to, tp, promo) = util::decode_u32_mov(mov);

            state.do_mov(from, to, tp, promo);
            let scout_score = -self.ab_search(state, false, false, true, &mut EMPTY_HISTORY_PV_TABLE, &mut EMPTY_PV_TABLE, -beta, -beta+1, depth - depth_reduction - 1, ply + 1, node_count, seldepth);
            state.undo_mov(from, to, tp);

            if self.abort {
                return false
            }

            if scout_score >= beta {
                cut_count += 1;

                if cut_count >= MULTICUT_CUT_COUNT {
                    return true
                }
            }
        }

        false
    }

    fn q_search(&mut self, state: &mut State, mut alpha: i32, beta: i32, ply: u8, node_count: &mut u64, seldepth: &mut u8) -> i32 {
        if self.abort {
            return alpha
        }

        *node_count += 1;

        if mov_table::is_in_check(state, def::get_opposite_player(state.player)) {
            return eval::MATE_VAL - ply as i32
        }

        if ply > *seldepth {
            *seldepth = ply;
        }

        let material_score = eval::eval_materials(state);

        if material_score - eval::DELTA_MARGIN >= beta {
            return beta
        }

        let score = eval::eval_state(state, material_score);

        if score >= beta {
            return score
        }

        if score > alpha {
            alpha = score;
        }

        if score + eval::DELTA_MAX_MARGIN < alpha {
            let promoting_pawn_mask = if state.player == def::PLAYER_W {
                state.bitboard.w_pawn & bitboard::WP_PROMO_PAWNS_MASK
            } else {
                state.bitboard.b_pawn & bitboard::BP_PROMO_PAWNS_MASK
            };

            if promoting_pawn_mask == 0 {
                return alpha
            }
        }

        let delta = alpha - score - eval::DELTA_MARGIN;

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
            let score = -self.q_search(state, -beta, -alpha, ply + 1, node_count, seldepth);
            state.undo_mov(from, to, tp);

            if self.abort {
                return alpha
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

    fn validate_pv(&self, state: &mut State, pv_table: &mut [u32], mov_index: usize) {
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

        if mov_table::is_in_check(state, player) {
            pv_table[mov_index] = 0;
            state.undo_mov(from, to, tp);
            return
        }

        self.validate_pv(state, pv_table, mov_index + 1);

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
    fn update_killer_table(&mut self, score: i32, ply: u8, mov: u32) {
        let ply_index = ply as usize;

        if ply_index >= PV_TRACK_LENGTH {
            return
        }

        let (primary_killer_score, primary_killer_mov) = self.primary_killer_table[ply_index];
        if score > primary_killer_score || primary_killer_score == 0 {
            self.primary_killer_table[ply_index] = (score, mov);
            self.secondary_killer_table[ply_index] = (primary_killer_score, primary_killer_mov);
        } else {
            let (killer_score, _killer_mov) = self.secondary_killer_table[ply_index];
            if score > killer_score || killer_score == 0 {
                self.secondary_killer_table[ply_index] = (score, mov);
            }
        }
    }

    #[inline]
    fn get_killer_mov(&self, ply: u8) -> (u32, u32) {
        let ply_index = ply as usize;

        if ply_index >= PV_TRACK_LENGTH {
            return (0, 0)
        }

        let (_killer_score, primary_killer) = self.primary_killer_table[ply_index];
        let (_killer_score, secondary_killer) = self.secondary_killer_table[ply_index];

        (primary_killer, secondary_killer)
    }

    #[inline]
    fn update_history_table(&mut self, depth: u8, from: usize, to: usize, moving_piece: u8) {
        let history_score_increment = depth as i32 * depth as i32;

        let current_index_history_score = self.index_history_table[from][to];
        if current_index_history_score < MAX_HISTORY_SCORE - history_score_increment {
            self.index_history_table[from][to] = current_index_history_score + history_score_increment;
        }

        let current_piece_history_score = self.piece_history_table[moving_piece as usize][to];
        if current_piece_history_score < MAX_HISTORY_SCORE - history_score_increment {
            self.piece_history_table[moving_piece as usize][to] = current_piece_history_score + history_score_increment;
        }
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

fn is_threating_pawn(state: &State, from: usize, to: usize) -> bool {
    if !def::is_p(state.squares[from]) {
        return false
    }

    let pawn_rank = def::get_rank(state.player, to);

    if pawn_rank <= 3 {
        return false
    }

    if pawn_rank >= 6 {
        return true
    }

    let bitboard = state.bitboard;
    let bitmask = state.bitmask;

    if state.player == def::PLAYER_W && bitmask.wp_forward_masks[to] & bitboard.b_pawn == 0 {
        return true
    } else if bitmask.bp_forward_masks[to] & bitboard.w_pawn == 0 {
        return true
    }

    false
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

#[inline]
const fn get_futility_margin(depth: u8) -> i32 {
    eval::FUTILITY_MARGIN_BASE * (depth / 2 + 1) as i32 + eval::MAX_POS_VAL
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
    fn test_see_1() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("pnkq3r/pb2nppp/2p1b3/1r1N2R1/2PPP3/1Q3B2/PPP2PPP/RNB1K1NR b - - 0 1", &zob_keys, &bitmask);

        assert_eq!(250, see(&mut state, util::map_sqr_notation_to_index("c6"), util::map_sqr_notation_to_index("d5"), def::MOV_REG, 0));
        assert_eq!(100, see(&mut state, util::map_sqr_notation_to_index("e6"), util::map_sqr_notation_to_index("d5"), def::MOV_REG, 0));
    }

    #[test]
    fn test_see_2() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("pnkq3r/pbr2ppp/P1pnb3/N5R1/3PP3/1R1Q1B2/PPP2PPP/1NB1K1NR w - - 0 1", &zob_keys, &bitmask);

        assert_eq!(-175, see(&mut state, util::map_sqr_notation_to_index("b3"), util::map_sqr_notation_to_index("b7"), def::MOV_REG, 0));
        assert_eq!(250, see(&mut state, util::map_sqr_notation_to_index("a6"), util::map_sqr_notation_to_index("b7"), def::MOV_REG, 0));
        assert_eq!(0, see(&mut state, util::map_sqr_notation_to_index("a5"), util::map_sqr_notation_to_index("b7"), def::MOV_REG, 0));
    }

    #[test]
    fn test_see_3() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("4n2r/5P2/8/7B/8/8/8/8 w - - 0 1", &zob_keys, &bitmask);

        assert_eq!(875, see(&mut state, util::map_sqr_notation_to_index("f7"), util::map_sqr_notation_to_index("e8"), def::MOV_PROMO, def::WQ));
        assert_eq!(700, see(&mut state, util::map_sqr_notation_to_index("f7"), util::map_sqr_notation_to_index("e8"), def::MOV_PROMO, def::WN));
        assert_eq!(875, see(&mut state, util::map_sqr_notation_to_index("f7"), util::map_sqr_notation_to_index("e8"), def::MOV_PROMO, def::WR));
    }

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

        let best_mov = search_engine.search(&mut state, 1500, 64);

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

        let best_mov = search_engine.search(&mut state, 25500, 64);

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
        let mut state = State::new("2r4k/pB4bp/1p4p1/6q1/1P1n4/2N5/P4PPP/2R1Q1K1 b - - 0 1", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 15500, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("g5"));
        assert_eq!(to, util::map_sqr_notation_to_index("c1"));
    }

    #[test]
    fn test_search_13() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("6k1/p3q2p/1nr3pB/8/3Q1P2/6P1/PP5P/3R2K1 b - - 0 1", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 25500, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("c6"));
        assert_eq!(to, util::map_sqr_notation_to_index("d6"));
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
        let mut state = State::new("q4rk1/1n1Qbppp/2p5/1p2p3/1P2P3/2P4P/6P1/2B1NRK1 b - - 0 1", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 15500, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("a8"));
        assert_eq!(to, util::map_sqr_notation_to_index("c8"));
    }

    #[test]
    fn test_search_17() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("8/1k6/8/2R5/3K4/8/8/8 w - - 0 1", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 15500, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("d4"));
        assert_eq!(to, util::map_sqr_notation_to_index("d5"));
    }

    #[test]
    fn test_search_18() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("r4rk1/1p2bppp/2n1p1n1/1p5P/3pPP2/1B1P4/PP1BN1P1/2KR3R b - - 0 18", &zob_keys, &bitmask);
        let mut search_engine = SearchEngine::new(65536);

        let best_mov = search_engine.search(&mut state, 15500, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("g6"));
        assert_eq!(to, util::map_sqr_notation_to_index("h8"));
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
