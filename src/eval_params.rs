use std::collections::HashMap;

#[derive(Debug)]
pub struct EvalParams {
    pub q_val: i32,
    pub r_val: i32,
    pub b_val: i32,
    pub n_val: i32,
    pub p_val: i32,

    pub mp_q_val: i32,
    pub mp_r_val: i32,
    pub mp_b_val: i32,
    pub mp_n_val: i32,
    pub mp_p_val: i32,

    pub pp_q_val: i32,
    pub pp_r_val: i32,
    pub pp_b_val: i32,
    pub pp_n_val: i32,
    pub pp_p_val: i32,

    pub mp_p_sqr_base_val: i32,
    pub mp_p_eg_sqr_base_val: i32,
    pub mp_n_sqr_base_val: i32,
    pub mp_b_sqr_base_val: i32,
    pub mp_r_sqr_base_val: i32,
    pub mp_k_sqr_base_val: i32,
    pub mp_k_eg_sqr_base_val: i32,

    pub pp_p_sqr_base_val: i32,
    pub pp_p_eg_sqr_base_val: i32,
    pub pp_n_sqr_base_val: i32,
    pub pp_b_sqr_base_val: i32,
    pub pp_r_sqr_base_val: i32,
    pub pp_k_sqr_base_val: i32,
    pub pp_k_eg_sqr_base_val: i32,

    pub mp_isolated_pawn_val: i32,
    pub mp_doubled_pawn_val: i32,
    pub mp_behind_pawn_val: i32,
    pub rmp_isolated_pawn_val: i32,
    pub rmp_doubled_pawn_val: i32,
    pub rmp_behind_pawn_val: i32,

    pub pp_isolated_pawn_val: i32,
    pub pp_doubled_pawn_val: i32,
    pub pp_behind_pawn_val: i32,
    pub rpp_isolated_pawn_val: i32,
    pub rpp_doubled_pawn_val: i32,
    pub rpp_behind_pawn_val: i32,

    pub mp_passer_base_val: i32,
    pub mp_passer_rank_val: i32,
    pub mp_candidate_passer_base_val: i32,
    pub mp_candidate_passer_rank_val: i32,
    pub rmp_passer_base_val: i32,
    pub rmp_passer_rank_val: i32,
    pub rmp_candidate_passer_base_val: i32,
    pub rmp_candidate_passer_rank_val: i32,

    pub pp_passer_base_val: i32,
    pub pp_passer_rank_val: i32,
    pub pp_candidate_passer_base_val: i32,
    pub pp_candidate_passer_rank_val: i32,
    pub rpp_passer_base_val: i32,
    pub rpp_passer_rank_val: i32,
    pub rpp_candidate_passer_base_val: i32,
    pub rpp_candidate_passer_rank_val: i32,

    pub mp_pk_attack_val: i32,
    pub mp_nk_attack_val: i32,
    pub mp_bk_attack_val: i32,
    pub mp_rk_attack_val: i32,
    pub mp_qk_attack_val: i32,

    pub pp_pk_attack_val: i32,
    pub pp_nk_attack_val: i32,
    pub pp_bk_attack_val: i32,
    pub pp_rk_attack_val: i32,
    pub pp_qk_attack_val: i32,

    pub mp_king_pawn_protection_val: i32,
    pub mp_king_front_open_val: i32,
    pub mp_king_front_total_open_val: i32,
    pub mp_king_front_enery_pawn_cover_val: i32,
    pub mp_king_side_file_open_val: i32,
    pub mp_king_near_side_file_open_val: i32,

    pub pp_king_pawn_protection_val: i32,
    pub pp_king_front_open_val: i32,
    pub pp_king_front_total_open_val: i32,
    pub pp_king_front_enery_pawn_cover_val: i32,
    pub pp_king_side_file_open_val: i32,
    pub pp_king_near_side_file_open_val: i32,

