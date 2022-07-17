/*
 * Copyright (C) 2020-2022 Zixiao Han
 */

use crate::{
    bitmask,
    def,
    mov_table,
    state::State,
    util,
};

pub const MATE_VAL: i32 = 20000;
pub const TERM_VAL: i32 = 10000;

pub const EQUAL_EXCHANGE: i32 = -20;

const Q_VAL: i32 = 1400;
const R_VAL: i32 = 700;
const B_VAL: i32 = 500;
const N_VAL: i32 = 490;
const P_VAL: i32 = 125;

const EG_Q_VAL: i32 = 90;
const EG_R_VAL: i32 = 50;
const EG_P_VAL: i32 = 10;

const EG_PAWN_ESSENTIAL_VAL: i32 = 190;
const EG_DIFFERENT_COLORED_BISHOP_VAL: i32 = 90;
const EG_DIFFERENT_COLORED_BISHOP_WITH_ROOK_VAL: i32 = 50;
const EG_BISHOP_PAIR_BONUS: i32 = 50;
const EG_RN_KNIGHT_PROTECTED_BONUS: i32 = 50;

const PASS_PAWN_VAL: [i32; def::DIM_SIZE] = [0, 50, 50, 80, 100, 150, 190, 0];
const CONNECTED_PASS_PAWN_BONUS: [i32; def::DIM_SIZE] = [0, 20, 20, 40, 60, 80, 100, 0];
const CANDIDATE_PASS_PAWN_VAL: [i32; def::DIM_SIZE] = [0, 10, 10, 10, 20, 20, 0, 0];

const KING_IN_PASSER_PATH_BONUS: i32 = 50;

const EG_P_SQR_DIFF_MULTIPLIER: i32 = 2;

const CONTROLLED_PASS_PAWN_VAL: i32 = 50;
const DOUBLED_PAWN_PEN: i32 = -20;
const ISOLATED_PAWN_PEN: i32 = -10;
const BEHIND_PAWN_PEN: i32 = -10;

const KING_EXPOSURE_PEN: i32 = -20;
const STRONG_K_ATTACK_COUNT_MULTIPLIER: i32 = 5;

const K_ATTACK_SCORE: [i32; 200] = [
      0,   0,   5,  10,  20,  30,  40,  50,  60,  70,
     80,  90, 100, 120, 140, 160, 180, 200, 240, 280,
    320, 380, 440, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
];

const KING_LOST_CAS_RIGHTS_PEN: i32 = -50;
const PIN_PEN: i32 = -20;
const SEMI_PIN_PEN: i32 = -10;

const ROOK_OPEN_BONUS: i32 = 10;

const THREAT_DISCOUNT_FACTOR: i32 = 12;

const WEAK_SQR_PEN: i32 = -5;

const TOTAL_PHASE: i32 = 96;
const TOTAL_PAWN_PHASE: i32 = 16;

const Q_PHASE_WEIGHT: i32 = 16;
const R_PHASE_WEIGHT: i32 = 8;
const B_PHASE_WEIGHT: i32 = 4;
const N_PHASE_WEIGHT: i32 = 4;
const EG_PHASE: i32 = 32;

const TEMPO_VAL: i32 = 20;

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
      0,  0,  0, 25, 25,  0,  0,  0,
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

const SQR_TABLE_W_K_SIDE_EXTRA: [i32; def::BOARD_SIZE] = [
      0,  0,  0,  0,  0, 20,  0,  0,
      0,  0,  0,  0,  0, 20, 20, 20,
      0,  0,  0,  0,  0, 20, 20, 20,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
];

const SQR_TABLE_W_Q_SIDE_EXTRA: [i32; def::BOARD_SIZE] = [
      0,  0, 20,  0,  0,  0,  0,  0,
     20, 20, 20,  0,  0,  0,  0,  0,
     20, 20, 20,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
];

const SQR_TABLE_W_CENTER_EXTRA: [i32; def::BOARD_SIZE] = [
      0,  0, 20,  0,  0, 20,  0,  0,
      0,  0, 20, 20, 20, 20,  0,  0,
      0,  0, 20, 20, 20, 20,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
];

const SQR_TABLE_B_K_SIDE_EXTRA: [i32; def::BOARD_SIZE] = [
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0, 20, 20, 20,
      0,  0,  0,  0,  0, 20, 20, 20,
      0,  0,  0,  0,  0, 20,  0,  0,
];

const SQR_TABLE_B_Q_SIDE_EXTRA: [i32; def::BOARD_SIZE] = [
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
     20, 20, 20,  0,  0,  0,  0,  0,
     20, 20, 20,  0,  0,  0,  0,  0,
      0,  0, 20,  0,  0,  0,  0,  0,
];

const SQR_TABLE_B_CENTER_EXTRA: [i32; def::BOARD_SIZE] = [
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0, 20, 20, 20, 20,  0,  0,
      0,  0, 20, 20, 20, 20,  0,  0,
      0,  0, 20,  0,  0, 20,  0,  0,
];

