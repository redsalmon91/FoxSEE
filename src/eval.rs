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

pub static DELTA_MARGIN: i32 = 90;
pub static DELTA_MAX_MARGIN: i32 = 1090;

pub const FUTILITY_MARGIN_BASE: i32 = 230;
pub const MAX_POS_VAL: i32 = 90;

static Q_VAL: i32 = 1000;
static R_VAL: i32 = 525;
static B_VAL: i32 = 350;
static N_VAL: i32 = 340;
static P_VAL: i32 = 100;

static KING_MIDGAME_SAFE_SQR_VAL: i32 = 60;
static KING_MIDGAME_SAFE_PREF_SQR_VAL: i32 = 10;
static KING_ENDGAME_SQR_VAL: i32 = 50;
static KING_ENDGAME_AVOID_SQR_PEN: i32 = -20;
static KING_EXPOSED_PEN: i32 = -50;
static KING_THREAT_BASE_PEN: i32 = -10;
static KING_LOST_CAS_RIGHTS_PEN: i32 = -30;

static PASS_PAWN_BASE_VAL: i32 = 30;
static PASS_PAWN_RANK_VAL: i32 = 20;
static QUEEN_SIDE_PAWN_VAL: i32 = 50;

static PASS_PAWN_THREAT_RANK: i32 = 5;

static DUP_PAWN_PEN: i32 = -20;
static ISOLATE_PAWN_PEN: i32 = -20;
static BEHIND_PAWN_PEN: i32 = -15;

static ROOK_SEMI_OPEN_LINE_VAL: i32 = 20;
static ROOK_OPEN_LINE_VAL: i32 = 25;

static QUEEN_OPEN_LINE_VAL: i32 = 20;
static QUEEN_PIN_PEN: i32 = -20;

static DEFENDED_PIECE_VAL: i32 = 10;

static PIECE_OVER_DEFEND_COUNT: i32 = 2;

static CENTER_PAWN_VAL: i32 = 20;
static CENTER_KNIGHT_VAL: i32 = 20;
static CENTER_ATTACK_VAL: i32 = 10;

static INVASION_VAL: i32 = 20;

static THREAT_ROOK_VAL: i32 = 30;
static THREAT_QUEEN_VAL: i32 = 20;

static MIDGAME_MOB_BASE_VAL: i32 = 2;
static ENDGMAE_MOB_BASE_VAL: i32 = 2;

static ENDGAME_ROOK_EXTRA_VAL: i32 = 50;
static ENDGAME_QUEEN_EXTRA_VAL: i32 = 50;

static TRAPPED_Q_PEN: i32 = -90;
static TRAPPED_R_PEN: i32 = -90;
static TRAPPED_B_PEN: i32 = -60;
static TRAPPED_N_PEN: i32 = -60;

static BLOCKED_Q_PEN: i32 = -30;
static BLOCKED_R_PEN: i32 = -30;
static BLOCKED_B_PEN: i32 = -30;
static BLOCKED_N_PEN: i32 = -30;

static TOTAL_PHASE: i32 = 96;
static Q_PHASE_WEIGHT: i32 = 16;
static R_PHASE_WEIGHT: i32 = 8;
static B_PHASE_WEIGHT: i32 = 4;
static N_PHASE_WEIGHT: i32 = 4;

pub static ENDGAME_PHASE: i32 = 16;

static TEMPO_VAL: i32 = 10;

static CENTER_MASK: u64 = 0b00000000_00000000_00000000_00011000_00011000_00000000_00000000_00000000;

static W_INVASION_MASK: u64 = 0b00000000_01111110_00111100_00000000_00000000_00000000_00000000_00000000;
static B_INVASION_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00111100_01111110_00000000;

static WR_THREAT_MASK: u64 = 0b11111111_01111110_00000000_00000000_00000000_00000000_00000000_00000000;
static BR_THREAT_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_01111110_11111111;

static WK_MIDGAME_PREF_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010;
static WK_MIDGAME_SAFE_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_11000011_11000111;
static BK_MIDGAME_PREF_MASK: u64 = 0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
static BK_MIDGAME_SAFE_MASK: u64 = 0b11000111_11000011_00000000_00000000_00000000_00000000_00000000_00000000;

static K_ENDGAME_PREF_MASK: u64 = 0b00000000_00000000_00111100_00111100_00111100_00111100_00000000_00000000;
static K_ENDGAME_AVOID_MASK: u64 = 0b11100111_11000011_10000001_00000000_00000000_10000001_11000011_11100111;

static BOARD_L_MASK: u64 = 0b00000111_00000111_00000111_00000111_00000111_00000111_00000111_00000111;
static BOARD_R_MASK: u64 = 0b11100000_11100000_11100000_11100000_11100000_11100000_11100000_11100000;

static BOARD_A_FILE: u64 = 0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;
static BOARD_H_FILE: u64 = 0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000;

static WK_PAWN_COVER_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_11111111_11111111_00000000;
static BK_PAWN_COVER_MASK: u64 = 0b00000000_11111111_11111111_00000000_00000000_00000000_00000000_00000000;

#[derive(PartialEq, Debug)]
pub struct FeatureMap {
    pawn_count: i32,
    queen_count: i32,
    rook_count: i32,
    bishop_count: i32,
    knight_count: i32,

    passed_pawn_count: i32,
    passed_pawn_rank_count: i32,
    queen_side_pawn_count: i32,

    dup_pawn_count: i32,
    isolate_pawn_count: i32,
    behind_pawn_count: i32,

    mobility: i32,

    trapped_knight_count: i32,
    trapped_bishop_count: i32,
    trapped_rook_count: i32,
    trapped_queen_count: i32,

    blocked_knight_count: i32,
    blocked_bishop_count: i32,
    blocked_rook_count: i32,
    blocked_queen_count: i32,

    semi_open_rook_count: i32,
    open_rook_count: i32,

    open_queen_count: i32,
    queen_pin_count: i32,

    center_pawn_count: i32,
    center_knight_count: i32,
    center_attack_count: i32,

    invasion_count: i32,

    threat_rook_count: i32,
    threat_queen_count: i32,

    defended_piece_count: i32,

    king_exposed: i32,
    king_threat_count: i32,
    king_midgame_safe_sqr_count: i32,
    king_lost_cas_rights: i32,
    king_midgame_safe_pref_sqr_count: i32,
    king_endgame_pref_sqr_count: i32,
    king_endgame_avoid_sqr_count: i32,
}

