/*
 * Copyright (C) 2020 Zixiao Han
 */

use crate::{
    def,
    state::State,
    util::{get_lowest_index, get_highest_index}
};

pub static MATE_VAL: i32 = 20000;
pub static TERM_VAL: i32 = 10000;

static Q_VAL: i32 = 1100;
static R_VAL: i32 = 550;
static B_VAL: i32 = 350;
static N_VAL: i32 = 350;
static P_VAL: i32 = 100;

static ENDGAME_PAWN_ESSENTIAL_VAL: i32 = 50;
static ENDGAME_DIFFERENT_COLORED_BISHOP_VAL: i32 = 90;

static PASS_PAWN_VAL: [i32; def::DIM_SIZE] = [0, 50, 50, 60, 70, 80, 90, 0];

static PASSED_PAWN_KING_DISTANCE_BASE_PEN: i32 = -10;
static UNSTOPPABLE_PASS_PAWN_VAL: i32 = 90;
static CONTROLLED_PASS_PAWN_VAL: i32 = 50;
static DOUBLED_PAWN_PEN: i32 = -20;
static ISOLATED_PAWN_PEN: i32 = -20;
static BEHIND_PAWN_PEN: i32 = -10;

static WEAK_SQR_PEN:i32 = -10;
static CENTER_SQR_VAL: i32 = 20;

static BISHOP_PAIR_VAL: i32 = 30;

static TOTAL_PHASE: i32 = 96;
static Q_PHASE_WEIGHT: i32 = 16;
static R_PHASE_WEIGHT: i32 = 8;
static B_PHASE_WEIGHT: i32 = 4;
static N_PHASE_WEIGHT: i32 = 4;

static TEMPO_VAL: i32 = 10;

static N_MOB_SCORE: [i32; 9] = [-30,-20,-5, 0, 0, 5, 10, 15, 20];
static B_MOB_SCORE: [i32; 14] = [-20,-10, 5, 10, 20, 25, 25, 30, 30, 35, 40, 40, 45, 50];
static R_MOB_SCORE: [i32; 15] = [-30,-10, 0, 0, 0, 5, 10, 15, 20, 20, 20, 25, 30, 30, 30];

static W_ZONE_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_11111111_11111111_11111111;
static B_ZONE_MASK: u64 = 0b11111111_11111111_11111111_00000000_00000000_00000000_00000000_00000000;

static CENTER_MASK: u64 = 0b00000000_00000000_00000000_00011000_00011000_00000000_00000000_00000000;

#[derive(PartialEq, Debug)]
pub struct FeatureMap {
    passed_pawn_point: i32,
    passed_pawn_king_distance: i32,
    unstoppable_passed_pawn_count: i32,
    controlled_passed_pawn_count: i32,
    connected_pawn_count: i32,
    doubled_pawn_count: i32,
    isolated_pawn_count: i32,
    behind_pawn_count: i32,

    mobility: i32,
    weak_sqrs_count: i32,
    center_count: i32,
}

impl FeatureMap {
    pub fn empty() -> Self {
        FeatureMap {
            passed_pawn_point: 0,
            passed_pawn_king_distance: 0,
            unstoppable_passed_pawn_count: 0,
            controlled_passed_pawn_count: 0,
            connected_pawn_count: 0,
            doubled_pawn_count: 0,
            isolated_pawn_count: 0,
            behind_pawn_count: 0,

            mobility: 0,
            weak_sqrs_count: 0,
            center_count: 0,
        }
    }
}

pub fn val_of(piece: u8) -> i32 {
    match piece {
        0 => 0,
        def::WK => MATE_VAL,
        def::WQ => Q_VAL,
        def::WR => R_VAL,
        def::WB => B_VAL,
        def::WN => N_VAL,
        def::WP => P_VAL,

        def::BK => MATE_VAL,
        def::BQ => Q_VAL,
        def::BR => R_VAL,
        def::BB => B_VAL,
        def::BN => N_VAL,
        def::BP => P_VAL,

        _ => 0,
    }
}

