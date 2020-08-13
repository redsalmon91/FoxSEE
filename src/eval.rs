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

static Q_VAL: i32 = 1200;
static R_VAL: i32 = 600;
static B_VAL: i32 = 400;
static N_VAL: i32 = 380;
static P_VAL: i32 = 100;

static ENDGAME_P_BONUS_VAL: i32 = 20;
static ENDGAME_R_BONUS_VAL: i32 = 50;
static ENDGAME_PAWN_ESSENTIAL_VAL: i32 = 50;

static KING_EXPOSED_PEN: i32 = -30;
static KING_LOST_CAS_RIGHTS_PEN: i32 = -30;

static KING_ATTACKER_PEN: i32 = -10;

static PASS_PAWN_VAL: [i32; def::DIM_SIZE] = [0, 10, 10, 20, 40, 80, 200, 0];
static CANDIDATE_PASS_PAWN_VAL: [i32; def::DIM_SIZE] = [0, 10, 10, 10, 20, 20, 0, 0];

static PASSED_PAWN_KING_DISTANCE_BASE_PEN: i32 = -6;
static CANDIDATE_PASSED_PAWN_KING_DISTANCE_BASE_PEN: i32 = -4;
static UNSTOPPABLE_PASS_PAWN_VAL: i32 = 90;
static CONTROLLED_PASS_PAWN_VAL: i32 = 50;
static DOUBLED_PAWN_PEN: i32 = -20;
static ISOLATED_PAWN_PEN: i32 = -20;
static OPEN_BEHIND_PAWN_PEN: i32 = -5;

static ROOK_OPEN_LINE_VAL: i32 = 30;
static QUEEN_OPEN_LINE_VAL: i32 = 20;

static BISHOP_PAIR_VAL: i32 = 30;

static HANGING_P_PEN: i32 = -10;
static HANGING_N_PEN: i32 = -20;
static HANGING_B_PEN: i32 = -20;
static HANGING_R_PEN: i32 = -30;
static HANGING_Q_PEN: i32 = -50;

static TOTAL_PHASE: i32 = 96;
static Q_PHASE_WEIGHT: i32 = 16;
static R_PHASE_WEIGHT: i32 = 8;
static B_PHASE_WEIGHT: i32 = 4;
static N_PHASE_WEIGHT: i32 = 4;

pub static ENDGAME_PHASE: i32 = 32;

static TEMPO_VAL: i32 = 10;

static N_MOB_SCORE: [i32; 9] = [-30,-20,-5, 0, 0, 5, 10, 15, 20];
static B_MOB_SCORE: [i32; 14] = [-20,-10, 5, 10, 20, 25, 25, 30, 30, 35, 40, 40, 45, 50];
static R_MOB_SCORE: [i32; 15] = [-30,-10, 0, 0, 0, 5, 10, 15, 20, 20, 20, 25, 30, 30, 30];
static Q_MOB_SCORE: [i32; 28] = [-15,-10,-5,-5,10,10,10,15,20,25,30,30,30,30,30,30,35,35,40,40,45,50,50,50,55,55,60,60];

static WK_PAWN_COVER_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_11111111_11111111_00000000;
static BK_PAWN_COVER_MASK: u64 = 0b00000000_11111111_11111111_00000000_00000000_00000000_00000000_00000000;

static SQR_TABLE_BP: [i32; def::BOARD_SIZE] = [
      0,  0,  0,  0,  0,  0,  0,  0,
     15, 30, 30, 30, 30, 30, 30, 15,
     10, 20, 20, 30, 30, 20, 20, 10,
      0,  0,  0, 25, 25,  0,  0,  0,
      0,  0,  0, 20, 20,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,-10,-10,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
];

static SQR_TABLE_BP_ENDGAME: [i32; def::BOARD_SIZE] = [
      0,  0,  0,  0,  0,  0,  0,  0,
     15, 30, 30, 30, 30, 30, 30, 15,
     10, 20, 20, 20, 20, 20, 20, 10,
      5, 15, 15, 15, 15, 15, 15,  5,
      0, 10, 10, 10, 10, 10, 10,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
];

static SQR_TABLE_WP: [i32; def::BOARD_SIZE] = [
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,-10,-10,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0, 20, 20,  0,  0,  0,
      0,  0,  5, 25, 25,  0,  0,  0,
     10, 20, 20, 30, 30, 20, 20, 10,
     15, 30, 30, 30, 30, 30, 30, 15,
      0,  0,  0,  0,  0,  0,  0,  0,
];

