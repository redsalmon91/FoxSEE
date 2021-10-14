/*
 * Copyright (C) 2020-2021 Zixiao Han
 */

use crate::{
    bitmask,
    def,
    state::State,
    util::{get_lowest_index, get_highest_index}
};

pub const MATE_VAL: i32 = 20000;
pub const TERM_VAL: i32 = 10000;

const Q_VAL: i32 = 1000;
const R_VAL: i32 = 525;
const B_VAL: i32 = 350;
const N_VAL: i32 = 345;
const P_VAL: i32 = 100;

const EG_Q_VAL: i32 = 90;
const EG_R_VAL: i32 = 50;
const EG_P_VAL: i32 = 10;

const EG_PAWN_ESSENTIAL_VAL: i32 = 190;
const EG_DIFFERENT_COLORED_BISHOP_VAL: i32 = 50;
const EG_BISHOP_PAIR_BONUS: i32 = 50;
const EG_RN_KNIGHT_PROTECTED_BONUS: i32 = 100;

const PASS_PAWN_VAL: [i32; def::DIM_SIZE] = [0, 50, 50, 80, 100, 150, 190, 0];
const CONNECTED_PASS_PAWN_BONUS: [i32; def::DIM_SIZE] = [0, 20, 20, 40, 60, 80, 100, 0];
const CANDIDATE_PASS_PAWN_VAL: [i32; def::DIM_SIZE] = [0, 10, 10, 10, 20, 20, 0, 0];

const KING_IN_PASSER_PATH_BONUS: i32 = 50;

const EG_P_SQR_DIFF_MULTIPLIER: i32 = 2;

const CONTROLLED_PASS_PAWN_VAL: i32 = 50;
const DOUBLED_PAWN_PEN: i32 = -20;
const ISOLATED_PAWN_PEN: i32 = -10;
const BEHIND_PAWN_PEN: i32 = -10;

const WEAK_SQR_PEN: i32 = -10;

const STRONG_K_ATTACK_COUNT_MULTIPLIER: i32 = 5;

const K_ATTACK_SCORE: [i32; 100] = [
      0,   0,   5,  10,  20,  30,  40,  50,  60,  70,
     80,  90, 100, 120, 140, 160, 180, 200, 240, 280,
    320, 380, 440, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500
];

const KING_LOST_CAS_RIGHTS_PEN: i32 = -50;

const ROOK_OPEN_BONUS: i32 = 10;

const THREAT_FACTOR: i32 = 10;

const TOTAL_PHASE: i32 = 96;
const Q_PHASE_WEIGHT: i32 = 16;
const R_PHASE_WEIGHT: i32 = 8;
const B_PHASE_WEIGHT: i32 = 4;
const N_PHASE_WEIGHT: i32 = 4;
const EG_PHASE: i32 = 32;

const TEMPO_VAL: i32 = 10;

const P_MOB_SCORE: i32 = 5;
const N_MOB_SCORE: [i32; 9] = [-50, -20, -5, 0, 5, 10, 15, 20, 25];
const B_MOB_SCORE: [i32; 14] = [-50, -20, 0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55];
const R_MOB_SCORE: [i32; 15] = [-50, -10, 0, 0, 0, 5, 10, 15, 20, 25, 30, 30, 30, 30, 30];
const Q_MOB_SCORE: [i32; 28] = [-30, -20, -10, -5, 0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50];
const K_MOB_SCORE: [i32; 9] = [-50, -10, 0, 5, 5, 5, 10, 10, 10];

const SQR_TABLE_BP: [i32; def::BOARD_SIZE] = [
      0,  0,  0,  0,  0,  0,  0,  0,
     15, 30, 30, 30, 30, 30, 30, 15,
     10, 20, 20, 30, 30, 20, 20, 10,
      0,  0,  0, 25, 25,  0,  0,  0,
      0,  0,  0, 20, 20,  0,  0,  0,
     10, 10,  0,  0,  0,  0, 10, 10,
     10, 10,  0,  0,  0,  0, 10, 10,
      0,  0,  0,  0,  0,  0,  0,  0,
];

const SQR_TABLE_BP_ENDGAME: [i32; def::BOARD_SIZE] = [
      0,  0,  0,  0,  0,  0,  0,  0,
     15, 30, 30, 30, 30, 30, 30, 15,
     10, 20, 20, 20, 20, 20, 20, 10,
      5, 15, 15, 15, 15, 15, 15,  5,
      0, 10, 10, 10, 10, 10, 10,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
];

const SQR_TABLE_WP: [i32; def::BOARD_SIZE] = [
      0,  0,  0,  0,  0,  0,  0,  0,
     10, 10,  0,  0,  0,  0, 10, 10,
     10, 10,  0,  0,  0,  0, 10, 10,
      0,  0,  0, 20, 20,  0,  0,  0,
      0,  0,  5, 25, 25,  0,  0,  0,
     10, 20, 20, 30, 30, 20, 20, 10,
     15, 30, 30, 30, 30, 30, 30, 15,
      0,  0,  0,  0,  0,  0,  0,  0,
];