const W_PAWN_PROMO_RANK: u64 = 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
const B_PAWN_PROMO_RANK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;

const WK_K_SIDE_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_11100000_11100000_11100000;
const WK_Q_SIDE_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000111_00000111_00000111;
const WK_CENTER_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00011000_00011000_00011000;

const BK_K_SIDE_MASK: u64 = 0b11100000_11100000_11100000_00000000_00000000_00000000_00000000_00000000;
const BK_Q_SIDE_MASK: u64 = 0b00000111_00000111_00000111_00000000_00000000_00000000_00000000_00000000;
const BK_CENTER_MASK: u64 = 0b00011000_00011000_00011000_00000000_00000000_00000000_00000000_00000000;

const W_CRITICAL_RANK_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_11111111_00000000_00000000;
const B_CRITICAL_RANK_MASK: u64 = 0b00000000_00000000_11111111_00000000_00000000_00000000_00000000_00000000;

#[derive(PartialEq, Debug)]
pub struct FeatureMap {
    mg_sqr_point: i32,
    eg_sqr_point: i32,

    passed_pawn_point: i32,
    controlled_passed_pawn_count: i32,
    doubled_pawn_count: i32,
    isolated_pawn_count: i32,
    behind_pawn_count: i32,

    king_in_passer_path_count: i32,

    mobility: i32,
    eg_mobility: i32,

    rook_open_count: i32,

    threat_point: i32,
    pin_count: i32,
    semi_pin_count: i32,

    weak_sqr_count: i32,

    strong_king_attack_count: i32,
    weak_king_attack_count: i32,

    king_exposure_count: i32,
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

            king_in_passer_path_count: 0,

            mobility: 0,
            eg_mobility: 0,

            rook_open_count: 0,

            threat_point: 0,
            pin_count: 0,
            semi_pin_count: 0,

            weak_sqr_count: 0,

            strong_king_attack_count: 0,
            weak_king_attack_count: 0,