    pub mp_king_cas_rights_val: i32,
    pub rmp_king_cas_rights_val: i32,
    pub pp_king_cas_rights_val: i32,
    pub rpp_king_cas_rights_val: i32,

    pub mp_rook_open_val: i32,
    pub mp_rook_semi_open_val: i32,

    pub pp_rook_open_val: i32,
    pub pp_rook_semi_open_val: i32,

    pub mp_pin_val: i32,
    pub mp_semi_pin_val: i32,

    pub pp_pin_val: i32,
    pub pp_semi_pin_val: i32,

    pub mp_unprotected_sqr_val: i32,
    pub mp_under_attacked_sqr_val: i32,

    pub pp_unprotected_sqr_val: i32,
    pub pp_under_attacked_sqr_val: i32,

    pub mp_protected_p_val: i32,
    pub mp_protected_n_val: i32,
    pub mp_protected_b_val: i32,
    pub mp_protected_r_val: i32,

    pub pp_protected_p_val: i32,
    pub pp_protected_n_val: i32,
    pub pp_protected_b_val: i32,
    pub pp_protected_r_val: i32,

    pub mp_np_protected_3rd_rank_sqr_val: i32,
    pub mp_np_protected_4th_rank_sqr_val: i32,

    pub pp_np_protected_3rd_rank_sqr_val: i32,
    pub pp_np_protected_4th_rank_sqr_val: i32,

    pub mp_n_mob_base_val: i32,
    pub mp_b_mob_base_val: i32,
    pub mp_r_mob_base_val: i32,
    pub mp_q_mob_base_val: i32,

    pub pp_n_mob_base_val: i32,
    pub pp_b_mob_base_val: i32,
    pub pp_r_mob_base_val: i32,
    pub pp_q_mob_base_val: i32,

    pub mp_n_weak_mob_base_val: i32,
    pub mp_b_weak_mob_base_val: i32,
    pub mp_r_weak_mob_base_val: i32,
    pub mp_q_weak_mob_base_val: i32,

    pub pp_n_weak_mob_base_val: i32,
    pub pp_b_weak_mob_base_val: i32,
    pub pp_r_weak_mob_base_val: i32,
    pub pp_q_weak_mob_base_val: i32,

    pub mp_n_stuck_val: i32,
    pub mp_b_stuck_val: i32,
    pub mp_r_stuck_val: i32,
    pub mp_q_stuck_val: i32,

    pub pp_n_stuck_val: i32,
    pub pp_b_stuck_val: i32,
    pub pp_r_stuck_val: i32,
    pub pp_q_stuck_val: i32,

    pub mp_tempo_val: i32,
    pub pp_tempo_val: i32,

    pub mp_pawn_essential_val: i32,
    pub pp_pawn_essential_val: i32,

    pub mp_different_color_bishop_val: i32,
    pub mp_different_color_bishop_with_rook_val: i32,

    pub pp_different_color_bishop_val: i32,
    pub pp_different_color_bishop_with_rook_val: i32,

    pub mp_bishop_pair_val: i32,
    pub pp_bishop_pair_val: i32,
}

