/*
 * Copyright (C) 2020-2022 Zixiao Han
 */

use crate::{
    bitmask,
    def,
    eval_params::EvalParams,
    mov_table,
    state::State,
    util,
};

pub const MATE_VAL: i32 = 20000;
pub const TERM_VAL: i32 = 10000;

const TOTAL_MAIN_PHASE: i32 = 96;
const Q_PHASE_WEIGHT: i32 = 16;
const R_PHASE_WEIGHT: i32 = 8;
const B_PHASE_WEIGHT: i32 = 4;
const N_PHASE_WEIGHT: i32 = 4;
const TOTAL_PAWN_PHASE: i32 = 16;

const WK_K_SIDE_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_11100000_11100000_11100000;
const WK_Q_SIDE_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000111_00000111_00000111;
const BK_K_SIDE_MASK: u64 = 0b11100000_11100000_11100000_00000000_00000000_00000000_00000000_00000000;
const BK_Q_SIDE_MASK: u64 = 0b00000111_00000111_00000111_00000000_00000000_00000000_00000000_00000000;

const W_BASE_MASK: u64 = 0b00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111;
const B_BASE_MASK: u64 = 0b11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000;

const W_3RD_RANK_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_11111111_00000000_00000000;
const W_4TH_RANK_MASK: u64 = 0b00000000_00000000_00000000_00000000_11111111_00000000_00000000_00000000;

const B_3RD_RANK_MASK: u64 = 0b00000000_00000000_11111111_00000000_00000000_00000000_00000000_00000000;
const B_4TH_RANK_MASK: u64 = 0b00000000_00000000_00000000_11111111_00000000_00000000_00000000_00000000;

const W_CAS_MASK: u8 = 0b1100;
const B_CAS_MASK: u8 = 0b0011;

const BISHOP_KNIGHT_MATE_MIN: i32 = 2;
const KNIGHT_MATE_MIN: i32 = 2;

const SQR_TIER_N: [i32; def::BOARD_SIZE] = [
    0, 1, 1, 1, 1, 1, 1, 0,
    1, 2, 2, 2, 2, 2, 2, 1,
    1, 2, 3, 3, 3, 3, 2, 1,
    1, 2, 3, 4, 4, 3, 2, 1,
    1, 2, 3, 4, 4, 3, 2, 1,
    1, 2, 3, 3, 3, 3, 2, 1,
    1, 2, 2, 2, 2, 2, 2, 1,
    0, 1, 1, 1, 1, 1, 1, 0,
];

const SQR_TIER_B: [i32; def::BOARD_SIZE] = [
    0, 1, 1, 1, 1, 1, 1, 0,
    1, 2, 2, 2, 2, 2, 2, 1,
    1, 2, 2, 2, 2, 2, 2, 1,
    1, 2, 2, 2, 2, 2, 2, 1,
    1, 2, 2, 2, 2, 2, 2, 1,
    1, 2, 2, 2, 2, 2, 2, 1,
    1, 2, 2, 2, 2, 2, 2, 1,
    0, 1, 1, 1, 1, 1, 1, 0,
];

const SQR_TIER_WK: [i32; def::BOARD_SIZE] = [
    1, 2, 1, 0, 0, 0, 2, 1,
    1, 1, 0, 0, 0, 0, 1, 1,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
];

const SQR_TIER_BK: [i32; def::BOARD_SIZE] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 0, 0, 0, 0, 1, 1,
    1, 2, 1, 0, 0, 0, 2, 1,
];

const SQR_TIER_K_EG: [i32; def::BOARD_SIZE] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 2, 2, 2, 2, 1, 0,
    0, 1, 2, 3, 3, 2, 1, 0,
    0, 1, 2, 3, 3, 2, 1, 0,
    0, 1, 2, 2, 2, 2, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
];

const SQR_TIER_WP: [i32; def::BOARD_SIZE] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 1,
    2, 2, 2, 3, 3, 2, 2, 2,
    2, 3, 3, 4, 4, 3, 3, 2,
    2, 3, 4, 5, 5, 4, 3, 2,
    2, 3, 4, 5, 5, 4, 3, 2,
    0, 0, 0, 0, 0, 0, 0, 0,
];

const SQR_TIER_WP_EG: [i32; def::BOARD_SIZE] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 1,
    2, 2, 2, 2, 2, 2, 2, 2,
    3, 3, 3, 3, 3, 3, 3, 3,
    4, 4, 4, 4, 4, 4, 4, 4,
    5, 5, 5, 5, 5, 5, 5, 5,
    0, 0, 0, 0, 0, 0, 0, 0,
];

const SQR_TIER_BP: [i32; def::BOARD_SIZE] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    2, 3, 4, 5, 5, 4, 3, 2,
    2, 3, 4, 5, 5, 4, 3, 2,
    2, 3, 3, 4, 4, 3, 3, 2,
    2, 2, 2, 3, 3, 2, 2, 2,
    1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
];

const SQR_TIER_BP_EG: [i32; def::BOARD_SIZE] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    5, 5, 5, 5, 5, 5, 5, 5,
    4, 4, 4, 4, 4, 4, 4, 4,
    3, 3, 3, 3, 3, 3, 3, 3,
    2, 2, 2, 2, 2, 2, 2, 2,
    1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
];

#[inline]
pub fn get_phase(state: &mut State) -> i32 {
    (state.wq_count + state.bq_count) * Q_PHASE_WEIGHT
    + (state.wr_count + state.br_count) * R_PHASE_WEIGHT
    + (state.wb_count + state.bb_count) * B_PHASE_WEIGHT
    + (state.wn_count + state.bn_count) * N_PHASE_WEIGHT
}

#[inline]
pub fn get_pawn_phase(state: &mut State) -> i32 {
    state.wp_count + state.bp_count
}

#[inline]
fn get_score_sign(state: &State) -> i32 {
    if state.player == def::PLAYER_W {
        1
    } else {
        -1
    }
}

#[derive(PartialEq, Debug)]
pub struct FeatureMap {
    p_sqr_count: i32,
    p_eg_sqr_count: i32,
    n_sqr_count: i32,
    b_sqr_count: i32,
    k_sqr_count: i32,
    k_eg_sqr_count: i32,

    passer_count: i32,
    passer_rank_count: i32,
    candidate_passer_count: i32,
    candidate_passer_rank_count: i32,

    controlled_passer_count: i32,
    doubled_pawn_count: i32,
    isolated_pawn_count: i32,
    behind_pawn_count: i32,