            king_exposure_count: 0,
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

pub fn get_square_val_diff(state: &mut State, moving_piece: u8, from_index: usize, to_index: usize) -> i32 {
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

pub fn is_in_endgame(state: &mut State) -> bool {
    get_phase(state) <= EG_PHASE
}

pub fn has_promoting_pawn(state: &State, player: u8) -> bool {
    if player == def::PLAYER_W {
        state.bitboard.w_pawn & W_PAWN_PROMO_RANK != 0
    } else {
        state.bitboard.b_pawn & B_PAWN_PROMO_RANK != 0
    }
}

pub fn eval_materials(state: &mut State) -> (i32, bool) {
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

    if bitboard.w_knight | bitboard.b_knight | bitboard.w_queen | bitboard.b_queen == 0 {
        if w_bishop_count == 1 && b_bishop_count == 1 {
            let mut wb_reachable_mask = 0;
            let mut bb_reachable_mask = 0;

            for index in 0..def::BOARD_SIZE {
                match state.squares[index] {
                    def::WB => {
                        wb_reachable_mask = bitmask.b_cover_masks[index];
                    },
                    def::BB => {
                        bb_reachable_mask = bitmask.b_cover_masks[index]
                    },
                    _ => {}
                }
            }

            is_endgame_with_different_colored_bishop = wb_reachable_mask & bb_reachable_mask == 0;
        }
    }

    let mut bishop_pair_bonus = 0;

    if is_endgame_with_different_colored_bishop {
        if bitboard.w_rook | bitboard.b_rook == 0 {
            if material_score > 0  {
                eg_score -= EG_DIFFERENT_COLORED_BISHOP_VAL;
            } else if material_score < 0 {
                eg_score += EG_DIFFERENT_COLORED_BISHOP_VAL;
            }
        } else {
            if material_score > 0  {
                eg_score -= EG_DIFFERENT_COLORED_BISHOP_WITH_ROOK_VAL;
            } else if material_score < 0 {
                eg_score += EG_DIFFERENT_COLORED_BISHOP_WITH_ROOK_VAL;
            }
        }
    } else {
        if w_bishop_count > 1 {
            bishop_pair_bonus += EG_BISHOP_PAIR_BONUS;
        }

        if b_bishop_count > 1 {
            bishop_pair_bonus -= EG_BISHOP_PAIR_BONUS;
        }
    }

    let phase = get_phase(state);
    let pawn_phase = get_pawn_phase(state);

    let score_sign = if state.player == def::PLAYER_W {
        1
    } else {
        -1
    };

    ((material_score + bishop_pair_bonus * (TOTAL_PAWN_PHASE - pawn_phase) / TOTAL_PAWN_PHASE + eg_score * (TOTAL_PHASE - phase) / TOTAL_PHASE) * score_sign, false)
}

pub fn get_phase(state: &mut State) -> i32 {
    (state.wq_count + state.bq_count) * Q_PHASE_WEIGHT
    + (state.wr_count + state.br_count) * R_PHASE_WEIGHT
    + (state.wb_count + state.bb_count) * B_PHASE_WEIGHT
    + (state.wn_count + state.bn_count) * N_PHASE_WEIGHT
}

pub fn get_pawn_phase(state: &mut State) -> i32 {
    state.wp_count + state.bp_count
}

pub fn eval_state(state: &mut State, material_score: i32) -> i32 {
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
        + w_features_map.pin_count * PIN_PEN
        + w_features_map.semi_pin_count * SEMI_PIN_PEN
        + w_features_map.rook_open_count * ROOK_OPEN_BONUS
        + w_features_map.weak_sqr_count * WEAK_SQR_PEN
        + w_features_map.king_exposure_count * KING_EXPOSURE_PEN
        + K_ATTACK_SCORE[w_king_attack_count as usize]
        - b_features_map.mg_sqr_point
        - b_features_map.pin_count * PIN_PEN
        + b_features_map.semi_pin_count * SEMI_PIN_PEN
        - b_features_map.rook_open_count * ROOK_OPEN_BONUS
        - b_features_map.weak_sqr_count * WEAK_SQR_PEN
        - b_features_map.king_exposure_count * KING_EXPOSURE_PEN
        - K_ATTACK_SCORE[b_king_attack_count as usize];

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
        + w_features_map.controlled_passed_pawn_count * CONTROLLED_PASS_PAWN_VAL
        + w_features_map.eg_mobility
        + w_features_map.king_in_passer_path_count * KING_IN_PASSER_PATH_BONUS
        - b_features_map.eg_sqr_point
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
        + w_features_map.passed_pawn_point
        + w_features_map.threat_point / THREAT_DISCOUNT_FACTOR
        + w_features_map.behind_pawn_count * BEHIND_PAWN_PEN
        + w_features_map.isolated_pawn_count * ISOLATED_PAWN_PEN
        + w_features_map.doubled_pawn_count * DOUBLED_PAWN_PEN
        - b_features_map.mobility
        - b_features_map.passed_pawn_point
        - b_features_map.threat_point / THREAT_DISCOUNT_FACTOR
        - b_features_map.behind_pawn_count * BEHIND_PAWN_PEN
        - b_features_map.isolated_pawn_count * ISOLATED_PAWN_PEN
        - b_features_map.doubled_pawn_count * DOUBLED_PAWN_PEN;

    let phase = get_phase(state);

    let extra_score = midgame_positional_score * phase / TOTAL_PHASE + endgame_positional_score * (TOTAL_PHASE - phase) / TOTAL_PHASE + shared_positional_score;

    material_score + extra_score * score_sign + TEMPO_VAL
}

fn extract_features(state: &mut State) -> (FeatureMap, FeatureMap) {
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

    let mut wq_index = 0;
    let mut bq_index = 0;

    let mut mov_mask_map = [0; def::BOARD_SIZE];

    let occupy_mask = bitboard.w_all | bitboard.b_all;
    let start_index = occupy_mask.trailing_zeros() as usize;
    let end_index = def::BOARD_SIZE - occupy_mask.leading_zeros() as usize;

    let piece_mask = bitboard.w_knight | bitboard.w_bishop | bitboard.w_rook | bitboard.w_queen | bitboard.b_knight | bitboard.b_bishop | bitboard.b_rook | bitboard.b_queen;

    for index in start_index..end_index {
        let moving_piece = squares[index];

        if moving_piece == 0 {
            continue;
        }

        match moving_piece {
            def::WP => {
                wp_attack_mask |= bitmask.wp_attack_masks[index];

                let file_mask = file_masks[index];
                let forward_mask = bitmask.wp_forward_masks[index];
                let rank = def::get_passer_rank(def::PLAYER_W, index) as i32;

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
                    let file = def::get_file(index);

                    if file == 0 || file == 7 {
                        w_feature_map.passed_pawn_point += PASS_PAWN_VAL[rank as usize] / 2;
                    } else {
                        w_feature_map.passed_pawn_point += PASS_PAWN_VAL[rank as usize];
                    }

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
                let rank = def::get_passer_rank(def::PLAYER_B, index) as i32;

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
                    let file = def::get_file(index);

                    if file == 0 || file == 7 {
                        b_feature_map.passed_pawn_point += PASS_PAWN_VAL[rank as usize] / 2;
                    } else {
                        b_feature_map.passed_pawn_point += PASS_PAWN_VAL[rank as usize];
                    }

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
                mov_mask |= bitmask.diag_up_attack_masks[index][util::kindergarten_transform_rank_diag(occupy_mask & bitmask.diag_up_masks[index]) as usize];
                mov_mask |= bitmask.diag_down_attack_masks[index][util::kindergarten_transform_rank_diag(occupy_mask & bitmask.diag_down_masks[index]) as usize];

                wb_attack_mask |= mov_mask;
                mov_mask_map[index] = mov_mask;
            },
            def::BB => {
                let mut mov_mask = 0;
                mov_mask |= bitmask.diag_up_attack_masks[index][util::kindergarten_transform_rank_diag(occupy_mask & bitmask.diag_up_masks[index]) as usize];
                mov_mask |= bitmask.diag_down_attack_masks[index][util::kindergarten_transform_rank_diag(occupy_mask & bitmask.diag_down_masks[index]) as usize];

                bb_attack_mask |= mov_mask;
                mov_mask_map[index] = mov_mask;
            },

            def::WR => {
                let mut mov_mask = 0;

                mov_mask |= bitmask.horizontal_attack_masks[index][util::kindergarten_transform_rank_diag(occupy_mask & bitmask.rank_masks[index]) as usize];
                mov_mask |= bitmask.vertical_attack_masks[index][util::kindergarten_transform_file(occupy_mask & bitmask.file_masks[index], index) as usize];

                wr_attack_mask |= mov_mask;
                mov_mask_map[index] = mov_mask;

                if file_masks[index] & bitboard.w_pawn == 0 {
                    w_feature_map.rook_open_count += 1;

                    if file_masks[index] & bitboard.b_pawn == 0 {
                        w_feature_map.rook_open_count += 1;
                    }
                }
            },
            def::BR => {
                let mut mov_mask = 0;

                mov_mask |= bitmask.horizontal_attack_masks[index][util::kindergarten_transform_rank_diag(occupy_mask & bitmask.rank_masks[index]) as usize];
                mov_mask |= bitmask.vertical_attack_masks[index][util::kindergarten_transform_file(occupy_mask & bitmask.file_masks[index], index) as usize];

                br_attack_mask |= mov_mask;
                mov_mask_map[index] = mov_mask;

                if file_masks[index] & bitboard.b_pawn == 0 {
                    b_feature_map.rook_open_count += 1;

                    if file_masks[index] & bitboard.w_pawn == 0 {
                        b_feature_map.rook_open_count += 1;
                    }
                }
            },

            def::WQ => {
                wq_index = index;

                let mut mov_mask = 0;

                mov_mask |= bitmask.horizontal_attack_masks[index][util::kindergarten_transform_rank_diag(occupy_mask & bitmask.rank_masks[index]) as usize];
                mov_mask |= bitmask.vertical_attack_masks[index][util::kindergarten_transform_file(occupy_mask & bitmask.file_masks[index], index) as usize];
                mov_mask |= bitmask.diag_up_attack_masks[index][util::kindergarten_transform_rank_diag(occupy_mask & bitmask.diag_up_masks[index]) as usize];
                mov_mask |= bitmask.diag_down_attack_masks[index][util::kindergarten_transform_rank_diag(occupy_mask & bitmask.diag_down_masks[index]) as usize];

                wq_attack_mask |= mov_mask;
                mov_mask_map[index] = mov_mask;
            },
            def::BQ => {
                bq_index = index;

                let mut mov_mask = 0;

                mov_mask |= bitmask.horizontal_attack_masks[index][util::kindergarten_transform_rank_diag(occupy_mask & bitmask.rank_masks[index]) as usize];
                mov_mask |= bitmask.vertical_attack_masks[index][util::kindergarten_transform_file(occupy_mask & bitmask.file_masks[index], index) as usize];
                mov_mask |= bitmask.diag_up_attack_masks[index][util::kindergarten_transform_rank_diag(occupy_mask & bitmask.diag_up_masks[index]) as usize];
                mov_mask |= bitmask.diag_down_attack_masks[index][util::kindergarten_transform_rank_diag(occupy_mask & bitmask.diag_down_masks[index]) as usize];

                bq_attack_mask |= mov_mask;
                mov_mask_map[index] = mov_mask;
            },
            _ => {}
        }
    }

    w_feature_map.weak_sqr_count = (W_CRITICAL_RANK_MASK & !wp_attack_mask).count_ones() as i32;
    b_feature_map.weak_sqr_count = (B_CRITICAL_RANK_MASK & !bp_attack_mask).count_ones() as i32;

    let wk_ring_mask = bitmask.k_attack_masks[state.wk_index];
    let bk_ring_mask = bitmask.k_attack_masks[state.bk_index];

    let w_attack_without_king_mask = wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask | wq_attack_mask;
    let w_attack_mask = w_attack_without_king_mask | wk_ring_mask;

    let b_attack_without_king_mask = bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask | bq_attack_mask;
    let b_attack_mask = b_attack_without_king_mask | bk_ring_mask;

    w_feature_map.eg_mobility += K_MOB_SCORE[(wk_ring_mask &!bitboard.w_all & !b_attack_mask).count_ones() as usize];
    b_feature_map.eg_mobility += K_MOB_SCORE[(bk_ring_mask &!bitboard.b_all & !w_attack_mask).count_ones() as usize];

    let wk_on_k_side = bitmask.index_masks[state.wk_index] & WK_K_SIDE_MASK != 0;
    let wk_on_q_side = bitmask.index_masks[state.wk_index] & WK_Q_SIDE_MASK != 0;
    let wk_in_center = bitmask.index_masks[state.wk_index] & WK_CENTER_MASK != 0;

    let bk_on_k_side = bitmask.index_masks[state.bk_index] & BK_K_SIDE_MASK != 0;
    let bk_on_q_side = bitmask.index_masks[state.bk_index] & BK_Q_SIDE_MASK != 0;
    let bk_in_center = bitmask.index_masks[state.bk_index] & BK_CENTER_MASK != 0;

    if wk_on_k_side {
        w_feature_map.weak_sqr_count += (W_CRITICAL_RANK_MASK & WK_K_SIDE_MASK & !w_attack_without_king_mask).count_ones() as i32;

        let protecting_pawn_count = (bitboard.w_pawn & WK_K_SIDE_MASK).count_ones();

        if protecting_pawn_count < 3 {
            w_feature_map.king_exposure_count += 1;

            if protecting_pawn_count < 2 {
                w_feature_map.king_exposure_count += 1;

                if protecting_pawn_count == 0 {
                    w_feature_map.king_exposure_count += 1;
                }
            }
        }

        if bitboard.w_pawn & bitmask.file_masks[6] == 0 {
            w_feature_map.king_exposure_count += 1;
        }

        if bitboard.w_pawn & bitmask.file_masks[7] == 0 {
            w_feature_map.king_exposure_count += 1;
        }
    } else if wk_on_q_side {
        w_feature_map.weak_sqr_count += (W_CRITICAL_RANK_MASK & WK_Q_SIDE_MASK & !w_attack_without_king_mask).count_ones() as i32;

        let protecting_pawn_count = (bitboard.w_pawn & WK_Q_SIDE_MASK).count_ones();
        if protecting_pawn_count < 3 {
            w_feature_map.king_exposure_count += 1;

            if protecting_pawn_count < 2 {
                w_feature_map.king_exposure_count += 1;

                if protecting_pawn_count == 0 {
                    w_feature_map.king_exposure_count += 1;
                }
            }
        }

        if bitboard.w_pawn & bitmask.file_masks[0] == 0 {
            w_feature_map.king_exposure_count += 1;
        }

        if bitboard.w_pawn & bitmask.file_masks[1] == 0 {
            w_feature_map.king_exposure_count += 1;
        }
    } else if wk_in_center {
        w_feature_map.weak_sqr_count += (W_CRITICAL_RANK_MASK & WK_CENTER_MASK & !w_attack_without_king_mask).count_ones() as i32;

        let protecting_pawn_count = (bitboard.w_pawn & WK_CENTER_MASK).count_ones();
        if protecting_pawn_count < 2 {
            w_feature_map.king_exposure_count += 1;

            if protecting_pawn_count == 0 {
                w_feature_map.king_exposure_count += 1;
            }
        }

        if bitboard.w_pawn & bitmask.file_masks[3] == 0 {
            w_feature_map.king_exposure_count += 1;
        }

        if bitboard.w_pawn & bitmask.file_masks[4] == 0 {
            w_feature_map.king_exposure_count += 1;
        }
    }

    if bk_on_k_side {
        b_feature_map.weak_sqr_count += (B_CRITICAL_RANK_MASK & BK_K_SIDE_MASK & !b_attack_without_king_mask).count_ones() as i32;

        let protecting_pawn_count = (bitboard.b_pawn & BK_K_SIDE_MASK).count_ones();
    
        if protecting_pawn_count < 3 {
            b_feature_map.king_exposure_count += 1;

            if protecting_pawn_count < 2 {
                b_feature_map.king_exposure_count += 1;

                if protecting_pawn_count == 0 {
                    b_feature_map.king_exposure_count += 1;
                }
            }
        }

        if bitboard.b_pawn & bitmask.file_masks[6] == 0 {
            b_feature_map.king_exposure_count += 1;
        }

        if bitboard.b_pawn & bitmask.file_masks[7] == 0 {
            b_feature_map.king_exposure_count += 1;
        }
    } else if bk_on_q_side {
        b_feature_map.weak_sqr_count += (B_CRITICAL_RANK_MASK & BK_Q_SIDE_MASK & !b_attack_without_king_mask).count_ones() as i32;

        let protecting_pawn_count = (bitboard.b_pawn & BK_Q_SIDE_MASK).count_ones();

        if protecting_pawn_count < 3 {
            b_feature_map.king_exposure_count += 1;

            if protecting_pawn_count < 2 {
                b_feature_map.king_exposure_count += 1;

                if protecting_pawn_count == 0 {
                    b_feature_map.king_exposure_count += 1;
                }
            }
        }

        if bitboard.b_pawn & bitmask.file_masks[0] == 0 {
            b_feature_map.king_exposure_count += 1;
        }

        if bitboard.b_pawn & bitmask.file_masks[1] == 0 {
            b_feature_map.king_exposure_count += 1;
        }
    } else if bk_in_center {
        b_feature_map.weak_sqr_count += (B_CRITICAL_RANK_MASK & BK_CENTER_MASK & !b_attack_without_king_mask).count_ones() as i32;

        let protecting_pawn_count = (bitboard.b_pawn & BK_CENTER_MASK).count_ones();

        if protecting_pawn_count < 2 {
            b_feature_map.king_exposure_count += 1;

            if protecting_pawn_count == 0 {
                b_feature_map.king_exposure_count += 1;
            }
        }

        if bitboard.b_pawn & bitmask.file_masks[3] == 0 {
            b_feature_map.king_exposure_count += 1;
        }

        if bitboard.b_pawn & bitmask.file_masks[4] == 0 {
            b_feature_map.king_exposure_count += 1;
        }
    }

    let w_in_check = mov_table::is_in_check(state, def::PLAYER_W);
    let b_in_check = mov_table::is_in_check(state, def::PLAYER_B);

    let w_queen_under_attack = if bitboard.w_queen == 0 {
        false
    } else {
        mov_table::is_under_attack(state, wq_index, def::PLAYER_W)
    };

    let b_queen_under_attack  = if bitboard.b_queen == 0 {
        false
    } else {
        mov_table::is_under_attack(state, bq_index, def::PLAYER_B)
    };


    for index in start_index..end_index {
        let piece = squares[index];

        if piece == 0 {
            continue;
        }

        let threat_val = val_of(piece);
        let index_mask = bitmask.index_masks[index];
        let mov_mask = mov_mask_map[index];

        if !(def::is_p(piece) || def::is_k(piece)) {
            if def::on_same_side(def::PLAYER_W, piece) {
                if bk_on_k_side {
                    w_feature_map.mg_sqr_point += SQR_TABLE_B_K_SIDE_EXTRA[index];
                } else if bk_on_q_side {
                    w_feature_map.mg_sqr_point += SQR_TABLE_B_Q_SIDE_EXTRA[index];
                } else if bk_in_center {
                    w_feature_map.mg_sqr_point += SQR_TABLE_B_CENTER_EXTRA[index];
                }
            } else {
                if wk_on_k_side {
                    b_feature_map.mg_sqr_point += SQR_TABLE_W_K_SIDE_EXTRA[index];
                } else if wk_on_q_side {
                    b_feature_map.mg_sqr_point += SQR_TABLE_W_Q_SIDE_EXTRA[index];
                } else if wk_in_center {
                    b_feature_map.mg_sqr_point += SQR_TABLE_W_CENTER_EXTRA[index];
                }
            }
        }

        if !(def::is_p(piece) || def::is_q(piece) || def::is_k(piece)) {
            if def::on_same_side(def::PLAYER_W, piece) {
                state.bitboard.w_all ^= index_mask;
                let king_under_attack = if w_in_check {
                    false
                } else {
                    mov_table::is_in_check(state, def::PLAYER_W)
                };

                let queen_under_attack = if w_queen_under_attack {
                    false
                } else {
                    if bitboard.w_queen == 0 {
                        false
                    } else {
                        mov_table::is_under_attack(state, wq_index, def::PLAYER_W)
                    }
                };

                state.bitboard.w_all ^= index_mask;

                if king_under_attack {
                    w_feature_map.pin_count += 1;

                    if w_attack_without_king_mask & index_mask == 0 {
                        w_feature_map.pin_count += 1;
                    }
                } else if queen_under_attack {
                    w_feature_map.semi_pin_count += 1;
                }
            } else {
                state.bitboard.b_all ^= index_mask;
                let king_under_attack = if b_in_check {
                    false
                } else {
                    mov_table::is_in_check(state, def::PLAYER_B)
                };

                let queen_under_attack = if b_queen_under_attack {
                    false
                } else {
                    if bitboard.b_queen == 0 {
                        false
                    } else {
                        mov_table::is_under_attack(state, bq_index, def::PLAYER_B)
                    }
                };

                state.bitboard.b_all ^= index_mask;

                if king_under_attack {
                    b_feature_map.pin_count += 1;

                    if b_attack_without_king_mask & index_mask == 0 {
                        b_feature_map.pin_count += 1;
                    }
                } else if queen_under_attack {
                    b_feature_map.semi_pin_count += 1;
                }
            }
        }

        match piece {
            def::WP => {
                w_feature_map.mg_sqr_point += SQR_TABLE_WP[index];
                w_feature_map.eg_sqr_point += SQR_TABLE_WP_ENDGAME[index];

                if index_mask & w_attack_mask == 0 {
                    if index_mask & b_attack_mask != 0 {
                        w_feature_map.threat_point -= threat_val;
                    }
                }

                w_feature_map.weak_king_attack_count += (bk_ring_mask & mov_mask).count_ones() as i32;
            },
            def::WN => {
                w_feature_map.mg_sqr_point += SQR_TABLE_WN[index];

                if index_mask & w_attack_mask == 0 {
                    if index_mask & b_attack_mask != 0 {
                        w_feature_map.threat_point -= threat_val;
                    }
                } else if index_mask & bp_attack_mask != 0 {
                    w_feature_map.threat_point -= threat_val - val_of(def::BP);
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

                if index_mask & w_attack_mask == 0 {
                    if index_mask & b_attack_mask != 0 {
                        w_feature_map.threat_point -= threat_val;
                    }
                } else if index_mask & bp_attack_mask != 0 {
                    w_feature_map.threat_point -= threat_val - val_of(def::BP);
                }

                let mobility_mask = mov_mask & !bp_attack_mask & !bitboard.w_all & !(b_attack_mask & !w_attack_mask);
                w_feature_map.mobility += B_MOB_SCORE[mobility_mask.count_ones() as usize];

                w_feature_map.weak_king_attack_count += (bk_ring_mask & mov_mask).count_ones() as i32;
                w_feature_map.weak_king_attack_count += (bk_ring_mask & mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask)).count_ones() as i32;
            },
            def::WR => {
                w_feature_map.mg_sqr_point += SQR_TABLE_WR[index];

                if index_mask & w_attack_mask == 0 {
                    if index_mask & b_attack_mask != 0 {
                        w_feature_map.threat_point -= threat_val;
                    }
                } else if index_mask & bp_attack_mask != 0 {
                    w_feature_map.threat_point -= threat_val - val_of(def::BP);
                } else if index_mask & (bn_attack_mask | bb_attack_mask) != 0 {
                    w_feature_map.threat_point -= threat_val - val_of(def::BN);
                }

                let mobility_mask = mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask) & !bitboard.w_all & !(b_attack_mask & !w_attack_mask);
                w_feature_map.mobility += R_MOB_SCORE[mobility_mask.count_ones() as usize];

                w_feature_map.weak_king_attack_count += (bk_ring_mask & mov_mask).count_ones() as i32;
                w_feature_map.strong_king_attack_count += (bk_ring_mask & mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask)).count_ones() as i32;
            },
            def::WQ => {
                w_feature_map.mg_sqr_point += SQR_TABLE_WQ[index];

                if index_mask & w_attack_mask == 0 {
                    if index_mask & b_attack_mask != 0 {
                        w_feature_map.threat_point -= threat_val;
                    }
                } else if index_mask & bp_attack_mask != 0 {
                    w_feature_map.threat_point -= threat_val - val_of(def::BP);
                } else if index_mask & (bn_attack_mask | bb_attack_mask) != 0 {
                    w_feature_map.threat_point -= threat_val - val_of(def::BN);
                } else if index_mask & br_attack_mask != 0 {
                    w_feature_map.threat_point -= threat_val - val_of(def::BR);
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

                if index_mask & b_attack_mask == 0 {
                    if index_mask & w_attack_mask != 0 {
                        b_feature_map.threat_point -= threat_val;
                    }
                }

                b_feature_map.weak_king_attack_count += (wk_ring_mask & mov_mask).count_ones() as i32;
            },
            def::BN => {
                b_feature_map.mg_sqr_point += SQR_TABLE_BN[index];

                if index_mask & b_attack_mask == 0 {
                    if index_mask & w_attack_mask != 0 {
                        b_feature_map.threat_point -= threat_val;
                    }
                } else if index_mask & wp_attack_mask != 0 {
                    b_feature_map.threat_point -= threat_val - val_of(def::WP);
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

                if index_mask & b_attack_mask == 0 {
                    if index_mask & w_attack_mask != 0 {
                        b_feature_map.threat_point -= threat_val;
                    }
                } else if index_mask & wp_attack_mask != 0 {
                    b_feature_map.threat_point -= threat_val - val_of(def::WP);
                }

                let mobility_mask = mov_mask & !wp_attack_mask & !bitboard.b_all & !(w_attack_mask & !b_attack_mask);
                b_feature_map.mobility += B_MOB_SCORE[mobility_mask.count_ones() as usize];

                b_feature_map.weak_king_attack_count += (wk_ring_mask & mov_mask).count_ones() as i32;
                b_feature_map.weak_king_attack_count += (wk_ring_mask & mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask)).count_ones() as i32;
            },
            def::BR => {
                b_feature_map.mg_sqr_point += SQR_TABLE_BR[index];

                if index_mask & b_attack_mask == 0 {
                    if index_mask & w_attack_mask != 0 {
                        b_feature_map.threat_point -= threat_val;
                    }
                } else if index_mask & wp_attack_mask != 0 {
                    b_feature_map.threat_point -= threat_val - val_of(def::WP);
                } else if index_mask & (wn_attack_mask | wb_attack_mask) != 0 {
                    b_feature_map.threat_point -= threat_val - val_of(def::WN);
                }

                let mobility_mask = mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask) & !bitboard.b_all & !(w_attack_mask & !b_attack_mask);
                b_feature_map.mobility += R_MOB_SCORE[mobility_mask.count_ones() as usize];

                b_feature_map.weak_king_attack_count += (wk_ring_mask & mov_mask).count_ones() as i32;
                b_feature_map.strong_king_attack_count += (wk_ring_mask & mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask)).count_ones() as i32;
            },
            def::BQ => {
                b_feature_map.mg_sqr_point += SQR_TABLE_BQ[index];

                if index_mask & b_attack_mask == 0 {
                    if index_mask & w_attack_mask != 0 {
                        b_feature_map.threat_point -= threat_val;
                    }
                } else if index_mask & wp_attack_mask != 0 {
                    b_feature_map.threat_point -= threat_val - val_of(def::WP);
                } else if index_mask & (wn_attack_mask | wb_attack_mask) != 0 {
                    b_feature_map.threat_point -= threat_val - val_of(def::WN);
                } else if index_mask & wr_attack_mask != 0 {
                    b_feature_map.threat_point -= threat_val - val_of(def::WR);
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
        zob_keys,
        bitmask,
        state::State,
    };

    #[test]
    fn test_find_weak_sqrs() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("rnbqkbnr/ppp1pppp/8/3p4/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 1");
        let (w_features, b_features) = extract_features(&mut state);

        assert_eq!(0, w_features.weak_sqr_count);
        assert_eq!(0, b_features.weak_sqr_count);
    }

    #[test]
    fn test_find_weak_sqrs1() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("rnbqkbnr/p1p1p1p1/1p3p1p/3p4/8/4P1P1/PPPP1P1P/RNBQKBNR w KQkq - 0 1");
        let (w_features, b_features) = extract_features(&mut state);

        assert_eq!(2, w_features.weak_sqr_count);
        assert_eq!(4, b_features.weak_sqr_count);
    }