static SQR_TABLE_WP_ENDGAME: [i32; def::BOARD_SIZE] = [
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0, 10, 10, 10, 10, 10, 10,  0,
      5, 15, 15, 15, 15, 15, 15,  5,
     10, 20, 20, 20, 20, 20, 20, 10,
     15, 30, 30, 30, 30, 30, 30, 15,
      0,  0,  0,  0,  0,  0,  0,  0,
];

static SQR_TABLE_BN: [i32; def::BOARD_SIZE] = [
    -60,-20,-20,-20,-20,-20,-20,-60,
    -30,-30, 20, 10, 10, 20,-30,-30,
    -20,  0, 15, 20, 20, 15,  0,-20,
    -20,  5, 10, 25, 25, 10,  5,-20,
    -20,  0, 10, 20, 20, 10,  0,-20,
    -20,  5, 10,  0,  0, 10,  5,-20,
    -30,-30,  0,  0,  0,  0,-30,-30,
    -60,-20,-20,-20,-20,-20,-20,-60,
];

static SQR_TABLE_WN: [i32; def::BOARD_SIZE] = [
    -60,-20,-20,-20,-20,-20,-20,-60,
    -30,-30,  0,  0,  0,  0,-30,-30,
    -20,  5, 10,  0,  0, 10,  5,-20,
    -20,  0, 10, 20, 20, 10,  0,-20,
    -20,  5, 10, 25, 25, 10,  5,-20,
    -20,  0, 15, 20, 20, 15,  0,-20,
    -30,-30, 20, 10, 10, 20,-30,-30,
    -60,-20,-20,-20,-20,-20,-20,-60,
];

static SQR_TABLE_BB: [i32; def::BOARD_SIZE] = [
    -50,-10,-10,-10,-10,-10,-10,-50,
    -20,  0, 10,  0,  0, 10,  0,-20,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10,  5,  5, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -50,-10,-10,-10,-10,-10,-10,-50,
];

static SQR_TABLE_WB: [i32; def::BOARD_SIZE] = [
    -50,-10,-10,-10,-10,-10,-10,-50,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -10, 10, 10,  5,  5, 10, 10,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -20,  0, 10,  0,  0, 10,  0,-20,
    -50,-10,-10,-10,-10,-10,-10,-50,
];

static SQR_TABLE_BR: [i32; def::BOARD_SIZE] = [
     10, 10, 20, 20, 20, 20, 10, 10,
     10, 20, 30, 30, 30, 30, 20, 10,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
    -10, -5,  0,  0,  0,  0, -5,-10,
     -5,  0,  0,  0,  0,  0,  0, -5,
];

static SQR_TABLE_WR: [i32; def::BOARD_SIZE] = [
     -5,  0,  0,  0,  0,  0,  0, -5,
    -10, -5,  0,  0,  0,  0, -5,-10,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     10, 20, 30, 30, 30, 30, 20, 10,
     10, 10, 20, 20, 20, 20, 10, 10,
];

static SQR_TABLE_BQ: [i32; def::BOARD_SIZE] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,-10,  0,  0,  0,  0,-10,-10,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
    -10,-10,  0,  0,  0,  0,-10,-10,
    -20,-10,-10, -5, -5,-10,-10,-20,
];

static SQR_TABLE_WQ: [i32; def::BOARD_SIZE] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,-10,  0,  0,  0,  0,-10,-10,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
    -10,-10,  0,  0,  0,  0,-10,-10,
    -20,-10,-10, -5, -5,-10,-10,-20,
];

static SQR_TABLE_BK: [i32; def::BOARD_SIZE] = [
    -90,-90,-90,-90,-90,-90,-90,-90,
    -90,-90,-90,-90,-90,-90,-90,-90,
    -90,-90,-90,-90,-90,-90,-90,-90,
    -90,-90,-90,-90,-90,-90,-90,-90,
    -90,-90,-90,-90,-90,-90,-90,-90,
    -60,-60,-60,-60,-60,-60,-60,-60,
      0, 10,-10,-30,-30,-10, 10, 0,
      0, 20,  0,-20,-20,  0, 20, 0,
];

