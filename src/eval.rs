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

#[derive(PartialEq, Debug)]
pub struct FeatureMap {
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

    threat_point: i32,
    pin_count: i32,
    semi_pin_count: i32,

    pk_attack_count: i32,
    nk_attack_count: i32,
    bk_attack_count: i32,
    rk_attack_count: i32,
    qk_attack_count: i32,

    king_exposure_count: i32,

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

    n_stuck_count: i32,
    b_stuck_count: i32,
    r_stuck_count: i32,
    q_stuck_count: i32,
}

impl FeatureMap {
    pub fn empty() -> Self {
        FeatureMap {
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

            threat_point: 0,
            pin_count: 0,
            semi_pin_count: 0,

            pk_attack_count: 0,
            nk_attack_count: 0,
            bk_attack_count: 0,
            rk_attack_count: 0,
            qk_attack_count: 0,
            king_exposure_count: 0,

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

    pub fn eval_materials(&self, state: &mut State) -> (i32, bool) {
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
                return (0, true);
            }

            if (bitboard.w_bishop | bitboard.w_knight) == 0 && bitboard.b_bishop == 0 && b_knight_count < 3 {
                return (0, true);
            }

            if (bitboard.b_bishop | bitboard.b_knight) == 0 && bitboard.w_bishop == 0 && w_knight_count < 3 {
                return (0, true);
            }
        }

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

        let material_score = material_base_score
            + material_mp_score * main_phase / TOTAL_MAIN_PHASE
            + material_pp_score * pawn_phase / TOTAL_PAWN_PHASE;

        let mut mp_material_score = 0;
        let mut pp_material_score = 0;

        if material_base_score > self.params.p_val && bitboard.w_pawn == 0 {
            mp_material_score -= self.params.mp_pawn_essential_val;
            pp_material_score -= self.params.pp_pawn_essential_val;
        }

        if material_base_score < -self.params.p_val && bitboard.b_pawn == 0 {
            mp_material_score += self.params.mp_pawn_essential_val;
            pp_material_score += self.params.pp_pawn_essential_val;
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

        if is_endgame_with_different_colored_bishop {
            if bitboard.w_rook | bitboard.b_rook == 0 {
                if material_base_score > 0  {
                    mp_material_score -= self.params.mp_different_color_bishop_val;
                    pp_material_score -= self.params.pp_different_color_bishop_val;
                } else if material_base_score < 0 {
                    mp_material_score += self.params.mp_different_color_bishop_val;
                    pp_material_score += self.params.pp_different_color_bishop_val;
                }
            } else {
                if material_base_score > 0  {
                    mp_material_score -= self.params.mp_different_color_bishop_with_rook_val;
                    pp_material_score -= self.params.pp_different_color_bishop_with_rook_val;
                } else if material_base_score < 0 {
                    mp_material_score += self.params.mp_different_color_bishop_with_rook_val;
                    pp_material_score += self.params.pp_different_color_bishop_with_rook_val;
                }
            }
        } else {
            if w_bishop_count > 1 {
                mp_material_score += self.params.mp_bishop_pair_val;
                pp_material_score += self.params.pp_bishop_pair_val;
            }

            if b_bishop_count > 1 {
                mp_material_score -= self.params.mp_bishop_pair_val;
                pp_material_score -= self.params.pp_bishop_pair_val;
            }
        }

        let score_sign = if state.player == def::PLAYER_W {
            1
        } else {
            -1
        };

        ((material_score + mp_material_score * main_phase / TOTAL_MAIN_PHASE + pp_material_score * pawn_phase / TOTAL_PAWN_PHASE) * score_sign, false)
    }

    pub fn eval_state(&self, state: &mut State, material_score: i32) -> i32 {
        let score_sign = if state.player == def::PLAYER_W {
            1
        } else {
            -1
        };

        let (w_features_map, b_features_map) = self.extract_features(state);

        let pos_mp_score =
            w_features_map.pin_count * self.params.mp_pin_val
            + w_features_map.semi_pin_count * self.params.mp_semi_pin_val
            + w_features_map.rook_open_count * self.params.mp_rook_open_val
            + w_features_map.rook_semi_open_count * self.params.mp_rook_semi_open_val
            + w_features_map.king_exposure_count * self.params.mp_k_exposure_val
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
            + w_features_map.n_stuck_count * self.params.mp_n_stuck_val
            + w_features_map.b_stuck_count * self.params.mp_b_stuck_val
            + w_features_map.r_stuck_count * self.params.mp_r_stuck_val
            + w_features_map.q_stuck_count * self.params.mp_q_stuck_val
            + w_features_map.threat_point / self.params.mp_threat_discount_factor

            - b_features_map.pin_count * self.params.mp_pin_val
            - b_features_map.semi_pin_count * self.params.mp_semi_pin_val
            - b_features_map.rook_open_count * self.params.mp_rook_open_val
            - b_features_map.rook_semi_open_count * self.params.mp_rook_semi_open_val
            - b_features_map.king_exposure_count * self.params.mp_k_exposure_val
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
            - b_features_map.n_stuck_count * self.params.mp_n_stuck_val
            - b_features_map.b_stuck_count * self.params.mp_b_stuck_val
            - b_features_map.r_stuck_count * self.params.mp_r_stuck_val
            - b_features_map.q_stuck_count * self.params.mp_q_stuck_val
            - b_features_map.threat_point / self.params.mp_threat_discount_factor;

        let pos_pp_score =
            w_features_map.pin_count * self.params.pp_pin_val
            + w_features_map.semi_pin_count * self.params.pp_semi_pin_val
            + w_features_map.rook_open_count * self.params.pp_rook_open_val
            + w_features_map.rook_semi_open_count * self.params.pp_rook_semi_open_val
            + w_features_map.king_exposure_count * self.params.pp_k_exposure_val
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
            + w_features_map.n_stuck_count * self.params.pp_n_stuck_val
            + w_features_map.b_stuck_count * self.params.pp_b_stuck_val
            + w_features_map.r_stuck_count * self.params.pp_r_stuck_val
            + w_features_map.q_stuck_count * self.params.pp_q_stuck_val
            + w_features_map.threat_point / self.params.pp_threat_discount_factor

            - b_features_map.pin_count * self.params.pp_pin_val
            - b_features_map.semi_pin_count * self.params.pp_semi_pin_val
            - b_features_map.rook_open_count * self.params.pp_rook_open_val
            - b_features_map.rook_semi_open_count * self.params.pp_rook_semi_open_val
            - b_features_map.king_exposure_count * self.params.pp_k_exposure_val
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
            - b_features_map.n_stuck_count * self.params.pp_n_stuck_val
            - b_features_map.b_stuck_count * self.params.pp_b_stuck_val
            - b_features_map.r_stuck_count * self.params.pp_r_stuck_val
            - b_features_map.q_stuck_count * self.params.pp_q_stuck_val
            - b_features_map.threat_point / self.params.pp_threat_discount_factor;

        let main_phase = get_phase(state);
        let pawn_phase = get_pawn_phase(state);

        let pos_score = pos_mp_score * main_phase / TOTAL_MAIN_PHASE + pos_pp_score * main_phase / TOTAL_PAWN_PHASE;
        let tempo_score = self.params.mp_tempo_val * main_phase / TOTAL_MAIN_PHASE + self.params.pp_tempo_val * pawn_phase / TOTAL_PAWN_PHASE;

        material_score + pos_score * score_sign + tempo_score
    }

    fn extract_features(&self, state: &mut State) -> (FeatureMap, FeatureMap) {
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
            let protecting_pawn_count = (bitboard.w_pawn & WK_K_SIDE_MASK).count_ones();

            if protecting_pawn_count < 2 {
                w_feature_map.king_exposure_count += 1;

                if protecting_pawn_count == 0 {
                    w_feature_map.king_exposure_count += 1;
                }
            }

            if bitboard.w_pawn & bitmask.file_masks[6] == 0 {
                w_feature_map.king_exposure_count += 1;
            }

            if bitboard.w_pawn & bitmask.file_masks[7] == 0 {
                w_feature_map.king_exposure_count += 1;
            }
        } else if bitmask.index_masks[state.wk_index] & WK_Q_SIDE_MASK != 0 {
            let protecting_pawn_count = (bitboard.w_pawn & WK_Q_SIDE_MASK).count_ones();

            if protecting_pawn_count < 2 {
                w_feature_map.king_exposure_count += 1;

                if protecting_pawn_count == 0 {
                    w_feature_map.king_exposure_count += 1;
                }
            }

            if bitboard.w_pawn & bitmask.file_masks[0] == 0 {
                w_feature_map.king_exposure_count += 1;
            }

            if bitboard.w_pawn & bitmask.file_masks[1] == 0 {
                w_feature_map.king_exposure_count += 1;
            }
        }

        if bitmask.index_masks[state.bk_index] & BK_K_SIDE_MASK != 0 {
            let protecting_pawn_count = (bitboard.b_pawn & BK_K_SIDE_MASK).count_ones();

            if protecting_pawn_count < 2 {
                b_feature_map.king_exposure_count += 1;

                if protecting_pawn_count == 0 {
                    b_feature_map.king_exposure_count += 1;
                }
            }

            if bitboard.b_pawn & bitmask.file_masks[6] == 0 {
                b_feature_map.king_exposure_count += 1;
            }

            if bitboard.b_pawn & bitmask.file_masks[7] == 0 {
                b_feature_map.king_exposure_count += 1;
            }
        } else if bitmask.index_masks[state.bk_index] & BK_Q_SIDE_MASK != 0 {
            let protecting_pawn_count = (bitboard.b_pawn & BK_Q_SIDE_MASK).count_ones();

            if protecting_pawn_count < 2 {
                b_feature_map.king_exposure_count += 1;

                if protecting_pawn_count == 0 {
                    b_feature_map.king_exposure_count += 1;
                }
            }

            if bitboard.b_pawn & bitmask.file_masks[0] == 0 {
                b_feature_map.king_exposure_count += 1;
            }

            if bitboard.b_pawn & bitmask.file_masks[1] == 0 {
                b_feature_map.king_exposure_count += 1;
            }
        }

        if state.wk_index < def::SEVENTH_RANK_INDEX {
            let k_front_index = bitmask.index_masks[state.wk_index + def::DIM_SIZE];

            if k_front_index & bitboard.b_pawn != 0 {
                w_feature_map.king_exposure_count -= 1;
            }

            if k_front_index & bitboard.w_pawn == 0 {
                w_feature_map.king_exposure_count += 1;

                if bitmask.file_masks[state.wk_index] & bitboard.w_pawn == 0 {
                    w_feature_map.king_exposure_count += 1;
                }
            }
        }

        if state.bk_index >= def::SECOND_RANK_INDEX {
            let k_front_index = bitmask.index_masks[state.bk_index - def::DIM_SIZE];

            if k_front_index & bitboard.w_pawn != 0 {
                b_feature_map.king_exposure_count -= 1;
            }

            if k_front_index & bitboard.b_pawn == 0 {
                b_feature_map.king_exposure_count += 1;

                if bitmask.file_masks[state.bk_index] & bitboard.b_pawn == 0 {
                    b_feature_map.king_exposure_count += 1;
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

            let threat_val = self.val_of(piece);
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
                    if index_mask & w_attack_mask == 0 {
                        if index_mask & b_attack_mask != 0 {
                            w_feature_map.threat_point -= threat_val;
                        }
                    }

                    w_feature_map.pk_attack_count += 1;
                },
                def::WN => {
                    if index_mask & w_attack_mask == 0 {
                        if index_mask & b_attack_mask != 0 {
                            w_feature_map.threat_point -= threat_val;
                        }
                    } else if index_mask & bp_attack_mask != 0 {
                        w_feature_map.threat_point -= threat_val - self.val_of(def::BP);
                    }

                    let mobility_mask = mov_mask & !bp_attack_mask & !bitboard.w_all & !(b_attack_mask & !w_attack_mask);
                    if mobility_mask == 0 {
                        w_feature_map.n_stuck_count += 1;
                    } else {
                        w_feature_map.n_mobility_count += mobility_mask.count_ones() as i32;
                    }

                    if bk_ring_mask & mov_mask != 0 {
                        w_feature_map.nk_attack_count += 1;
                    }

                    if bk_ring_mask & mov_mask & !bp_attack_mask != 0 {
                        w_feature_map.nk_attack_count += 1;
                    }
                },
                def::WB => {
                    if index_mask & w_attack_mask == 0 {
                        if index_mask & b_attack_mask != 0 {
                            w_feature_map.threat_point -= threat_val;
                        }
                    } else if index_mask & bp_attack_mask != 0 {
                        w_feature_map.threat_point -= threat_val - self.val_of(def::BP);
                    }

                    let mobility_mask = mov_mask & !bp_attack_mask & !bitboard.w_all & !(b_attack_mask & !w_attack_mask);
                    if mobility_mask == 0 {
                        w_feature_map.b_stuck_count += 1;
                    } else {
                        w_feature_map.b_mobility_count += mobility_mask.count_ones() as i32;
                    }

                    if bk_ring_mask & mov_mask != 0 {
                        w_feature_map.bk_attack_count += 1;
                    }

                    if bk_ring_mask & mov_mask & !bp_attack_mask != 0 {
                        w_feature_map.bk_attack_count += 1;
                    }
                },
                def::WR => {
                    if index_mask & w_attack_mask == 0 {
                        if index_mask & b_attack_mask != 0 {
                            w_feature_map.threat_point -= threat_val;
                        }
                    } else if index_mask & bp_attack_mask != 0 {
                        w_feature_map.threat_point -= threat_val - self.val_of(def::BP);
                    } else if index_mask & (bn_attack_mask | bb_attack_mask) != 0 {
                        w_feature_map.threat_point -= threat_val - self.val_of(def::BN);
                    }

                    let mobility_mask = mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask) & !bitboard.w_all & !(b_attack_mask & !w_attack_mask);
                    if mobility_mask == 0 {
                        w_feature_map.r_stuck_count += 1;
                    } else {
                        w_feature_map.r_mobility_count += mobility_mask.count_ones() as i32;
                    }

                    if bk_ring_mask & mov_mask != 0 {
                        w_feature_map.rk_attack_count += 1;
                    }

                    if bk_ring_mask & mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask) != 0 {
                        w_feature_map.rk_attack_count += 1;
                    }
                },
                def::WQ => {
                    if index_mask & w_attack_mask == 0 {
                        if index_mask & b_attack_mask != 0 {
                            w_feature_map.threat_point -= threat_val;
                        }
                    } else if index_mask & bp_attack_mask != 0 {
                        w_feature_map.threat_point -= threat_val - self.val_of(def::BP);
                    } else if index_mask & (bn_attack_mask | bb_attack_mask) != 0 {
                        w_feature_map.threat_point -= threat_val - self.val_of(def::BN);
                    } else if index_mask & br_attack_mask != 0 {
                        w_feature_map.threat_point -= threat_val - self.val_of(def::BR);
                    }

                    let mobility_mask = mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask) & !bitboard.w_all & !(b_attack_mask & !w_attack_mask);
                    if mobility_mask == 0 {
                        w_feature_map.q_stuck_count += 1;
                    } else {
                        w_feature_map.q_mobility_count += mobility_mask.count_ones() as i32;
                    }

                    if bk_ring_mask & mov_mask != 0 {
                        w_feature_map.qk_attack_count += 1;
                    }

                    if bk_ring_mask & mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask) != 0 {
                        w_feature_map.qk_attack_count += 1;
                    }
                },
                def::BP => {
                    if index_mask & b_attack_mask == 0 {
                        if index_mask & w_attack_mask != 0 {
                            b_feature_map.threat_point -= threat_val;
                        }
                    }

                    b_feature_map.pk_attack_count += 1;
                },
                def::BN => {
                    if index_mask & b_attack_mask == 0 {
                        if index_mask & w_attack_mask != 0 {
                            b_feature_map.threat_point -= threat_val;
                        }
                    } else if index_mask & wp_attack_mask != 0 {
                        b_feature_map.threat_point -= threat_val - self.val_of(def::WP);
                    }

                    let mobility_mask = mov_mask & !wp_attack_mask & !bitboard.b_all & !(w_attack_mask & !b_attack_mask);
                    if mobility_mask == 0 {
                        b_feature_map.n_stuck_count += 1;
                    } else {
                        b_feature_map.n_mobility_count += mobility_mask.count_ones() as i32;
                    }

                    if wk_ring_mask & mov_mask != 0 {
                        b_feature_map.nk_attack_count += 1;
                    }

                    if wk_ring_mask & mov_mask & !wp_attack_mask != 0 {
                        b_feature_map.nk_attack_count += 1;
                    }
                },
                def::BB => {
                    if index_mask & b_attack_mask == 0 {
                        if index_mask & w_attack_mask != 0 {
                            b_feature_map.threat_point -= threat_val;
                        }
                    } else if index_mask & wp_attack_mask != 0 {
                        b_feature_map.threat_point -= threat_val - self.val_of(def::WP);
                    }

                    let mobility_mask = mov_mask & !wp_attack_mask & !bitboard.b_all & !(w_attack_mask & !b_attack_mask);
                    if mobility_mask == 0 {
                        b_feature_map.b_stuck_count += 1;
                    } else {
                        b_feature_map.b_mobility_count += mobility_mask.count_ones() as i32;
                    }

                    if wk_ring_mask & mov_mask != 0 {
                        b_feature_map.bk_attack_count += 1;
                    }

                    if wk_ring_mask & mov_mask & !wp_attack_mask != 0 {
                        b_feature_map.bk_attack_count += 1;
                    }
                },
                def::BR => {
                    if index_mask & b_attack_mask == 0 {
                        if index_mask & w_attack_mask != 0 {
                            b_feature_map.threat_point -= threat_val;
                        }
                    } else if index_mask & wp_attack_mask != 0 {
                        b_feature_map.threat_point -= threat_val - self.val_of(def::WP);
                    } else if index_mask & (wn_attack_mask | wb_attack_mask) != 0 {
                        b_feature_map.threat_point -= threat_val - self.val_of(def::WN);
                    }

                    let mobility_mask = mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask) & !bitboard.b_all & !(w_attack_mask & !b_attack_mask);
                    if mobility_mask == 0 {
                        b_feature_map.r_stuck_count += 1;
                    } else {
                        b_feature_map.r_mobility_count += mobility_mask.count_ones() as i32;
                    }

                    if wk_ring_mask & mov_mask != 0 {
                        b_feature_map.rk_attack_count += 1;
                    }

                    if wk_ring_mask & mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask) != 0 {
                        b_feature_map.rk_attack_count += 1;
                    }
                },
                def::BQ => {
                    if index_mask & b_attack_mask == 0 {
                        if index_mask & w_attack_mask != 0 {
                            b_feature_map.threat_point -= threat_val;
                        }
                    } else if index_mask & wp_attack_mask != 0 {
                        b_feature_map.threat_point -= threat_val - self.val_of(def::WP);
                    } else if index_mask & (wn_attack_mask | wb_attack_mask) != 0 {
                        b_feature_map.threat_point -= threat_val - self.val_of(def::WN);
                    } else if index_mask & wr_attack_mask != 0 {
                        b_feature_map.threat_point -= threat_val - self.val_of(def::WR);
                    }

                    let mobility_mask = mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask) & !bitboard.b_all & !(w_attack_mask & !b_attack_mask);
                    if mobility_mask == 0 {
                        b_feature_map.q_stuck_count += 1;
                    } else {
                        b_feature_map.q_mobility_count += mobility_mask.count_ones() as i32;
                    }

                    if wk_ring_mask & mov_mask != 0 {
                        b_feature_map.qk_attack_count += 1;
                    }

                    if wk_ring_mask & mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask) != 0 {
                        b_feature_map.qk_attack_count += 1;
                    }
                },
                _ => {},
            }
        }

        (w_feature_map, b_feature_map)
    }
}
