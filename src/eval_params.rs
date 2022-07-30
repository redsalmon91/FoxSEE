use std::collections::HashMap;

#[derive(Debug)]
pub struct EvalParams {
    pub q_val: i32,
    pub r_val: i32,
    pub b_val: i32,
    pub n_val: i32,
    pub p_val: i32,

    pub eg_q_val: i32,
    pub eg_r_val: i32,
    pub eg_b_val: i32,
    pub eg_n_val: i32,
    pub eg_p_val: i32,

    pub isolated_pawn_pen: i32,
    pub doubled_pawn_pen: i32,
    pub behind_pawn_pen: i32,

    pub passer_base_val: i32,
    pub passer_rank_bonus: i32,

    pub candidate_passer_base_val: i32,
    pub candidate_passer_rank_bonus: i32,

    pub k_attack_score: i32,
    pub k_attack_ignore_base: i32,
    pub nk_attack_weight: i32,
    pub bk_attack_weight: i32,
    pub rk_attack_weight: i32,
    pub qk_attack_weight: i32,
    pub k_exposure_pen: i32,
    pub k_no_cas_rights_pen: i32,

    pub rook_open_bonus: i32,
    
    pub weak_sqr_pen: i32,

    pub pin_pen: i32,
    pub semi_pin_pen: i32,

    pub p_mob_score: i32,
    pub n_mob_score: i32,
    pub b_mob_score: i32,
    pub r_mob_score: i32,
    pub q_mob_score: i32,

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
            q_val: 1456,
            r_val: 732,
            b_val: 500,
            n_val: 490,
            p_val: 120,
        
            eg_q_val: 98,
            eg_r_val: 32,
            eg_b_val: 30,
            eg_n_val: 44,
            eg_p_val: 16,

            isolated_pawn_pen: -12,
            doubled_pawn_pen: -20,
            behind_pawn_pen: -10,
        
            passer_base_val: 32,
            passer_rank_bonus: 38,
        
            candidate_passer_base_val: 20,
            candidate_passer_rank_bonus: 15,
        
            k_attack_score: 5,
            k_attack_ignore_base: 8,
            nk_attack_weight: 3,
            bk_attack_weight: 3,
            rk_attack_weight: 5,
            qk_attack_weight: 8,
            k_exposure_pen: -20,
            k_no_cas_rights_pen: -50,

            rook_open_bonus: 26,
            
            weak_sqr_pen: -5,

            pin_pen: -20,
            semi_pin_pen: -10,
        
            p_mob_score: 5,
            n_mob_score: 5,
            b_mob_score: 5,
            r_mob_score: 5,
            q_mob_score: 5,
        
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

            eg_q_val: config_map.get("eg_q_val").unwrap().parse::<i32>().unwrap(),
            eg_r_val: config_map.get("eg_r_val").unwrap().parse::<i32>().unwrap(),
            eg_b_val: config_map.get("eg_b_val").unwrap().parse::<i32>().unwrap(),
            eg_n_val: config_map.get("eg_n_val").unwrap().parse::<i32>().unwrap(),
            eg_p_val: config_map.get("eg_p_val").unwrap().parse::<i32>().unwrap(),

            isolated_pawn_pen: config_map.get("isolated_pawn_pen").unwrap().parse::<i32>().unwrap(),
            doubled_pawn_pen: config_map.get("doubled_pawn_pen").unwrap().parse::<i32>().unwrap(),
            behind_pawn_pen: config_map.get("behind_pawn_pen").unwrap().parse::<i32>().unwrap(),

            passer_base_val: config_map.get("passer_base_val").unwrap().parse::<i32>().unwrap(),
            passer_rank_bonus: config_map.get("passer_rank_bonus").unwrap().parse::<i32>().unwrap(),

            candidate_passer_base_val: config_map.get("candidate_passer_base_val").unwrap().parse::<i32>().unwrap(),
            candidate_passer_rank_bonus: config_map.get("candidate_passer_rank_bonus").unwrap().parse::<i32>().unwrap(),

            k_attack_score: config_map.get("k_attack_score").unwrap().parse::<i32>().unwrap(),
            k_attack_ignore_base: config_map.get("k_attack_ignore_base").unwrap().parse::<i32>().unwrap(),
            nk_attack_weight: config_map.get("nk_attack_weight").unwrap().parse::<i32>().unwrap(),
            bk_attack_weight: config_map.get("bk_attack_weight").unwrap().parse::<i32>().unwrap(),
            rk_attack_weight: config_map.get("rk_attack_weight").unwrap().parse::<i32>().unwrap(),
            qk_attack_weight: config_map.get("qk_attack_weight").unwrap().parse::<i32>().unwrap(),
            k_exposure_pen: config_map.get("k_exposure_pen").unwrap().parse::<i32>().unwrap(),
            k_no_cas_rights_pen: config_map.get("k_no_cas_rights_pen").unwrap().parse::<i32>().unwrap(),

            rook_open_bonus: config_map.get("rook_open_bonus").unwrap().parse::<i32>().unwrap(),

            weak_sqr_pen: config_map.get("weak_sqr_pen").unwrap().parse::<i32>().unwrap(),

            pin_pen: config_map.get("pin_pen").unwrap().parse::<i32>().unwrap(),
            semi_pin_pen: config_map.get("semi_pin_pen").unwrap().parse::<i32>().unwrap(),

            p_mob_score: config_map.get("p_mob_score").unwrap().parse::<i32>().unwrap(),
            n_mob_score: config_map.get("n_mob_score").unwrap().parse::<i32>().unwrap(),
            b_mob_score: config_map.get("b_mob_score").unwrap().parse::<i32>().unwrap(),
            r_mob_score: config_map.get("r_mob_score").unwrap().parse::<i32>().unwrap(),
            q_mob_score: config_map.get("q_mob_score").unwrap().parse::<i32>().unwrap(),

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
