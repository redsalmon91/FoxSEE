/*
 * Copyright (C) 2020-2022 Zixiao Han
 */

use crate::{
    bitmask,
    def,
    eval,
    hashtable::{
        DepthPreferredHashTable,
        LookupResult,
        HASH_TYPE_ALPHA,
        HASH_TYPE_BETA,
        HASH_TYPE_EXACT
    },
    mov_table,
    state::State,
    search_params::SearchParams,
    time_control::TimeCapacity,
    util,
    zob_keys,
};

use std::time::Instant;

const PV_TRACK_LENGTH: usize = 128;
const PV_PRINT_LENGTH: usize = 16;
const TIME_CHECK_INTEVAL: u64 = 1023;

pub const WINNING_EXCHANGE: i32 = 20;
pub const EQUAL_EXCHANGE: i32 = -20;

pub static mut ABORT_SEARCH: bool = false;

struct OrderedMov {
    mov: u32,
    sort_score: i32,
    gives_check: bool,
    is_passer: bool,
}

struct OrderedQMov {
    mov: u32,
    sort_score: i32,
}

pub struct SearchEngine {
    evaluator: eval::Evaluator,
    depth_preferred_hash_table: DepthPreferredHashTable,
    primary_killer_table: [(u32, i32, u8); PV_TRACK_LENGTH],
    secondary_killer_table: [(u32, i32, u8); PV_TRACK_LENGTH],
    counter_mov_table: [[[u32; def::BOARD_SIZE]; def::BOARD_SIZE]; 2],
    history_table: [[[i32; def::BOARD_SIZE]; def::BOARD_SIZE]; 2],
    butterfly_table: [[[i32; def::BOARD_SIZE]; def::BOARD_SIZE]; 2],
    params: SearchParams,
    null_mov_count: u8,
    root_full_mov_count: u16,
    root_half_mov_count: u16,
    time_tracker: Instant,
    max_time_millis: u128,
    node_count: u64,
    seldepth: u8,
    has_one_legal_mov: bool,
}

impl SearchEngine {
    pub fn new(hash_size: usize) -> Self {
        SearchEngine {
            evaluator: eval::Evaluator::new(),
            depth_preferred_hash_table: DepthPreferredHashTable::new(hash_size),
            primary_killer_table: [(0, 0, 0); PV_TRACK_LENGTH],
            secondary_killer_table: [(0, 0, 0); PV_TRACK_LENGTH],
            counter_mov_table: [[[0; def::BOARD_SIZE]; def::BOARD_SIZE]; 2],
            history_table: [[[0; def::BOARD_SIZE]; def::BOARD_SIZE]; 2],
            butterfly_table: [[[1; def::BOARD_SIZE]; def::BOARD_SIZE]; 2],
            params: SearchParams::default(),
            null_mov_count: 0,
            root_full_mov_count: 0,
            root_half_mov_count: 0,
            time_tracker: Instant::now(),
            max_time_millis: 0,
            node_count: 0,
            seldepth: 0,
            has_one_legal_mov: false,
        }
    }

    pub fn set_params(&mut self, params_file: &str) {
        self.params = SearchParams::from_config(&util::load_params(params_file));
        self.evaluator.set_params(params_file);
    }

    pub fn reset(&mut self) {
        self.depth_preferred_hash_table.clear();
    }