const SQR_TABLE_WP_ENDGAME: [i32; def::BOARD_SIZE] = [
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0, 10, 10, 10, 10, 10, 10,  0,
      5, 15, 15, 15, 15, 15, 15,  5,
     10, 20, 20, 20, 20, 20, 20, 10,
     15, 30, 30, 30, 30, 30, 30, 15,
      0,  0,  0,  0,  0,  0,  0,  0,
];

const SQR_TABLE_BN: [i32; def::BOARD_SIZE] = [
    -60,-20,-20,-20,-20,-20,-20,-60,
    -30,-30, 20, 10, 10, 20,-30,-30,
    -20,  0, 15, 20, 20, 15,  0,-20,
    -20,  5, 10, 25, 25, 10,  5,-20,
    -20,  0, 10, 20, 20, 10,  0,-20,
    -20,  5, 10,  0,  0, 10,  5,-20,
    -30,-30,  0,  0,  0,  0,-30,-30,
    -60,-20,-20,-20,-20,-20,-20,-60,
];

const SQR_TABLE_WN: [i32; def::BOARD_SIZE] = [
    -60,-20,-20,-20,-20,-20,-20,-60,
    -30,-30,  0,  0,  0,  0,-30,-30,
    -20,  5, 10,  0,  0, 10,  5,-20,
    -20,  0, 10, 20, 20, 10,  0,-20,
    -20,  5, 10, 25, 25, 10,  5,-20,
    -20,  0, 15, 20, 20, 15,  0,-20,
    -30,-30, 20, 10, 10, 20,-30,-30,
    -60,-20,-20,-20,-20,-20,-20,-60,
];

const SQR_TABLE_BB: [i32; def::BOARD_SIZE] = [
    -50,-10,-10,-10,-10,-10,-10,-50,
    -20,  0, 10,  0,  0, 10,  0,-20,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10,  5,  5, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -50,-10,-10,-10,-10,-10,-10,-50,
];

const SQR_TABLE_WB: [i32; def::BOARD_SIZE] = [
    -50,-10,-10,-10,-10,-10,-10,-50,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -10, 10, 10,  5,  5, 10, 10,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -20,  0, 10,  0,  0, 10,  0,-20,
    -50,-10,-10,-10,-10,-10,-10,-50,
];

const SQR_TABLE_BR: [i32; def::BOARD_SIZE] = [
     10, 10, 20, 20, 20, 20, 10, 10,
     10, 20, 30, 30, 30, 30, 20, 10,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
    -10, -5,  0,  0,  0,  0, -5,-10,
     -5,  0,  0,  0,  0,  0,  0, -5,
];

const SQR_TABLE_WR: [i32; def::BOARD_SIZE] = [
     -5,  0,  0,  0,  0,  0,  0, -5,
    -10, -5,  0,  0,  0,  0, -5,-10,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     10, 20, 30, 30, 30, 30, 20, 10,
     10, 10, 20, 20, 20, 20, 10, 10,
];

const SQR_TABLE_BQ: [i32; def::BOARD_SIZE] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,-10,  0,  0,  0,  0,-10,-10,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
    -10,-10,  0,  0,  0,  0,-10,-10,
    -20,-10,-10, -5, -5,-10,-10,-20,
];

const SQR_TABLE_WQ: [i32; def::BOARD_SIZE] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,-10,  0,  0,  0,  0,-10,-10,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
    -10,-10,  0,  0,  0,  0,-10,-10,
    -20,-10,-10, -5, -5,-10,-10,-20,
];

const SQR_TABLE_BK: [i32; def::BOARD_SIZE] = [
    -90,-90,-90,-90,-90,-90,-90,-90,
    -90,-90,-90,-90,-90,-90,-90,-90,
    -90,-90,-90,-90,-90,-90,-90,-90,
    -90,-90,-90,-90,-90,-90,-90,-90,
    -90,-90,-90,-90,-90,-90,-90,-90,
    -60,-60,-60,-60,-60,-60,-60,-60,
      0, 10,-10,-30,-30,-10, 10,  0,
      0, 20,  0,-20,-20,  0, 20,  0,
];

const SQR_TABLE_WK: [i32; def::BOARD_SIZE] = [
      0, 20,  0,-20,-20,  0, 20,  0,
      0, 10,-10,-30,-30,-10, 10,  0,
    -60,-60,-60,-60,-60,-60,-60,-60,
    -90,-90,-90,-90,-90,-90,-90,-90,
    -90,-90,-90,-90,-90,-90,-90,-90,
    -90,-90,-90,-90,-90,-90,-90,-90,
    -90,-90,-90,-90,-90,-90,-90,-90,
    -90,-90,-90,-90,-90,-90,-90,-90,
];