    rook_open_count: i32,
    rook_semi_open_count: i32,

    pin_count: i32,
    semi_pin_count: i32,

    pk_attack_count: i32,
    nk_attack_count: i32,
    bk_attack_count: i32,
    rk_attack_count: i32,
    qk_attack_count: i32,

    king_pawn_protection_count: i32,
    king_on_open_file_count: i32,
    king_on_opponent_semi_open_file_count: i32,
    king_on_own_semi_open_file_count: i32,
    king_near_open_file_count: i32,
    king_near_opponent_semi_open_file_count: i32,
    king_near_own_semi_open_file_count: i32,

    king_cas_rights_count: i32,
    king_caslted_count: i32,

    unprotected_sqr_count: i32,
    under_attacked_sqr_count: i32,

    protected_p_count: i32,
    protected_n_count: i32,
    protected_b_count: i32,
    protected_r_count: i32,

    np_protected_3rd_rank_sqr_count: i32,
    np_protected_4th_rank_sqr_count: i32,

    n_mobility_count: i32,
    b_mobility_count: i32,
    r_mobility_count: i32,
    q_mobility_count: i32,
    k_mobility_count: i32,

    n_stuck_count: i32,
    b_stuck_count: i32,
    r_stuck_count: i32,
    q_stuck_count: i32,
}

impl FeatureMap {
    pub fn empty() -> Self {
        FeatureMap {
            p_sqr_count: 0,
            p_eg_sqr_count: 0,
            n_sqr_count: 0,
            b_sqr_count: 0,
            k_sqr_count: 0,
            k_eg_sqr_count: 0,

            passer_count: 0,
            passer_rank_count: 0,
            candidate_passer_count: 0,
            candidate_passer_rank_count: 0,

            controlled_passer_count: 0,
            doubled_pawn_count: 0,
            isolated_pawn_count: 0,
            behind_pawn_count: 0,

            rook_open_count: 0,
            rook_semi_open_count: 0,

            pin_count: 0,
            semi_pin_count: 0,

            pk_attack_count: 0,
            nk_attack_count: 0,
            bk_attack_count: 0,
            rk_attack_count: 0,
            qk_attack_count: 0,

            king_pawn_protection_count: 0,
            king_on_open_file_count: 0,
            king_on_opponent_semi_open_file_count: 0,
            king_on_own_semi_open_file_count: 0,
            king_near_open_file_count: 0,
            king_near_opponent_semi_open_file_count: 0,
            king_near_own_semi_open_file_count: 0,

            king_cas_rights_count: 0,
            king_caslted_count: 0,

            unprotected_sqr_count: 0,
            under_attacked_sqr_count: 0,

            protected_p_count: 0,
            protected_n_count: 0,
            protected_b_count: 0,
            protected_r_count: 0,

            np_protected_3rd_rank_sqr_count: 0,
            np_protected_4th_rank_sqr_count: 0,

            n_mobility_count: 0,
            b_mobility_count: 0,
            r_mobility_count: 0,
            q_mobility_count: 0,
            k_mobility_count: 0,

            n_stuck_count: 0,
            b_stuck_count: 0,
            r_stuck_count: 0,
            q_stuck_count: 0,
        }
    }
}