    pub fn set_hash_size(&mut self, hash_size: usize) {
        self.depth_preferred_hash_table = DepthPreferredHashTable::new(hash_size);
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

        self.root_full_mov_count = state.full_mov_count;
        self.root_half_mov_count = state.half_mov_count;

        self.node_count = 0;
        self.seldepth = 0;
        self.has_one_legal_mov = false;

        unsafe {
            ABORT_SEARCH = false;
        }

        let in_check = mov_table::is_in_check(state, state.player);

        let mut alpha = -eval::MATE_VAL;
        let mut beta = eval::MATE_VAL;

        let mut depth = 1;
        let mut best_mov = 0;
        let mut accumulated_time_taken = 0;

        loop {
            let score = self.ab_search(state, in_check, false, alpha, beta, depth, 0);

            unsafe {
                if ABORT_SEARCH {
                    break;
                }
            }

            if score >= beta {
                beta = eval::MATE_VAL;
                continue;
            }

            if score <= alpha {
                alpha = -eval::MATE_VAL;
                continue;
            }

            let mut pv_table = [0; PV_TRACK_LENGTH];
            self.retrieve_pv(state, &mut pv_table, 0);

            let checkmate = score.abs() > eval::TERM_VAL;

            let total_time_taken = self.time_tracker.elapsed().as_millis();

            if pv_table[0] != 0 {
                best_mov = pv_table[0];

                let nps = self.node_count as u128 / (total_time_taken / 1000).max(1);
                let hashfull_permill = self.depth_preferred_hash_table.get_utilization_permill();

                if checkmate {
                    let mate_score = if score > 0 {
                        (eval::MATE_VAL - score + 1) / 2
                    } else {
                        (-eval::MATE_VAL - score - 1) / 2
                    };

                    println!("info score mate {} depth {} seldepth {} nodes {} nps {} hashfull {} time {} pv {}",mate_score, depth, self.seldepth, self.node_count, nps, hashfull_permill, total_time_taken, util::format_pv(&pv_table));
                } else {
                    println!("info score cp {} depth {} seldepth {} nodes {} nps {} hashfull {} time {} pv {}", score, depth, self.seldepth, self.node_count, nps, hashfull_permill, total_time_taken, util::format_pv(&pv_table));
                }

                if checkmate {
                    break
                }

                if total_time_taken - accumulated_time_taken > self.max_time_millis / 2 {
                    break
                }

                if self.has_one_legal_mov {
                    break;
                }
            }

            alpha = score - self.params.aspiration_window_size;
            beta = score + self.params.aspiration_window_size;

            depth += 1;
            accumulated_time_taken = total_time_taken;

            if depth > max_depth {
                break
            }
        }

        best_mov
    }