pub fn eval_materials(state: &State) -> (i32, bool) {
    let bitboard = state.bitboard;

    let mut is_endgame_with_different_colored_bishop = false;

    if bitboard.w_pawn | bitboard.b_pawn | bitboard.w_rook | bitboard.b_rook | bitboard.w_queen | bitboard.b_queen == 0 {
        if (bitboard.w_bishop | bitboard.w_knight).count_ones() < 2 && (bitboard.b_bishop | bitboard.b_knight).count_ones() < 2 {
            return (0, true)
        }

        if (bitboard.w_bishop | bitboard.w_knight) == 0 && bitboard.b_bishop == 0 && bitboard.b_knight.count_ones() < 3 {
            return (0, true)
        }

        if (bitboard.b_bishop | bitboard.b_knight) == 0 && bitboard.w_bishop == 0 && bitboard.w_knight.count_ones() < 3 {
            return (0, true)
        }
    }

    if bitboard.w_knight | bitboard.b_knight | bitboard.w_rook | bitboard.b_rook | bitboard.w_queen | bitboard.b_queen == 0 {
        if bitboard.w_bishop.count_ones() == 1 && bitboard.b_bishop.count_ones() == 1 {
            let mut wb_reachable_mask = 0;
            let mut bb_reachable_mask = 0;

            for index in 0..def::BOARD_SIZE {
                match state.squares[index] {
                    def::WB => {
                        wb_reachable_mask = state.bitmask.b_attack_masks[index];
                    },
                    def::BB => {
                        bb_reachable_mask = state.bitmask.b_attack_masks[index]
                    },
                    _ => {}
                }
            }

            is_endgame_with_different_colored_bishop = wb_reachable_mask & bb_reachable_mask == 0;
        }
    }

    let w_queen_count = bitboard.w_queen.count_ones() as i32;
    let w_rook_count = bitboard.w_rook.count_ones() as i32;
    let w_bishop_count = bitboard.w_bishop.count_ones() as i32;
    let w_knight_count = bitboard.w_knight.count_ones() as i32;
    let w_pawn_count = bitboard.w_pawn.count_ones() as i32;

    let b_queen_count = bitboard.b_queen.count_ones() as i32;
    let b_rook_count = bitboard.b_rook.count_ones() as i32;
    let b_bishop_count = bitboard.b_bishop.count_ones() as i32;
    let b_knight_count = bitboard.b_knight.count_ones() as i32;
    let b_pawn_count = bitboard.b_pawn.count_ones() as i32;

    let material_score = w_queen_count * Q_VAL
    + w_rook_count * R_VAL
    + w_bishop_count * B_VAL
    + w_knight_count * N_VAL
    + w_pawn_count * P_VAL
    - b_queen_count * Q_VAL
    - b_rook_count * R_VAL
    - b_bishop_count * B_VAL
    - b_knight_count * N_VAL
    - b_pawn_count * P_VAL;

    if material_score > 0 && (bitboard.w_pawn | bitboard.w_rook | bitboard.w_queen) == 0 && (bitboard.w_knight | bitboard.w_bishop).count_ones() == 1 && bitboard.b_pawn.count_ones() == 1 {
        return (0, false)
    }

    if material_score < 0 && (bitboard.b_pawn | bitboard.b_rook | bitboard.b_queen) == 0 && (bitboard.b_knight | bitboard.b_bishop).count_ones() == 1 && bitboard.w_pawn.count_ones() == 1 {
        return (0, false)
    }

    let mut eg_score = 0;

    if bitboard.w_bishop.count_ones() > 1 {
        eg_score += BISHOP_PAIR_VAL;
    }

    if bitboard.b_bishop.count_ones() > 1 {
        eg_score -= BISHOP_PAIR_VAL;
    }

    if bitboard.w_pawn == 0 {
        eg_score -= ENDGAME_PAWN_ESSENTIAL_VAL;
    }

    if bitboard.b_pawn == 0 {
        eg_score += ENDGAME_PAWN_ESSENTIAL_VAL;
    }

    if is_endgame_with_different_colored_bishop {
        if material_score > 0  {
            eg_score -= ENDGAME_DIFFERENT_COLORED_BISHOP_VAL;
        } else if material_score < 0 {
            eg_score += ENDGAME_DIFFERENT_COLORED_BISHOP_VAL;
        }
    }

    let phase = get_phase(state);

    let score_sign = if state.player == def::PLAYER_W {
        1
    } else {
        -1
    };

    ((material_score + eg_score * (TOTAL_PHASE - phase) / TOTAL_PHASE) * score_sign, false)
}