    #[test]
    fn test_find_weak_sqrs2() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("rnbq1rk1/ppppppbp/6p1/8/4P3/6P1/PPPP1P1P/RNBQ1RK1 w Qq - 0 1");
        let (w_features, b_features) = extract_features(&mut state);

        assert_eq!(3, w_features.weak_sqr_count);
        assert_eq!(1, b_features.weak_sqr_count);
    }

    #[test]
    fn test_find_weak_sqrs3() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("rnbqkb1r/ppp2ppp/8/8/8/2P5/PP1P1PPP/RNBQKR2 w Qq - 0 1");
        let (w_features, b_features) = extract_features(&mut state);

        assert_eq!(2, w_features.weak_sqr_count);
        assert_eq!(0, b_features.weak_sqr_count);
    }

    #[test]
    fn test_find_pin() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("r2qk1nr/ppnpbppp/2p5/B3R3/4P1b1/5N2/PPPP1PPP/RN1QKB2 w Qkq - 0 1");
        let (w_features, b_features) = extract_features(&mut state);

        assert_eq!(0, w_features.pin_count);
        assert_eq!(1, b_features.pin_count);

        assert_eq!(1, w_features.semi_pin_count);
        assert_eq!(1, b_features.semi_pin_count);
    }

    #[test]
    fn test_find_pin1() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("r2qk1nr/ppnpbppp/2p5/B3R3/4P1b1/8/PPPP1PPP/RN1QKBN1 w Qkq - 0 1");
        let (w_features, b_features) = extract_features(&mut state);

        assert_eq!(0, w_features.pin_count);
        assert_eq!(1, b_features.pin_count);

        assert_eq!(0, w_features.semi_pin_count);
        assert_eq!(1, b_features.semi_pin_count);
    }

    #[test]
    fn test_find_pin2() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("r2qkbnr/ppnp1ppp/2p5/B3R3/4P1b1/5N2/PPPP1PPP/RN1QKB2 w Qkq - 0 1");
        let (w_features, b_features) = extract_features(&mut state);

        assert_eq!(0, w_features.pin_count);
        assert_eq!(0, b_features.pin_count);

        assert_eq!(1, w_features.semi_pin_count);
        assert_eq!(1, b_features.semi_pin_count);
    }

    #[test]
    fn test_find_king_exposure() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("rnbqk2r/ppp2pbp/6p1/8/8/2P5/PP1P1PPP/RNBQKR2 w Qq - 0 1");
        let (w_features, b_features) = extract_features(&mut state);

        assert_eq!(2, w_features.king_exposure_count);
        assert_eq!(4, b_features.king_exposure_count);
    }
}