    fn ab_search(&mut self, state: &mut State, in_check: bool, on_extend: bool, mut alpha: i32, beta: i32, depth: u8, ply: u8) -> i32 {
        self.node_count += 1;
        if (self.node_count & TIME_CHECK_INTEVAL == 0) && self.time_tracker.elapsed().as_millis() > self.max_time_millis {
            unsafe {
                ABORT_SEARCH = true;
                return alpha;
            }
        }

        if ply > 0 && self.null_mov_count == 0 && state.is_draw(ply) {
            return 0;
        }

        let on_pv = beta - alpha > 1;

        if depth == 0 {
            let mut score = self.q_search(state, alpha, beta, ply);

            if !on_pv {
                return score;
            }

            let full_mov_count_discount = state.full_mov_count as i32 - self.root_full_mov_count as i32;
            let mut half_mov_count_discount = state.half_mov_count as i32 - self.root_half_mov_count as i32;

            if half_mov_count_discount < 0 {
                half_mov_count_discount = 0;
            }

            let time_discount = full_mov_count_discount + half_mov_count_discount;

            if score > time_discount {
                score -= time_discount;
            } else if score < time_discount {
                score += time_discount;
            } else {
                score = 0;
            }

            return score;
        }

        let original_alpha = alpha;

        let mut hash_mov = 0;
        let mut is_singular_mov = false;

        let static_eval;

        match self.get_hash(state) {
            Some(entry) => {
                hash_mov = entry.mov;
                static_eval = entry.eval;

                if !on_pv && entry.depth >= depth {
                    let mut hash_score = entry.score;

                    if hash_score > eval::TERM_VAL {
                        hash_score -= ply as i32;
                    } else if hash_score < -eval::TERM_VAL {
                        hash_score += ply as i32;
                    }

                    match entry.flag {
                        HASH_TYPE_EXACT => {
                            return hash_score;
                        },
                        HASH_TYPE_BETA => {
                            if hash_score >= beta {
                                return hash_score;
                            }
                        },
                        HASH_TYPE_ALPHA => {
                            if hash_score <= alpha {
                                return hash_score;
                            }
                        },
                        _ => {},
                    }
                }

                if on_pv && entry.flag == HASH_TYPE_BETA {
                    is_singular_mov = true;
                }
            },
            _ => {
                let (material_score, is_draw) = self.evaluator.eval_materials(state);

                if is_draw && ply > 0 {
                    return 0;
                }

                static_eval = self.evaluator.eval_state(state, material_score);
            },
        }

        let mut under_mate_threat = false;

        if !on_pv && !on_extend && !in_check {
            if depth <= self.params.razoring_depth && !eval::has_promoting_pawn(state, state.player) {
                if static_eval + self.params.razoring_margin * depth as i32 <= alpha {
                    return self.q_search(state, alpha, beta, ply);
                }
            }

            if depth >= self.params.null_move_pruning_depth && static_eval >= beta {
                let depth_reduction = if depth > self.params.null_move_pruning_depth {
                    self.params.null_move_pruning_reduction + 1
                } else {
                    self.params.null_move_pruning_reduction
                };

                self.null_mov_count += 1;

                state.do_null_mov();
                let scout_score = -self.ab_search(state, false, false, -beta, -beta+1, depth - depth_reduction - 1, ply + 1);
                state.undo_null_mov();

                self.null_mov_count -= 1;

                unsafe {
                    if ABORT_SEARCH {
                        return alpha;
                    }
                }

                if scout_score >= beta && scout_score != 0 && scout_score < eval::TERM_VAL {
                    return beta;
                } else if scout_score < -eval::TERM_VAL {
                    under_mate_threat = true;
                }
            }
        }

        if on_pv && hash_mov == 0 && depth >= self.params.internal_iterative_deepening_depth {
            self.ab_search(state, in_check, on_extend, alpha, beta, depth - self.params.internal_iterative_deepening_reduction, ply);

            unsafe {
                if ABORT_SEARCH {
                    return alpha;
                }
            }

            match self.get_hash(state) {
                Some(entry) => {
                    hash_mov = entry.mov;

                    if entry.flag == HASH_TYPE_BETA {
                        is_singular_mov = true;
                    }
                },
                _ => (),
            }
        }

        let mut mov_count = 0;
        let mut best_mov = hash_mov;
        let mut pv_found = false;
        let mut pv_extended = false;
        let mut legal_mov_count = 0;

        if hash_mov != 0 {
            mov_count += 1;
            legal_mov_count += 1;

            let (from, to, tp, promo) = util::decode_u32_mov(hash_mov);

            let is_capture = state.squares[to] != 0;

            state.do_mov(from, to, tp, promo);

            let gives_check = mov_table::is_in_check(state, state.player);

            let mut depth = depth;

            if gives_check || under_mate_threat {
                depth += 1;
                pv_extended = true;
            }

            let score = -self.ab_search(state, gives_check, pv_extended, -beta, -alpha, depth - 1, ply + 1);

            state.undo_mov(from, to, tp);

            unsafe {
                if ABORT_SEARCH {
                    return alpha;
                }
            }

            if score >= beta {
                if !is_capture && promo == 0 {
                    self.update_history_table(state.player, depth, from, to);
                    self.update_killer_table(ply, hash_mov, score, depth);
                    self.update_counter_mov_table(state, hash_mov);
                }

                self.set_hash(state, depth, ply, HASH_TYPE_BETA, score, static_eval, hash_mov);

                return score
            }

            if score > alpha {
                alpha = score;
                best_mov = hash_mov;
                pv_found = true;
            }

            if score - self.params.singular_extension_margin <= alpha {
                is_singular_mov = false;
            }
        }

        let mut mov_list = [0; def::MAX_MOV_COUNT];

        mov_table::gen_reg_mov_list(state, &mut mov_list);

        let (primary_killer, secondary_killer) = self.get_killer_mov(ply);
        let counter_mov = self.get_counter_mov(state);

        let mut ordered_mov_list = Vec::new();

        for mov_index in 0..def::MAX_MOV_COUNT {
            let mov = mov_list[mov_index];

            if mov == 0 {
                break;
            }

            if mov == hash_mov {
                continue;
            }

            let (from, to, tp, promo) = util::decode_u32_mov(mov);

            state.do_mov(from, to, tp, promo);

            let gives_check = mov_table::is_in_check(state, state.player);
            let illegal_mov = mov_table::is_in_check(state, def::get_opposite_player(state.player));
            let is_passer = is_passed_pawn(state, state.squares[to], to);

            state.undo_mov(from, to, tp);

            if !illegal_mov {
                legal_mov_count += 1;

                let mut ordered_mov = OrderedMov {
                    mov,
                    gives_check,
                    is_passer,
                    sort_score: 0,
                };

                if state.squares[to] != 0 || promo != 0 {
                    let mvv_lva_score = self.mvv_lva(state, from, to, promo);

                    if mvv_lva_score > WINNING_EXCHANGE {
                        if gives_check {
                            ordered_mov.sort_score = self.params.sorting_capture_base_val + mvv_lva_score + self.params.sorting_check_capture_bonus;
                        } else {
                            ordered_mov.sort_score = self.params.sorting_capture_base_val + mvv_lva_score;
                        }
                    } else {
                        let see_score = self.see(state, from, to, tp, promo);

                        if gives_check {
                            ordered_mov.sort_score = self.params.sorting_capture_base_val + see_score + self.params.sorting_check_capture_bonus;
                        } else {
                            ordered_mov.sort_score = self.params.sorting_capture_base_val + see_score;
                        }
                    }
                } else if mov == counter_mov {
                    ordered_mov.sort_score = self.params.sorting_capture_base_val + self.params.sorting_counter_move_val;
                } else if mov == primary_killer {
                    ordered_mov.sort_score = self.params.sorting_capture_base_val + self.params.sorting_killer_primary_val;
                } else if mov == secondary_killer {
                    ordered_mov.sort_score = self.params.sorting_capture_base_val + self.params.sorting_killer_secondary_val;
                } else if gives_check {
                    ordered_mov.sort_score = self.params.sorting_capture_base_val + self.params.sorting_checker_val;
                } else if is_passer {
                    ordered_mov.sort_score = self.params.sorting_capture_base_val + self.params.sorting_passer_val;
                } else {
                    let history_score = self.history_table[state.player as usize - 1][from][to];
                    let butterfly_score = self.butterfly_table[state.player as usize - 1][from][to];

                    if history_score != 0 {
                        if history_score > butterfly_score {
                            ordered_mov.sort_score = self.params.sorting_good_history_base_val + history_score / butterfly_score;
                        } else {
                            ordered_mov.sort_score = self.params.sorting_normal_history_base_val + history_score - butterfly_score;
                        }
                    } else {
                        if !on_pv && !on_extend && !in_check && legal_mov_count > 0 && butterfly_score > self.params.butterfly_pruning_count {
                            continue;
                        }

                        ordered_mov.sort_score = eval::get_square_val_diff(state, state.squares[from], from, to);
                    }
                }

                ordered_mov_list.push(ordered_mov);
            }
        }

        if legal_mov_count == 0 {
            if !in_check {
                return 0;
            }

            return -eval::MATE_VAL + ply as i32;
        } else if legal_mov_count == 1 && ply == 0 {
            if hash_mov != 0 {
                self.has_one_legal_mov = true;
                return alpha;
            }
        }

        ordered_mov_list.sort_by(|ordered_mov_a, ordered_mov_b| {
            ordered_mov_b.sort_score.partial_cmp(&ordered_mov_a.sort_score).unwrap()
        });

        if !on_pv && !on_extend && !in_check && !under_mate_threat && depth >= self.params.multi_cut_pruning_depth {
            let mut cut_mov_count = 0;
            let mut cut_count = 0;

            for ordered_mov in &ordered_mov_list {
                cut_mov_count += 1;

                if cut_mov_count > self.params.multi_cut_pruning_move_count {
                    break;
                }

                let mov = ordered_mov.mov;
                let gives_check = ordered_mov.gives_check;

                let (from, to, tp, promo) = util::decode_u32_mov(mov);

                state.do_mov(from, to, tp, promo);
                let scout_score = -self.ab_search(state, gives_check, false, -beta, -beta+1, depth - self.params.multi_cut_pruning_reduction - 1, ply + 1);
                state.undo_mov(from, to, tp);

                unsafe {
                    if ABORT_SEARCH {
                        return alpha;
                    }
                }

                if scout_score >= beta {
                    cut_count += 1;

                    if cut_count >= self.params.multi_cut_pruning_cut_count {
                        return beta;
                    }
                }
            }
        }

        for ordered_mov in &ordered_mov_list {
            mov_count += 1;

            let mov = ordered_mov.mov;
            let gives_check = ordered_mov.gives_check;
            let is_passer = ordered_mov.is_passer;

            let (from, to, tp, promo) = util::decode_u32_mov(mov);

            let is_capture = state.squares[to] != 0;

            if mov_count > 1 && !gives_check && !in_check && !under_mate_threat && !is_passer && depth <= self.params.futility_pruning_depth {
                if static_eval + self.evaluator.val_of(state.squares[to]) + self.evaluator.val_of(promo) + self.params.futility_pruning_margin * depth as i32 <= alpha {
                    continue;
                }
            }

            state.do_mov(from, to, tp, promo);

            let mut depth = depth;
            let mut extended = false;

            if gives_check || under_mate_threat {
                depth += 1;
                extended = true;
            }

            let score = if depth > self.params.late_move_reductions_depth && mov_count > self.params.late_move_reductions_move_count && !extended {
                let score = -self.ab_search(state, gives_check, extended, -alpha - 1, -alpha, depth - ((mov_count as f64).sqrt() as u8).min(depth-1), ply + 1);
                if score > alpha {
                    if on_pv && pv_found {
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
                if on_pv && pv_found {
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
                    return alpha;
                }
            }

            if score >= beta {
                if !is_capture && promo == 0 {
                    self.update_history_table(state.player, depth, from, to);
                    self.update_killer_table(ply, mov, score, depth);
                    self.update_counter_mov_table(state, mov);

                    if mov_count > 1 {
                        if hash_mov != 0 {
                            let (from, to, _tp, _promo) = util::decode_u32_mov(hash_mov);

                            let is_capture = state.squares[to] != 0;

                            if !is_capture && promo == 0 {
                                self.update_butterfly_table(state.player, from, to);
                            }

                            for index in 0..mov_count-2 {
                                let prev_mov = &ordered_mov_list[index];

                                let (from, to, _tp, _promo) = util::decode_u32_mov(prev_mov.mov);

                                let is_capture = state.squares[to] != 0;

                                if !is_capture && promo == 0 {
                                    self.update_butterfly_table(state.player, from, to);
                                }
                            }
                        } else {
                            for index in 0..mov_count-1 {
                                let prev_mov = &ordered_mov_list[index];

                                let (from, to, _tp, _promo) = util::decode_u32_mov(prev_mov.mov);

                                let is_capture = state.squares[to] != 0;

                                if !is_capture && promo == 0 {
                                    self.update_butterfly_table(state.player, from, to);
                                }
                            }
                        }
                    }
                }

                self.set_hash(state, depth, ply, HASH_TYPE_BETA, score, static_eval, mov);

                return score;
            }

            if score > alpha {
                alpha = score;
                pv_found = true;
                best_mov = mov;
                is_singular_mov = false;
            }
        }

        if is_singular_mov && !pv_extended {
            let depth = depth + 1;

            let (from, to, tp, promo) = util::decode_u32_mov(hash_mov);

            let is_capture = state.squares[to] != 0;

            state.do_mov(from, to, tp, promo);

            let gives_check = mov_table::is_in_check(state, state.player);

            let score = -self.ab_search(state, gives_check, true, -beta, -alpha, depth-1, ply + 1);

            state.undo_mov(from, to, tp);

            unsafe {
                if ABORT_SEARCH {
                    return alpha;
                }
            }

            if score >= beta {
                if !is_capture && promo == 0 {
                    self.update_history_table(state.player, depth, from, to);
                    self.update_killer_table(ply, hash_mov, score, depth);
                    self.update_counter_mov_table(state, hash_mov);
                }

                self.set_hash(state, depth, ply, HASH_TYPE_BETA, score, static_eval, hash_mov);

                return score;
            }

            if score > alpha {
                self.set_hash(state, depth, ply, HASH_TYPE_EXACT, score, static_eval, hash_mov);
                return score;
            }
        }

        if alpha > original_alpha {
            self.set_hash(state, depth, ply, HASH_TYPE_EXACT, alpha, static_eval, best_mov);
        } else {
            self.set_hash(state, depth, ply, HASH_TYPE_ALPHA, alpha, static_eval, best_mov);
        }

        alpha
    }

    fn q_search(&mut self, state: &mut State, mut alpha: i32, beta: i32, ply: u8) -> i32 {
        self.node_count += 1;
        if (self.node_count & TIME_CHECK_INTEVAL == 0) && self.time_tracker.elapsed().as_millis() > self.max_time_millis {
            unsafe {
                ABORT_SEARCH = true;
                return alpha;
            }
        }

        if ply > self.seldepth {
            self.seldepth = ply;
        }

        let on_pv = beta - alpha > 1;

        let mut hash_mov = 0;

        let static_eval;

        match self.get_hash(state) {
            Some(entry) => {
                static_eval = entry.eval;

                let (_from, to, _tp, promo) = util::decode_u32_mov(entry.mov);

                if promo != 0 || state.squares[to] != 0 {
                    hash_mov = entry.mov;
                }

                if !on_pv {
                    let mut hash_score = entry.score;

                    if hash_score > eval::TERM_VAL {
                        hash_score -= ply as i32;
                    } else if hash_score < -eval::TERM_VAL {
                        hash_score += ply as i32;
                    }

                    match entry.flag {
                        HASH_TYPE_EXACT => {
                            return hash_score;
                        },
                        HASH_TYPE_BETA => {
                            if hash_score >= beta {
                                return hash_score;
                            }
                        },
                        HASH_TYPE_ALPHA => {
                            if hash_score <= alpha {
                                return hash_score;
                            }
                        },
                        _ => {},
                    }
                }
            },
            _ => {
                let (material_score, is_draw) = self.evaluator.eval_materials(state);

                if is_draw {
                    return 0;
                }

                static_eval = self.evaluator.eval_state(state, material_score);
            },
        }

        if mov_table::is_in_check(state, state.player) {
            return self.ab_search(state, true, false, alpha, beta, 1, ply);
        }

        if static_eval >= beta {
            return static_eval;
        }

        if static_eval > alpha {
            alpha = static_eval;
        }

        let in_endgame = eval::is_in_endgame(state);

        let original_alpha = alpha;

        let mut best_score = static_eval;
        let mut best_mov = 0;

        if hash_mov != 0 {
            let (from, to, tp, promo) = util::decode_u32_mov(hash_mov);

            state.do_mov(from, to, tp, promo);
            let score = -self.q_search(state, -beta, -alpha, ply + 1);
            state.undo_mov(from, to, tp);

            unsafe {
                if ABORT_SEARCH {
                    return alpha;
                }
            }

            if score >= beta {
                self.set_hash(state, 0, ply, HASH_TYPE_BETA, score, static_eval, hash_mov);
                return score;
            }

            if score > alpha {
                alpha = score;
            }

            if score > best_score {
                best_score = score;
                best_mov = hash_mov;
            }
        }

        let mut mov_list = [0; def::MAX_CAP_COUNT];
        mov_table::gen_capture_and_promo_list(state, &mut mov_list);

        if mov_list[0] == 0 {
            return static_eval;
        }

        let mut scored_mov_list = Vec::new();

        for mov_index in 0..def::MAX_CAP_COUNT {
            let mov = mov_list[mov_index];

            if mov == 0 {
                break;
            }

            if mov == hash_mov {
                continue;
            }

            let (from, to, tp, promo) = util::decode_u32_mov(mov);

            state.do_mov(from, to, tp, promo);
            let illegal_mov = mov_table::is_in_check(state, def::get_opposite_player(state.player));
            state.undo_mov(from, to, tp);

            if !illegal_mov {
                if !in_endgame {
                    let gain = self.evaluator.val_of(state.squares[to]) + self.evaluator.val_of(promo);

                    if static_eval + gain + self.params.delta_margin < alpha {
                        continue;
                    }
                }

                let mvv_lva_score = self.mvv_lva(state, from, to, promo);

                if mvv_lva_score > WINNING_EXCHANGE {
                    scored_mov_list.push(OrderedQMov {
                        mov,
                        sort_score: mvv_lva_score,
                    });
                } else {
                    let see_score = self.see(state, from, to, tp, promo);

                    if see_score < EQUAL_EXCHANGE {
                        continue;
                    }

                    scored_mov_list.push(OrderedQMov {
                        mov,
                        sort_score: see_score,
                    });
                }
            }
        }

        if scored_mov_list.is_empty() {
            return alpha;
        }

        scored_mov_list.sort_by(|ordered_mov_a, ordered_mov_b| {
            ordered_mov_b.sort_score.partial_cmp(&ordered_mov_a.sort_score).unwrap()
        });

        for ordered_mov in scored_mov_list {
            let mov = ordered_mov.mov;
            let (from, to, tp, promo) = util::decode_u32_mov(mov);

            state.do_mov(from, to, tp, promo);
            let score = -self.q_search(state, -beta, -alpha, ply + 1);
            state.undo_mov(from, to, tp);

            unsafe {
                if ABORT_SEARCH {
                    return alpha;
                }
            }

            if score >= beta {
                self.set_hash(state, 0, ply, HASH_TYPE_BETA, score, static_eval, mov);
                return score;
            }

            if score > alpha {
                alpha = score;
            }

            if score > best_score {
                best_score = score;
                best_mov = mov;
            }
        }

        if alpha > original_alpha {
            self.set_hash(state, 0, ply, HASH_TYPE_EXACT, alpha, static_eval, best_mov);
        } else {
            self.set_hash(state, 0, ply, HASH_TYPE_ALPHA, alpha, static_eval, best_mov);
        }

        alpha
    }

    fn retrieve_pv(&self, state: &mut State, pv_table: &mut [u32], mov_index: usize) {
        if mov_index == PV_PRINT_LENGTH {
            return
        }

        let mut mov = 0;

        match self.get_hash(state) {
            Some(entry) => {
                mov = entry.mov;
            }
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
    fn mvv_lva(&self, state: &State, from: usize, to: usize, promo: u8) -> i32 {
        self.evaluator.val_of(state.squares[to]) + self.evaluator.val_of(promo) - self.evaluator.val_of(state.squares[from])
    }

    fn see(&self, state: &mut State, from: usize, to: usize, tp: u8, promo: u8) -> i32 {
        let initial_gain = self.evaluator.val_of(state.squares[to]) + self.evaluator.val_of(promo);

        state.do_mov(from, to, tp, promo);

        let score = initial_gain - self.see_exchange(state, to, state.squares[to]);

        state.undo_mov(from, to, tp);

        score
    }

    fn see_exchange(&self, state: &mut State, to: usize, last_attacker: u8) -> i32 {
        let (attacker, tp, promo, attack_from) = mov_table::get_smallest_attacker_index(state, to);

        if attacker == 0 {
            return 0
        }

        state.do_mov(attack_from, to, tp, promo);

        let score = (self.evaluator.val_of(last_attacker) + self.evaluator.val_of(promo) - self.see_exchange(state, to, attacker)).max(0);

        state.undo_mov(attack_from, to, tp);

        score
    }

    #[inline]
    fn get_hash(&self, state: &State) -> Option<LookupResult> {
        self.depth_preferred_hash_table.get(get_hash_key(state), state.hash_key)
    }

    #[inline]
    fn set_hash(&mut self, state: &State, depth: u8, ply: u8, hash_flag: u8, mut score: i32, eval: i32, mov: u32) {
        if score > eval::TERM_VAL {
            score += ply as i32;
        } else if score < -eval::TERM_VAL {
            score -= ply as i32;
        }

        self.depth_preferred_hash_table.set(get_hash_key(state), state.hash_key, depth, self.root_full_mov_count, hash_flag, score, eval, mov);
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
                let (_secondary_killer_mov, killer_score, killer_depth) = self.secondary_killer_table[ply_index];
                if score >= killer_score && depth >= killer_depth {
                    self.secondary_killer_table[ply_index] = (mov, score, depth);
                }
            }
        } else {
            self.primary_killer_table[ply_index] = (mov, score, depth);
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
    fn update_counter_mov_table(&mut self, state: &State, mov: u32) {
        match state.history_mov_stack.last() {
            Some((player, from, to)) => {
                if *player != state.player {
                    self.counter_mov_table[state.player as usize - 1][*from][*to] = mov;
                }
            },
            None => {}
        }
    }

    #[inline]
    fn get_counter_mov(&self, state: &State) -> u32 {
        match state.history_mov_stack.last() {
            Some((player, from, to)) => {
                if *player != state.player {
                    self.counter_mov_table[state.player as usize - 1][*from][*to]
                } else {
                    0
                }
            },
            None => 0
        }
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

#[inline]
fn is_passed_pawn(state: &State, moving_piece: u8, to_index: usize) -> bool {
    let bitmask = bitmask::get_bitmask();

    match moving_piece {
        def::WP => {
            bitmask.wp_forward_masks[to_index] & state.bitboard.b_pawn == 0
            && bitmask.file_masks[to_index] & state.bitboard.b_all == 0
        },
        def::BP => {
            bitmask.bp_forward_masks[to_index] & state.bitboard.w_pawn == 0
            && bitmask.file_masks[to_index] & state.bitboard.w_all == 0
        },
        _ => false
    }
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
    fn test_search_0() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("r1bq1rk1/5p2/pb1p1n1p/npp1p3/4P1pB/2PP4/PPB2PPP/RN1QNRK1 w - - 0 15");
        let mut search_engine = SearchEngine::new(131072);

        let time_capacity = TimeCapacity {
            main_time_millis: 55500,
            extra_time_millis: 5500,
        };

        let best_mov = search_engine.search(&mut state, time_capacity, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("f2"));
        assert_eq!(to, util::map_sqr_notation_to_index("f4"));
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

        let mut state = State::new("rnbq1rk1/pp1p1pbp/5np1/3Pp3/2P5/2NB1N2/PP3PPP/R1BQK2R b KQ - 1 8");
        let mut search_engine = SearchEngine::new(131072);

        let time_capacity = TimeCapacity {
            main_time_millis: 55500,
            extra_time_millis: 5500,
        };

        let best_mov = search_engine.search(&mut state, time_capacity, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("e5"));
        assert_eq!(to, util::map_sqr_notation_to_index("e4"));
    }

    #[test]
    fn test_search_x() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("rnbqkbnr/pppppppp/8/8/8/3PPpPN/PPPQKP1P/R2NBB1R w kq - 0 1");
        let mut search_engine = SearchEngine::new(131072);

        let time_capacity = TimeCapacity {
            main_time_millis: 55500,
            extra_time_millis: 5500,
        };

        let best_mov = search_engine.search(&mut state, time_capacity, 64);

        let (from, to, _, _) = util::decode_u32_mov(best_mov);
        assert_eq!(from, util::map_sqr_notation_to_index("e2"));
        assert_eq!(to, util::map_sqr_notation_to_index("f3"));
    }
}