static SQR_TABLE_WK: [i32; def::BOARD_SIZE] = [
      0, 20,  0,-20,-20,  0, 20,  0,
      0, 10,-10,-30,-30,-10, 10,  0,
    -60,-60,-60,-60,-60,-60,-60,-60,
    -90,-90,-90,-90,-90,-90,-90,-90,
    -90,-90,-90,-90,-90,-90,-90,-90,
    -90,-90,-90,-90,-90,-90,-90,-90,
    -90,-90,-90,-90,-90,-90,-90,-90,
    -90,-90,-90,-90,-90,-90,-90,-90,
];

static SQR_TABLE_K_ENDGAME: [i32; def::BOARD_SIZE] = [
    -50,-40,-30,-20,-20,-30,-40,-50,
    -40,-30,-10,-10,-10,-10,-30,-40,
    -30,-10, 10, 30, 30, 10,-10,-30,
    -30,-10, 30, 50, 50, 30,-10,-30,
    -30,-10, 30, 50, 50, 30,-10,-30,
    -30,-10, 10, 30, 30, 10,-10,-30,
    -30,-30,-10,-10,-10,-10,-30,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

#[derive(PartialEq, Debug)]
pub struct FeatureMap {
    midgame_sqr_point_count: i32,
    endgame_sqr_point_count: i32,

    passed_pawn_point: i32,
    candidate_passed_pawn_point: i32,
    passed_pawn_king_distance: i32,
    candidate_passed_pawn_king_distance: i32,
    unstoppable_passed_pawn_count: i32,
    controlled_passed_pawn_count: i32,
    connected_pawn_count: i32,
    doubled_pawn_count: i32,
    isolated_pawn_count: i32,
    open_behind_pawn_count: i32,

    open_rook_count: i32,
    open_queen_count: i32,

    mobility: i32,

    hanging_p_count: i32,
    hanging_n_count: i32,
    hanging_b_count: i32,
    hanging_r_count: i32,
    hanging_q_count: i32,

    king_exposure_count: i32,
    king_lost_cas_rights: i32,
    king_attacker_count: i32,
}

impl FeatureMap {
    pub fn empty() -> Self {
        FeatureMap {
            midgame_sqr_point_count: 0,
            endgame_sqr_point_count: 0,

            passed_pawn_point: 0,
            candidate_passed_pawn_point: 0,
            passed_pawn_king_distance: 0,
            candidate_passed_pawn_king_distance: 0,
            unstoppable_passed_pawn_count: 0,
            controlled_passed_pawn_count: 0,
            connected_pawn_count: 0,
            doubled_pawn_count: 0,
            isolated_pawn_count: 0,
            open_behind_pawn_count: 0,

            open_rook_count: 0,
            open_queen_count: 0,

            mobility: 0,

            hanging_p_count: 0,
            hanging_n_count: 0,
            hanging_b_count: 0,
            hanging_r_count: 0,
            hanging_q_count: 0,

            king_exposure_count: 0,
            king_lost_cas_rights: 0,
            king_attacker_count: 0,
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

    let mut score = bitboard.w_queen.count_ones() as i32 * Q_VAL
    + bitboard.w_rook.count_ones() as i32 * R_VAL
    + bitboard.w_bishop.count_ones() as i32 * B_VAL
    + bitboard.w_knight.count_ones() as i32 * N_VAL
    + bitboard.w_pawn.count_ones() as i32 * P_VAL
    - bitboard.b_queen.count_ones() as i32 * Q_VAL
    - bitboard.b_rook.count_ones() as i32 * R_VAL
    - bitboard.b_bishop.count_ones() as i32 * B_VAL
    - bitboard.b_knight.count_ones() as i32 * N_VAL
    - bitboard.b_pawn.count_ones() as i32 * P_VAL;

    if score > 0 && (bitboard.w_pawn | bitboard.w_rook | bitboard.w_queen) == 0 && (bitboard.w_knight | bitboard.w_bishop).count_ones() == 1 && bitboard.b_pawn.count_ones() == 1 {
        return (0, false)
    }

    if score < 0 && (bitboard.b_pawn | bitboard.b_rook | bitboard.b_queen) == 0 && (bitboard.b_knight | bitboard.b_bishop).count_ones() == 1 && bitboard.w_pawn.count_ones() == 1 {
        return (0, false)
    }

    if is_endgame_with_different_colored_bishop {
        if score > 0  {
            score -= P_VAL;
        } else if score < 0 {
            score += P_VAL;
        }
    }

    let mut endgame_material_score = 0;

    endgame_material_score += bitboard.w_pawn.count_ones() as i32 * ENDGAME_P_BONUS_VAL;
    endgame_material_score += bitboard.w_rook.count_ones() as i32 * ENDGAME_R_BONUS_VAL;

    endgame_material_score -= bitboard.b_pawn.count_ones() as i32 * ENDGAME_P_BONUS_VAL;
    endgame_material_score -= bitboard.b_rook.count_ones() as i32 * ENDGAME_R_BONUS_VAL;

    if bitboard.w_bishop.count_ones() > 1 {
        endgame_material_score += BISHOP_PAIR_VAL;
    }

    if bitboard.b_bishop.count_ones() > 1 {
        endgame_material_score -= BISHOP_PAIR_VAL;
    }

    if bitboard.w_pawn == 0 {
        endgame_material_score -= ENDGAME_PAWN_ESSENTIAL_VAL;
    }

    if bitboard.b_pawn == 0 {
        endgame_material_score += ENDGAME_PAWN_ESSENTIAL_VAL;
    }

    let phase = get_phase(state);

    let score_sign = if state.player == def::PLAYER_W {
        1
    } else {
        -1
    };

    ((score + endgame_material_score * (TOTAL_PHASE - phase) / TOTAL_PHASE) * score_sign, false)
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

    let shared_positional_score =
        w_features_map.passed_pawn_point
        + w_features_map.candidate_passed_pawn_point        
        + w_features_map.mobility
        + w_features_map.hanging_p_count * HANGING_P_PEN
        + w_features_map.hanging_n_count * HANGING_N_PEN
        + w_features_map.hanging_b_count * HANGING_B_PEN
        + w_features_map.hanging_r_count * HANGING_R_PEN
        + w_features_map.hanging_q_count * HANGING_Q_PEN
        - b_features_map.passed_pawn_point
        - b_features_map.candidate_passed_pawn_point
        - b_features_map.mobility
        - b_features_map.hanging_p_count * HANGING_P_PEN
        - b_features_map.hanging_n_count * HANGING_N_PEN
        - b_features_map.hanging_b_count * HANGING_B_PEN
        - b_features_map.hanging_r_count * HANGING_R_PEN
        - b_features_map.hanging_q_count * HANGING_Q_PEN;

    let midgame_positional_score =
        def::square(w_features_map.king_attacker_count) * KING_ATTACKER_PEN
        + w_features_map.midgame_sqr_point_count
        + w_features_map.open_rook_count * ROOK_OPEN_LINE_VAL
        + w_features_map.open_queen_count * QUEEN_OPEN_LINE_VAL
        + def::square(w_features_map.king_exposure_count) * KING_EXPOSED_PEN
        + w_features_map.king_lost_cas_rights * KING_LOST_CAS_RIGHTS_PEN
        + w_features_map.open_behind_pawn_count * OPEN_BEHIND_PAWN_PEN
        - def::square(b_features_map.king_attacker_count) * KING_ATTACKER_PEN
        - b_features_map.midgame_sqr_point_count
        - b_features_map.open_rook_count * ROOK_OPEN_LINE_VAL
        - b_features_map.open_queen_count * QUEEN_OPEN_LINE_VAL
        - def::square(b_features_map.king_exposure_count) * KING_EXPOSED_PEN
        - b_features_map.king_lost_cas_rights * KING_LOST_CAS_RIGHTS_PEN
        - b_features_map.open_behind_pawn_count * OPEN_BEHIND_PAWN_PEN;

    let endgame_positional_score =
        w_features_map.doubled_pawn_count * DOUBLED_PAWN_PEN
        + w_features_map.isolated_pawn_count * ISOLATED_PAWN_PEN
        + w_features_map.endgame_sqr_point_count
        + w_features_map.passed_pawn_king_distance * PASSED_PAWN_KING_DISTANCE_BASE_PEN
        + w_features_map.candidate_passed_pawn_king_distance * CANDIDATE_PASSED_PAWN_KING_DISTANCE_BASE_PEN
        + w_features_map.unstoppable_passed_pawn_count * UNSTOPPABLE_PASS_PAWN_VAL
        + w_features_map.controlled_passed_pawn_count * CONTROLLED_PASS_PAWN_VAL
        - b_features_map.doubled_pawn_count * DOUBLED_PAWN_PEN
        - b_features_map.isolated_pawn_count * ISOLATED_PAWN_PEN
        - b_features_map.endgame_sqr_point_count
        - b_features_map.passed_pawn_king_distance * PASSED_PAWN_KING_DISTANCE_BASE_PEN
        - b_features_map.candidate_passed_pawn_king_distance * CANDIDATE_PASSED_PAWN_KING_DISTANCE_BASE_PEN
        - b_features_map.unstoppable_passed_pawn_count * UNSTOPPABLE_PASS_PAWN_VAL
        - b_features_map.controlled_passed_pawn_count * CONTROLLED_PASS_PAWN_VAL;

    let phase = get_phase(state);

    let extra_score = shared_positional_score + (midgame_positional_score * phase + endgame_positional_score * (TOTAL_PHASE - phase)) / TOTAL_PHASE;

    material_score + extra_score * score_sign + TEMPO_VAL
}

#[inline]
pub fn extract_features(state: &State) -> (FeatureMap, FeatureMap) {
    let squares = state.squares;
    let index_masks = state.bitmask.index_masks;
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

        let index_mask = index_masks[index];

        match moving_piece {
            def::WP => {
                w_feature_map.midgame_sqr_point_count += SQR_TABLE_WP[index];
                w_feature_map.endgame_sqr_point_count += SQR_TABLE_WP_ENDGAME[index];

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
                } else if is_connected && forward_mask & file_mask & (bitboard.w_pawn | bitboard.b_pawn) == 0 && (forward_mask & bitboard.b_pawn).count_ones() == 1 {
                    w_feature_map.candidate_passed_pawn_point += CANDIDATE_PASS_PAWN_VAL[rank as usize];

                    let king_distance = def::get_file_distance(index, state.wk_index);
                    w_feature_map.candidate_passed_pawn_king_distance += king_distance;

                    let opponent_king_distance = def::get_file_distance(index, state.bk_index);
                    w_feature_map.candidate_passed_pawn_king_distance -= opponent_king_distance;
                }

                if is_connected {
                    w_feature_map.connected_pawn_count += 1;
                } else {
                    if forward_mask & !file_mask & bitboard.w_pawn == 0 {
                        w_feature_map.isolated_pawn_count += 1;
                    } else if forward_mask & file_mask & bitboard.b_pawn == 0 {
                        w_feature_map.open_behind_pawn_count += 1;
                    }
                }

                if (file_mask & bitboard.w_pawn).count_ones() > 1 {
                    w_feature_map.doubled_pawn_count += 1;
                }
            },
            def::BP => {
                b_feature_map.midgame_sqr_point_count += SQR_TABLE_BP[index];
                b_feature_map.endgame_sqr_point_count += SQR_TABLE_BP_ENDGAME[index];

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
                } else if is_connected && forward_mask & file_mask & (bitboard.b_pawn | bitboard.w_pawn) == 0 && (forward_mask & bitboard.w_pawn).count_ones() == 1 {
                    b_feature_map.candidate_passed_pawn_point += CANDIDATE_PASS_PAWN_VAL[rank as usize];

                    let king_distance = def::get_file_distance(index, state.bk_index);
                    b_feature_map.candidate_passed_pawn_king_distance += king_distance;

                    let opponent_king_distance = def::get_file_distance(index, state.wk_index);
                    b_feature_map.candidate_passed_pawn_king_distance -= opponent_king_distance;
                }

                if is_connected {
                    b_feature_map.connected_pawn_count += 1;
                } else {
                    if forward_mask & !file_mask & bitboard.b_pawn == 0 {
                        b_feature_map.isolated_pawn_count += 1;
                    } else if forward_mask & file_mask & bitboard.w_pawn == 0 {
                        b_feature_map.open_behind_pawn_count += 1;
                    }
                }

                if (file_mask & bitboard.b_pawn).count_ones() > 1 {
                    b_feature_map.doubled_pawn_count += 1;
                }
            },

            def::WN => {
                w_feature_map.midgame_sqr_point_count += SQR_TABLE_WN[index];

                let mov_mask = bitmask.n_attack_masks[index];
                wn_attack_mask |= mov_mask;
                mov_mask_map[index] = mov_mask;
            },
            def::BN => {
                b_feature_map.midgame_sqr_point_count += SQR_TABLE_BN[index];

                let mov_mask = bitmask.n_attack_masks[index];
                bn_attack_mask |= mov_mask;
                mov_mask_map[index] = mov_mask;
            },

            def::WB => {
                w_feature_map.midgame_sqr_point_count += SQR_TABLE_WB[index];

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
                b_feature_map.midgame_sqr_point_count += SQR_TABLE_BB[index];

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
                w_feature_map.midgame_sqr_point_count += SQR_TABLE_WR[index];

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

                let file_mask = file_masks[index];
                if file_mask & (bitboard.w_all ^ bitboard.w_rook) == 0 {
                    if file_mask & bitboard.b_all == 0 {
                        w_feature_map.open_rook_count += 1;
                    }
                }
            },
            def::BR => {
                b_feature_map.midgame_sqr_point_count += SQR_TABLE_BR[index];

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

                let file_mask = file_masks[index];
                if file_mask & (bitboard.b_all ^ bitboard.b_rook) == 0 {
                    if file_mask & bitboard.w_all == 0 {
                        b_feature_map.open_rook_count += 1;
                    }
                }
            },

            def::WQ => {
                w_feature_map.midgame_sqr_point_count += SQR_TABLE_WQ[index];

                let file_mask = file_masks[index];
                if file_mask & ((bitboard.w_all | bitboard.b_all) ^ index_mask) == 0 {
                    w_feature_map.open_queen_count += 1;
                }

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
                b_feature_map.midgame_sqr_point_count += SQR_TABLE_BQ[index];

                let file_mask = file_masks[index];
                if file_mask & ((bitboard.w_all | bitboard.b_all) ^ index_mask) == 0 {
                    b_feature_map.open_queen_count += 1;
                }

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

            def::WK => {
                if bitboard.b_queen != 0 {
                    w_feature_map.midgame_sqr_point_count += SQR_TABLE_WK[index];
                }

                w_feature_map.endgame_sqr_point_count += SQR_TABLE_K_ENDGAME[index];

                let file_mask = file_masks[index];

                if bitboard.b_queen != 0 {
                    if file_mask & bitboard.w_pawn & WK_PAWN_COVER_MASK == 0 {
                        w_feature_map.king_exposure_count += 1;
                    }

                    if (bitmask.wk_attack_zone_masks[state.wk_index] & bitboard.w_pawn).count_ones() < 2 {
                        w_feature_map.king_exposure_count += 1;
                    }
                }
            },
            def::BK => {
                if bitboard.w_queen != 0 {
                    b_feature_map.midgame_sqr_point_count += SQR_TABLE_BK[index];
                }

                b_feature_map.endgame_sqr_point_count += SQR_TABLE_K_ENDGAME[index];

                let file_mask = file_masks[index];

                if bitboard.w_queen != 0 {
                    if file_mask & bitboard.b_pawn & BK_PAWN_COVER_MASK == 0 {
                        b_feature_map.king_exposure_count += 1;
                    }

                    if (bitmask.bk_attack_zone_masks[state.bk_index] & bitboard.b_pawn).count_ones() < 2 {
                        b_feature_map.king_exposure_count += 1;
                    }
                }
            },
            _ => {}
        }
    }

    let wk_ring_mask = bitmask.wk_attack_zone_masks[state.wk_index];
    let bk_ring_mask = bitmask.bk_attack_zone_masks[state.bk_index];

    for index in start_index..end_index {
        let moving_piece = squares[index];

        if moving_piece == 0 {
            continue
        }

        match moving_piece {
            def::WN => {
                let mov_mask = mov_mask_map[index];
                w_feature_map.mobility += N_MOB_SCORE[(mov_mask & !bp_attack_mask & !bitboard.w_all).count_ones() as usize];

                b_feature_map.king_attacker_count += (bk_ring_mask & mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask)).count_ones() as i32;
            },
            def::WB => {
                let mov_mask = mov_mask_map[index];
                w_feature_map.mobility += B_MOB_SCORE[(mov_mask & !bp_attack_mask & !bitboard.w_all).count_ones() as usize];

                b_feature_map.king_attacker_count += (bk_ring_mask & mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask)).count_ones() as i32;
            },
            def::WR => {
                let mov_mask = mov_mask_map[index];
                w_feature_map.mobility += R_MOB_SCORE[(mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask) & !bitboard.w_all).count_ones() as usize];

                b_feature_map.king_attacker_count += (bk_ring_mask & mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask)).count_ones() as i32;
            },
            def::WQ => {
                let mov_mask = mov_mask_map[index];
                w_feature_map.mobility += Q_MOB_SCORE[(mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask) & !bitboard.w_all).count_ones() as usize];

                b_feature_map.king_attacker_count += (bk_ring_mask & mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask | bq_attack_mask)).count_ones() as i32;
            },
            def::BN => {
                let mov_mask = mov_mask_map[index];
                b_feature_map.mobility += N_MOB_SCORE[(mov_mask & !wp_attack_mask & !bitboard.b_all).count_ones() as usize];

                w_feature_map.king_attacker_count += (wk_ring_mask & mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask)).count_ones() as i32;
            },
            def::BB => {
                let mov_mask = mov_mask_map[index];
                b_feature_map.mobility += B_MOB_SCORE[(mov_mask & !wp_attack_mask & !bitboard.b_all).count_ones() as usize];

                w_feature_map.king_attacker_count += (wk_ring_mask & mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask)).count_ones() as i32;
            },
            def::BR => {
                let mov_mask = mov_mask_map[index];
                b_feature_map.mobility += R_MOB_SCORE[(mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask) & !bitboard.b_all).count_ones() as usize];

                w_feature_map.king_attacker_count += (wk_ring_mask & mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask)).count_ones() as i32;
            },
            def::BQ => {
                let mov_mask = mov_mask_map[index];
                b_feature_map.mobility += Q_MOB_SCORE[(mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask) & !bitboard.b_all).count_ones() as usize];

                w_feature_map.king_attacker_count += (wk_ring_mask & mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask | wq_attack_mask)).count_ones() as i32;
            },
            _ => {},
        }
    }

    let w_attack_mask = wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask | wq_attack_mask | bitmask.k_attack_masks[state.wk_index];
    let b_attack_mask = bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask | bq_attack_mask | bitmask.k_attack_masks[state.bk_index];

    w_feature_map.hanging_p_count = (bitboard.w_pawn & !w_attack_mask & b_attack_mask).count_ones() as i32;
    w_feature_map.hanging_n_count = ((bitboard.w_knight & bp_attack_mask) | (bitboard.w_knight & !w_attack_mask & b_attack_mask)).count_ones() as i32;
    w_feature_map.hanging_b_count = ((bitboard.w_bishop & bp_attack_mask) | (bitboard.w_bishop & !w_attack_mask & b_attack_mask)).count_ones() as i32;
    w_feature_map.hanging_r_count = ((bitboard.w_rook & (bp_attack_mask | bn_attack_mask | bb_attack_mask)) | (bitboard.w_rook & !w_attack_mask & b_attack_mask)).count_ones() as i32;
    w_feature_map.hanging_q_count = ((bitboard.w_queen & (bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask)) | (bitboard.b_queen & !w_attack_mask & b_attack_mask)).count_ones() as i32;

    b_feature_map.hanging_p_count = (bitboard.b_pawn & !b_attack_mask & w_attack_mask).count_ones() as i32;
    b_feature_map.hanging_n_count = ((bitboard.b_knight & wp_attack_mask) | (bitboard.b_knight & !b_attack_mask & w_attack_mask)).count_ones() as i32;
    b_feature_map.hanging_b_count = ((bitboard.b_bishop & wp_attack_mask) | (bitboard.b_bishop & !b_attack_mask & w_attack_mask)).count_ones() as i32;
    b_feature_map.hanging_r_count = ((bitboard.b_rook & (wp_attack_mask | wn_attack_mask | wb_attack_mask)) | (bitboard.b_rook & !b_attack_mask & w_attack_mask)).count_ones() as i32;
    b_feature_map.hanging_q_count = ((bitboard.b_queen & (wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask)) | (bitboard.b_queen & !b_attack_mask & w_attack_mask)).count_ones() as i32;

    if bitboard.b_queen != 0 && (state.cas_rights | state.cas_history) & 0b1100 == 0 {
        w_feature_map.king_lost_cas_rights = 1;
    }

    if bitboard.w_queen != 0 && (state.cas_rights | state.cas_history) & 0b0011 == 0 {
        b_feature_map.king_lost_cas_rights = 1;
    }

    (w_feature_map, b_feature_map)
}