impl FeatureMap {
    pub fn empty() -> Self {
        FeatureMap {
            pawn_count: 0,
            queen_count: 0,
            rook_count: 0,
            bishop_count: 0,
            knight_count: 0,

            passed_pawn_count: 0,
            passed_pawn_rank_count: 0,
            queen_side_pawn_count: 0,

            dup_pawn_count: 0,
            isolate_pawn_count: 0,
            behind_pawn_count: 0,

            mobility: 0,

            trapped_knight_count: 0,
            trapped_bishop_count: 0,
            trapped_rook_count: 0,
            trapped_queen_count: 0,

            blocked_knight_count: 0,
            blocked_bishop_count: 0,
            blocked_rook_count: 0,
            blocked_queen_count: 0,

            semi_open_rook_count: 0,
            open_rook_count: 0,

            open_queen_count: 0,
            queen_pin_count: 0,

            center_pawn_count: 0,
            center_knight_count: 0,
            center_attack_count: 0,

            invasion_count: 0,

            threat_rook_count: 0,
            threat_queen_count: 0,

            defended_piece_count: 0,

            king_exposed: 0,
            king_threat_count: 0,
            king_midgame_safe_sqr_count: 0,
            king_lost_cas_rights: 0,
            king_midgame_safe_pref_sqr_count: 0,
            king_endgame_pref_sqr_count: 0,
            king_endgame_avoid_sqr_count: 0,
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

pub fn eval_materials(state: &State) -> i32 {
    let bitboard = state.bitboard;

    let score_sign = if state.player == def::PLAYER_W {
        1
    } else {
        -1
    };

    (bitboard.w_queen.count_ones() as i32 * Q_VAL
    + bitboard.w_rook.count_ones() as i32 * R_VAL
    + bitboard.w_bishop.count_ones() as i32 * B_VAL
    + bitboard.w_knight.count_ones() as i32 * N_VAL
    + bitboard.w_pawn.count_ones() as i32 * P_VAL
    - bitboard.b_queen.count_ones() as i32 * Q_VAL
    - bitboard.b_rook.count_ones() as i32 * R_VAL
    - bitboard.b_bishop.count_ones() as i32 * B_VAL
    - bitboard.b_knight.count_ones() as i32 * N_VAL
    - bitboard.b_pawn.count_ones() as i32 * P_VAL) * score_sign
}

pub fn get_phase(state: &State) -> i32 {
    let bitboard = state.bitboard;

    (bitboard.w_queen | bitboard.b_queen).count_ones() as i32 * Q_PHASE_WEIGHT
    + (bitboard.w_rook | bitboard.b_rook).count_ones() as i32 * R_PHASE_WEIGHT
    + (bitboard.w_bishop | bitboard.b_bishop).count_ones() as i32 * B_PHASE_WEIGHT
    + (bitboard.w_knight | bitboard.b_knight).count_ones() as i32 * N_PHASE_WEIGHT
}

pub fn eval_state(state: &State, material_score: i32) -> i32 {
    let bitboard = state.bitboard;
    if bitboard.w_pawn | bitboard.b_pawn | bitboard.w_rook | bitboard.b_rook | bitboard.w_queen | bitboard.b_queen == 0 {
        if ((bitboard.w_bishop | bitboard.w_knight).count_ones() as i32 - (bitboard.b_bishop | bitboard.b_knight).count_ones() as i32).abs() < 2 {
            return 0
        }

        if (bitboard.w_bishop | bitboard.w_knight) == 0 && bitboard.b_bishop == 0 && bitboard.b_knight.count_ones() < 3 {
            return 0
        }

        if (bitboard.b_bishop | bitboard.b_knight) == 0 && bitboard.w_bishop == 0 && bitboard.w_knight.count_ones() < 3 {
            return 0
        }
    }

    let score_sign = if state.player == def::PLAYER_W {
        1
    } else {
        -1
    };

    let (w_features_map, b_features_map) = extract_features(state);

    let shared_positional_score =
        w_features_map.trapped_knight_count * TRAPPED_N_PEN
        + w_features_map.trapped_bishop_count * TRAPPED_B_PEN
        + w_features_map.trapped_rook_count * TRAPPED_R_PEN
        + w_features_map.trapped_queen_count * TRAPPED_Q_PEN
        + w_features_map.blocked_knight_count * BLOCKED_N_PEN
        + w_features_map.blocked_bishop_count * BLOCKED_B_PEN
        + w_features_map.blocked_rook_count * BLOCKED_R_PEN
        + w_features_map.blocked_queen_count * BLOCKED_Q_PEN
        + w_features_map.threat_rook_count * THREAT_ROOK_VAL
        + w_features_map.threat_queen_count * THREAT_QUEEN_VAL
        - b_features_map.trapped_knight_count * TRAPPED_N_PEN
        - b_features_map.trapped_bishop_count * TRAPPED_B_PEN
        - b_features_map.trapped_rook_count * TRAPPED_R_PEN
        - b_features_map.trapped_queen_count * TRAPPED_Q_PEN
        - b_features_map.blocked_knight_count * BLOCKED_N_PEN
        - b_features_map.blocked_bishop_count * BLOCKED_B_PEN
        - b_features_map.blocked_rook_count * BLOCKED_R_PEN
        - b_features_map.blocked_queen_count * BLOCKED_Q_PEN
        - b_features_map.threat_rook_count * THREAT_ROOK_VAL
        - b_features_map.threat_queen_count * THREAT_QUEEN_VAL;

    let midgame_positional_score =
        w_features_map.semi_open_rook_count * ROOK_SEMI_OPEN_LINE_VAL
        + w_features_map.open_rook_count * ROOK_OPEN_LINE_VAL
        + w_features_map.open_queen_count * QUEEN_OPEN_LINE_VAL
        + w_features_map.queen_pin_count * QUEEN_PIN_PEN
        + w_features_map.mobility * MIDGAME_MOB_BASE_VAL
        + w_features_map.king_exposed * KING_EXPOSED_PEN
        + w_features_map.king_threat_count * KING_THREAT_BASE_PEN
        + w_features_map.king_midgame_safe_sqr_count * KING_MIDGAME_SAFE_SQR_VAL
        + w_features_map.king_midgame_safe_pref_sqr_count * KING_MIDGAME_SAFE_PREF_SQR_VAL
        + w_features_map.center_pawn_count * CENTER_PAWN_VAL
        + w_features_map.center_knight_count * CENTER_KNIGHT_VAL
        + w_features_map.center_attack_count * CENTER_ATTACK_VAL
        + w_features_map.invasion_count * INVASION_VAL
        + w_features_map.defended_piece_count * DEFENDED_PIECE_VAL
        + w_features_map.behind_pawn_count * BEHIND_PAWN_PEN
        + w_features_map.king_lost_cas_rights * KING_LOST_CAS_RIGHTS_PEN
        - b_features_map.semi_open_rook_count * ROOK_SEMI_OPEN_LINE_VAL
        - b_features_map.open_rook_count * ROOK_OPEN_LINE_VAL
        - b_features_map.open_queen_count * QUEEN_OPEN_LINE_VAL
        - b_features_map.queen_pin_count * QUEEN_PIN_PEN
        - b_features_map.mobility * MIDGAME_MOB_BASE_VAL
        - b_features_map.king_exposed * KING_EXPOSED_PEN
        - b_features_map.king_threat_count * KING_THREAT_BASE_PEN
        - b_features_map.king_midgame_safe_sqr_count * KING_MIDGAME_SAFE_SQR_VAL
        - b_features_map.king_midgame_safe_pref_sqr_count * KING_MIDGAME_SAFE_PREF_SQR_VAL
        - b_features_map.center_pawn_count * CENTER_PAWN_VAL
        - b_features_map.center_knight_count * CENTER_KNIGHT_VAL
        - b_features_map.center_attack_count * CENTER_ATTACK_VAL
        - b_features_map.invasion_count * INVASION_VAL
        - b_features_map.defended_piece_count * DEFENDED_PIECE_VAL
        - b_features_map.behind_pawn_count * BEHIND_PAWN_PEN
        - b_features_map.king_lost_cas_rights * KING_LOST_CAS_RIGHTS_PEN;

    let endgame_positional_score =
        w_features_map.passed_pawn_count * PASS_PAWN_BASE_VAL
        + w_features_map.passed_pawn_rank_count * PASS_PAWN_RANK_VAL
        + w_features_map.queen_side_pawn_count * QUEEN_SIDE_PAWN_VAL
        + w_features_map.isolate_pawn_count * ISOLATE_PAWN_PEN
        + w_features_map.dup_pawn_count * DUP_PAWN_PEN
        + w_features_map.king_endgame_pref_sqr_count * KING_ENDGAME_SQR_VAL
        + w_features_map.king_endgame_avoid_sqr_count * KING_ENDGAME_AVOID_SQR_PEN
        + w_features_map.mobility * ENDGMAE_MOB_BASE_VAL
        + w_features_map.rook_count * ENDGAME_ROOK_EXTRA_VAL
        + w_features_map.queen_count * ENDGAME_QUEEN_EXTRA_VAL
        - b_features_map.passed_pawn_count * PASS_PAWN_BASE_VAL
        - b_features_map.passed_pawn_rank_count * PASS_PAWN_RANK_VAL
        - b_features_map.queen_side_pawn_count * QUEEN_SIDE_PAWN_VAL
        - b_features_map.isolate_pawn_count * ISOLATE_PAWN_PEN
        - b_features_map.dup_pawn_count * DUP_PAWN_PEN
        - b_features_map.king_endgame_pref_sqr_count * KING_ENDGAME_SQR_VAL
        - b_features_map.king_endgame_avoid_sqr_count * KING_ENDGAME_AVOID_SQR_PEN
        - b_features_map.mobility * ENDGMAE_MOB_BASE_VAL
        - b_features_map.rook_count * ENDGAME_ROOK_EXTRA_VAL
        - b_features_map.queen_count * ENDGAME_QUEEN_EXTRA_VAL;

    let phase = w_features_map.queen_count * Q_PHASE_WEIGHT
    + w_features_map.rook_count * R_PHASE_WEIGHT
    + w_features_map.bishop_count * B_PHASE_WEIGHT
    + w_features_map.knight_count * N_PHASE_WEIGHT
    + b_features_map.queen_count * Q_PHASE_WEIGHT
    + b_features_map.rook_count * R_PHASE_WEIGHT
    + b_features_map.bishop_count * B_PHASE_WEIGHT
    + b_features_map.knight_count * N_PHASE_WEIGHT;

    let extra_score = shared_positional_score + (midgame_positional_score * phase + endgame_positional_score * (TOTAL_PHASE - phase)) / TOTAL_PHASE;

    material_score + extra_score * score_sign + TEMPO_VAL
}

#[inline]
fn extract_features(state: &State) -> (FeatureMap, FeatureMap) {
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

    let mut w_passed_pawn_surrounding_mask = 0;
    let mut b_passed_pawn_surrounding_mask = 0;

    let occupy_mask = bitboard.w_all | bitboard.b_all;
    let start_index = occupy_mask.trailing_zeros() as usize;
    let end_index = def::BOARD_SIZE - occupy_mask.leading_zeros() as usize;


    for index in start_index..end_index {
        let moving_piece = squares[index];

        if moving_piece == 0 {
            continue
        }

        let index_mask = index_masks[index];

        match moving_piece {
            def::WP => {
                wp_attack_mask |= bitmask.wp_attack_masks[index];

                let file_mask = file_masks[index];
                let forward_mask = bitmask.wp_forward_masks[index];
                let behind_mask = bitmask.wp_behind_masks[index];
                let rank = def::get_rank(def::PLAYER_W, index) as i32;

                if forward_mask & (bitboard.b_pawn | (bitboard.w_pawn & file_mask)) == 0 {
                    w_feature_map.passed_pawn_count += 1;

                    if rank >= PASS_PAWN_THREAT_RANK {
                        w_feature_map.passed_pawn_rank_count += rank * 2;
                    } else {
                        w_feature_map.passed_pawn_rank_count += rank;
                    }

                    w_passed_pawn_surrounding_mask |= bitmask.k_attack_masks[index];

                    if behind_mask & bitmask.k_attack_masks[index] & bitboard.w_pawn != 0 {
                        w_feature_map.passed_pawn_rank_count += rank / 2;
                    }
                }

                if behind_mask & bitboard.w_pawn == 0 {
                    if forward_mask & !file_mask & bitboard.w_pawn == 0 {
                        w_feature_map.isolate_pawn_count += 1;
                    } else {
                        w_feature_map.behind_pawn_count += 1;

                        if forward_mask & file_mask & bitboard.b_pawn == 0 {
                            w_feature_map.behind_pawn_count += 1;
                        }
                    }
                }

                if (file_mask & bitboard.w_pawn).count_ones() > 1 {
                    w_feature_map.dup_pawn_count += 1;
                }
            },
            def::BP => {
                bp_attack_mask |= bitmask.bp_attack_masks[index];

                let file_mask = file_masks[index];
                let forward_mask = bitmask.bp_forward_masks[index];
                let behind_mask = bitmask.bp_behind_masks[index];
                let rank = def::get_rank(def::PLAYER_B, index) as i32;

                if forward_mask & (bitboard.w_pawn | (bitboard.b_pawn & file_mask)) == 0 {
                    b_feature_map.passed_pawn_count += 1;

                    if rank >= PASS_PAWN_THREAT_RANK {
                        b_feature_map.passed_pawn_rank_count += rank * 2;
                    } else {
                        b_feature_map.passed_pawn_rank_count += rank;
                    }

                    b_passed_pawn_surrounding_mask |= bitmask.k_attack_masks[index];

                    if behind_mask & bitmask.k_attack_masks[index] & bitboard.b_pawn != 0 {
                        b_feature_map.passed_pawn_rank_count += rank / 2;
                    }
                }

                if behind_mask & bitboard.b_pawn == 0 {
                    if forward_mask & !file_mask & bitboard.b_pawn == 0 {
                        b_feature_map.isolate_pawn_count += 1;
                    } else {
                        b_feature_map.behind_pawn_count += 1;

                        if forward_mask & file_mask & bitboard.w_pawn == 0 {
                            b_feature_map.behind_pawn_count += 1;
                        }
                    }
                }

                if (file_mask & bitboard.b_pawn).count_ones() > 1 {
                    b_feature_map.dup_pawn_count += 1;
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

                let file_mask = file_masks[index];
                if file_mask & (bitboard.w_all ^ bitboard.w_rook) == 0 {
                    if file_mask & bitboard.b_all == 0 {
                        w_feature_map.open_rook_count += 1;
                    } else {
                        w_feature_map.semi_open_rook_count += 1;
                    }
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

                let file_mask = file_masks[index];
                if file_mask & (bitboard.b_all ^ bitboard.b_rook) == 0 {
                    if file_mask & bitboard.w_all == 0 {
                        b_feature_map.open_rook_count += 1;
                    } else {
                        b_feature_map.semi_open_rook_count += 1;
                    }
                }
            },

            def::WQ => {
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

                let pawn_mask = bitboard.w_pawn | bitboard.b_pawn;
                let opponent_r_mask = bitboard.b_rook;
                let opponent_b_mask = bitboard.b_bishop;

                if bitmask.r_attack_masks[index] & opponent_r_mask != 0 {
                    let up_mask = bitmask.up_attack_masks[index];
                    if up_mask & opponent_r_mask != 0 && up_mask & (pawn_mask | bitboard.w_rook) == 0 {
                        w_feature_map.queen_pin_count += 1;
                    }

                    let down_mask = bitmask.down_attack_masks[index];
                    if down_mask & opponent_r_mask != 0 && down_mask & (pawn_mask | bitboard.w_rook) == 0 {
                        w_feature_map.queen_pin_count += 1;
                    }

                    let left_mask = bitmask.left_attack_masks[index];
                    if left_mask & opponent_r_mask != 0 && left_mask & (pawn_mask | bitboard.w_rook) == 0 {
                        w_feature_map.queen_pin_count += 1;
                    }

                    let right_mask = bitmask.right_attack_masks[index];
                    if right_mask & opponent_r_mask != 0 && right_mask & (pawn_mask | bitboard.w_rook) == 0 {
                        w_feature_map.queen_pin_count += 1;
                    }
                }

                if bitmask.b_attack_masks[index] & opponent_b_mask != 0 {
                    let up_left_mask = bitmask.up_left_attack_masks[index];
                    if up_left_mask & opponent_b_mask != 0 && up_left_mask & (pawn_mask | bitboard.w_bishop) == 0 {
                        w_feature_map.queen_pin_count += 1;
                    }

                    let up_right_mask = bitmask.up_right_attack_masks[index];
                    if up_right_mask & opponent_b_mask != 0 && up_right_mask & (pawn_mask | bitboard.w_bishop) == 0 {
                        w_feature_map.queen_pin_count += 1;
                    }

                    let down_left_mask = bitmask.down_left_attack_masks[index];
                    if down_left_mask & opponent_b_mask != 0 && down_left_mask & (pawn_mask | bitboard.w_bishop) == 0 {
                        w_feature_map.queen_pin_count += 1;
                    }

                    let down_right_mask = bitmask.down_right_attack_masks[index];
                    if down_right_mask & opponent_b_mask != 0 && down_right_mask & (pawn_mask | bitboard.w_bishop) == 0 {
                        w_feature_map.queen_pin_count += 1;
                    }
                }
            },
            def::BQ => {
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

                let pawn_mask = bitboard.w_pawn | bitboard.b_pawn;
                let opponent_r_mask = bitboard.w_rook;
                let opponent_b_mask = bitboard.w_bishop;

                if bitmask.r_attack_masks[index] & opponent_r_mask != 0 {
                    let up_mask = bitmask.up_attack_masks[index];
                    if up_mask & opponent_r_mask != 0 && up_mask & (pawn_mask | bitboard.b_rook) == 0 {
                        b_feature_map.queen_pin_count += 1;
                    }

                    let down_mask = bitmask.down_attack_masks[index];
                    if down_mask & opponent_r_mask != 0 && down_mask & (pawn_mask | bitboard.b_rook) == 0 {
                        b_feature_map.queen_pin_count += 1;
                    }

                    let left_mask = bitmask.left_attack_masks[index];
                    if left_mask & opponent_r_mask != 0 && left_mask & (pawn_mask | bitboard.b_rook) == 0 {
                        b_feature_map.queen_pin_count += 1;
                    }

                    let right_mask = bitmask.right_attack_masks[index];
                    if right_mask & opponent_r_mask != 0 && right_mask & (pawn_mask | bitboard.b_rook) == 0 {
                        b_feature_map.queen_pin_count += 1;
                    }
                }

                if bitmask.b_attack_masks[index] & opponent_b_mask != 0 {
                    let up_left_mask = bitmask.up_left_attack_masks[index];
                    if up_left_mask & opponent_b_mask != 0 && up_left_mask & (pawn_mask | bitboard.b_bishop) == 0 {
                        b_feature_map.queen_pin_count += 1;
                    }

                    let up_right_mask = bitmask.up_right_attack_masks[index];
                    if up_right_mask & opponent_b_mask != 0 && up_right_mask & (pawn_mask | bitboard.b_bishop) == 0 {
                        b_feature_map.queen_pin_count += 1;
                    }

                    let down_left_mask = bitmask.down_left_attack_masks[index];
                    if down_left_mask & opponent_b_mask != 0 && down_left_mask & (pawn_mask | bitboard.b_bishop) == 0 {
                        b_feature_map.queen_pin_count += 1;
                    }

                    let down_right_mask = bitmask.down_right_attack_masks[index];
                    if down_right_mask & opponent_b_mask != 0 && down_right_mask & (pawn_mask | bitboard.b_bishop) == 0 {
                        b_feature_map.queen_pin_count += 1;
                    }
                }
            },

            def::WK => {
                if index_mask & WK_MIDGAME_SAFE_MASK != 0 {
                    w_feature_map.king_midgame_safe_sqr_count = 1;

                    if index_mask & WK_MIDGAME_PREF_MASK != 0 {
                        w_feature_map.king_midgame_safe_pref_sqr_count = 1;
                    }
                }

                if index_mask & K_ENDGAME_PREF_MASK != 0 || index_mask & w_passed_pawn_surrounding_mask != 0 {
                    w_feature_map.king_endgame_pref_sqr_count = 1;
                } else if index_mask & K_ENDGAME_AVOID_MASK != 0 {
                    w_feature_map.king_endgame_avoid_sqr_count = 1;
                }

                let file_mask = file_masks[index];

                if file_mask & bitboard.w_pawn & WK_PAWN_COVER_MASK == 0 {
                    w_feature_map.king_exposed += 1;

                    if file_mask & bitboard.w_pawn == 0 {
                        w_feature_map.king_exposed += 1;
                    }
                }

                if file_mask & BOARD_A_FILE == 0 {
                    let lower_file_mask = file_masks[index - 1];
                    if lower_file_mask & bitboard.w_pawn == 0 && lower_file_mask & bitboard.w_rook == 0 {
                        w_feature_map.king_exposed += 1;
                    }
                }

                if file_mask & BOARD_H_FILE == 0 {
                    let higher_file_mask = file_masks[index + 1];
                    if higher_file_mask & bitboard.w_pawn == 0 && higher_file_mask & bitboard.w_rook == 0 {
                        w_feature_map.king_exposed += 1;
                    }
                }

            },
            def::BK => {
                if index_mask & BK_MIDGAME_SAFE_MASK != 0 {
                    b_feature_map.king_midgame_safe_sqr_count = 1;

                    if index_mask & BK_MIDGAME_PREF_MASK != 0 {
                        b_feature_map.king_midgame_safe_pref_sqr_count = 1;
                    }
                }

                if index_mask & K_ENDGAME_PREF_MASK != 0 || index_mask & b_passed_pawn_surrounding_mask != 0 {
                    b_feature_map.king_endgame_pref_sqr_count = 1;
                } else if index_mask & K_ENDGAME_AVOID_MASK != 0 {
                    b_feature_map.king_endgame_avoid_sqr_count = 1;
                }

                let file_mask = file_masks[index];

                if file_mask & bitboard.b_pawn & BK_PAWN_COVER_MASK == 0 {
                    b_feature_map.king_exposed += 1;

                    if file_mask & bitboard.b_pawn == 0 {
                        b_feature_map.king_exposed += 1;
                    }
                }

                if file_mask & BOARD_A_FILE == 0 {
                    let lower_file_mask = file_masks[index - 1];
                    if lower_file_mask & bitboard.b_pawn == 0 && lower_file_mask & bitboard.b_rook == 0 {
                        b_feature_map.king_exposed += 1;
                    }
                }

                if file_mask & BOARD_H_FILE == 0 {
                    let higher_file_mask = file_masks[index + 1];
                    if higher_file_mask & bitboard.b_pawn == 0 && higher_file_mask & bitboard.b_rook == 0 {
                        b_feature_map.king_exposed += 1;
                    }
                }
            },
            _ => {}
        }
    }

    // piece counts

    w_feature_map.pawn_count = bitboard.w_pawn.count_ones() as i32;
    w_feature_map.knight_count = bitboard.w_knight.count_ones() as i32;
    w_feature_map.bishop_count = bitboard.w_bishop.count_ones() as i32;
    w_feature_map.rook_count = bitboard.w_rook.count_ones() as i32;
    w_feature_map.queen_count = bitboard.w_queen.count_ones() as i32;

    b_feature_map.pawn_count = bitboard.b_pawn.count_ones() as i32;
    b_feature_map.knight_count = bitboard.b_knight.count_ones() as i32;
    b_feature_map.bishop_count = bitboard.b_bishop.count_ones() as i32;
    b_feature_map.rook_count = bitboard.b_rook.count_ones() as i32;
    b_feature_map.queen_count = bitboard.b_queen.count_ones() as i32;

    // queen-side pawns

    let wk_index_mask = bitmask.index_masks[state.wk_index];
    let bk_index_mask = bitmask.index_masks[state.bk_index];

    if wk_index_mask & BOARD_L_MASK != 0 {
        b_feature_map.queen_side_pawn_count = (bitboard.b_pawn & BOARD_R_MASK).count_ones() as i32;
    } else if wk_index_mask & BOARD_R_MASK != 0 {
        b_feature_map.queen_side_pawn_count = (bitboard.b_pawn & BOARD_L_MASK).count_ones() as i32;
    }

    if bk_index_mask & BOARD_L_MASK != 0 {
        w_feature_map.queen_side_pawn_count = (bitboard.w_pawn & BOARD_R_MASK).count_ones() as i32;
    } else if bk_index_mask & BOARD_R_MASK != 0 {
        w_feature_map.queen_side_pawn_count = (bitboard.w_pawn & BOARD_L_MASK).count_ones() as i32;
    }

    // check trapped pieces

    let w_attack_mask = wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask | wq_attack_mask | bitmask.k_attack_masks[state.wk_index];
    let b_attack_mask = bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask | bq_attack_mask | bitmask.k_attack_masks[state.bk_index];

    for index in start_index..end_index {
        let moving_piece = squares[index];

        if moving_piece == 0 {
            continue
        }

        match moving_piece {
            def::WN => {
                let mut mov_mask = mov_mask_map[index];

                mov_mask &= !bp_attack_mask;
                mov_mask &= !(b_attack_mask & !(w_attack_mask & !bitmask.n_attack_masks[index]));

                if mov_mask == 0 && w_attack_mask & index_masks[index] == 0 {
                    w_feature_map.trapped_knight_count += 1;
                } else if mov_mask & !bitboard.w_pawn == 0 {
                    w_feature_map.blocked_knight_count += 1;
                }
            },
            def::WB => {
                let mut mov_mask = mov_mask_map[index];

                mov_mask &= !bp_attack_mask;
                mov_mask &= !(b_attack_mask & !(w_attack_mask & !bitmask.b_attack_masks[index]));

                if mov_mask == 0 && w_attack_mask & index_masks[index] == 0 {
                    w_feature_map.trapped_bishop_count += 1;
                } else if mov_mask & !bitboard.w_pawn == 0 {
                    w_feature_map.blocked_bishop_count += 1;
                }
            },
            def::WR => {
                let mut mov_mask = mov_mask_map[index];

                mov_mask &= !(bp_attack_mask | bn_attack_mask | bb_attack_mask);
                mov_mask &= !(b_attack_mask & !(w_attack_mask & !bitmask.r_attack_masks[index]));

                if mov_mask == 0 && w_attack_mask & index_masks[index] == 0 {
                    w_feature_map.trapped_rook_count += 1;
                } else if mov_mask & !bitboard.w_pawn == 0 {
                    w_feature_map.blocked_rook_count += 1;
                }
            },
            def::WQ => {
                let mut mov_mask = mov_mask_map[index];

                mov_mask &= !(bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask);

                if mov_mask == 0 {
                    w_feature_map.trapped_queen_count += 1;
                } else if mov_mask & !bitboard.w_pawn == 0 {
                    w_feature_map.blocked_queen_count += 1;
                }
            },

            def::BN => {
                let mut mov_mask = mov_mask_map[index];

                mov_mask &= !wp_attack_mask;
                mov_mask &= !(w_attack_mask & !(b_attack_mask & !bitmask.n_attack_masks[index]));

                if mov_mask == 0 && b_attack_mask & index_masks[index] == 0 {
                    b_feature_map.trapped_knight_count += 1;
                } else if mov_mask & !bitboard.b_pawn == 0 {
                    b_feature_map.blocked_knight_count += 1;
                }
            },
            def::BB => {
                let mut mov_mask = mov_mask_map[index];

                mov_mask &= !wp_attack_mask;
                mov_mask &= !(w_attack_mask & !(b_attack_mask & !bitmask.b_attack_masks[index]));

                if mov_mask == 0 && b_attack_mask & index_masks[index] == 0 {
                    b_feature_map.trapped_bishop_count += 1;
                } else if mov_mask & !bitboard.b_pawn == 0 {
                    b_feature_map.blocked_bishop_count += 1;
                }
            },
            def::BR => {
                let mut mov_mask = mov_mask_map[index];

                mov_mask &= !(wp_attack_mask | wn_attack_mask | wb_attack_mask);
                mov_mask &= !(w_attack_mask & !(b_attack_mask & !bitmask.r_attack_masks[index]));

                if mov_mask == 0 && b_attack_mask & index_masks[index] == 0 {
                    b_feature_map.trapped_rook_count += 1;
                } else if mov_mask & !bitboard.b_pawn == 0 {
                    b_feature_map.blocked_rook_count += 1;
                }
            },
            def::BQ => {
                let mut mov_mask = mov_mask_map[index];

                mov_mask &= !(wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask);

                if mov_mask == 0 {
                    b_feature_map.trapped_queen_count += 1;
                } else if mov_mask & !bitboard.b_pawn == 0 {
                    b_feature_map.blocked_queen_count += 1;
                }
            },
            _ => {},
        }
    }

    // king threats

    let mut protector_mask = bitmask.k_attack_masks[state.wk_index];
    let mut protector_index = 0;

    while protector_mask != 0 {
        if protector_mask & 1u64 != 0 {
            let index_mask = index_masks[protector_index];

            let mut attack_count = 0;
            let mut defend_count = 0;

            if wp_attack_mask & index_mask != 0 {
                defend_count += 1;
            }

            if wn_attack_mask & !bp_attack_mask & index_mask != 0 {
                defend_count += 1;
            }

            if wb_attack_mask & !bp_attack_mask & index_mask != 0 {
                defend_count += 1;
            }

            if wr_attack_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask) & index_mask != 0 {
                defend_count += 1;
            }

            if wq_attack_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask) & index_mask != 0 {
                defend_count += 1;
            }

            if bp_attack_mask & index_mask != 0 {
                attack_count += 1;
            }

            if bn_attack_mask & !wp_attack_mask & index_mask != 0 {
                attack_count += 1;
            }

            if bb_attack_mask & !wp_attack_mask & index_mask != 0 {
                attack_count += 1;
            }

            if br_attack_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask) & index_mask != 0 {
                attack_count += 1;
            }

            if bq_attack_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask) & index_mask != 0 {
                attack_count += 1;
            }

            let attack_diff = (attack_count - defend_count).max(0);

            w_feature_map.king_threat_count += attack_diff * attack_diff;
        }

        protector_mask >>= 1;
        protector_index += 1;
    }

    let mut protector_mask = bitmask.k_attack_masks[state.bk_index];
    let mut protector_index = 0;

    while protector_mask != 0 {
        if protector_mask & 1u64 != 0 {
            let index_mask = index_masks[protector_index];

            let mut attack_count = 0;
            let mut defend_count = 0;

            if bp_attack_mask & index_mask != 0 {
                defend_count += 1;
            }

            if bn_attack_mask & !wp_attack_mask & index_mask != 0 {
                defend_count += 1;
            }

            if bb_attack_mask & !wp_attack_mask & index_mask != 0 {
                defend_count += 1;
            }

            if br_attack_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask) & index_mask != 0 {
                defend_count += 1;
            }

            if bq_attack_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask) & index_mask != 0 {
                defend_count += 1;
            }

            if wp_attack_mask & index_mask != 0 {
                attack_count += 1;
            }

            if wn_attack_mask & !bp_attack_mask & index_mask != 0 {
                attack_count += 1;
            }

            if wb_attack_mask & !bp_attack_mask & index_mask != 0 {
                attack_count += 1;
            }

            if wr_attack_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask) & index_mask != 0 {
                attack_count += 1;
            }