impl EvalParams {
    pub const fn default() -> Self {
        EvalParams {
            q_val: 1000,
            r_val: 500,
            b_val: 350,
            n_val: 345,
            p_val: 100,

            mp_q_val: 0,
            mp_r_val: 0,
            mp_b_val: 0,
            mp_n_val: 0,
            mp_p_val: 0,

            pp_q_val: 0,
            pp_r_val: 0,
            pp_b_val: 0,
            pp_n_val: 0,
            pp_p_val: 0,

            mp_p_sqr_base_val: 0,
            mp_p_eg_sqr_base_val: 0,
            mp_n_sqr_base_val: 7,
            mp_b_sqr_base_val: 2,
            mp_r_sqr_base_val: 0,
            mp_k_sqr_base_val: 0,
            mp_k_eg_sqr_base_val: 0,

            pp_p_sqr_base_val: 0,
            pp_p_eg_sqr_base_val: 0,
            pp_n_sqr_base_val: 0,
            pp_b_sqr_base_val: 0,
            pp_r_sqr_base_val: 0,
            pp_k_sqr_base_val: 0,
            pp_k_eg_sqr_base_val: 0,

            mp_isolated_pawn_val: -10,
            mp_doubled_pawn_val: 8,
            mp_behind_pawn_val: 0,
            rmp_isolated_pawn_val: -10,
            rmp_doubled_pawn_val: 8,
            rmp_behind_pawn_val: 0,

            pp_isolated_pawn_val: 0,
            pp_doubled_pawn_val: 0,
            pp_behind_pawn_val: 0,
            rpp_isolated_pawn_val: 0,
            rpp_doubled_pawn_val: 0,
            rpp_behind_pawn_val: 0,

            mp_passer_base_val: 0,
            mp_passer_rank_val: 0,
            mp_candidate_passer_base_val: 0,
            mp_candidate_passer_rank_val: 0,
            rmp_passer_base_val: 0,
            rmp_passer_rank_val: 24,
            rmp_candidate_passer_base_val: 0,
            rmp_candidate_passer_rank_val: 0,

            pp_passer_base_val: 0,
            pp_passer_rank_val: 0,
            pp_candidate_passer_base_val: 0,
            pp_candidate_passer_rank_val: 0,
            rpp_passer_base_val: 0,
            rpp_passer_rank_val: 0,
            rpp_candidate_passer_base_val: 0,
            rpp_candidate_passer_rank_val: 8,

            mp_pk_attack_val: 0,
            mp_nk_attack_val: 0,
            mp_bk_attack_val: 0,
            mp_rk_attack_val: 56,
            mp_qk_attack_val: 48,

            pp_pk_attack_val: 0,
            pp_nk_attack_val: 0,
            pp_bk_attack_val: 0,
            pp_rk_attack_val: 0,
            pp_qk_attack_val: 0,

            mp_king_pawn_protection_val: 0,
            mp_king_front_open_val: 0,
            mp_king_front_total_open_val: 0,
            mp_king_front_enery_pawn_cover_val: 0,
            mp_king_side_file_open_val: 0,
            mp_king_near_side_file_open_val: 0,

            pp_king_pawn_protection_val: 0,
            pp_king_front_open_val: 0,
            pp_king_front_total_open_val: 0,
            pp_king_front_enery_pawn_cover_val: 0,
            pp_king_side_file_open_val: 0,
            pp_king_near_side_file_open_val: 0,

            mp_king_cas_rights_val: 0,
            rmp_king_cas_rights_val: 0,
            pp_king_cas_rights_val: 0,
            rpp_king_cas_rights_val: 0,

            mp_rook_open_val: 16,
            mp_rook_semi_open_val: 48,

            pp_rook_open_val: 8,
            pp_rook_semi_open_val: 64,

            mp_pin_val: 0,
            mp_semi_pin_val: 0,

            pp_pin_val: 0,
            pp_semi_pin_val: 0,

            mp_unprotected_sqr_val: 0,
            mp_under_attacked_sqr_val: 0,

            pp_unprotected_sqr_val: 0,
            pp_under_attacked_sqr_val: 0,

            mp_protected_p_val: 16,
            mp_protected_n_val: 0,
            mp_protected_b_val: 0,
            mp_protected_r_val: 0,

            pp_protected_p_val: 0,
            pp_protected_n_val: 0,
            pp_protected_b_val: 0,
            pp_protected_r_val: 0,

            mp_np_protected_3rd_rank_sqr_val: 0,
            mp_np_protected_4th_rank_sqr_val: 0,

            pp_np_protected_3rd_rank_sqr_val: 0,
            pp_np_protected_4th_rank_sqr_val: 0,

            mp_n_mob_base_val: 6,
            mp_b_mob_base_val: 12,
            mp_r_mob_base_val: 0,
            mp_q_mob_base_val: 0,

            pp_n_mob_base_val: 0,
            pp_b_mob_base_val: 0,
            pp_r_mob_base_val: 0,
            pp_q_mob_base_val: 0,

            mp_n_weak_mob_base_val: 0,
            mp_b_weak_mob_base_val: 0,
            mp_r_weak_mob_base_val: 0,
            mp_q_weak_mob_base_val: 0,

            pp_n_weak_mob_base_val: 0,
            pp_b_weak_mob_base_val: 0,
            pp_r_weak_mob_base_val: 0,
            pp_q_weak_mob_base_val: 0,

            mp_n_stuck_val: 0,
            mp_b_stuck_val: 0,
            mp_r_stuck_val: 0,
            mp_q_stuck_val: 0,

            pp_n_stuck_val: 0,
            pp_b_stuck_val: 0,
            pp_r_stuck_val: 0,
            pp_q_stuck_val: 0,

            mp_tempo_val: 0,
            pp_tempo_val: 0,

            mp_pawn_essential_val: 0,
            pp_pawn_essential_val: 0,

            mp_different_color_bishop_val: 0,
            mp_different_color_bishop_with_rook_val: 0,

            pp_different_color_bishop_val: 0,
            pp_different_color_bishop_with_rook_val: 0,

            mp_bishop_pair_val: 0,
            pp_bishop_pair_val: 0,
        }
    }