pub fn get_phase(state: &State) -> i32 {
    let bitboard = state.bitboard;

    (bitboard.w_queen | bitboard.b_queen).count_ones() as i32 * Q_PHASE_WEIGHT
    + (bitboard.w_rook | bitboard.b_rook).count_ones() as i32 * R_PHASE_WEIGHT
    + (bitboard.w_bishop | bitboard.b_bishop).count_ones() as i32 * B_PHASE_WEIGHT
    + (bitboard.w_knight | bitboard.b_knight).count_ones() as i32 * N_PHASE_WEIGHT
}

pub fn eval_state(state: &State, material_score: i32) -> i32 {
    let score_sign = if state.player == def::PLAYER_W {
        1
    } else {
        -1
    };

    let (w_features_map, b_features_map) = extract_features(state);

    let midgame_positional_score =
        w_features_map.mobility
        + w_features_map.weak_sqrs_count * WEAK_SQR_PEN
        + w_features_map.center_count * CENTER_SQR_VAL
        - b_features_map.mobility
        - b_features_map.weak_sqrs_count * WEAK_SQR_PEN
        - b_features_map.center_count * CENTER_SQR_VAL;

    let endgame_positional_score =
        w_features_map.passed_pawn_point
        + w_features_map.passed_pawn_king_distance * PASSED_PAWN_KING_DISTANCE_BASE_PEN
        + w_features_map.unstoppable_passed_pawn_count * UNSTOPPABLE_PASS_PAWN_VAL
        + w_features_map.controlled_passed_pawn_count * CONTROLLED_PASS_PAWN_VAL
        - b_features_map.passed_pawn_point
        - b_features_map.passed_pawn_king_distance * PASSED_PAWN_KING_DISTANCE_BASE_PEN
        - b_features_map.unstoppable_passed_pawn_count * UNSTOPPABLE_PASS_PAWN_VAL
        - b_features_map.controlled_passed_pawn_count * CONTROLLED_PASS_PAWN_VAL;

    let shared_positional_score =
        w_features_map.behind_pawn_count * BEHIND_PAWN_PEN
        + w_features_map.doubled_pawn_count * DOUBLED_PAWN_PEN
        + w_features_map.isolated_pawn_count * ISOLATED_PAWN_PEN
        - b_features_map.behind_pawn_count * BEHIND_PAWN_PEN
        - b_features_map.doubled_pawn_count * DOUBLED_PAWN_PEN
        - b_features_map.isolated_pawn_count * ISOLATED_PAWN_PEN;

    let phase = get_phase(state);

    let extra_score = midgame_positional_score * phase / TOTAL_PHASE + endgame_positional_score * (TOTAL_PHASE - phase) / TOTAL_PHASE + shared_positional_score;

    material_score + extra_score * score_sign + TEMPO_VAL
}