            if wq_attack_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask) & index_mask != 0 {
                attack_count += 1;
            }

            let attack_diff = (attack_count - defend_count).max(0);

            b_feature_map.king_threat_count += attack_diff * attack_diff;
        }

        protector_mask >>= 1;
        protector_index += 1;
    }

    // center control, invasion, and threats

    w_feature_map.center_pawn_count = (bitboard.w_pawn & CENTER_MASK).count_ones() as i32;
    b_feature_map.center_pawn_count = (bitboard.b_pawn & CENTER_MASK).count_ones() as i32;

    w_feature_map.center_knight_count = (bitboard.w_knight & CENTER_MASK).count_ones() as i32;
    b_feature_map.center_knight_count = (bitboard.b_knight & CENTER_MASK).count_ones() as i32;

    w_feature_map.center_attack_count = (wp_attack_mask & CENTER_MASK).count_ones() as i32;
    b_feature_map.center_attack_count = (bp_attack_mask & CENTER_MASK).count_ones() as i32;

    w_feature_map.invasion_count = ((bitboard.w_pawn | bitboard.w_knight | bitboard.w_bishop) & W_INVASION_MASK).count_ones() as i32;
    b_feature_map.invasion_count = ((bitboard.b_pawn | bitboard.b_knight | bitboard.b_bishop) & B_INVASION_MASK).count_ones() as i32;

    w_feature_map.threat_rook_count = (bitboard.w_rook & WR_THREAT_MASK).count_ones() as i32;
    w_feature_map.threat_queen_count = (bitboard.w_queen & WR_THREAT_MASK).count_ones() as i32;

    b_feature_map.threat_rook_count = (bitboard.b_rook & BR_THREAT_MASK).count_ones() as i32;
    b_feature_map.threat_queen_count = (bitboard.b_queen & BR_THREAT_MASK).count_ones() as i32;

    // mobility

    w_feature_map.mobility = (wn_attack_mask & !bitboard.w_pawn & !bp_attack_mask).count_ones() as i32;
    w_feature_map.mobility += (wb_attack_mask & !bitboard.w_pawn & !bp_attack_mask).count_ones() as i32;

    b_feature_map.mobility = (bn_attack_mask & !bitboard.b_pawn & !wp_attack_mask).count_ones() as i32;
    b_feature_map.mobility += (bb_attack_mask & !bitboard.b_pawn & !wp_attack_mask).count_ones() as i32;

    // penalty for losing castling rights

    if (state.cas_rights | state.cas_history) & 0b1100 == 0 {
        w_feature_map.king_lost_cas_rights = 1;
    }

    if (state.cas_rights | state.cas_history) & 0b0011 == 0 {
        b_feature_map.king_lost_cas_rights = 1;
    }

    // defense on pieces

    let wk_attack_mask = bitmask.k_attack_masks[state.wk_index];
    let bk_attack_mask = bitmask.k_attack_masks[state.bk_index];

    for index in 0..def::BOARD_SIZE {
        let index_mask = index_masks[index];

        if index_mask & bitboard.w_all == 0 {
            continue
        }

        let piece = state.squares[index];

        if piece == def::WP || piece == def::WR || piece == def::WQ || piece == def::WK {
            continue
        }

        let mut defense_count = 0;

        match piece {
            def::WN => {
                if bp_attack_mask & index_mask != 0 {
                    defense_count -= 1;
                } else {
                    if wp_attack_mask & index_mask != 0 {
                        defense_count += (bitmask.bp_attack_masks[index] & bitboard.w_pawn).count_ones() as i32;
                    }

                    if wn_attack_mask & index_mask != 0 {
                        defense_count += 1;
                    }

                    if wb_attack_mask & index_mask != 0 {
                        defense_count += 1;
                    }

                    if wr_attack_mask & index_mask != 0 {
                        defense_count += 1;
                    }

                    if wq_attack_mask & index_mask != 0 {
                        defense_count += 1;
                    }

                    if wk_attack_mask & index_mask != 0 {
                        defense_count += 1;
                    }

                    if bn_attack_mask & index_mask != 0 {
                        defense_count -= 1;
                    }

                    if bb_attack_mask & index_mask != 0 {
                        defense_count -= 1;
                    }

                    if br_attack_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask) & index_mask != 0 {
                        defense_count -= 1;
                    }

                    if bq_attack_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask) & index_mask != 0 {
                        defense_count -= 1;
                    }
                }
            },
            def::WB => {
                if bp_attack_mask & index_mask != 0 {
                    defense_count -= 1;
                } else {
                    if wp_attack_mask & index_mask != 0 {
                        defense_count += (bitmask.bp_attack_masks[index] & bitboard.w_pawn).count_ones() as i32;
                    }

                    if wn_attack_mask & index_mask != 0 {
                        defense_count += 1;
                    }

                    if wb_attack_mask & index_mask != 0 {
                        defense_count += 1;
                    }

                    if wr_attack_mask & index_mask != 0 {
                        defense_count += 1;
                    }

                    if wq_attack_mask & index_mask != 0 {
                        defense_count += 1;
                    }

                    if wk_attack_mask & index_mask != 0 {
                        defense_count += 1;
                    }

                    if bn_attack_mask & index_mask != 0 {
                        defense_count -= 1;
                    }

                    if bb_attack_mask & index_mask != 0 {
                        defense_count -= 1;
                    }

                    if br_attack_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask) & index_mask != 0 {
                        defense_count -= 1;
                    }

                    if bq_attack_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask) & index_mask != 0 {
                        defense_count -= 1;
                    }
                }
            },
            _ => {},
        }

        w_feature_map.defended_piece_count += defense_count.min(PIECE_OVER_DEFEND_COUNT);
    }

    for index in 0..def::BOARD_SIZE {
        let index_mask = index_masks[index];

        if index_mask & bitboard.b_all == 0 {
            continue
        }

        let piece = state.squares[index];

        if piece == def::BP || piece == def::BR || piece == def::BQ || piece == def::BK {
            continue
        }

        let mut defense_count = 0;

        match piece {
            def::BN => {
                if wp_attack_mask & index_mask != 0 {
                    defense_count -= 1;
                } else {
                    if bp_attack_mask & index_mask != 0 {
                        defense_count += (bitmask.wp_attack_masks[index] & bitboard.b_pawn).count_ones() as i32;
                    }

                    if bn_attack_mask & index_mask != 0 {
                        defense_count += 1;
                    }

                    if bb_attack_mask & index_mask != 0 {
                        defense_count += 1;
                    }

                    if br_attack_mask & index_mask != 0 {
                        defense_count += 1;
                    }

                    if bq_attack_mask & index_mask != 0 {
                        defense_count += 1;
                    }

                    if bk_attack_mask & index_mask != 0 {
                        defense_count += 1;
                    }

                    if wn_attack_mask & index_mask != 0 {
                        defense_count -= 1;
                    }

                    if wb_attack_mask & index_mask != 0 {
                        defense_count -= 1;
                    }

                    if wr_attack_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask) & index_mask != 0 {
                        defense_count -= 1;
                    }

                    if wq_attack_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask) & index_mask != 0 {
                        defense_count -= 1;
                    }
                }
            },
            def::BB => {
                if wp_attack_mask & index_mask != 0 {
                    defense_count -= 1;
                } else {
                    if bp_attack_mask & index_mask != 0 {
                        defense_count += (bitmask.wp_attack_masks[index] & bitboard.b_pawn).count_ones() as i32;
                    }

                    if bn_attack_mask & index_mask != 0 {
                        defense_count += 1;
                    }

                    if bb_attack_mask & index_mask != 0 {
                        defense_count += 1;
                    }

                    if br_attack_mask & index_mask != 0 {
                        defense_count += 1;
                    }

                    if bq_attack_mask & index_mask != 0 {
                        defense_count += 1;
                    }

                    if bk_attack_mask & index_mask != 0 {
                        defense_count += 1;
                    }

                    if wn_attack_mask & index_mask != 0 {
                        defense_count -= 1;
                    }

                    if wb_attack_mask & index_mask != 0 {
                        defense_count -= 1;
                    }

                    if wr_attack_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask) & index_mask != 0 {
                        defense_count -= 1;
                    }

                    if wq_attack_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask) & index_mask != 0 {
                        defense_count -= 1;
                    }
                }
            },
            _ => {},
        }

        b_feature_map.defended_piece_count += defense_count.min(PIECE_OVER_DEFEND_COUNT);
    }

    (w_feature_map, b_feature_map)
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
    fn test_extract_features_01() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("r1bq1rk1/pp2ppbp/2np2p1/2n5/P3PP2/N1P2N2/1PB3PP/R1B1QRK1 b - - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(7, w_features.defended_piece_count);
        assert_eq!(5, b_features.defended_piece_count);
    }

    #[test]
    fn test_extract_features_2() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("8/p2R2p1/1p3p1p/2p5/8/1P6/Pr1r1PPP/8 w - - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(1, w_features.threat_rook_count);
        assert_eq!(0, w_features.threat_queen_count);

        assert_eq!(2, b_features.threat_rook_count);
        assert_eq!(0, b_features.threat_queen_count);
    }

    #[test]
    fn test_extract_features_3() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("1q4kn/3r1p1p/1pbN1Pp1/r1ppP1P1/P4R2/2B1P3/2Q4P/3R2K1 b - - 2 29", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(9, w_features.mobility);
        assert_eq!(4, b_features.mobility);
    }

    #[test]
    fn test_extract_features_4() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("rnb1kbnr/ppP2Rpp/8/8/8/4pP2/PrP1P1qP/RNBQKBNR w KQkq - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(1, w_features.threat_rook_count);
        assert_eq!(0, w_features.threat_queen_count);

        assert_eq!(1, b_features.threat_rook_count);
        assert_eq!(1, b_features.threat_queen_count);
    }

    #[test]
    fn test_extract_features_5() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("1kr3r1/pp3p1p/P1pn4/2Bpb3/4p2q/3PP3/PPP1NPPP/R2Q1RK1 b - - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(0, w_features.king_exposed);
        assert_eq!(5, w_features.king_threat_count);

        assert_eq!(0, b_features.king_exposed);
        assert_eq!(2, b_features.king_threat_count);
    }

    #[test]
    fn test_extract_features_6() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("r1bqk1nr/pppp1pbp/6p1/3np3/2PP4/4PN2/PP3PPP/RNBQKB1R w KQkq - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(1, w_features.center_pawn_count);
        assert_eq!(1, b_features.center_pawn_count);
    }

    #[test]
    fn test_extract_features_7() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("8/p1p2pp1/5pp1/8/7P/5PPP/1P3P2/8 w - - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(4, w_features.dup_pawn_count);
        assert_eq!(1, w_features.isolate_pawn_count);
        assert_eq!(1, w_features.behind_pawn_count);

        assert_eq!(4, b_features.dup_pawn_count);
        assert_eq!(2, b_features.isolate_pawn_count);
        assert_eq!(0, b_features.behind_pawn_count);
    }

    #[test]
    fn test_extract_features_8() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("8/p7/1p6/2p5/4P3/4P3/4P3/8 w - - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(3, w_features.dup_pawn_count);
        assert_eq!(3, w_features.isolate_pawn_count);

        assert_eq!(0, b_features.dup_pawn_count);
        assert_eq!(0, b_features.isolate_pawn_count);
    }

    #[test]
    fn test_extract_features_9() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("rnbqkbnr/3p4/6P1/3P4/1pp5/1N6/3P4/R1BQKBNR w KQkq - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(1, w_features.passed_pawn_count);
        assert_eq!(10, w_features.passed_pawn_rank_count);

        assert_eq!(1, b_features.passed_pawn_count);
        assert_eq!(6, b_features.passed_pawn_rank_count);
    }

    #[test]
    fn test_extract_features_10() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("5k2/5ppp/1p2p3/8/2P5/1P4P1/5P1P/6K1 w - - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(2, w_features.queen_side_pawn_count);
        assert_eq!(1, b_features.queen_side_pawn_count);
    }

    #[test]
    fn test_extract_features_11() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("rnbqkbnr/ppp3p1/3p1p1p/4p3/2P1P3/P2P4/1P3PPP/RNBQKBNR w KQkq - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(2, w_features.behind_pawn_count);
        assert_eq!(1, b_features.behind_pawn_count);
    }

    #[test]
    fn test_extract_features_12() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("rnbqk1nr/p1p1pppp/3p4/Np6/2p1P3/6P1/PPPPNP1b/R1BQKB1R w KQkq - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(1, w_features.trapped_knight_count);
        assert_eq!(0, w_features.trapped_bishop_count);
        assert_eq!(0, w_features.trapped_rook_count);
        assert_eq!(0, w_features.trapped_queen_count);

        assert_eq!(0, b_features.trapped_knight_count);
        assert_eq!(1, b_features.trapped_bishop_count);
        assert_eq!(0, b_features.trapped_rook_count);
        assert_eq!(0, b_features.trapped_queen_count);

        assert_eq!(0, w_features.blocked_knight_count);
        assert_eq!(1, w_features.blocked_bishop_count);
        assert_eq!(0, w_features.blocked_rook_count);
        assert_eq!(0, w_features.blocked_queen_count);

        assert_eq!(0, b_features.blocked_knight_count);
        assert_eq!(0, b_features.blocked_bishop_count);
        assert_eq!(0, b_features.blocked_rook_count);
        assert_eq!(0, b_features.blocked_queen_count);
    }

    #[test]
    fn test_extract_features_13() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("1krq1bnr/p1ppppp1/8/8/1P3P2/8/2PPP1PP/RNBQR1K1 w Qk - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(0, w_features.king_exposed);
        assert_eq!(2, b_features.king_exposed);
    }

    #[test]
    fn test_extract_features_14() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("r2qkb1r/pp4p1/2nPN2p/2bBPp2/8/2PP2P1/PP2Pn1P/RNBQK2R w KQkq - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(2, w_features.invasion_count);
        assert_eq!(1, b_features.invasion_count);
    }

    #[test]
    fn test_extract_features_15() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let mut state = State::new("r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(0, w_features.king_lost_cas_rights);
        assert_eq!(0, b_features.king_lost_cas_rights);

        state.do_mov(util::map_sqr_notation_to_index("e1"), util::map_sqr_notation_to_index("e2"), def::MOV_REG, 0);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(1, w_features.king_lost_cas_rights);
        assert_eq!(0, b_features.king_lost_cas_rights);

        state.do_mov(util::map_sqr_notation_to_index("h8"), util::map_sqr_notation_to_index("f8"), def::MOV_REG, 0);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(1, w_features.king_lost_cas_rights);
        assert_eq!(0, b_features.king_lost_cas_rights);

        state.do_mov(util::map_sqr_notation_to_index("e2"), util::map_sqr_notation_to_index("e1"), def::MOV_REG, 0);
        state.do_mov(util::map_sqr_notation_to_index("a8"), util::map_sqr_notation_to_index("b8"), def::MOV_REG, 0);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(1, w_features.king_lost_cas_rights);
        assert_eq!(1, b_features.king_lost_cas_rights);
    }

    #[test]
    fn test_extract_features_16() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("r1bq1rk1/ppp1p2p/1b1pnpb1/8/3NPP2/1QNB2R1/PPP3PP/2B2RK1 w q - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(0, w_features.king_exposed);
        assert_eq!(2, b_features.king_exposed);
    }

    #[test]
    fn test_extract_features_17() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("rnbqr1k1/ppp2ppp/5n2/3b2B1/4N3/8/PPP1QPPP/RNBR2K1 w Qq - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(1, w_features.queen_pin_count);
        assert_eq!(2, b_features.queen_pin_count);
    }

    #[test]
    fn test_extract_features_18() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("rnbqr1k1/ppp1bppp/5n2/3b2B1/4N3/3B4/PPP1QPPP/1N1RR1K1 w q - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(1, w_features.queen_pin_count);
        assert_eq!(1, b_features.queen_pin_count);
    }

    #[test]
    fn test_extract_features_19() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("rnbqk2r/ppp2pbp/4p1p1/3pB3/2PPn3/2N1P1P1/PP3PBP/R2QK1NR w KQkq - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state);

        assert_eq!(1, w_features.center_pawn_count);
        assert_eq!(0, w_features.center_knight_count);
        assert_eq!(3, w_features.center_attack_count);

        assert_eq!(1, b_features.center_pawn_count);
        assert_eq!(1, b_features.center_knight_count);
        assert_eq!(2, b_features.center_attack_count);
    }

    #[test]
    fn test_draw_endgame() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("8/2k5/8/8/8/4N3/5K2/8 w - - 0 1", &zob_keys, &bitmask);
        assert_eq!(0, eval_state(&state, eval_materials(&state)));
    }
}
