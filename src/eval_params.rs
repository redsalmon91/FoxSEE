use std::collections::HashMap;

#[derive(Debug)]
pub struct EvalParams {
    pub q_val: i32,
    pub r_val: i32,
    pub b_val: i32,
    pub n_val: i32,
    pub p_val: i32,

    pub mg_isolated_pawn_pen: i32,
    pub mg_doubled_pawn_pen: i32,
    pub mg_behind_pawn_pen: i32,

    pub eg_isolated_pawn_pen: i32,
    pub eg_doubled_pawn_pen: i32,
    pub eg_behind_pawn_pen: i32,

    pub mg_passer_base_val: i32,
    pub mg_passer_rank_bonus: i32,
    pub eg_passer_base_val: i32,
    pub eg_passer_rank_bonus: i32,

    pub mg_candidate_passer_base_val: i32,
    pub mg_candidate_passer_rank_bonus: i32,
    pub eg_candidate_passer_base_val: i32,
    pub eg_candidate_passer_rank_bonus: i32,

    pub k_attack_score: i32,
    pub k_attack_ignore_base: i32,
    pub nk_attack_weight: i32,
    pub bk_attack_weight: i32,
    pub rk_attack_weight: i32,
    pub qk_attack_weight: i32,
    pub k_exposure_pen: i32,
    pub k_no_cas_rights_pen: i32,

    pub rook_open_bonus: i32,
    pub rook_semi_open_bonus: i32,

    pub pin_pen: i32,
    pub semi_pin_pen: i32,

    pub unprotected_sqr_pen: i32,
    pub under_attacked_sqr_pen: i32,

    pub protected_p_val: i32,
    pub protected_n_val: i32,
    pub protected_b_val: i32,
    pub protected_r_val: i32,

    pub np_protected_3rd_rank_sqr_pen: i32, 
    pub np_protected_4th_rank_sqr_pen: i32, 

    pub p_mob_score: i32,
    pub n_mob_score: i32,
    pub b_mob_score: i32,
    pub r_mob_score: i32,
    pub q_mob_score: i32,

    pub n_mob_offset_index: i32,
    pub b_mob_offset_index: i32,
    pub r_mob_offset_index: i32,
    pub q_mob_offset_index: i32,

    pub n_mob_zero_pen: i32,
    pub b_mob_zero_pen: i32,
    pub r_mob_zero_pen: i32,
    pub q_mob_zero_pen: i32,

    pub threat_discount_factor: i32,

    pub tempo_val: i32,

    pub eg_pawn_essential_val: i32,
    pub eg_different_color_bishop_val: i32,
    pub eg_different_color_bishop_with_rook_val: i32,
    pub eg_bishop_pair_bonus: i32,
    pub eg_rn_knight_protected_bonus: i32,
    pub eg_king_in_passer_path_bonus: i32,
    pub eg_controlled_passer_val: i32,
    pub eg_no_piece_bonus: i32,
}

impl EvalParams {
    pub const fn default() -> Self {
        EvalParams {
            q_val: 1400,
            r_val: 732,
            b_val: 500,
            n_val: 490,
            p_val: 120,

            mg_isolated_pawn_pen: 0,
            mg_doubled_pawn_pen: 0,
            mg_behind_pawn_pen: -10,

            eg_isolated_pawn_pen: -12,
            eg_doubled_pawn_pen: -20,
            eg_behind_pawn_pen: 0,

            mg_passer_base_val: 0,
            mg_passer_rank_bonus: 0,
            eg_passer_base_val: 32,
            eg_passer_rank_bonus: 38,

            mg_candidate_passer_base_val: 0,
            mg_candidate_passer_rank_bonus: 0,
            eg_candidate_passer_base_val: 20,
            eg_candidate_passer_rank_bonus: 15,

            k_attack_score: 5,
            k_attack_ignore_base: 3,
            nk_attack_weight: 3,
            bk_attack_weight: 3,
            rk_attack_weight: 5,
            qk_attack_weight: 8,
            k_exposure_pen: -20,
            k_no_cas_rights_pen: -50,

            rook_open_bonus: 26,
            rook_semi_open_bonus: 10,

            pin_pen: -20,
            semi_pin_pen: -10,

            unprotected_sqr_pen: -1,
            under_attacked_sqr_pen: -1,

            protected_p_val: 1,
            protected_n_val: 1,
            protected_b_val: 1,
            protected_r_val: 1,

            np_protected_3rd_rank_sqr_pen: -4,
            np_protected_4th_rank_sqr_pen: -2,

            p_mob_score: 5,
            n_mob_score: 5,
            b_mob_score: 5,
            r_mob_score: 5,
            q_mob_score: 5,

            n_mob_offset_index: 3,
            b_mob_offset_index: 3,
            r_mob_offset_index: 3,
            q_mob_offset_index: 3,

            n_mob_zero_pen: -20,
            b_mob_zero_pen: -20,
            r_mob_zero_pen: -50,
            q_mob_zero_pen: -50,

            threat_discount_factor: 12,

            tempo_val: 20,

            eg_pawn_essential_val: 190,
            eg_different_color_bishop_val: 90,
            eg_different_color_bishop_with_rook_val: 50,
            eg_bishop_pair_bonus: 50,
            eg_rn_knight_protected_bonus: 50,
            eg_king_in_passer_path_bonus: 50,
            eg_controlled_passer_val: 50,
            eg_no_piece_bonus: 50,
        }
    }