const SQR_TABLE_K_ENDGAME: [i32; def::BOARD_SIZE] = [
    -50,-40,-30,-20,-20,-30,-40,-50,
    -40, 10, 10, 10, 10, 10, 10,-40,
    -30, 10, 30, 30, 30, 30, 10,-30,
    -30, 10, 30, 50, 50, 30, 10,-30,
    -30, 10, 30, 50, 50, 30, 10,-30,
    -30, 10, 30, 30, 30, 30, 10,-30,
    -30, 10, 10, 10, 10, 10, 10,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

const W_PAWN_PROMO_RANK: u64 = 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
const B_PAWN_PROMO_RANK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;

const W_BASE_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_11111111_11111111_00000000;
const B_BASE_MASK: u64 = 0b00000000_11111111_11111111_00000000_00000000_00000000_00000000_00000000;

#[derive(PartialEq, Debug)]
pub struct FeatureMap {
    mg_sqr_point: i32,
    eg_sqr_point: i32,

    passed_pawn_point: i32,
    controlled_passed_pawn_count: i32,
    doubled_pawn_count: i32,
    isolated_pawn_count: i32,
    behind_pawn_count: i32,

    weak_sqr_count: i32,

    king_in_passer_path_count: i32,

    mobility: i32,
    eg_mobility: i32,

    rook_open_count: i32,

    threat_point: i32,

    strong_king_attack_count: i32,
    weak_king_attack_count: i32,
}

impl FeatureMap {
    pub fn empty() -> Self {
        FeatureMap {
            mg_sqr_point: 0,
            eg_sqr_point: 0,

            passed_pawn_point: 0,
            controlled_passed_pawn_count: 0,
            doubled_pawn_count: 0,
            isolated_pawn_count: 0,
            behind_pawn_count: 0,

            weak_sqr_count: 0,

            king_in_passer_path_count: 0,

            mobility: 0,
            eg_mobility: 0,

            rook_open_count: 0,

            threat_point: 0,

            strong_king_attack_count: 0,
            weak_king_attack_count: 0,
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

pub fn get_square_val_diff(state: &State, moving_piece: u8, from_index: usize, to_index: usize) -> i32 {
    match moving_piece {
        def::WP => {
            if is_in_endgame(state) {
                (SQR_TABLE_WP_ENDGAME[to_index] - SQR_TABLE_WP_ENDGAME[from_index]) * EG_P_SQR_DIFF_MULTIPLIER
            } else {
                SQR_TABLE_WP[to_index] - SQR_TABLE_WP[from_index]
            }
        },
        def::WN => SQR_TABLE_WN[to_index] - SQR_TABLE_WN[from_index],
        def::WB => SQR_TABLE_WB[to_index] - SQR_TABLE_WB[from_index],
        def::WR => SQR_TABLE_WR[to_index] - SQR_TABLE_WR[from_index],
        def::WQ => SQR_TABLE_WQ[to_index] - SQR_TABLE_WQ[from_index],
        def::WK => {
            if is_in_endgame(state) {
                SQR_TABLE_K_ENDGAME[to_index] - SQR_TABLE_K_ENDGAME[from_index]
            } else {
                SQR_TABLE_WK[to_index] - SQR_TABLE_WK[from_index]
            }
        },

        def::BP => {
            if is_in_endgame(state) {
                (SQR_TABLE_BP_ENDGAME[to_index] - SQR_TABLE_BP_ENDGAME[from_index]) * EG_P_SQR_DIFF_MULTIPLIER
            } else {
                SQR_TABLE_BP[to_index] - SQR_TABLE_BP[from_index]
            }
        },
        def::BN => SQR_TABLE_BN[to_index] - SQR_TABLE_BN[from_index],
        def::BB => SQR_TABLE_BB[to_index] - SQR_TABLE_BB[from_index],
        def::BR => SQR_TABLE_BR[to_index] - SQR_TABLE_BR[from_index],
        def::BQ => SQR_TABLE_BQ[to_index] - SQR_TABLE_BQ[from_index],
        def::BK => {
            if is_in_endgame(state) {
                SQR_TABLE_K_ENDGAME[to_index] - SQR_TABLE_K_ENDGAME[from_index]
            } else {
                SQR_TABLE_BK[to_index] - SQR_TABLE_BK[from_index]
            }
        },

        _ => 0,
    }
}

pub fn is_in_endgame(state: &State) -> bool {
    get_phase(state) <= EG_PHASE
}

pub fn has_promoting_pawn(state: &State, player: u8) -> bool {
    if player == def::PLAYER_W {
        state.bitboard.w_pawn & W_PAWN_PROMO_RANK != 0
    } else {
        state.bitboard.b_pawn & B_PAWN_PROMO_RANK != 0
    }
}

pub fn eval_materials(state: &State) -> (i32, bool) {
    let bitboard = state.bitboard;
    let bitmask = bitmask::get_bitmask();

    let w_queen_count = state.wq_count;
    let w_rook_count = state.wr_count;
    let w_bishop_count = state.wb_count;
    let w_knight_count = state.wn_count;
    let w_pawn_count = state.wp_count;

    let b_queen_count = state.bq_count;
    let b_rook_count = state.br_count;
    let b_bishop_count = state.bb_count;
    let b_knight_count = state.bn_count;
    let b_pawn_count = state.bp_count;

    if bitboard.w_pawn | bitboard.b_pawn | bitboard.w_rook | bitboard.b_rook | bitboard.w_queen | bitboard.b_queen == 0 {
        if w_bishop_count + w_knight_count < 2 && b_bishop_count + b_knight_count < 2 {
            return (0, true)
        }

        if (bitboard.w_bishop | bitboard.w_knight) == 0 && bitboard.b_bishop == 0 && b_knight_count < 3 {
            return (0, true)
        }

        if (bitboard.b_bishop | bitboard.b_knight) == 0 && bitboard.w_bishop == 0 && w_knight_count < 3 {
            return (0, true)
        }
    }

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

    if material_score > 0 && (bitboard.w_pawn | bitboard.w_rook | bitboard.w_queen) == 0 && w_knight_count + w_bishop_count == 1 && b_pawn_count == 1 {
        return (0, false)
    }

    if material_score < 0 && (bitboard.b_pawn | bitboard.b_rook | bitboard.b_queen) == 0 && b_knight_count + b_bishop_count == 1 && w_pawn_count == 1 {
        return (0, false)
    }

    let mut eg_score = 0;

    eg_score += w_queen_count * EG_Q_VAL;
    eg_score += w_rook_count * EG_R_VAL;
    eg_score += w_pawn_count * EG_P_VAL;

    eg_score -= b_queen_count * EG_Q_VAL;
    eg_score -= b_rook_count * EG_R_VAL;
    eg_score -= b_pawn_count * EG_P_VAL;

    if material_score > P_VAL && bitboard.w_pawn == 0 {
        eg_score -= EG_PAWN_ESSENTIAL_VAL;
    }

    if material_score < -P_VAL && bitboard.b_pawn == 0 {
        eg_score += EG_PAWN_ESSENTIAL_VAL;
    }

    let mut is_endgame_with_different_colored_bishop = false;

    if bitboard.w_knight | bitboard.b_knight | bitboard.w_rook | bitboard.b_rook | bitboard.w_queen | bitboard.b_queen == 0 {
        if w_bishop_count == 1 && b_bishop_count == 1 {
            let mut wb_reachable_mask = 0;
            let mut bb_reachable_mask = 0;

            for index in 0..def::BOARD_SIZE {
                match state.squares[index] {
                    def::WB => {
                        wb_reachable_mask = bitmask.b_attack_masks[index];
                    },
                    def::BB => {
                        bb_reachable_mask = bitmask.b_attack_masks[index]
                    },
                    _ => {}
                }
            }

            is_endgame_with_different_colored_bishop = wb_reachable_mask & bb_reachable_mask == 0;
        }
    }

    if is_endgame_with_different_colored_bishop {
        if material_score > 0  {
            eg_score -= EG_DIFFERENT_COLORED_BISHOP_VAL;
        } else if material_score < 0 {
            eg_score += EG_DIFFERENT_COLORED_BISHOP_VAL;
        }
    } else {
        if w_bishop_count > 1 {
            eg_score += EG_BISHOP_PAIR_BONUS;
        }

        if b_bishop_count > 1 {
            eg_score -= EG_BISHOP_PAIR_BONUS;
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
    (state.wq_count + state.bq_count) * Q_PHASE_WEIGHT
    + (state.wr_count + state.br_count) * R_PHASE_WEIGHT
    + (state.wb_count + state.bb_count) * B_PHASE_WEIGHT
    + (state.wn_count + state.bn_count) * N_PHASE_WEIGHT
}

pub fn eval_state(state: &State, material_score: i32) -> i32 {
    let score_sign = if state.player == def::PLAYER_W {
        1
    } else {
        -1
    };

    let (w_features_map, b_features_map) = extract_features(state);

    let w_king_attack_count = w_features_map.weak_king_attack_count + w_features_map.strong_king_attack_count * STRONG_K_ATTACK_COUNT_MULTIPLIER;
    let b_king_attack_count = b_features_map.weak_king_attack_count + b_features_map.strong_king_attack_count * STRONG_K_ATTACK_COUNT_MULTIPLIER;

    let mut midgame_positional_score =
        w_features_map.mg_sqr_point
        + w_features_map.rook_open_count * ROOK_OPEN_BONUS
        + K_ATTACK_SCORE[w_king_attack_count as usize]
        + w_features_map.weak_sqr_count * WEAK_SQR_PEN
        - b_features_map.mg_sqr_point
        - b_features_map.rook_open_count * ROOK_OPEN_BONUS
        - K_ATTACK_SCORE[b_king_attack_count as usize]
        - b_features_map.weak_sqr_count * WEAK_SQR_PEN;

    if state.bitboard.b_queen != 0 {
        if (state.cas_rights | state.cas_history) & 0b1100 == 0 {
            midgame_positional_score += KING_LOST_CAS_RIGHTS_PEN;
        }
    }

    if state.bitboard.w_queen != 0 {
        if (state.cas_rights | state.cas_history) & 0b0011 == 0 {
            midgame_positional_score -= KING_LOST_CAS_RIGHTS_PEN;
        }
    }

    let mut endgame_positional_score =
        w_features_map.eg_sqr_point
        + w_features_map.passed_pawn_point
        + w_features_map.controlled_passed_pawn_count * CONTROLLED_PASS_PAWN_VAL
        + w_features_map.eg_mobility
        + w_features_map.king_in_passer_path_count * KING_IN_PASSER_PATH_BONUS
        - b_features_map.eg_sqr_point
        - b_features_map.passed_pawn_point
        - b_features_map.controlled_passed_pawn_count * CONTROLLED_PASS_PAWN_VAL
        - b_features_map.eg_mobility
        - b_features_map.king_in_passer_path_count * KING_IN_PASSER_PATH_BONUS;

    let bitboard = state.bitboard;
    let bitmask = bitmask::get_bitmask();

    if bitboard.w_queen | bitboard.b_queen | bitboard.w_bishop | bitboard.b_bishop | bitboard.w_pawn | bitboard.b_pawn == 0 {
        if bitboard.w_rook | bitboard.b_knight == 0 {
            if bitboard.b_rook.count_ones() == 1 && bitboard.w_knight.count_ones() == 1 {
                if bitmask.k_attack_masks[state.wk_index] & bitboard.w_knight != 0 {
                    endgame_positional_score += EG_RN_KNIGHT_PROTECTED_BONUS;
                }
            }
        } else if bitboard.b_rook | bitboard.w_knight == 0 {
            if bitboard.w_rook.count_ones() == 1 && bitboard.b_knight.count_ones() == 1 {
                if bitmask.k_attack_masks[state.bk_index] & bitboard.b_knight != 0 {
                    endgame_positional_score -= EG_RN_KNIGHT_PROTECTED_BONUS;
                }
            }
        }
    }

    let shared_positional_score =
        w_features_map.mobility
        + w_features_map.threat_point
        + w_features_map.behind_pawn_count * BEHIND_PAWN_PEN
        + w_features_map.isolated_pawn_count * ISOLATED_PAWN_PEN
        + w_features_map.doubled_pawn_count * DOUBLED_PAWN_PEN
        - b_features_map.mobility
        - b_features_map.threat_point
        - b_features_map.behind_pawn_count * BEHIND_PAWN_PEN
        - b_features_map.isolated_pawn_count * ISOLATED_PAWN_PEN
        - b_features_map.doubled_pawn_count * DOUBLED_PAWN_PEN;

    let phase = get_phase(state);

    let extra_score = midgame_positional_score * phase / TOTAL_PHASE + endgame_positional_score * (TOTAL_PHASE - phase) / TOTAL_PHASE + shared_positional_score;

    material_score + extra_score * score_sign + TEMPO_VAL
}

fn extract_features(state: &State) -> (FeatureMap, FeatureMap) {
    let squares = state.squares;
    let bitboard = state.bitboard;
    let bitmask = bitmask::get_bitmask();
    let file_masks = bitmask.file_masks;

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

                if (bitmask.bp_forward_masks[index] & !file_mask) & bitboard.w_pawn == 0 {
                    if (bitmask.wp_forward_masks[index] & !file_mask) & bitboard.w_pawn == 0 {
                        w_feature_map.isolated_pawn_count += 1;

                        if file_mask & bitboard.b_pawn == 0 {
                            w_feature_map.isolated_pawn_count += 1;
                        }
                    } else {
                        w_feature_map.behind_pawn_count += 1;

                        if file_mask & bitboard.b_pawn == 0 {
                            w_feature_map.behind_pawn_count += 1;
                        }
                    }
                }

                if forward_mask & (bitboard.b_pawn | (bitboard.w_pawn & file_mask)) == 0 {
                    w_feature_map.passed_pawn_point += PASS_PAWN_VAL[rank as usize];

                    if forward_mask & bitmask.k_attack_masks[state.wk_index] != 0 {
                        w_feature_map.king_in_passer_path_count += 1;
                    }

                    if forward_mask & bitmask.k_attack_masks[state.bk_index] != 0 {
                        b_feature_map.king_in_passer_path_count += 1;
                    }

                    if bitmask.wp_connected_sqr_masks[index] & bitboard.w_pawn != 0 {
                        w_feature_map.passed_pawn_point += CONNECTED_PASS_PAWN_BONUS[rank as usize];
                    }

                    if piece_mask == 0 {
                        let pawn_control_mask = bitmask.wp_front_control_sqr_masks[index];
                        if pawn_control_mask == 0 || pawn_control_mask & bitmask.index_masks[state.wk_index] != 0 {
                            w_feature_map.controlled_passed_pawn_count += 1;
                        }
                    }

                    if bitmask.index_masks[index+def::DIM_SIZE] & occupy_mask == 0 {
                        w_feature_map.eg_mobility += P_MOB_SCORE;
                    }
                } else if forward_mask & (bitboard.w_pawn | bitboard.b_pawn) & file_mask == 0 && (forward_mask & bitboard.b_pawn).count_ones() == 1 && bitmask.wp_connected_sqr_masks[index] & bitboard.w_pawn != 0 {
                    w_feature_map.passed_pawn_point += CANDIDATE_PASS_PAWN_VAL[rank as usize];
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

                if (bitmask.wp_forward_masks[index] & !file_mask) & bitboard.b_pawn == 0 {
                    if (bitmask.bp_forward_masks[index] & !file_mask) & bitboard.b_pawn == 0 {
                        b_feature_map.isolated_pawn_count += 1;

                        if file_mask & bitboard.w_pawn == 0 {
                            b_feature_map.isolated_pawn_count += 1;
                        }
                    } else {
                        b_feature_map.behind_pawn_count += 1;

                        if file_mask & bitboard.w_pawn == 0 {
                            b_feature_map.behind_pawn_count += 1;
                        }
                    }
                }

                if forward_mask & (bitboard.w_pawn | (bitboard.b_pawn & file_mask)) == 0 {
                    b_feature_map.passed_pawn_point += PASS_PAWN_VAL[rank as usize];

                    if forward_mask & bitmask.k_attack_masks[state.bk_index] != 0 {
                        b_feature_map.king_in_passer_path_count += 1;
                    }

                    if forward_mask & bitmask.k_attack_masks[state.wk_index] != 0 {
                        w_feature_map.king_in_passer_path_count += 1;
                    }

                    if bitmask.bp_connected_sqr_masks[index] & bitboard.b_pawn != 0 {
                        b_feature_map.passed_pawn_point += CONNECTED_PASS_PAWN_BONUS[rank as usize];
                    }

                    if piece_mask == 0 {
                        let pawn_control_mask = bitmask.bp_front_control_sqr_masks[index];
                        if pawn_control_mask == 0 || pawn_control_mask & bitmask.index_masks[state.bk_index] != 0 {
                            b_feature_map.controlled_passed_pawn_count += 1;
                        }
                    }

                    if bitmask.index_masks[index-def::DIM_SIZE] & occupy_mask == 0 {
                        b_feature_map.eg_mobility += P_MOB_SCORE;
                    }
                } else if forward_mask & (bitboard.w_pawn | bitboard.b_pawn) & file_mask == 0 && (forward_mask & bitboard.w_pawn).count_ones() == 1 && bitmask.bp_connected_sqr_masks[index] & bitboard.b_pawn != 0 {
                    b_feature_map.passed_pawn_point += CANDIDATE_PASS_PAWN_VAL[rank as usize];
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

                if file_masks[index] & bitboard.w_pawn == 0 {
                    w_feature_map.rook_open_count += 1;
                }

                if file_masks[index] & bitboard.b_pawn == 0 {
                    w_feature_map.rook_open_count += 1;
                }
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

                if file_masks[index] & bitboard.b_pawn == 0 {
                    b_feature_map.rook_open_count += 1;
                }

                if file_masks[index] & bitboard.w_pawn == 0 {
                    b_feature_map.rook_open_count += 1;
                }
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

    let w_attack_mask = wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask | wq_attack_mask | bitmask.k_attack_masks[state.wk_index];
    let b_attack_mask = bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask | bq_attack_mask | bitmask.k_attack_masks[state.bk_index];

    w_feature_map.weak_sqr_count = (W_BASE_MASK & b_attack_mask & !w_attack_mask).count_ones() as i32;
    b_feature_map.weak_sqr_count = (B_BASE_MASK & w_attack_mask & !b_attack_mask).count_ones() as i32;

    let wk_ring_mask = bitmask.k_attack_masks[state.wk_index];
    let bk_ring_mask = bitmask.k_attack_masks[state.bk_index];

    w_feature_map.eg_mobility += K_MOB_SCORE[(wk_ring_mask &!bitboard.w_all & !b_attack_mask).count_ones() as usize];
    b_feature_map.eg_mobility += K_MOB_SCORE[(bk_ring_mask &!bitboard.b_all & !w_attack_mask).count_ones() as usize];

    for index in start_index..end_index {
        let piece = squares[index];

        if piece == 0 {
            continue
        }

        let index_mask = bitmask.index_masks[index];

        let mov_mask = mov_mask_map[index];

        match piece {
            def::WP => {
                w_feature_map.mg_sqr_point += SQR_TABLE_WP[index];
                w_feature_map.eg_sqr_point += SQR_TABLE_WP_ENDGAME[index];

                let threat_val = val_of(piece) / THREAT_FACTOR;
                if index_mask & w_attack_mask == 0 {
                    if index_mask & b_attack_mask != 0 {
                        w_feature_map.threat_point -= threat_val;
                    }
                } else if index_mask & b_attack_mask != 0 {
                    w_feature_map.threat_point -= (threat_val as f64).sqrt() as i32;
                }

                w_feature_map.weak_king_attack_count += (bk_ring_mask & mov_mask).count_ones() as i32;
            },
            def::WN => {
                w_feature_map.mg_sqr_point += SQR_TABLE_WN[index];

                let threat_val = val_of(piece) / THREAT_FACTOR;
                if index_mask & w_attack_mask == 0 {
                    w_feature_map.threat_point -= (threat_val as f64).sqrt() as i32;

                    if index_mask & b_attack_mask != 0 {
                        w_feature_map.threat_point -= threat_val;
                    }
                } else if index_mask & b_attack_mask != 0 {
                    w_feature_map.threat_point -= (threat_val as f64).sqrt() as i32;
                }

                let mobility_mask = mov_mask & !bp_attack_mask & !bitboard.w_all & !(b_attack_mask & !w_attack_mask);
                w_feature_map.mobility += N_MOB_SCORE[mobility_mask.count_ones() as usize];

                w_feature_map.weak_king_attack_count += (bk_ring_mask & mov_mask).count_ones() as i32;
                w_feature_map.weak_king_attack_count += (bitmask.n_attack_masks[state.bk_index] & mov_mask).count_ones() as i32;

                w_feature_map.weak_king_attack_count += (bk_ring_mask & mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask)).count_ones() as i32;
                w_feature_map.weak_king_attack_count += (bitmask.n_attack_masks[state.bk_index] & mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask)).count_ones() as i32;
            },
            def::WB => {
                w_feature_map.mg_sqr_point += SQR_TABLE_WB[index];

                let threat_val = val_of(piece) / THREAT_FACTOR;
                if index_mask & w_attack_mask == 0 {
                    w_feature_map.threat_point -= (threat_val as f64).sqrt() as i32;

                    if index_mask & b_attack_mask != 0 {
                        w_feature_map.threat_point -= threat_val;
                    }
                } else if index_mask & b_attack_mask != 0 {
                    w_feature_map.threat_point -= (threat_val as f64).sqrt() as i32;
                }

                let mobility_mask = mov_mask & !bp_attack_mask & !bitboard.w_all & !(b_attack_mask & !w_attack_mask);
                w_feature_map.mobility += B_MOB_SCORE[mobility_mask.count_ones() as usize];

                w_feature_map.weak_king_attack_count += (bk_ring_mask & mov_mask).count_ones() as i32;
                w_feature_map.weak_king_attack_count += (bk_ring_mask & mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask)).count_ones() as i32;
            },
            def::WR => {
                w_feature_map.mg_sqr_point += SQR_TABLE_WR[index];

                let threat_val = val_of(piece) / THREAT_FACTOR;
                if index_mask & w_attack_mask == 0 {
                    w_feature_map.threat_point -= (threat_val as f64).sqrt() as i32;

                    if index_mask & b_attack_mask != 0 {
                        w_feature_map.threat_point -= threat_val;
                    }
                } else if index_mask & b_attack_mask != 0 {
                    w_feature_map.threat_point -= (threat_val as f64).sqrt() as i32;
                }

                let mobility_mask = mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask) & !bitboard.w_all & !(b_attack_mask & !w_attack_mask);
                w_feature_map.mobility += R_MOB_SCORE[mobility_mask.count_ones() as usize];

                w_feature_map.weak_king_attack_count += (bk_ring_mask & mov_mask).count_ones() as i32;
                w_feature_map.strong_king_attack_count += (bk_ring_mask & mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask)).count_ones() as i32;
            },
            def::WQ => {
                w_feature_map.mg_sqr_point += SQR_TABLE_WQ[index];

                let threat_val = val_of(piece) / THREAT_FACTOR;
                if index_mask & w_attack_mask == 0 {
                    if index_mask & b_attack_mask != 0 {
                        w_feature_map.threat_point -= threat_val;
                    }
                } else if index_mask & b_attack_mask != 0 {
                    w_feature_map.threat_point -= (threat_val as f64).sqrt() as i32;
                }

                let mobility_mask = mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask) & !bitboard.w_all & !(b_attack_mask & !w_attack_mask);
                w_feature_map.mobility += Q_MOB_SCORE[mobility_mask.count_ones() as usize];

                w_feature_map.weak_king_attack_count += (bk_ring_mask & mov_mask).count_ones() as i32;
                w_feature_map.strong_king_attack_count += (bk_ring_mask & mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask | bq_attack_mask)).count_ones() as i32;
            },
            def::WK => {
                w_feature_map.mg_sqr_point += SQR_TABLE_WK[index];
                w_feature_map.eg_sqr_point += SQR_TABLE_K_ENDGAME[index];
            },
            def::BP => {
                b_feature_map.mg_sqr_point += SQR_TABLE_BP[index];
                b_feature_map.eg_sqr_point += SQR_TABLE_BP_ENDGAME[index];

                let threat_val = val_of(piece) / THREAT_FACTOR;
                if index_mask & b_attack_mask == 0 {
                    if index_mask & w_attack_mask != 0 {
                        b_feature_map.threat_point -= threat_val;
                    }
                } else if index_mask & w_attack_mask != 0 {
                    b_feature_map.threat_point -= (threat_val as f64).sqrt() as i32;
                }

                b_feature_map.weak_king_attack_count += (wk_ring_mask & mov_mask).count_ones() as i32;
            },
            def::BN => {
                b_feature_map.mg_sqr_point += SQR_TABLE_BN[index];

                let threat_val = val_of(piece) / THREAT_FACTOR;
                if index_mask & b_attack_mask == 0 {
                    b_feature_map.threat_point -= (threat_val as f64).sqrt() as i32;

                    if index_mask & w_attack_mask != 0 {
                        b_feature_map.threat_point -= threat_val;
                    }
                } else if index_mask & w_attack_mask != 0 {
                    b_feature_map.threat_point -= (threat_val as f64).sqrt() as i32;
                }

                let mobility_mask = mov_mask & !wp_attack_mask & !bitboard.b_all & !(w_attack_mask & !b_attack_mask);
                b_feature_map.mobility += N_MOB_SCORE[mobility_mask.count_ones() as usize];

                b_feature_map.weak_king_attack_count += (wk_ring_mask & mov_mask).count_ones() as i32;
                b_feature_map.weak_king_attack_count += (bitmask.n_attack_masks[state.wk_index] & mov_mask).count_ones() as i32;

                b_feature_map.weak_king_attack_count += (wk_ring_mask & mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask)).count_ones() as i32;
                b_feature_map.weak_king_attack_count += (bitmask.n_attack_masks[state.wk_index] & mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask)).count_ones() as i32;
            },
            def::BB => {
                b_feature_map.mg_sqr_point += SQR_TABLE_BB[index];

                let threat_val = val_of(piece) / THREAT_FACTOR;
                if index_mask & b_attack_mask == 0 {
                    b_feature_map.threat_point -= (threat_val as f64).sqrt() as i32;

                    if index_mask & w_attack_mask != 0 {
                        b_feature_map.threat_point -= threat_val;
                    }
                } else if index_mask & w_attack_mask != 0 {
                    b_feature_map.threat_point -= (threat_val as f64).sqrt() as i32;
                }

                let mobility_mask = mov_mask & !wp_attack_mask & !bitboard.b_all & !(w_attack_mask & !b_attack_mask);
                b_feature_map.mobility += B_MOB_SCORE[mobility_mask.count_ones() as usize];

                b_feature_map.weak_king_attack_count += (wk_ring_mask & mov_mask).count_ones() as i32;
                b_feature_map.weak_king_attack_count += (wk_ring_mask & mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask)).count_ones() as i32;
            },
            def::BR => {
                b_feature_map.mg_sqr_point += SQR_TABLE_BR[index];

                let threat_val = val_of(piece) / THREAT_FACTOR;
                if index_mask & b_attack_mask == 0 {
                    b_feature_map.threat_point -= (threat_val as f64).sqrt() as i32;

                    if index_mask & w_attack_mask != 0 {
                        b_feature_map.threat_point -= threat_val;
                    }
                } else if index_mask & w_attack_mask != 0 {
                    b_feature_map.threat_point -= (threat_val as f64).sqrt() as i32;
                }

                let mobility_mask = mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask) & !bitboard.b_all & !(w_attack_mask & !b_attack_mask);
                b_feature_map.mobility += R_MOB_SCORE[mobility_mask.count_ones() as usize];

                b_feature_map.weak_king_attack_count += (wk_ring_mask & mov_mask).count_ones() as i32;
                b_feature_map.strong_king_attack_count += (wk_ring_mask & mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask)).count_ones() as i32;
            },
            def::BQ => {
                b_feature_map.mg_sqr_point += SQR_TABLE_BQ[index];

                let threat_val = val_of(piece) / THREAT_FACTOR;
                if index_mask & b_attack_mask == 0 {
                    if index_mask & w_attack_mask != 0 {
                        b_feature_map.threat_point -= threat_val;
                    }
                } else if index_mask & w_attack_mask != 0 {
                    b_feature_map.threat_point -= (threat_val as f64).sqrt() as i32;
                }

                let mobility_mask = mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask) & !bitboard.b_all & !(w_attack_mask & !b_attack_mask);
                b_feature_map.mobility += Q_MOB_SCORE[mobility_mask.count_ones() as usize];

                b_feature_map.weak_king_attack_count += (wk_ring_mask & mov_mask).count_ones() as i32;
                b_feature_map.strong_king_attack_count += (wk_ring_mask & mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask | wq_attack_mask)).count_ones() as i32;
            },
            def::BK => {
                b_feature_map.mg_sqr_point += SQR_TABLE_BK[index];
                b_feature_map.eg_sqr_point += SQR_TABLE_K_ENDGAME[index];
            },
            _ => {},
        }
    }

    (w_feature_map, b_feature_map)
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
    fn test_eval_passer() {
        zob_keys::init();
        bitmask::init();

        let state = State::new("8/p2P2k1/2P3pp/1P6/P3pP2/8/2K5/8 w - - 0 1");
        let (w_features_map, b_features_map) = extract_features(&state);

        assert_eq!(100 + 60 + 80 + 40 + 40, w_features_map.passed_pawn_point);
        assert_eq!(60 + 20 + 10 + 10, b_features_map.passed_pawn_point);
    }

    #[test]
    fn test_eval_pawns() {
        zob_keys::init();
        bitmask::init();

        let state = State::new("8/1p2k2p/p1p5/2p1p3/2PpP3/3P2P1/P5P1/5K2 w - - 0 1");
        let (w_features_map, b_features_map) = extract_features(&state);

        assert_eq!(2, w_features_map.doubled_pawn_count);
        assert_eq!(1, w_features_map.behind_pawn_count);
        assert_eq!(5, w_features_map.isolated_pawn_count);

        assert_eq!(2, b_features_map.doubled_pawn_count);
        assert_eq!(3, b_features_map.behind_pawn_count);
        assert_eq!(2, b_features_map.isolated_pawn_count);
    }
}