#[inline]
pub fn extract_features(state: &State) -> (FeatureMap, FeatureMap) {
    let squares = state.squares;
    let file_masks = state.bitmask.file_masks;
    let bitboard = state.bitboard;
    let bitmask = state.bitmask;

    let mut w_feature_map = FeatureMap::empty();
    let mut b_feature_map = FeatureMap::empty();

    let mut wp_attack_mask = 0;
    let mut wn_attack_mask = 0;
    let mut wb_attack_mask = 0;
    let mut wr_attack_mask = 0;
    let mut wq_attack_mask = 0;

    let mut bp_attack_mask = 0;
    let mut bn_attack_mask = 0;
    let mut bb_attack_mask = 0;
    let mut br_attack_mask = 0;
    let mut bq_attack_mask = 0;

    let mut mov_mask_map = [0; def::BOARD_SIZE];

    let occupy_mask = bitboard.w_all | bitboard.b_all;
    let start_index = occupy_mask.trailing_zeros() as usize;
    let end_index = def::BOARD_SIZE - occupy_mask.leading_zeros() as usize;

    let piece_mask = bitboard.w_knight | bitboard.w_bishop | bitboard.w_rook | bitboard.w_queen | bitboard.b_knight | bitboard.b_bishop | bitboard.b_rook | bitboard.b_queen;

    for index in start_index..end_index {
        let moving_piece = squares[index];

        if moving_piece == 0 {
            continue
        }
 
        match moving_piece {
            def::WP => {
                wp_attack_mask |= bitmask.wp_attack_masks[index];

                let file_mask = file_masks[index];
                let forward_mask = bitmask.wp_forward_masks[index];
                let rank = def::get_rank(def::PLAYER_W, index) as i32;

                let is_connected = bitmask.wp_connected_sqr_masks[index] & bitboard.w_pawn != 0;

                if forward_mask & (bitboard.b_pawn | (bitboard.w_pawn & file_mask)) == 0 {
                    w_feature_map.passed_pawn_point += PASS_PAWN_VAL[rank as usize];

                    let king_distance = def::get_file_distance(index, state.wk_index);
                    w_feature_map.passed_pawn_king_distance += king_distance;

                    let opponent_king_distance = def::get_file_distance(index, state.bk_index);
                    w_feature_map.passed_pawn_king_distance -= opponent_king_distance;

                    if piece_mask == 0 {
                        if state.player == def::PLAYER_W {
                            if opponent_king_distance > 7 - rank {
                                w_feature_map.unstoppable_passed_pawn_count += 1;
                            }
                        } else {
                            if opponent_king_distance - 1 > 7 - rank {
                                w_feature_map.unstoppable_passed_pawn_count += 1;
                            }
                        }

                        let pawn_control_mask = bitmask.wp_front_control_sqr_masks[index];
                        if pawn_control_mask == 0 || pawn_control_mask & bitmask.index_masks[state.wk_index] != 0 {
                            w_feature_map.controlled_passed_pawn_count += 1;
                        }
                    }
                }

                if is_connected {
                    w_feature_map.connected_pawn_count += 1;
                } else {
                    if forward_mask & !file_mask & bitboard.w_pawn == 0 {
                        w_feature_map.isolated_pawn_count += 1;
                    } else {
                        w_feature_map.behind_pawn_count += 1;
                    }
                }

                if (file_mask & bitboard.w_pawn).count_ones() > 1 {
                    w_feature_map.doubled_pawn_count += 1;
                }
            },
            def::BP => {
                bp_attack_mask |= bitmask.bp_attack_masks[index];

                let file_mask = file_masks[index];
                let forward_mask = bitmask.bp_forward_masks[index];
                let rank = def::get_rank(def::PLAYER_B, index) as i32;

                let is_connected = bitmask.bp_connected_sqr_masks[index] & bitboard.b_pawn != 0;

                if forward_mask & (bitboard.w_pawn | (bitboard.b_pawn & file_mask)) == 0 {
                    b_feature_map.passed_pawn_point += PASS_PAWN_VAL[rank as usize];

                    let king_distance = def::get_file_distance(index, state.bk_index);
                    b_feature_map.passed_pawn_king_distance += king_distance;

                    let opponent_king_distance = def::get_file_distance(index, state.wk_index);
                    b_feature_map.passed_pawn_king_distance -= opponent_king_distance;

                    if piece_mask == 0 {
                        if state.player == def::PLAYER_B {
                            if opponent_king_distance > 7 - rank {
                                b_feature_map.unstoppable_passed_pawn_count += 1;
                            }
                        } else {
                            if opponent_king_distance - 1 > 7 - rank {
                                b_feature_map.unstoppable_passed_pawn_count += 1;
                            }
                        }

                        let pawn_control_mask = bitmask.bp_front_control_sqr_masks[index];
                        if pawn_control_mask == 0 || pawn_control_mask & bitmask.index_masks[state.bk_index] != 0 {
                            b_feature_map.controlled_passed_pawn_count += 1;
                        }
                    }
                }

                if is_connected {
                    b_feature_map.connected_pawn_count += 1;
                } else {
                    if forward_mask & !file_mask & bitboard.b_pawn == 0 {
                        b_feature_map.isolated_pawn_count += 1;
                    } else {
                        b_feature_map.behind_pawn_count += 1;
                    }
                }

                if (file_mask & bitboard.b_pawn).count_ones() > 1 {
                    b_feature_map.doubled_pawn_count += 1;
                }
            },

            def::WN => {
                let mov_mask = bitmask.n_attack_masks[index];
                wn_attack_mask |= mov_mask;
                mov_mask_map[index] = mov_mask;
            },
            def::BN => {
                let mov_mask = bitmask.n_attack_masks[index];
                bn_attack_mask |= mov_mask;
                mov_mask_map[index] = mov_mask;
            },

            def::WB => {
                let mut mov_mask = 0;

                let up_left_attack_mask = bitmask.up_left_attack_masks[index];
                mov_mask ^= up_left_attack_mask;
                if up_left_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_left_attack_masks[lowest_blocker_index];
                }

                let up_right_attack_mask = bitmask.up_right_attack_masks[index];
                mov_mask ^= up_right_attack_mask;
                if up_right_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_right_attack_masks[lowest_blocker_index];
                }

                let down_left_attack_mask = bitmask.down_left_attack_masks[index];
                mov_mask ^= down_left_attack_mask;
                if down_left_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_left_attack_masks[highest_blocker_index];
                }

                let down_right_attack_mask = bitmask.down_right_attack_masks[index];
                mov_mask ^= down_right_attack_mask;
                if down_right_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_right_attack_masks[highest_blocker_index];
                }

                wb_attack_mask |= mov_mask;
                mov_mask_map[index] = mov_mask;
            },
            def::BB => {
                let mut mov_mask = 0;

                let up_left_attack_mask = bitmask.up_left_attack_masks[index];
                mov_mask ^= up_left_attack_mask;
                if up_left_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_left_attack_masks[lowest_blocker_index];
                }

                let up_right_attack_mask = bitmask.up_right_attack_masks[index];
                mov_mask ^= up_right_attack_mask;
                if up_right_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_right_attack_masks[lowest_blocker_index];
                }

                let down_left_attack_mask = bitmask.down_left_attack_masks[index];
                mov_mask ^= down_left_attack_mask;
                if down_left_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_left_attack_masks[highest_blocker_index];
                }

                let down_right_attack_mask = bitmask.down_right_attack_masks[index];
                mov_mask ^= down_right_attack_mask;
                if down_right_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_right_attack_masks[highest_blocker_index];
                }

                bb_attack_mask |= mov_mask;
                mov_mask_map[index] = mov_mask;
            },

            def::WR => {
                let mut mov_mask = 0;

                let up_attack_mask = bitmask.up_attack_masks[index];
                mov_mask ^= up_attack_mask;
                if up_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_attack_masks[lowest_blocker_index];
                }

                let right_attack_mask = bitmask.right_attack_masks[index];
                mov_mask ^= right_attack_mask;
                if right_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.right_attack_masks[lowest_blocker_index];
                }

                let down_attack_mask = bitmask.down_attack_masks[index];
                mov_mask ^= down_attack_mask;
                if down_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_attack_masks[highest_blocker_index];
                }

                let left_attack_mask = bitmask.left_attack_masks[index];
                mov_mask ^= left_attack_mask;
                if left_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.left_attack_masks[highest_blocker_index];
                }

                wr_attack_mask |= mov_mask;
                mov_mask_map[index] = mov_mask;
            },
            def::BR => {
                let mut mov_mask = 0;

                let up_attack_mask = bitmask.up_attack_masks[index];
                mov_mask ^= up_attack_mask;
                if up_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_attack_masks[lowest_blocker_index];
                }

                let right_attack_mask = bitmask.right_attack_masks[index];
                mov_mask ^= right_attack_mask;
                if right_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.right_attack_masks[lowest_blocker_index];
                }

                let down_attack_mask = bitmask.down_attack_masks[index];
                mov_mask ^= down_attack_mask;
                if down_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_attack_masks[highest_blocker_index];
                }

                let left_attack_mask = bitmask.left_attack_masks[index];
                mov_mask ^= left_attack_mask;
                if left_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.left_attack_masks[highest_blocker_index];
                }

                br_attack_mask |= mov_mask;
                mov_mask_map[index] = mov_mask;
            },

            def::WQ => {
                let mut mov_mask = 0;

                let up_left_attack_mask = bitmask.up_left_attack_masks[index];
                mov_mask ^= up_left_attack_mask;
                if up_left_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_left_attack_masks[lowest_blocker_index];
                }

                let up_right_attack_mask = bitmask.up_right_attack_masks[index];
                mov_mask ^= up_right_attack_mask;
                if up_right_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_right_attack_masks[lowest_blocker_index];
                }

                let down_left_attack_mask = bitmask.down_left_attack_masks[index];
                mov_mask ^= down_left_attack_mask;
                if down_left_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_left_attack_masks[highest_blocker_index];
                }

                let down_right_attack_mask = bitmask.down_right_attack_masks[index];
                mov_mask ^= down_right_attack_mask;
                if down_right_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_right_attack_masks[highest_blocker_index];
                }

                let up_attack_mask = bitmask.up_attack_masks[index];
                mov_mask ^= up_attack_mask;
                if up_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_attack_masks[lowest_blocker_index];
                }

                let right_attack_mask = bitmask.right_attack_masks[index];
                mov_mask ^= right_attack_mask;
                if right_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.right_attack_masks[lowest_blocker_index];
                }

                let down_attack_mask = bitmask.down_attack_masks[index];
                mov_mask ^= down_attack_mask;
                if down_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_attack_masks[highest_blocker_index];
                }

                let left_attack_mask = bitmask.left_attack_masks[index];
                mov_mask ^= left_attack_mask;
                if left_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.left_attack_masks[highest_blocker_index];
                }

                wq_attack_mask |= mov_mask;
                mov_mask_map[index] = mov_mask;
            },
            def::BQ => {
                let mut mov_mask = 0;

                let up_left_attack_mask = bitmask.up_left_attack_masks[index];
                mov_mask ^= up_left_attack_mask;
                if up_left_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_left_attack_masks[lowest_blocker_index];
                }

                let up_right_attack_mask = bitmask.up_right_attack_masks[index];
                mov_mask ^= up_right_attack_mask;
                if up_right_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_right_attack_masks[lowest_blocker_index];
                }

                let down_left_attack_mask = bitmask.down_left_attack_masks[index];
                mov_mask ^= down_left_attack_mask;
                if down_left_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_left_attack_masks[highest_blocker_index];
                }

                let down_right_attack_mask = bitmask.down_right_attack_masks[index];
                mov_mask ^= down_right_attack_mask;
                if down_right_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_right_attack_masks[highest_blocker_index];
                }

                let up_attack_mask = bitmask.up_attack_masks[index];
                mov_mask ^= up_attack_mask;
                if up_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_attack_masks[lowest_blocker_index];
                }

                let right_attack_mask = bitmask.right_attack_masks[index];
                mov_mask ^= right_attack_mask;
                if right_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.right_attack_masks[lowest_blocker_index];
                }

                let down_attack_mask = bitmask.down_attack_masks[index];
                mov_mask ^= down_attack_mask;
                if down_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_attack_masks[highest_blocker_index];
                }

                let left_attack_mask = bitmask.left_attack_masks[index];
                mov_mask ^= left_attack_mask;
                if left_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.left_attack_masks[highest_blocker_index];
                }

                bq_attack_mask |= mov_mask;
                mov_mask_map[index] = mov_mask;
            },
            _ => {}
        }
    }

    let w_attack_mask = wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask | wq_attack_mask;
    let w_defense_mask = w_attack_mask | bitmask.k_attack_masks[state.wk_index];

    let b_attack_mask = bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask | bq_attack_mask;
    let b_defense_mask = b_attack_mask | bitmask.k_attack_masks[state.bk_index];

    w_feature_map.weak_sqrs_count += (W_ZONE_MASK & !w_defense_mask).count_ones() as i32;
    w_feature_map.weak_sqrs_count += (W_ZONE_MASK & (b_attack_mask | bitboard.b_all) & !w_defense_mask).count_ones() as i32;
    w_feature_map.weak_sqrs_count += (bitmask.k_attack_masks[state.wk_index] & b_attack_mask).count_ones() as i32;
    w_feature_map.weak_sqrs_count += (bitmask.k_attack_masks[state.wk_index] & b_attack_mask & !w_attack_mask).count_ones() as i32;

    b_feature_map.weak_sqrs_count += (B_ZONE_MASK & !b_defense_mask).count_ones() as i32;
    b_feature_map.weak_sqrs_count += (B_ZONE_MASK & (w_attack_mask | bitboard.w_all) & !b_defense_mask).count_ones() as i32;
    b_feature_map.weak_sqrs_count += (bitmask.b_attack_masks[state.bk_index] & w_attack_mask).count_ones() as i32;
    b_feature_map.weak_sqrs_count += (bitmask.b_attack_masks[state.bk_index] & w_attack_mask & !b_attack_mask).count_ones() as i32;

    w_feature_map.center_count = ((bitboard.w_pawn | bitboard.w_knight | bitboard.w_bishop) & CENTER_MASK).count_ones() as i32;
    b_feature_map.center_count = ((bitboard.b_pawn | bitboard.b_knight | bitboard.b_bishop) & CENTER_MASK).count_ones() as i32;

    for index in start_index..end_index {
        let piece = squares[index];

        if piece == 0 {
            continue
        }

        match piece {
            def::WN => {
                let mov_mask = mov_mask_map[index];
                w_feature_map.mobility += N_MOB_SCORE[(mov_mask & !bp_attack_mask & !bitboard.w_all).count_ones() as usize];
            },
            def::WB => {
                let mov_mask = mov_mask_map[index];
                w_feature_map.mobility += B_MOB_SCORE[(mov_mask & !bp_attack_mask & !bitboard.w_all).count_ones() as usize];
            },
            def::WR => {
                let mov_mask = mov_mask_map[index];
                w_feature_map.mobility += R_MOB_SCORE[(mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask) & !bitboard.w_all).count_ones() as usize];
            },
            def::BN => {
                let mov_mask = mov_mask_map[index];
                b_feature_map.mobility += N_MOB_SCORE[(mov_mask & !wp_attack_mask & !bitboard.b_all).count_ones() as usize];
            },
            def::BB => {
                let mov_mask = mov_mask_map[index];
                b_feature_map.mobility += B_MOB_SCORE[(mov_mask & !wp_attack_mask & !bitboard.b_all).count_ones() as usize];
            },
            def::BR => {
                let mov_mask = mov_mask_map[index];
                b_feature_map.mobility += R_MOB_SCORE[(mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask) & !bitboard.b_all).count_ones() as usize];
            },
            _ => {},
        }
    }

    (w_feature_map, b_feature_map)
}