    pub fn from_config(config_map: &HashMap<String, String>) -> Self {
        EvalParams {
            q_val: config_map.get("q_val").unwrap().parse::<i32>().unwrap(),
            r_val: config_map.get("r_val").unwrap().parse::<i32>().unwrap(),
            b_val: config_map.get("b_val").unwrap().parse::<i32>().unwrap(),
            n_val: config_map.get("n_val").unwrap().parse::<i32>().unwrap(),
            p_val: config_map.get("p_val").unwrap().parse::<i32>().unwrap(),

            mg_isolated_pawn_pen: config_map.get("mg_isolated_pawn_pen").unwrap().parse::<i32>().unwrap(),
            mg_doubled_pawn_pen: config_map.get("mg_doubled_pawn_pen").unwrap().parse::<i32>().unwrap(),
            mg_behind_pawn_pen: config_map.get("mg_behind_pawn_pen").unwrap().parse::<i32>().unwrap(),

            eg_isolated_pawn_pen: config_map.get("eg_isolated_pawn_pen").unwrap().parse::<i32>().unwrap(),
            eg_doubled_pawn_pen: config_map.get("eg_doubled_pawn_pen").unwrap().parse::<i32>().unwrap(),
            eg_behind_pawn_pen: config_map.get("eg_behind_pawn_pen").unwrap().parse::<i32>().unwrap(),

            mg_passer_base_val: config_map.get("mg_passer_base_val").unwrap().parse::<i32>().unwrap(),
            mg_passer_rank_bonus: config_map.get("mg_passer_rank_bonus").unwrap().parse::<i32>().unwrap(),
            eg_passer_base_val: config_map.get("eg_passer_base_val").unwrap().parse::<i32>().unwrap(),
            eg_passer_rank_bonus: config_map.get("eg_passer_rank_bonus").unwrap().parse::<i32>().unwrap(),

            mg_candidate_passer_base_val: config_map.get("mg_candidate_passer_base_val").unwrap().parse::<i32>().unwrap(),
            mg_candidate_passer_rank_bonus: config_map.get("mg_candidate_passer_rank_bonus").unwrap().parse::<i32>().unwrap(),
            eg_candidate_passer_base_val: config_map.get("eg_candidate_passer_base_val").unwrap().parse::<i32>().unwrap(),
            eg_candidate_passer_rank_bonus: config_map.get("eg_candidate_passer_rank_bonus").unwrap().parse::<i32>().unwrap(),

            k_attack_score: config_map.get("k_attack_score").unwrap().parse::<i32>().unwrap(),
            k_attack_ignore_base: config_map.get("k_attack_ignore_base").unwrap().parse::<i32>().unwrap(),
            nk_attack_weight: config_map.get("nk_attack_weight").unwrap().parse::<i32>().unwrap(),
            bk_attack_weight: config_map.get("bk_attack_weight").unwrap().parse::<i32>().unwrap(),
            rk_attack_weight: config_map.get("rk_attack_weight").unwrap().parse::<i32>().unwrap(),
            qk_attack_weight: config_map.get("qk_attack_weight").unwrap().parse::<i32>().unwrap(),
            k_exposure_pen: config_map.get("k_exposure_pen").unwrap().parse::<i32>().unwrap(),
            k_no_cas_rights_pen: config_map.get("k_no_cas_rights_pen").unwrap().parse::<i32>().unwrap(),

            rook_open_bonus: config_map.get("rook_open_bonus").unwrap().parse::<i32>().unwrap(),
            rook_semi_open_bonus: config_map.get("rook_semi_open_bonus").unwrap().parse::<i32>().unwrap(),

            pin_pen: config_map.get("pin_pen").unwrap().parse::<i32>().unwrap(),
            semi_pin_pen: config_map.get("semi_pin_pen").unwrap().parse::<i32>().unwrap(),

            unprotected_sqr_pen: config_map.get("unprotected_sqr_pen").unwrap().parse::<i32>().unwrap(),
            under_attacked_sqr_pen: config_map.get("under_attacked_sqr_pen").unwrap().parse::<i32>().unwrap(),

            protected_p_val: config_map.get("protected_p_val").unwrap().parse::<i32>().unwrap(),
            protected_n_val: config_map.get("protected_n_val").unwrap().parse::<i32>().unwrap(),
            protected_b_val: config_map.get("protected_b_val").unwrap().parse::<i32>().unwrap(),
            protected_r_val: config_map.get("protected_r_val").unwrap().parse::<i32>().unwrap(),

            np_protected_3rd_rank_sqr_pen: config_map.get("np_protected_3rd_rank_sqr_pen").unwrap().parse::<i32>().unwrap(),
            np_protected_4th_rank_sqr_pen: config_map.get("np_protected_4th_rank_sqr_pen").unwrap().parse::<i32>().unwrap(),

            p_mob_score: config_map.get("p_mob_score").unwrap().parse::<i32>().unwrap(),
            n_mob_score: config_map.get("n_mob_score").unwrap().parse::<i32>().unwrap(),
            b_mob_score: config_map.get("b_mob_score").unwrap().parse::<i32>().unwrap(),
            r_mob_score: config_map.get("r_mob_score").unwrap().parse::<i32>().unwrap(),
            q_mob_score: config_map.get("q_mob_score").unwrap().parse::<i32>().unwrap(),

            n_mob_offset_index: config_map.get("n_mob_offset_index").unwrap().parse::<i32>().unwrap(),
            b_mob_offset_index: config_map.get("b_mob_offset_index").unwrap().parse::<i32>().unwrap(),
            r_mob_offset_index: config_map.get("r_mob_offset_index").unwrap().parse::<i32>().unwrap(),
            q_mob_offset_index: config_map.get("q_mob_offset_index").unwrap().parse::<i32>().unwrap(),

            n_mob_zero_pen: config_map.get("n_mob_zero_pen").unwrap().parse::<i32>().unwrap(),
            b_mob_zero_pen: config_map.get("b_mob_zero_pen").unwrap().parse::<i32>().unwrap(),
            r_mob_zero_pen: config_map.get("r_mob_zero_pen").unwrap().parse::<i32>().unwrap(),
            q_mob_zero_pen: config_map.get("q_mob_zero_pen").unwrap().parse::<i32>().unwrap(),

            threat_discount_factor: config_map.get("threat_discount_factor").unwrap().parse::<i32>().unwrap(),

            tempo_val: config_map.get("tempo_val").unwrap().parse::<i32>().unwrap(),

            eg_pawn_essential_val: config_map.get("eg_pawn_essential_val").unwrap().parse::<i32>().unwrap(),
            eg_different_color_bishop_val: config_map.get("eg_different_color_bishop_val").unwrap().parse::<i32>().unwrap(),
            eg_different_color_bishop_with_rook_val: config_map.get("eg_different_color_bishop_with_rook_val").unwrap().parse::<i32>().unwrap(),
            eg_bishop_pair_bonus: config_map.get("eg_bishop_pair_bonus").unwrap().parse::<i32>().unwrap(),
            eg_rn_knight_protected_bonus: config_map.get("eg_rn_knight_protected_bonus").unwrap().parse::<i32>().unwrap(),
            eg_king_in_passer_path_bonus: config_map.get("eg_king_in_passer_path_bonus").unwrap().parse::<i32>().unwrap(),
            eg_controlled_passer_val: config_map.get("eg_controlled_passer_val").unwrap().parse::<i32>().unwrap(),
            eg_no_piece_bonus: config_map.get("eg_no_piece_bonus").unwrap().parse::<i32>().unwrap(),
        }
    }
}