    pub fn from_config(config_map: &HashMap<String, String>) -> Self {
        EvalParams {
            q_val: config_map.get("q_val").unwrap().parse::<i32>().unwrap(),
            r_val: config_map.get("r_val").unwrap().parse::<i32>().unwrap(),
            b_val: config_map.get("b_val").unwrap().parse::<i32>().unwrap(),
            n_val: config_map.get("n_val").unwrap().parse::<i32>().unwrap(),
            p_val: config_map.get("p_val").unwrap().parse::<i32>().unwrap(),
            mp_q_val: config_map.get("mp_q_val").unwrap().parse::<i32>().unwrap(),
            mp_r_val: config_map.get("mp_r_val").unwrap().parse::<i32>().unwrap(),
            mp_b_val: config_map.get("mp_b_val").unwrap().parse::<i32>().unwrap(),
            mp_n_val: config_map.get("mp_n_val").unwrap().parse::<i32>().unwrap(),
            mp_p_val: config_map.get("mp_p_val").unwrap().parse::<i32>().unwrap(),
            pp_q_val: config_map.get("pp_q_val").unwrap().parse::<i32>().unwrap(),
            pp_r_val: config_map.get("pp_r_val").unwrap().parse::<i32>().unwrap(),
            pp_b_val: config_map.get("pp_b_val").unwrap().parse::<i32>().unwrap(),
            pp_n_val: config_map.get("pp_n_val").unwrap().parse::<i32>().unwrap(),
            pp_p_val: config_map.get("pp_p_val").unwrap().parse::<i32>().unwrap(),

            mp_p_sqr_base_val: config_map.get("mp_p_sqr_base_val").unwrap().parse::<i32>().unwrap(),
            mp_p_eg_sqr_base_val: config_map.get("mp_p_eg_sqr_base_val").unwrap().parse::<i32>().unwrap(),
            mp_n_sqr_base_val: config_map.get("mp_n_sqr_base_val").unwrap().parse::<i32>().unwrap(),
            mp_b_sqr_base_val: config_map.get("mp_b_sqr_base_val").unwrap().parse::<i32>().unwrap(),
            mp_r_sqr_base_val: config_map.get("mp_r_sqr_base_val").unwrap().parse::<i32>().unwrap(),
            mp_k_sqr_base_val: config_map.get("mp_k_sqr_base_val").unwrap().parse::<i32>().unwrap(),
            mp_k_eg_sqr_base_val: config_map.get("mp_k_eg_sqr_base_val").unwrap().parse::<i32>().unwrap(),
            pp_p_sqr_base_val: config_map.get("pp_p_sqr_base_val").unwrap().parse::<i32>().unwrap(),
            pp_p_eg_sqr_base_val: config_map.get("pp_p_eg_sqr_base_val").unwrap().parse::<i32>().unwrap(),
            pp_n_sqr_base_val: config_map.get("pp_n_sqr_base_val").unwrap().parse::<i32>().unwrap(),
            pp_b_sqr_base_val: config_map.get("pp_b_sqr_base_val").unwrap().parse::<i32>().unwrap(),
            pp_r_sqr_base_val: config_map.get("pp_r_sqr_base_val").unwrap().parse::<i32>().unwrap(),
            pp_k_sqr_base_val: config_map.get("pp_k_sqr_base_val").unwrap().parse::<i32>().unwrap(),
            pp_k_eg_sqr_base_val: config_map.get("pp_k_eg_sqr_base_val").unwrap().parse::<i32>().unwrap(),

            mp_isolated_pawn_val: config_map.get("mp_isolated_pawn_val").unwrap().parse::<i32>().unwrap(),
            mp_doubled_pawn_val: config_map.get("mp_doubled_pawn_val").unwrap().parse::<i32>().unwrap(),
            mp_behind_pawn_val: config_map.get("mp_behind_pawn_val").unwrap().parse::<i32>().unwrap(),
            rmp_isolated_pawn_val: config_map.get("rmp_isolated_pawn_val").unwrap().parse::<i32>().unwrap(),
            rmp_doubled_pawn_val: config_map.get("rmp_doubled_pawn_val").unwrap().parse::<i32>().unwrap(),
            rmp_behind_pawn_val: config_map.get("rmp_behind_pawn_val").unwrap().parse::<i32>().unwrap(),
            pp_isolated_pawn_val: config_map.get("pp_isolated_pawn_val").unwrap().parse::<i32>().unwrap(),
            pp_doubled_pawn_val: config_map.get("pp_doubled_pawn_val").unwrap().parse::<i32>().unwrap(),
            pp_behind_pawn_val: config_map.get("pp_behind_pawn_val").unwrap().parse::<i32>().unwrap(),
            rpp_isolated_pawn_val: config_map.get("rpp_isolated_pawn_val").unwrap().parse::<i32>().unwrap(),
            rpp_doubled_pawn_val: config_map.get("rpp_doubled_pawn_val").unwrap().parse::<i32>().unwrap(),
            rpp_behind_pawn_val: config_map.get("rpp_behind_pawn_val").unwrap().parse::<i32>().unwrap(),

            mp_passer_base_val: config_map.get("mp_passer_base_val").unwrap().parse::<i32>().unwrap(),
            mp_passer_rank_val: config_map.get("mp_passer_rank_val").unwrap().parse::<i32>().unwrap(),
            mp_candidate_passer_base_val: config_map.get("mp_candidate_passer_base_val").unwrap().parse::<i32>().unwrap(),
            mp_candidate_passer_rank_val: config_map.get("mp_candidate_passer_rank_val").unwrap().parse::<i32>().unwrap(),
            rmp_passer_base_val: config_map.get("rmp_passer_base_val").unwrap().parse::<i32>().unwrap(),
            rmp_passer_rank_val: config_map.get("rmp_passer_rank_val").unwrap().parse::<i32>().unwrap(),
            rmp_candidate_passer_base_val: config_map.get("rmp_candidate_passer_base_val").unwrap().parse::<i32>().unwrap(),
            rmp_candidate_passer_rank_val: config_map.get("rmp_candidate_passer_rank_val").unwrap().parse::<i32>().unwrap(),
            pp_passer_base_val: config_map.get("pp_passer_base_val").unwrap().parse::<i32>().unwrap(),
            pp_passer_rank_val: config_map.get("pp_passer_rank_val").unwrap().parse::<i32>().unwrap(),
            pp_candidate_passer_base_val: config_map.get("pp_candidate_passer_base_val").unwrap().parse::<i32>().unwrap(),
            pp_candidate_passer_rank_val: config_map.get("pp_candidate_passer_rank_val").unwrap().parse::<i32>().unwrap(),
            rpp_passer_base_val: config_map.get("rpp_passer_base_val").unwrap().parse::<i32>().unwrap(),
            rpp_passer_rank_val: config_map.get("rpp_passer_rank_val").unwrap().parse::<i32>().unwrap(),
            rpp_candidate_passer_base_val: config_map.get("rpp_candidate_passer_base_val").unwrap().parse::<i32>().unwrap(),
            rpp_candidate_passer_rank_val: config_map.get("rpp_candidate_passer_rank_val").unwrap().parse::<i32>().unwrap(),

            mp_pk_attack_val: config_map.get("mp_pk_attack_val").unwrap().parse::<i32>().unwrap(),
            mp_nk_attack_val: config_map.get("mp_nk_attack_val").unwrap().parse::<i32>().unwrap(),
            mp_bk_attack_val: config_map.get("mp_bk_attack_val").unwrap().parse::<i32>().unwrap(),
            mp_rk_attack_val: config_map.get("mp_rk_attack_val").unwrap().parse::<i32>().unwrap(),
            mp_qk_attack_val: config_map.get("mp_qk_attack_val").unwrap().parse::<i32>().unwrap(),
            pp_pk_attack_val: config_map.get("pp_pk_attack_val").unwrap().parse::<i32>().unwrap(),
            pp_nk_attack_val: config_map.get("pp_nk_attack_val").unwrap().parse::<i32>().unwrap(),
            pp_bk_attack_val: config_map.get("pp_bk_attack_val").unwrap().parse::<i32>().unwrap(),
            pp_rk_attack_val: config_map.get("pp_rk_attack_val").unwrap().parse::<i32>().unwrap(),
            pp_qk_attack_val: config_map.get("pp_qk_attack_val").unwrap().parse::<i32>().unwrap(),

            mp_king_pawn_protection_val: config_map.get("mp_king_pawn_protection_val").unwrap().parse::<i32>().unwrap(),
            mp_king_front_open_val: config_map.get("mp_king_front_open_val").unwrap().parse::<i32>().unwrap(),
            mp_king_front_total_open_val: config_map.get("mp_king_front_total_open_val").unwrap().parse::<i32>().unwrap(),
            mp_king_front_enery_pawn_cover_val: config_map.get("mp_king_front_enery_pawn_cover_val").unwrap().parse::<i32>().unwrap(),
            mp_king_side_file_open_val: config_map.get("mp_king_side_file_open_val").unwrap().parse::<i32>().unwrap(),
            mp_king_near_side_file_open_val: config_map.get("mp_king_near_side_file_open_val").unwrap().parse::<i32>().unwrap(),

            pp_king_pawn_protection_val: config_map.get("pp_king_pawn_protection_val").unwrap().parse::<i32>().unwrap(),
            pp_king_front_open_val: config_map.get("pp_king_front_open_val").unwrap().parse::<i32>().unwrap(),
            pp_king_front_total_open_val: config_map.get("pp_king_front_total_open_val").unwrap().parse::<i32>().unwrap(),
            pp_king_front_enery_pawn_cover_val: config_map.get("pp_king_front_enery_pawn_cover_val").unwrap().parse::<i32>().unwrap(),
            pp_king_side_file_open_val: config_map.get("pp_king_side_file_open_val").unwrap().parse::<i32>().unwrap(),
            pp_king_near_side_file_open_val: config_map.get("pp_king_near_side_file_open_val").unwrap().parse::<i32>().unwrap(),

            mp_king_cas_rights_val: config_map.get("mp_king_cas_rights_val").unwrap().parse::<i32>().unwrap(),
            rmp_king_cas_rights_val: config_map.get("rmp_king_cas_rights_val").unwrap().parse::<i32>().unwrap(),
            pp_king_cas_rights_val: config_map.get("pp_king_cas_rights_val").unwrap().parse::<i32>().unwrap(),
            rpp_king_cas_rights_val: config_map.get("rpp_king_cas_rights_val").unwrap().parse::<i32>().unwrap(),

            mp_rook_open_val: config_map.get("mp_rook_open_val").unwrap().parse::<i32>().unwrap(),
            mp_rook_semi_open_val: config_map.get("mp_rook_semi_open_val").unwrap().parse::<i32>().unwrap(),
            pp_rook_open_val: config_map.get("pp_rook_open_val").unwrap().parse::<i32>().unwrap(),
            pp_rook_semi_open_val: config_map.get("pp_rook_semi_open_val").unwrap().parse::<i32>().unwrap(),

            mp_pin_val: config_map.get("mp_pin_val").unwrap().parse::<i32>().unwrap(),
            mp_semi_pin_val: config_map.get("mp_semi_pin_val").unwrap().parse::<i32>().unwrap(),
            pp_pin_val: config_map.get("pp_pin_val").unwrap().parse::<i32>().unwrap(),
            pp_semi_pin_val: config_map.get("pp_semi_pin_val").unwrap().parse::<i32>().unwrap(),

            mp_unprotected_sqr_val: config_map.get("mp_unprotected_sqr_val").unwrap().parse::<i32>().unwrap(),
            mp_under_attacked_sqr_val: config_map.get("mp_under_attacked_sqr_val").unwrap().parse::<i32>().unwrap(),
            pp_unprotected_sqr_val: config_map.get("pp_unprotected_sqr_val").unwrap().parse::<i32>().unwrap(),
            pp_under_attacked_sqr_val: config_map.get("pp_under_attacked_sqr_val").unwrap().parse::<i32>().unwrap(),

            mp_protected_p_val: config_map.get("mp_protected_p_val").unwrap().parse::<i32>().unwrap(),
            mp_protected_n_val: config_map.get("mp_protected_n_val").unwrap().parse::<i32>().unwrap(),
            mp_protected_b_val: config_map.get("mp_protected_b_val").unwrap().parse::<i32>().unwrap(),
            mp_protected_r_val: config_map.get("mp_protected_r_val").unwrap().parse::<i32>().unwrap(),
            pp_protected_p_val: config_map.get("pp_protected_p_val").unwrap().parse::<i32>().unwrap(),
            pp_protected_n_val: config_map.get("pp_protected_n_val").unwrap().parse::<i32>().unwrap(),
            pp_protected_b_val: config_map.get("pp_protected_b_val").unwrap().parse::<i32>().unwrap(),
            pp_protected_r_val: config_map.get("pp_protected_r_val").unwrap().parse::<i32>().unwrap(),

            mp_np_protected_3rd_rank_sqr_val: config_map.get("mp_np_protected_3rd_rank_sqr_val").unwrap().parse::<i32>().unwrap(),
            mp_np_protected_4th_rank_sqr_val: config_map.get("mp_np_protected_4th_rank_sqr_val").unwrap().parse::<i32>().unwrap(),
            pp_np_protected_3rd_rank_sqr_val: config_map.get("pp_np_protected_3rd_rank_sqr_val").unwrap().parse::<i32>().unwrap(),
            pp_np_protected_4th_rank_sqr_val: config_map.get("pp_np_protected_4th_rank_sqr_val").unwrap().parse::<i32>().unwrap(),

            mp_n_mob_base_val: config_map.get("mp_n_mob_base_val").unwrap().parse::<i32>().unwrap(),
            mp_b_mob_base_val: config_map.get("mp_b_mob_base_val").unwrap().parse::<i32>().unwrap(),
            mp_r_mob_base_val: config_map.get("mp_r_mob_base_val").unwrap().parse::<i32>().unwrap(),
            mp_q_mob_base_val: config_map.get("mp_q_mob_base_val").unwrap().parse::<i32>().unwrap(),
            pp_n_mob_base_val: config_map.get("pp_n_mob_base_val").unwrap().parse::<i32>().unwrap(),
            pp_b_mob_base_val: config_map.get("pp_b_mob_base_val").unwrap().parse::<i32>().unwrap(),
            pp_r_mob_base_val: config_map.get("pp_r_mob_base_val").unwrap().parse::<i32>().unwrap(),
            pp_q_mob_base_val: config_map.get("pp_q_mob_base_val").unwrap().parse::<i32>().unwrap(),

            mp_n_weak_mob_base_val: config_map.get("mp_n_weak_mob_base_val").unwrap().parse::<i32>().unwrap(),
            mp_b_weak_mob_base_val: config_map.get("mp_b_weak_mob_base_val").unwrap().parse::<i32>().unwrap(),
            mp_r_weak_mob_base_val: config_map.get("mp_r_weak_mob_base_val").unwrap().parse::<i32>().unwrap(),
            mp_q_weak_mob_base_val: config_map.get("mp_q_weak_mob_base_val").unwrap().parse::<i32>().unwrap(),

            pp_n_weak_mob_base_val: config_map.get("pp_n_weak_mob_base_val").unwrap().parse::<i32>().unwrap(),
            pp_b_weak_mob_base_val: config_map.get("pp_b_weak_mob_base_val").unwrap().parse::<i32>().unwrap(),
            pp_r_weak_mob_base_val: config_map.get("pp_r_weak_mob_base_val").unwrap().parse::<i32>().unwrap(),
            pp_q_weak_mob_base_val: config_map.get("pp_q_weak_mob_base_val").unwrap().parse::<i32>().unwrap(),

            mp_n_stuck_val: config_map.get("mp_n_stuck_val").unwrap().parse::<i32>().unwrap(),
            mp_b_stuck_val: config_map.get("mp_b_stuck_val").unwrap().parse::<i32>().unwrap(),
            mp_r_stuck_val: config_map.get("mp_r_stuck_val").unwrap().parse::<i32>().unwrap(),
            mp_q_stuck_val: config_map.get("mp_q_stuck_val").unwrap().parse::<i32>().unwrap(),
            pp_n_stuck_val: config_map.get("pp_n_stuck_val").unwrap().parse::<i32>().unwrap(),
            pp_b_stuck_val: config_map.get("pp_b_stuck_val").unwrap().parse::<i32>().unwrap(),
            pp_r_stuck_val: config_map.get("pp_r_stuck_val").unwrap().parse::<i32>().unwrap(),
            pp_q_stuck_val: config_map.get("pp_q_stuck_val").unwrap().parse::<i32>().unwrap(),

            mp_tempo_val: config_map.get("mp_tempo_val").unwrap().parse::<i32>().unwrap(),
            pp_tempo_val: config_map.get("pp_tempo_val").unwrap().parse::<i32>().unwrap(),

            mp_pawn_essential_val: config_map.get("mp_pawn_essential_val").unwrap().parse::<i32>().unwrap(),
            pp_pawn_essential_val: config_map.get("pp_pawn_essential_val").unwrap().parse::<i32>().unwrap(),

            mp_different_color_bishop_val: config_map.get("mp_different_color_bishop_val").unwrap().parse::<i32>().unwrap(),
            mp_different_color_bishop_with_rook_val: config_map.get("mp_different_color_bishop_with_rook_val").unwrap().parse::<i32>().unwrap(),
            pp_different_color_bishop_val: config_map.get("pp_different_color_bishop_val").unwrap().parse::<i32>().unwrap(),
            pp_different_color_bishop_with_rook_val: config_map.get("pp_different_color_bishop_with_rook_val").unwrap().parse::<i32>().unwrap(),

            mp_bishop_pair_val: config_map.get("mp_bishop_pair_val").unwrap().parse::<i32>().unwrap(),
            pp_bishop_pair_val: config_map.get("pp_bishop_pair_val").unwrap().parse::<i32>().unwrap(),
        }
    }
}