pub struct Evaluator {
    params: EvalParams,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            params: EvalParams::default(),
        }
    }

    #[allow(dead_code)]
    pub fn set_params(&mut self, params_file: &str) {
        self.params = EvalParams::from_config(&util::load_params(params_file));
    }

    pub fn val_of(&self, piece: u8) -> i32 {
        match piece {
            0 => 0,
            def::WK => MATE_VAL,
            def::WQ => self.params.q_val,
            def::WR => self.params.r_val,
            def::WB => self.params.b_val,
            def::WN => self.params.n_val,
            def::WP => self.params.p_val,

            def::BK => MATE_VAL,
            def::BQ => self.params.q_val,
            def::BR => self.params.r_val,
            def::BB => self.params.b_val,
            def::BN => self.params.n_val,
            def::BP => self.params.p_val,

            _ => 0,
        }
    }

    pub fn is_material_draw(&self, state: &State) -> bool {
        let bitboard = state.bitboard;

        if bitboard.w_pawn | bitboard.b_pawn | bitboard.w_rook | bitboard.b_rook | bitboard.w_queen | bitboard.b_queen == 0 {
            if state.wb_count + state.wn_count < BISHOP_KNIGHT_MATE_MIN && state.bb_count + state.bn_count < BISHOP_KNIGHT_MATE_MIN {
                return true;
            }

            if (bitboard.w_bishop | bitboard.w_knight) == 0 && bitboard.b_bishop == 0 && state.bn_count < KNIGHT_MATE_MIN {
                return true;
            }

            if (bitboard.b_bishop | bitboard.b_knight) == 0 && bitboard.w_bishop == 0 && state.wn_count < KNIGHT_MATE_MIN {
                return true;
            }
        }

        false
    }

    fn eval_materials(&self, state: &mut State) -> i32 {
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

        let material_base_score = (w_queen_count - b_queen_count) * self.params.q_val
            + (w_rook_count - b_rook_count) * self.params.r_val
            + (w_bishop_count - b_bishop_count) * self.params.b_val
            + (w_knight_count - b_knight_count) * self.params.n_val
            + (w_pawn_count - b_pawn_count) * self.params.p_val;

        let material_mp_score = (w_queen_count - b_queen_count) * self.params.mp_q_val
        + (w_rook_count - b_rook_count) * self.params.mp_r_val
        + (w_bishop_count - b_bishop_count) * self.params.mp_b_val
        + (w_knight_count - b_knight_count) * self.params.mp_n_val
        + (w_pawn_count - b_pawn_count) * self.params.mp_p_val;

        let material_pp_score = (w_queen_count - b_queen_count) * self.params.pp_q_val
        + (w_rook_count - b_rook_count) * self.params.pp_r_val
        + (w_bishop_count - b_bishop_count) * self.params.pp_b_val
        + (w_knight_count - b_knight_count) * self.params.pp_n_val
        + (w_pawn_count - b_pawn_count) * self.params.pp_p_val;

        let main_phase = get_phase(state);
        let pawn_phase = get_pawn_phase(state);

        material_base_score
            + material_mp_score * main_phase / TOTAL_MAIN_PHASE
            + material_pp_score * pawn_phase / TOTAL_PAWN_PHASE
    }

    pub fn eval_state(&self, state: &mut State) -> i32 {
        let (w_features_map, b_features_map) = self.extract_features(state);

        let pos_mp_score =
            w_features_map.p_sqr_count * self.params.mp_p_sqr_base_val
            + w_features_map.p_eg_sqr_count * self.params.mp_p_eg_sqr_base_val
            + w_features_map.n_sqr_count * self.params.mp_n_sqr_base_val
            + w_features_map.b_sqr_count * self.params.mp_b_sqr_base_val
            + w_features_map.k_sqr_count * self.params.mp_k_sqr_base_val
            + w_features_map.k_eg_sqr_count * self.params.mp_k_eg_sqr_base_val
            + w_features_map.pin_count * self.params.mp_pin_val
            + w_features_map.semi_pin_count * self.params.mp_semi_pin_val
            + w_features_map.rook_open_count * self.params.mp_rook_open_val
            + w_features_map.rook_semi_open_count * self.params.mp_rook_semi_open_val
            + w_features_map.king_pawn_protection_count * self.params.mp_king_pawn_protection_val
            + w_features_map.king_on_open_file_count * self.params.mp_king_on_open_file_val
            + w_features_map.king_on_opponent_semi_open_file_count * self.params.mp_king_on_opponent_semi_open_file_val
            + w_features_map.king_on_own_semi_open_file_count * self.params.mp_king_on_own_semi_open_file_val
            + w_features_map.king_near_open_file_count * self.params.mp_king_near_open_file_val
            + w_features_map.king_near_opponent_semi_open_file_count * self.params.mp_king_near_opponent_semi_open_file_val
            + w_features_map.king_near_own_semi_open_file_count * self.params.mp_king_near_own_semi_open_file_val
            + w_features_map.king_cas_rights_count * self.params.mp_king_cas_rights_val
            + w_features_map.king_caslted_count * self.params.mp_king_castled_val
            + w_features_map.pk_attack_count * self.params.mp_pk_attack_val
            + w_features_map.nk_attack_count * self.params.mp_nk_attack_val
            + w_features_map.bk_attack_count * self.params.mp_bk_attack_val
            + w_features_map.rk_attack_count * self.params.mp_rk_attack_val
            + w_features_map.qk_attack_count * self.params.mp_qk_attack_val
            + w_features_map.protected_p_count * self.params.mp_protected_p_val
            + w_features_map.protected_n_count * self.params.mp_protected_n_val
            + w_features_map.protected_b_count * self.params.mp_protected_b_val
            + w_features_map.protected_r_count * self.params.mp_protected_r_val
            + w_features_map.np_protected_3rd_rank_sqr_count * self.params.mp_np_protected_3rd_rank_sqr_val
            + w_features_map.np_protected_4th_rank_sqr_count * self.params.mp_np_protected_4th_rank_sqr_val
            + w_features_map.behind_pawn_count * self.params.mp_behind_pawn_val
            + w_features_map.isolated_pawn_count * self.params.mp_isolated_pawn_val
            + w_features_map.doubled_pawn_count * self.params.mp_doubled_pawn_val
            + w_features_map.passer_count * self.params.mp_passer_base_val
            + w_features_map.passer_rank_count * self.params.mp_passer_rank_val
            + w_features_map.candidate_passer_count * self.params.mp_candidate_passer_base_val
            + w_features_map.candidate_passer_rank_count * self.params.mp_candidate_passer_rank_val
            + w_features_map.unprotected_sqr_count * self.params.mp_unprotected_sqr_val
            + w_features_map.under_attacked_sqr_count * self.params.mp_under_attacked_sqr_val
            + w_features_map.n_mobility_count * self.params.mp_n_mob_base_val
            + w_features_map.b_mobility_count * self.params.mp_b_mob_base_val
            + w_features_map.r_mobility_count * self.params.mp_r_mob_base_val
            + w_features_map.q_mobility_count * self.params.mp_q_mob_base_val
            + w_features_map.k_mobility_count * self.params.mp_k_mob_base_val
            + w_features_map.n_stuck_count * self.params.mp_n_stuck_val
            + w_features_map.b_stuck_count * self.params.mp_b_stuck_val
            + w_features_map.r_stuck_count * self.params.mp_r_stuck_val
            + w_features_map.q_stuck_count * self.params.mp_q_stuck_val

            - b_features_map.p_sqr_count * self.params.mp_p_sqr_base_val
            - b_features_map.p_eg_sqr_count * self.params.mp_p_eg_sqr_base_val
            - b_features_map.n_sqr_count * self.params.mp_n_sqr_base_val
            - b_features_map.b_sqr_count * self.params.mp_b_sqr_base_val
            - b_features_map.k_sqr_count * self.params.mp_k_sqr_base_val
            - b_features_map.k_eg_sqr_count * self.params.mp_k_eg_sqr_base_val
            - b_features_map.pin_count * self.params.mp_pin_val
            - b_features_map.semi_pin_count * self.params.mp_semi_pin_val
            - b_features_map.rook_open_count * self.params.mp_rook_open_val
            - b_features_map.rook_semi_open_count * self.params.mp_rook_semi_open_val
            - b_features_map.king_pawn_protection_count * self.params.mp_king_pawn_protection_val
            - b_features_map.king_on_open_file_count * self.params.mp_king_on_open_file_val
            - b_features_map.king_on_opponent_semi_open_file_count * self.params.mp_king_on_opponent_semi_open_file_val
            - b_features_map.king_on_own_semi_open_file_count * self.params.mp_king_on_own_semi_open_file_val
            - b_features_map.king_near_open_file_count * self.params.mp_king_near_open_file_val
            - b_features_map.king_near_opponent_semi_open_file_count * self.params.mp_king_near_opponent_semi_open_file_val
            - b_features_map.king_near_own_semi_open_file_count * self.params.mp_king_near_own_semi_open_file_val
            - b_features_map.king_cas_rights_count * self.params.mp_king_cas_rights_val
            - b_features_map.king_caslted_count * self.params.mp_king_castled_val
            - b_features_map.pk_attack_count * self.params.mp_pk_attack_val
            - b_features_map.nk_attack_count * self.params.mp_nk_attack_val
            - b_features_map.bk_attack_count * self.params.mp_bk_attack_val
            - b_features_map.rk_attack_count * self.params.mp_rk_attack_val
            - b_features_map.qk_attack_count * self.params.mp_qk_attack_val
            - b_features_map.protected_p_count * self.params.mp_protected_p_val
            - b_features_map.protected_n_count * self.params.mp_protected_n_val
            - b_features_map.protected_b_count * self.params.mp_protected_b_val
            - b_features_map.protected_r_count * self.params.mp_protected_r_val
            - b_features_map.np_protected_3rd_rank_sqr_count * self.params.mp_np_protected_3rd_rank_sqr_val
            - b_features_map.np_protected_4th_rank_sqr_count * self.params.mp_np_protected_4th_rank_sqr_val
            - b_features_map.behind_pawn_count * self.params.mp_behind_pawn_val
            - b_features_map.isolated_pawn_count * self.params.mp_isolated_pawn_val
            - b_features_map.doubled_pawn_count * self.params.mp_doubled_pawn_val
            - b_features_map.passer_count * self.params.mp_passer_base_val
            - b_features_map.passer_rank_count * self.params.mp_passer_rank_val
            - b_features_map.candidate_passer_count * self.params.mp_candidate_passer_base_val
            - b_features_map.candidate_passer_rank_count * self.params.mp_candidate_passer_rank_val
            - b_features_map.unprotected_sqr_count * self.params.mp_unprotected_sqr_val
            - b_features_map.under_attacked_sqr_count * self.params.mp_under_attacked_sqr_val
            - b_features_map.n_mobility_count * self.params.mp_n_mob_base_val
            - b_features_map.b_mobility_count * self.params.mp_b_mob_base_val
            - b_features_map.r_mobility_count * self.params.mp_r_mob_base_val
            - b_features_map.q_mobility_count * self.params.mp_q_mob_base_val
            - b_features_map.k_mobility_count * self.params.mp_k_mob_base_val
            - b_features_map.n_stuck_count * self.params.mp_n_stuck_val
            - b_features_map.b_stuck_count * self.params.mp_b_stuck_val
            - b_features_map.r_stuck_count * self.params.mp_r_stuck_val
            - b_features_map.q_stuck_count * self.params.mp_q_stuck_val;

        let pos_pp_score =
            w_features_map.p_sqr_count * self.params.pp_p_sqr_base_val
            + w_features_map.p_eg_sqr_count * self.params.pp_p_eg_sqr_base_val
            + w_features_map.n_sqr_count * self.params.pp_n_sqr_base_val
            + w_features_map.b_sqr_count * self.params.pp_b_sqr_base_val
            + w_features_map.k_sqr_count * self.params.pp_k_sqr_base_val
            + w_features_map.k_eg_sqr_count * self.params.pp_k_eg_sqr_base_val
            + w_features_map.pin_count * self.params.pp_pin_val
            + w_features_map.semi_pin_count * self.params.pp_semi_pin_val
            + w_features_map.rook_open_count * self.params.pp_rook_open_val
            + w_features_map.rook_semi_open_count * self.params.pp_rook_semi_open_val
            + w_features_map.king_cas_rights_count * self.params.pp_king_cas_rights_val
            + w_features_map.king_caslted_count * self.params.pp_king_castled_val
            + w_features_map.pk_attack_count * self.params.pp_pk_attack_val
            + w_features_map.nk_attack_count * self.params.pp_nk_attack_val
            + w_features_map.bk_attack_count * self.params.pp_bk_attack_val
            + w_features_map.rk_attack_count * self.params.pp_rk_attack_val
            + w_features_map.qk_attack_count * self.params.pp_qk_attack_val
            + w_features_map.protected_p_count * self.params.pp_protected_p_val
            + w_features_map.protected_n_count * self.params.pp_protected_n_val
            + w_features_map.protected_b_count * self.params.pp_protected_b_val
            + w_features_map.protected_r_count * self.params.pp_protected_r_val
            + w_features_map.np_protected_3rd_rank_sqr_count * self.params.pp_np_protected_3rd_rank_sqr_val
            + w_features_map.np_protected_4th_rank_sqr_count * self.params.pp_np_protected_4th_rank_sqr_val
            + w_features_map.behind_pawn_count * self.params.pp_behind_pawn_val
            + w_features_map.isolated_pawn_count * self.params.pp_isolated_pawn_val
            + w_features_map.doubled_pawn_count * self.params.pp_doubled_pawn_val
            + w_features_map.passer_count * self.params.pp_passer_base_val
            + w_features_map.passer_rank_count * self.params.pp_passer_rank_val
            + w_features_map.candidate_passer_count * self.params.pp_candidate_passer_base_val
            + w_features_map.candidate_passer_rank_count * self.params.pp_candidate_passer_rank_val
            + w_features_map.unprotected_sqr_count * self.params.pp_unprotected_sqr_val
            + w_features_map.under_attacked_sqr_count * self.params.pp_under_attacked_sqr_val
            + w_features_map.n_mobility_count * self.params.pp_n_mob_base_val
            + w_features_map.b_mobility_count * self.params.pp_b_mob_base_val
            + w_features_map.r_mobility_count * self.params.pp_r_mob_base_val
            + w_features_map.q_mobility_count * self.params.pp_q_mob_base_val
            + w_features_map.k_mobility_count * self.params.pp_k_mob_base_val
            + w_features_map.n_stuck_count * self.params.pp_n_stuck_val
            + w_features_map.b_stuck_count * self.params.pp_b_stuck_val
            + w_features_map.r_stuck_count * self.params.pp_r_stuck_val
            + w_features_map.q_stuck_count * self.params.pp_q_stuck_val

            - b_features_map.p_sqr_count * self.params.pp_p_sqr_base_val
            - b_features_map.p_eg_sqr_count * self.params.pp_p_eg_sqr_base_val
            - b_features_map.n_sqr_count * self.params.pp_n_sqr_base_val
            - b_features_map.b_sqr_count * self.params.pp_b_sqr_base_val
            - b_features_map.k_sqr_count * self.params.pp_k_sqr_base_val
            - b_features_map.k_eg_sqr_count * self.params.pp_k_eg_sqr_base_val
            - b_features_map.pin_count * self.params.pp_pin_val
            - b_features_map.semi_pin_count * self.params.pp_semi_pin_val
            - b_features_map.rook_open_count * self.params.pp_rook_open_val
            - b_features_map.rook_semi_open_count * self.params.pp_rook_semi_open_val
            - b_features_map.king_cas_rights_count * self.params.pp_king_cas_rights_val
            - b_features_map.king_caslted_count * self.params.pp_king_castled_val
            - b_features_map.pk_attack_count * self.params.pp_pk_attack_val
            - b_features_map.nk_attack_count * self.params.pp_nk_attack_val
            - b_features_map.bk_attack_count * self.params.pp_bk_attack_val
            - b_features_map.rk_attack_count * self.params.pp_rk_attack_val
            - b_features_map.qk_attack_count * self.params.pp_qk_attack_val
            - b_features_map.protected_p_count * self.params.pp_protected_p_val
            - b_features_map.protected_n_count * self.params.pp_protected_n_val
            - b_features_map.protected_b_count * self.params.pp_protected_b_val
            - b_features_map.protected_r_count * self.params.pp_protected_r_val
            - b_features_map.np_protected_3rd_rank_sqr_count * self.params.pp_np_protected_3rd_rank_sqr_val
            - b_features_map.np_protected_4th_rank_sqr_count * self.params.pp_np_protected_4th_rank_sqr_val
            - b_features_map.behind_pawn_count * self.params.pp_behind_pawn_val
            - b_features_map.isolated_pawn_count * self.params.pp_isolated_pawn_val
            - b_features_map.doubled_pawn_count * self.params.pp_doubled_pawn_val
            - b_features_map.passer_count * self.params.pp_passer_base_val
            - b_features_map.passer_rank_count * self.params.pp_passer_rank_val
            - b_features_map.candidate_passer_count * self.params.pp_candidate_passer_base_val
            - b_features_map.candidate_passer_rank_count * self.params.pp_candidate_passer_rank_val
            - b_features_map.unprotected_sqr_count * self.params.pp_unprotected_sqr_val
            - b_features_map.under_attacked_sqr_count * self.params.pp_under_attacked_sqr_val
            - b_features_map.n_mobility_count * self.params.pp_n_mob_base_val
            - b_features_map.b_mobility_count * self.params.pp_b_mob_base_val
            - b_features_map.r_mobility_count * self.params.pp_r_mob_base_val
            - b_features_map.q_mobility_count * self.params.pp_q_mob_base_val
            - b_features_map.k_mobility_count * self.params.pp_k_mob_base_val
            - b_features_map.n_stuck_count * self.params.pp_n_stuck_val
            - b_features_map.b_stuck_count * self.params.pp_b_stuck_val
            - b_features_map.r_stuck_count * self.params.pp_r_stuck_val
            - b_features_map.q_stuck_count * self.params.pp_q_stuck_val;

        let pos_rmp_score =
            w_features_map.behind_pawn_count * self.params.rmp_behind_pawn_val
            + w_features_map.isolated_pawn_count * self.params.rmp_isolated_pawn_val
            + w_features_map.doubled_pawn_count * self.params.rmp_doubled_pawn_val
            + w_features_map.passer_count * self.params.rmp_passer_base_val
            + w_features_map.passer_rank_count * self.params.rmp_passer_rank_val
            + w_features_map.candidate_passer_count * self.params.rmp_candidate_passer_base_val
            + w_features_map.candidate_passer_rank_count * self.params.rmp_candidate_passer_rank_val
            + w_features_map.king_cas_rights_count * self.params.rmp_king_cas_rights_val
            + w_features_map.king_caslted_count * self.params.rmp_king_castled_val
            + w_features_map.n_mobility_count * self.params.rmp_n_mob_base_val
            + w_features_map.b_mobility_count * self.params.rmp_b_mob_base_val
            + w_features_map.r_mobility_count * self.params.rmp_r_mob_base_val
            + w_features_map.q_mobility_count * self.params.rmp_q_mob_base_val
            + w_features_map.k_mobility_count * self.params.rmp_k_mob_base_val

            - b_features_map.behind_pawn_count * self.params.rmp_behind_pawn_val
            - b_features_map.isolated_pawn_count * self.params.rmp_isolated_pawn_val
            - b_features_map.doubled_pawn_count * self.params.rmp_doubled_pawn_val
            - b_features_map.passer_count * self.params.rmp_passer_base_val
            - b_features_map.passer_rank_count * self.params.rmp_passer_rank_val
            - b_features_map.candidate_passer_count * self.params.rmp_candidate_passer_base_val
            - b_features_map.candidate_passer_rank_count * self.params.rmp_candidate_passer_rank_val
            - b_features_map.king_cas_rights_count * self.params.rmp_king_cas_rights_val
            - b_features_map.king_caslted_count * self.params.rmp_king_castled_val
            - b_features_map.n_mobility_count * self.params.rmp_n_mob_base_val
            - b_features_map.b_mobility_count * self.params.rmp_b_mob_base_val
            - b_features_map.r_mobility_count * self.params.rmp_r_mob_base_val
            - b_features_map.q_mobility_count * self.params.rmp_q_mob_base_val
            - b_features_map.k_mobility_count * self.params.rmp_k_mob_base_val;

        let pos_rpp_score =
            w_features_map.behind_pawn_count * self.params.rpp_behind_pawn_val
            + w_features_map.isolated_pawn_count * self.params.rpp_isolated_pawn_val
            + w_features_map.doubled_pawn_count * self.params.rpp_doubled_pawn_val
            + w_features_map.passer_count * self.params.rpp_passer_base_val
            + w_features_map.passer_rank_count * self.params.rpp_passer_rank_val
            + w_features_map.candidate_passer_count * self.params.rpp_candidate_passer_base_val
            + w_features_map.candidate_passer_rank_count * self.params.rpp_candidate_passer_rank_val
            + w_features_map.king_caslted_count * self.params.rpp_king_castled_val
            + w_features_map.n_mobility_count * self.params.rpp_n_mob_base_val
            + w_features_map.b_mobility_count * self.params.rpp_b_mob_base_val
            + w_features_map.r_mobility_count * self.params.rpp_r_mob_base_val
            + w_features_map.q_mobility_count * self.params.rpp_q_mob_base_val
            + w_features_map.k_mobility_count * self.params.rpp_k_mob_base_val

            - b_features_map.behind_pawn_count * self.params.rpp_behind_pawn_val
            - b_features_map.isolated_pawn_count * self.params.rpp_isolated_pawn_val
            - b_features_map.doubled_pawn_count * self.params.rpp_doubled_pawn_val
            - b_features_map.passer_count * self.params.rpp_passer_base_val
            - b_features_map.passer_rank_count * self.params.rpp_passer_rank_val
            - b_features_map.candidate_passer_count * self.params.rpp_candidate_passer_base_val
            - b_features_map.candidate_passer_rank_count * self.params.rpp_candidate_passer_rank_val
            - b_features_map.king_caslted_count * self.params.rpp_king_castled_val
            - b_features_map.n_mobility_count * self.params.rpp_n_mob_base_val
            - b_features_map.b_mobility_count * self.params.rpp_b_mob_base_val
            - b_features_map.r_mobility_count * self.params.rpp_r_mob_base_val
            - b_features_map.q_mobility_count * self.params.rpp_q_mob_base_val
            - b_features_map.k_mobility_count * self.params.rpp_k_mob_base_val;

        let main_phase = get_phase(state);
        let pawn_phase = get_pawn_phase(state);

        let pos_score = pos_mp_score * main_phase / TOTAL_MAIN_PHASE 
            + pos_rmp_score * (TOTAL_MAIN_PHASE - main_phase) / TOTAL_MAIN_PHASE
            + pos_pp_score * pawn_phase / TOTAL_PAWN_PHASE
            + pos_rpp_score * (TOTAL_PAWN_PHASE - pawn_phase) / TOTAL_PAWN_PHASE;

        let tempo_score = self.params.mp_tempo_val * main_phase / TOTAL_MAIN_PHASE + self.params.pp_tempo_val * pawn_phase / TOTAL_PAWN_PHASE;

        (self.eval_materials(state) + pos_score) * get_score_sign(state) + tempo_score
    }

    fn extract_features(&self, state: &mut State) -> (FeatureMap, FeatureMap) {
        let squares = state.squares;
        let bitboard = state.bitboard;
        let bitmask = bitmask::get_bitmask();
        let file_masks = bitmask.file_masks;

        let mut w_feature_map = FeatureMap::empty();
        let mut b_feature_map = FeatureMap::empty();

        if state.cas_history & W_CAS_MASK != 0 {
            w_feature_map.king_caslted_count = 1;
        } else if state.cas_rights & W_CAS_MASK != 0 {
            w_feature_map.king_cas_rights_count = 1;
        }
    
        if state.cas_history & B_CAS_MASK != 0 {
            b_feature_map.king_caslted_count = 1;
        } else if state.cas_rights & B_CAS_MASK != 0 {
            b_feature_map.king_cas_rights_count = 1;
        }

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
                    let passer_rank = def::get_passer_rank(def::PLAYER_W, index) as i32;

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
                        w_feature_map.passer_count += 1;
                        w_feature_map.passer_rank_count += passer_rank - 1;

                        if piece_mask == 0 {
                            let pawn_control_mask = bitmask.wp_front_control_sqr_masks[index];
                            if pawn_control_mask == 0 || pawn_control_mask & bitmask.index_masks[state.wk_index] != 0 {
                                w_feature_map.controlled_passer_count += 1;
                            }
                        }
                    } else if forward_mask & (bitboard.w_pawn | bitboard.b_pawn) & file_mask == 0 && (forward_mask & bitboard.b_pawn).count_ones() == 1 && bitmask.wp_connected_sqr_masks[index] & bitboard.w_pawn != 0 {
                        w_feature_map.candidate_passer_count += 1;
                        w_feature_map.candidate_passer_rank_count += passer_rank - 1;
                    }

                    if (file_mask & bitboard.w_pawn).count_ones() > 1 {
                        w_feature_map.doubled_pawn_count += 1;
                    }
                },
                def::BP => {
                    bp_attack_mask |= bitmask.bp_attack_masks[index];

                    let file_mask = file_masks[index];
                    let forward_mask = bitmask.bp_forward_masks[index];
                    let passer_rank = def::get_passer_rank(def::PLAYER_B, index) as i32;

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
                        b_feature_map.passer_count += 1;
                        b_feature_map.passer_rank_count += passer_rank - 1;

                        if piece_mask == 0 {
                            let pawn_control_mask = bitmask.bp_front_control_sqr_masks[index];
                            if pawn_control_mask == 0 || pawn_control_mask & bitmask.index_masks[state.bk_index] != 0 {
                                b_feature_map.controlled_passer_count += 1;
                            }
                        }
                    } else if forward_mask & (bitboard.w_pawn | bitboard.b_pawn) & file_mask == 0 && (forward_mask & bitboard.w_pawn).count_ones() == 1 && bitmask.bp_connected_sqr_masks[index] & bitboard.b_pawn != 0 {
                        b_feature_map.candidate_passer_count += 1;
                        b_feature_map.candidate_passer_rank_count += passer_rank - 1;
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

                    let file_mask = file_masks[index];

                    if file_mask & (bitboard.w_pawn | bitboard.b_pawn) == 0 {
                        w_feature_map.rook_open_count += 1;
                    } else if file_mask & bitboard.w_pawn == 0 {
                        w_feature_map.rook_semi_open_count += 1;
                    }
                },
                def::BR => {
                    let mut mov_mask = 0;

                    mov_mask |= bitmask.horizontal_attack_masks[index][util::kindergarten_transform_rank_diag(occupy_mask & bitmask.rank_masks[index]) as usize];
                    mov_mask |= bitmask.vertical_attack_masks[index][util::kindergarten_transform_file(occupy_mask & bitmask.file_masks[index], index) as usize];

                    br_attack_mask |= mov_mask;
                    mov_mask_map[index] = mov_mask;

                    let file_mask = file_masks[index];

                    if file_mask & (bitboard.b_pawn | bitboard.w_pawn) == 0 {
                        b_feature_map.rook_open_count += 1;
                    } else if file_mask & bitboard.b_pawn == 0 {
                        b_feature_map.rook_semi_open_count += 1;
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

        let wk_ring_mask = bitmask.k_attack_masks[state.wk_index];
        let bk_ring_mask = bitmask.k_attack_masks[state.bk_index];

        let w_attack_without_king_mask = wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask | wq_attack_mask;
        let w_attack_mask = w_attack_without_king_mask | wk_ring_mask;

        let b_attack_without_king_mask = bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask | bq_attack_mask;
        let b_attack_mask = b_attack_without_king_mask | bk_ring_mask;

        w_feature_map.protected_p_count = (bitboard.w_pawn & w_attack_mask).count_ones() as i32;
        w_feature_map.protected_n_count = (bitboard.w_knight & w_attack_mask & !bp_attack_mask).count_ones() as i32;
        w_feature_map.protected_b_count = (bitboard.w_bishop & w_attack_mask & !bp_attack_mask).count_ones() as i32;
        w_feature_map.protected_r_count = (bitboard.w_rook & w_attack_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask)).count_ones() as i32;

        b_feature_map.protected_p_count = (bitboard.b_pawn & b_attack_mask).count_ones() as i32;
        b_feature_map.protected_n_count = (bitboard.b_knight & b_attack_mask & !wp_attack_mask).count_ones() as i32;
        b_feature_map.protected_b_count = (bitboard.b_bishop & b_attack_mask & !wp_attack_mask).count_ones() as i32;
        b_feature_map.protected_r_count = (bitboard.b_rook & b_attack_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask)).count_ones() as i32;

        w_feature_map.np_protected_3rd_rank_sqr_count = (W_3RD_RANK_MASK & !wp_attack_mask).count_ones() as i32;
        w_feature_map.np_protected_4th_rank_sqr_count = (W_4TH_RANK_MASK & !wp_attack_mask).count_ones() as i32;

        b_feature_map.np_protected_3rd_rank_sqr_count = (B_3RD_RANK_MASK & !bp_attack_mask).count_ones() as i32;
        b_feature_map.np_protected_4th_rank_sqr_count = (B_4TH_RANK_MASK & !bp_attack_mask).count_ones() as i32;

        w_feature_map.unprotected_sqr_count = (W_BASE_MASK & !w_attack_mask).count_ones() as i32;
        w_feature_map.under_attacked_sqr_count = (W_BASE_MASK & b_attack_mask & !w_attack_mask).count_ones() as i32;
        w_feature_map.under_attacked_sqr_count += (W_BASE_MASK & bp_attack_mask).count_ones() as i32;
        w_feature_map.under_attacked_sqr_count += (W_BASE_MASK & (bn_attack_mask | bb_attack_mask) & !(wp_attack_mask | wn_attack_mask | wb_attack_mask)).count_ones() as i32;
        w_feature_map.under_attacked_sqr_count += (W_BASE_MASK & br_attack_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask)).count_ones() as i32;
        w_feature_map.under_attacked_sqr_count += (W_BASE_MASK & bq_attack_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask | wq_attack_mask)).count_ones() as i32;

        b_feature_map.unprotected_sqr_count = (B_BASE_MASK & !b_attack_mask).count_ones() as i32;
        b_feature_map.under_attacked_sqr_count = (B_BASE_MASK & w_attack_mask & !b_attack_mask).count_ones() as i32;
        b_feature_map.under_attacked_sqr_count += (B_BASE_MASK & wp_attack_mask).count_ones() as i32;
        b_feature_map.under_attacked_sqr_count += (B_BASE_MASK & (wn_attack_mask | wb_attack_mask) & !(bp_attack_mask | bn_attack_mask | bb_attack_mask)).count_ones() as i32;
        b_feature_map.under_attacked_sqr_count += (B_BASE_MASK & wr_attack_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask)).count_ones() as i32;
        b_feature_map.under_attacked_sqr_count += (B_BASE_MASK & wq_attack_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask | bq_attack_mask)).count_ones() as i32;

        if bitmask.index_masks[state.wk_index] & WK_K_SIDE_MASK != 0 {
            w_feature_map.king_pawn_protection_count = (bitboard.w_pawn & WK_K_SIDE_MASK).count_ones() as i32;
        } else if bitmask.index_masks[state.wk_index] & WK_Q_SIDE_MASK != 0 {
            w_feature_map.king_pawn_protection_count = (bitboard.w_pawn & WK_Q_SIDE_MASK).count_ones() as i32;
        }

        if bitmask.index_masks[state.bk_index] & BK_K_SIDE_MASK != 0 {
            b_feature_map.king_pawn_protection_count = (bitboard.b_pawn & BK_K_SIDE_MASK).count_ones() as i32;
        } else if bitmask.index_masks[state.bk_index] & BK_Q_SIDE_MASK != 0 {
            b_feature_map.king_pawn_protection_count = (bitboard.b_pawn & BK_Q_SIDE_MASK).count_ones() as i32;
        }

        {
            let wk_file_mask = file_masks[state.wk_index];
            let wk_file = def::get_file(state.wk_index);

            if (bitboard.w_pawn | bitboard.b_pawn) & wk_file_mask == 0 {
                w_feature_map.king_on_open_file_count = 1;
            } else {
                if bitboard.b_pawn & wk_file_mask == 0 {
                    w_feature_map.king_on_opponent_semi_open_file_count = 1;
                }

                if bitboard.w_pawn & wk_file_mask == 0 {
                    w_feature_map.king_on_own_semi_open_file_count = 1;
                }
            }

            if wk_file > 0 {
                let wk_left_file_mask = file_masks[state.wk_index - 1];

                if (bitboard.w_pawn | bitboard.b_pawn) & wk_left_file_mask == 0 {
                    w_feature_map.king_near_open_file_count += 1;
                } else {
                    if bitboard.b_pawn & wk_left_file_mask == 0 {
                        w_feature_map.king_near_opponent_semi_open_file_count += 1;
                    }

                    if bitboard.w_pawn & wk_left_file_mask == 0 {
                        w_feature_map.king_near_own_semi_open_file_count += 1;
                    }
                }
            }

            if wk_file < 7 {
                let wk_right_file_mask = file_masks[state.wk_index + 1];

                if (bitboard.w_pawn | bitboard.b_pawn) & wk_right_file_mask == 0 {
                    w_feature_map.king_near_open_file_count += 1;
                } else {
                    if bitboard.b_pawn & wk_right_file_mask == 0 {
                        w_feature_map.king_near_opponent_semi_open_file_count += 1;
                    }

                    if bitboard.w_pawn & wk_right_file_mask == 0 {
                        w_feature_map.king_near_own_semi_open_file_count += 1;
                    }
                }
            }
        }

        {
            let bk_file_mask = file_masks[state.bk_index];
            let bk_file = def::get_file(state.bk_index);

            if (bitboard.b_pawn | bitboard.w_pawn) & bk_file_mask == 0 {
                b_feature_map.king_on_open_file_count = 1;
            } else {
                if bitboard.w_pawn & bk_file_mask == 0 {
                    b_feature_map.king_on_opponent_semi_open_file_count = 1;
                }

                if bitboard.b_pawn & bk_file_mask == 0 {
                    b_feature_map.king_on_own_semi_open_file_count = 1;
                }
            }

            if bk_file > 0 {
                let bk_left_file_mask = file_masks[state.bk_index - 1];

                if (bitboard.b_pawn | bitboard.w_pawn) & bk_left_file_mask == 0 {
                    b_feature_map.king_near_open_file_count += 1;
                } else {
                    if bitboard.w_pawn & bk_left_file_mask == 0 {
                        b_feature_map.king_near_opponent_semi_open_file_count += 1;
                    }

                    if bitboard.b_pawn & bk_left_file_mask == 0 {
                        b_feature_map.king_near_own_semi_open_file_count += 1;
                    }
                }
            }

            if bk_file < 7 {
                let bk_right_file_mask = file_masks[state.bk_index + 1];

                if (bitboard.b_pawn | bitboard.w_pawn) & bk_right_file_mask == 0 {
                    b_feature_map.king_near_open_file_count += 1;
                } else {
                    if bitboard.w_pawn & bk_right_file_mask == 0 {
                        b_feature_map.king_near_opponent_semi_open_file_count += 1;
                    }

                    if bitboard.w_pawn & bk_right_file_mask == 0 {
                        b_feature_map.king_near_own_semi_open_file_count += 1;
                    }
                }
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

            let index_mask = bitmask.index_masks[index];
            let mov_mask = mov_mask_map[index];

            if !def::is_k(piece) {
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
                    w_feature_map.p_sqr_count += SQR_TIER_WP[index];
                    w_feature_map.p_eg_sqr_count += SQR_TIER_WP_EG[index];

                    w_feature_map.pk_attack_count += 1;
                },
                def::WN => {
                    w_feature_map.n_sqr_count += SQR_TIER_N[index];

                    let mobility_mask = mov_mask & !bp_attack_mask & !bitboard.w_all & !(b_attack_mask & !w_attack_mask);
                    if mobility_mask == 0 {
                        w_feature_map.n_stuck_count += 1;
                    } else {
                        w_feature_map.n_mobility_count += mobility_mask.count_ones() as i32;
                    }

                    if bk_ring_mask & mov_mask & !bp_attack_mask != 0 {
                        w_feature_map.nk_attack_count += 1;
                    }
                },
                def::WB => {
                    w_feature_map.b_sqr_count += SQR_TIER_B[index];

                    let mobility_mask = mov_mask & !bp_attack_mask & !bitboard.w_all & !(b_attack_mask & !w_attack_mask);
                    if mobility_mask == 0 {
                        w_feature_map.b_stuck_count += 1;
                    } else {
                        w_feature_map.b_mobility_count += mobility_mask.count_ones() as i32;
                    }

                    if bk_ring_mask & mov_mask & !bp_attack_mask != 0 {
                        w_feature_map.bk_attack_count += 1;
                    }
                },
                def::WR => {
                    let mobility_mask = mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask) & !bitboard.w_all & !(b_attack_mask & !w_attack_mask);
                    if mobility_mask == 0 {
                        w_feature_map.r_stuck_count += 1;
                    } else {
                        w_feature_map.r_mobility_count += mobility_mask.count_ones() as i32;
                    }

                    if bk_ring_mask & mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask) != 0 {
                        w_feature_map.rk_attack_count += 1;
                    }
                },
                def::WQ => {
                    let mobility_mask = mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask) & !bitboard.w_all & !(b_attack_mask & !w_attack_mask);
                    if mobility_mask == 0 {
                        w_feature_map.q_stuck_count += 1;
                    } else {
                        w_feature_map.q_mobility_count += mobility_mask.count_ones() as i32;
                    }

                    if bk_ring_mask & mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask) != 0 {
                        w_feature_map.qk_attack_count += 1;
                    }
                },
                def::WK => {
                    w_feature_map.k_sqr_count += SQR_TIER_WK[index];
                    w_feature_map.k_eg_sqr_count += SQR_TIER_K_EG[index];
                },
                def::BP => {
                    b_feature_map.p_sqr_count += SQR_TIER_BP[index];
                    b_feature_map.p_eg_sqr_count += SQR_TIER_BP_EG[index];

                    b_feature_map.pk_attack_count += 1;
                },
                def::BN => {
                    b_feature_map.n_sqr_count += SQR_TIER_N[index];

                    let mobility_mask = mov_mask & !wp_attack_mask & !bitboard.b_all & !(w_attack_mask & !b_attack_mask);
                    if mobility_mask == 0 {
                        b_feature_map.n_stuck_count += 1;
                    } else {
                        b_feature_map.n_mobility_count += mobility_mask.count_ones() as i32;
                    }

                    if wk_ring_mask & mov_mask & !wp_attack_mask != 0 {
                        b_feature_map.nk_attack_count += 1;
                    }
                },
                def::BB => {
                    b_feature_map.b_sqr_count += SQR_TIER_B[index];

                    let mobility_mask = mov_mask & !wp_attack_mask & !bitboard.b_all & !(w_attack_mask & !b_attack_mask);
                    if mobility_mask == 0 {
                        b_feature_map.b_stuck_count += 1;
                    } else {
                        b_feature_map.b_mobility_count += mobility_mask.count_ones() as i32;
                    }

                    if wk_ring_mask & mov_mask & !wp_attack_mask != 0 {
                        b_feature_map.bk_attack_count += 1;
                    }
                },
                def::BR => {
                    let mobility_mask = mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask) & !bitboard.b_all & !(w_attack_mask & !b_attack_mask);
                    if mobility_mask == 0 {
                        b_feature_map.r_stuck_count += 1;
                    } else {
                        b_feature_map.r_mobility_count += mobility_mask.count_ones() as i32;
                    }

                    if wk_ring_mask & mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask) != 0 {
                        b_feature_map.rk_attack_count += 1;
                    }
                },
                def::BQ => {
                    let mobility_mask = mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask) & !bitboard.b_all & !(w_attack_mask & !b_attack_mask);
                    if mobility_mask == 0 {
                        b_feature_map.q_stuck_count += 1;
                    } else {
                        b_feature_map.q_mobility_count += mobility_mask.count_ones() as i32;
                    }

                    if wk_ring_mask & mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask) != 0 {
                        b_feature_map.qk_attack_count += 1;
                    }
                },
                def::BK => {
                    b_feature_map.k_sqr_count += SQR_TIER_BK[index];
                    b_feature_map.k_eg_sqr_count += SQR_TIER_K_EG[index];
                }
                _ => {},
            }
        }

        (w_feature_map, b_feature_map)
    }
}
