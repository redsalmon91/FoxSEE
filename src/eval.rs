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
pub const EQUAL_EXCHANGE: i32 = -20;

const EG_P_SQR_DIFF_MULTIPLIER: i32 = 2;

const TOTAL_PHASE: i32 = 96;
const Q_PHASE_WEIGHT: i32 = 16;
const R_PHASE_WEIGHT: i32 = 8;
const B_PHASE_WEIGHT: i32 = 4;
const N_PHASE_WEIGHT: i32 = 4;
const EG_PHASE: i32 = 32;

const SQR_TABLE_BP: [i32; def::BOARD_SIZE] = [
      0,  0,  0,  0,  0,  0,  0,  0,
     15, 30, 30, 30, 30, 30, 30, 15,
     10, 20, 20, 30, 30, 20, 20, 10,
      0,  0,  0, 20, 20,  0,  0,  0,
      0,  0,  0, 20, 20,  0,  0,  0,
     10, 10,  0,  0,  0,  0, 10, 10,
     10, 10,  5,  0,  0,  5, 10, 10,
      0,  0,  0,  0,  0,  0,  0,  0,
];

const SQR_TABLE_BP_ENDGAME: [i32; def::BOARD_SIZE] = [
      0,  0,  0,  0,  0,  0,  0,  0,
     20, 30, 30, 30, 30, 30, 30, 20,
     15, 20, 20, 20, 20, 20, 20, 15,
     10, 15, 15, 15, 15, 15, 15, 10,
      5, 10, 10, 10, 10, 10, 10,  5,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
];

const SQR_TABLE_WP: [i32; def::BOARD_SIZE] = [
      0,  0,  0,  0,  0,  0,  0,  0,
     10, 10,  5,  0,  0,  5, 10, 10,
     10, 10,  0,  0,  0,  0, 10, 10,
      0,  0,  0, 20, 20,  0,  0,  0,
      0,  0,  0, 20, 20,  0,  0,  0,
     10, 20, 20, 30, 30, 20, 20, 10,
     15, 30, 30, 30, 30, 30, 30, 15,
      0,  0,  0,  0,  0,  0,  0,  0,
];

const SQR_TABLE_WP_ENDGAME: [i32; def::BOARD_SIZE] = [
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,
      5, 10, 10, 10, 10, 10, 10,  5,
     10, 15, 15, 15, 15, 15, 15, 10,
     15, 20, 20, 20, 20, 20, 20, 15,
     20, 30, 30, 30, 30, 30, 30, 20,
      0,  0,  0,  0,  0,  0,  0,  0,
];

const SQR_TABLE_BN: [i32; def::BOARD_SIZE] = [
    -60,-30,-20,-20,-20,-20,-30,-60,
    -30,-30, 20, 10, 10, 20,-30,-30,
    -20, 10, 15, 20, 20, 15, 10,-20,
    -20,  5,  0, 20, 20,  0,  5,-20,
    -20,  0,  0, 20, 20,  0,  0,-20,
    -20,  5, 10,  0,  0, 10,  5,-20,
    -30,-30,  0,  0,  0,  0,-30,-30,
    -60,-30,-20,-20,-20,-20,-30,-60,
];

const SQR_TABLE_WN: [i32; def::BOARD_SIZE] = [
    -60,-30,-20,-20,-20,-20,-30,-60,
    -30,-30,  0,  0,  0,  0,-30,-30,
    -20,  5, 10,  0,  0, 10,  5,-20,
    -20,  0,  0, 20, 20,  0,  0,-20,
    -20,  5,  0, 20, 20,  0,  5,-20,
    -20, 10, 15, 20, 20, 15, 10,-20,
    -30,-30, 20, 10, 10, 20,-30,-30,
    -60,-30,-20,-20,-20,-20,-30,-60,
];

const SQR_TABLE_BB: [i32; def::BOARD_SIZE] = [
    -50,-10,-10,-10,-10,-10,-10,-50,
    -20,  0, 10,  0,  0, 10,  0,-20,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  5,  0, 10, 10,  0,  5,-10,
    -10,  0,  0, 10, 10,  0,  0,-10,
    -10,  5,  5,  5,  5,  5,  5,-10,
    -10, 10,  0,  0,  0,  0, 10,-10,
    -50,-10,-10,-10,-10,-10,-10,-50,
];

const SQR_TABLE_WB: [i32; def::BOARD_SIZE] = [
    -50,-10,-10,-10,-10,-10,-10,-50,
    -10, 10,  0,  0,  0,  0, 10,-10,
    -10,  5,  5,  5,  5,  5,  5,-10,
    -10,  0,  0, 10, 10,  0,  0,-10,
    -10,  5,  0, 10, 10,  0,  5,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -20,  0, 10,  0,  0, 10,  0,-20,
    -50,-10,-10,-10,-10,-10,-10,-50,
];

const SQR_TABLE_BR: [i32; def::BOARD_SIZE] = [
     10, 10, 20, 20, 20, 20, 10, 10,
     10, 20, 30, 30, 30, 30, 20, 10,
     -5, -5, -5, -5, -5, -5, -5, -5,
     -5, -5, -5, -5, -5, -5, -5, -5,
     -5, -5, -5, -5, -5, -5, -5, -5,
     -5, -5, -5, -5, -5, -5, -5, -5,
    -10, -5, -5, -5, -5, -5, -5,-10,
     -5,  0,  0,  5,  5,  0,  0, -5,
];

const SQR_TABLE_WR: [i32; def::BOARD_SIZE] = [
     -5,  0,  0,  5,  5,  0,  0, -5,
    -10, -5, -5, -5, -5, -5, -5,-10,
     -5, -5, -5, -5, -5, -5, -5, -5,
     -5, -5, -5, -5, -5, -5, -5, -5,
     -5, -5, -5, -5, -5, -5, -5, -5,
     -5, -5, -5, -5, -5, -5, -5, -5,
     10, 20, 30, 30, 30, 30, 20, 10,
     10, 10, 20, 20, 20, 20, 10, 10,
];

const SQR_TABLE_BQ: [i32; def::BOARD_SIZE] = [
    -30,-20,-10, -5, -5,-10,-20,-30,
    -20,-10,  0,  0,  0,  0,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -20,-10,  0,  0,  0,  0,-10,-20,
    -30,-20,-10, -5, -5,-10,-20,-30,
];

const SQR_TABLE_WQ: [i32; def::BOARD_SIZE] = [
    -30,-20,-10, -5, -5,-10,-20,-30,
    -20,-10,  0,  0,  0,  0,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -20,-10,  0,  0,  0,  0,-10,-20,
    -30,-20,-10, -5, -5,-10,-20,-30,
];

const SQR_TABLE_BK: [i32; def::BOARD_SIZE] = [
    -90,-90,-90,-90,-90,-90,-90,-90,
    -90,-90,-90,-90,-90,-90,-90,-90,
    -90,-90,-90,-90,-90,-90,-90,-90,
    -90,-90,-90,-90,-90,-90,-90,-90,
    -90,-90,-90,-90,-90,-90,-90,-90,
    -60,-60,-60,-60,-60,-60,-60,-60,
     10, 10,-10,-30,-30,-10, 10, 10,
     10, 20,  0,-20,-20,  0, 20, 10,
];

const SQR_TABLE_WK: [i32; def::BOARD_SIZE] = [
     10, 20,  0,-20,-20,  0, 20, 10,
     10, 10,-10,-30,-30,-10, 10, 10,
    -60,-60,-60,-60,-60,-60,-60,-60,
    -90,-90,-90,-90,-90,-90,-90,-90,
    -90,-90,-90,-90,-90,-90,-90,-90,
    -90,-90,-90,-90,-90,-90,-90,-90,
    -90,-90,-90,-90,-90,-90,-90,-90,
    -90,-90,-90,-90,-90,-90,-90,-90,
];

const SQR_TABLE_K_ENDGAME: [i32; def::BOARD_SIZE] = [
    -50,-40,-30,-20,-20,-30,-40,-50,
    -40,-20,-20,-20,-20,-20,-20,-40,
    -30,-20,  0,  0,  0,  0,-20,-30,
    -30,-20,  0, 10, 10,  0,-20,-30,
    -30,-20,  0, 10, 10,  0,-20,-30,
    -30,-20,  0,  0,  0,  0,-20,-30,
    -30,-20,-20,-20,-20,-20,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

const W_PAWN_PROMO_RANK: u64 = 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
const B_PAWN_PROMO_RANK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;

const WK_K_SIDE_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_11100000_11100000_11100000;
const WK_Q_SIDE_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000111_00000111_00000111;
const BK_K_SIDE_MASK: u64 = 0b11100000_11100000_11100000_00000000_00000000_00000000_00000000_00000000;
const BK_Q_SIDE_MASK: u64 = 0b00000111_00000111_00000111_00000000_00000000_00000000_00000000_00000000;

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

pub fn get_phase(state: &mut State) -> i32 {
    (state.wq_count + state.bq_count) * Q_PHASE_WEIGHT
    + (state.wr_count + state.br_count) * R_PHASE_WEIGHT
    + (state.wb_count + state.bb_count) * B_PHASE_WEIGHT
    + (state.wn_count + state.bn_count) * N_PHASE_WEIGHT
}

pub fn has_promoting_pawn(state: &State, player: u8) -> bool {
    if player == def::PLAYER_W {
        state.bitboard.w_pawn & W_PAWN_PROMO_RANK != 0
    } else {
        state.bitboard.b_pawn & B_PAWN_PROMO_RANK != 0
    }
}

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

    king_attack_count: i32,
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

            king_attack_count: 0,
            king_exposure_count: 0,
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
                return (0, true)
            }
    
            if (bitboard.w_bishop | bitboard.w_knight) == 0 && bitboard.b_bishop == 0 && b_knight_count < 3 {
                return (0, true)
            }
    
            if (bitboard.b_bishop | bitboard.b_knight) == 0 && bitboard.w_bishop == 0 && w_knight_count < 3 {
                return (0, true)
            }
        }
    
        let material_score = (w_queen_count - b_queen_count) * self.params.q_val
            + (w_rook_count - b_rook_count) * self.params.r_val
            + (w_bishop_count - b_bishop_count) * self.params.b_val
            + (w_knight_count - b_knight_count) * self.params.n_val
            + (w_pawn_count - b_pawn_count) * self.params.p_val;
    
        if material_score > 0 && (bitboard.w_pawn | bitboard.w_rook | bitboard.w_queen) == 0 && w_knight_count + w_bishop_count == 1 && b_pawn_count == 1 {
            return (0, false)
        }
    
        if material_score < 0 && (bitboard.b_pawn | bitboard.b_rook | bitboard.b_queen) == 0 && b_knight_count + b_bishop_count == 1 && w_pawn_count == 1 {
            return (0, false)
        }
    
        let mut eg_score = 0;
    
        eg_score += (w_queen_count - b_queen_count) * self.params.eg_q_val;
        eg_score += (w_rook_count - b_rook_count) * self.params.eg_r_val;
        eg_score += (w_bishop_count - b_bishop_count) * self.params.eg_b_val;
        eg_score += (w_knight_count - w_knight_count) * self.params.eg_n_val;
        eg_score += (w_pawn_count - b_pawn_count) * self.params.eg_p_val;
    
        if material_score > self.params.p_val && bitboard.w_pawn == 0 {
            eg_score -= self.params.eg_pawn_essential_val;
        }
    
        if material_score < -self.params.p_val && bitboard.b_pawn == 0 {
            eg_score += self.params.eg_pawn_essential_val;
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
                if material_score > 0  {
                    eg_score -= self.params.eg_different_color_bishop_val;
                } else if material_score < 0 {
                    eg_score += self.params.eg_different_color_bishop_val;
                }
            } else {
                if material_score > 0  {
                    eg_score -= self.params.eg_different_color_bishop_with_rook_val;
                } else if material_score < 0 {
                    eg_score += self.params.eg_different_color_bishop_with_rook_val;
                }
            }
        } else {
            if w_bishop_count > 1 {
                eg_score += self.params.eg_bishop_pair_bonus;
            }
    
            if b_bishop_count > 1 {
                eg_score -= self.params.eg_bishop_pair_bonus;
            }
        }
    
        let phase = get_phase(state);
    
        if phase == 0 {
            eg_score += (w_pawn_count - b_pawn_count) * self.params.eg_no_piece_bonus;
        }
    
        let score_sign = if state.player == def::PLAYER_W {
            1
        } else {
            -1
        };
    
        ((material_score + eg_score * (TOTAL_PHASE - phase) / TOTAL_PHASE) * score_sign, false)
    }

    pub fn eval_state(&self, state: &mut State, material_score: i32) -> i32 {
        let score_sign = if state.player == def::PLAYER_W {
            1
        } else {
            -1
        };

        let (w_features_map, b_features_map) = self.extract_features(state);

        let w_king_attack_count = (w_features_map.king_attack_count - self.params.k_attack_ignore_base).max(0);
        let b_king_attack_count = (b_features_map.king_attack_count - self.params.k_attack_ignore_base).max(0) ;

        let mut midgame_positional_score =
            w_features_map.mg_sqr_point
            + w_features_map.pin_count * self.params.pin_pen
            + w_features_map.semi_pin_count * self.params.semi_pin_pen
            + w_features_map.rook_open_count * self.params.rook_open_bonus
            + w_features_map.king_exposure_count * self.params.k_exposure_pen
            + w_king_attack_count * self.params.k_attack_score
            - b_features_map.mg_sqr_point
            - b_features_map.pin_count * self.params.pin_pen
            - b_features_map.semi_pin_count * self.params.semi_pin_pen
            - b_features_map.rook_open_count * self.params.rook_open_bonus
            - b_features_map.king_exposure_count * self.params.k_exposure_pen
            - b_king_attack_count * self.params.k_attack_score;

        if state.bitboard.b_queen != 0 {
            if (state.cas_rights | state.cas_history) & 0b1100 == 0 {
                midgame_positional_score += self.params.k_no_cas_rights_pen;
            }
        }

        if state.bitboard.w_queen != 0 {
            if (state.cas_rights | state.cas_history) & 0b0011 == 0 {
                midgame_positional_score -= self.params.k_no_cas_rights_pen;
            }
        }

        let mut endgame_positional_score =
            w_features_map.eg_sqr_point
            + w_features_map.passed_pawn_point
            + w_features_map.controlled_passed_pawn_count * self.params.eg_controlled_passer_val
            + w_features_map.eg_mobility
            + w_features_map.king_in_passer_path_count * self.params.eg_king_in_passer_path_bonus
            - b_features_map.eg_sqr_point
            - b_features_map.passed_pawn_point
            - b_features_map.controlled_passed_pawn_count * self.params.eg_controlled_passer_val
            - b_features_map.eg_mobility
            - b_features_map.king_in_passer_path_count * self.params.eg_king_in_passer_path_bonus;

        let bitboard = state.bitboard;
        let bitmask = bitmask::get_bitmask();

        if bitboard.w_queen | bitboard.b_queen | bitboard.w_bishop | bitboard.b_bishop | bitboard.w_pawn | bitboard.b_pawn == 0 {
            if bitboard.w_rook | bitboard.b_knight == 0 {
                if bitboard.b_rook.count_ones() == 1 && bitboard.w_knight.count_ones() == 1 {
                    if bitmask.k_attack_masks[state.wk_index] & bitboard.w_knight != 0 {
                        endgame_positional_score += self.params.eg_rn_knight_protected_bonus;
                    }
                }
            } else if bitboard.b_rook | bitboard.w_knight == 0 {
                if bitboard.w_rook.count_ones() == 1 && bitboard.b_knight.count_ones() == 1 {
                    if bitmask.k_attack_masks[state.bk_index] & bitboard.b_knight != 0 {
                        endgame_positional_score -= self.params.eg_rn_knight_protected_bonus;
                    }
                }
            }
        }

        let shared_positional_score =
            w_features_map.mobility
            + w_features_map.threat_point / self.params.threat_discount_factor
            + w_features_map.behind_pawn_count * self.params.behind_pawn_pen
            + w_features_map.isolated_pawn_count * self.params.isolated_pawn_pen
            + w_features_map.doubled_pawn_count * self.params.doubled_pawn_pen
            - b_features_map.mobility
            - b_features_map.threat_point / self.params.threat_discount_factor
            - b_features_map.behind_pawn_count * self.params.behind_pawn_pen
            - b_features_map.isolated_pawn_count * self.params.isolated_pawn_pen
            - b_features_map.doubled_pawn_count * self.params.doubled_pawn_pen;

        let phase = get_phase(state);

        let extra_score = midgame_positional_score * phase / TOTAL_PHASE + endgame_positional_score * (TOTAL_PHASE - phase) / TOTAL_PHASE + shared_positional_score;

        material_score + extra_score * score_sign + self.params.tempo_val
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
                        w_feature_map.passed_pawn_point += self.params.passer_base_val + (passer_rank - 1) * self.params.passer_rank_bonus;
    
                        if forward_mask & bitmask.k_attack_masks[state.wk_index] != 0 {
                            w_feature_map.king_in_passer_path_count += 1;
                        }
    
                        if forward_mask & bitmask.k_attack_masks[state.bk_index] != 0 {
                            b_feature_map.king_in_passer_path_count += 1;
                        }
    
                        if piece_mask == 0 {
                            let pawn_control_mask = bitmask.wp_front_control_sqr_masks[index];
                            if pawn_control_mask == 0 || pawn_control_mask & bitmask.index_masks[state.wk_index] != 0 {
                                w_feature_map.controlled_passed_pawn_count += 1;
                            }
                        }
    
                        if bitmask.index_masks[index+def::DIM_SIZE] & occupy_mask == 0 {
                            w_feature_map.eg_mobility += self.params.p_mob_score;
                        }
                    } else if forward_mask & (bitboard.w_pawn | bitboard.b_pawn) & file_mask == 0 && (forward_mask & bitboard.b_pawn).count_ones() == 1 && bitmask.wp_connected_sqr_masks[index] & bitboard.w_pawn != 0 {
                        w_feature_map.passed_pawn_point += self.params.candidate_passer_base_val + (passer_rank - 1) * self.params.candidate_passer_rank_bonus;
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
                        b_feature_map.passed_pawn_point += self.params.passer_base_val + (passer_rank - 1) *  self.params.passer_rank_bonus;
    
                        if forward_mask & bitmask.k_attack_masks[state.bk_index] != 0 {
                            b_feature_map.king_in_passer_path_count += 1;
                        }
    
                        if forward_mask & bitmask.k_attack_masks[state.wk_index] != 0 {
                            w_feature_map.king_in_passer_path_count += 1;
                        }
    
                        if piece_mask == 0 {
                            let pawn_control_mask = bitmask.bp_front_control_sqr_masks[index];
                            if pawn_control_mask == 0 || pawn_control_mask & bitmask.index_masks[state.bk_index] != 0 {
                                b_feature_map.controlled_passed_pawn_count += 1;
                            }
                        }
    
                        if bitmask.index_masks[index-def::DIM_SIZE] & occupy_mask == 0 {
                            b_feature_map.eg_mobility += self.params.p_mob_score;
                        }
                    } else if forward_mask & (bitboard.w_pawn | bitboard.b_pawn) & file_mask == 0 && (forward_mask & bitboard.w_pawn).count_ones() == 1 && bitmask.bp_connected_sqr_masks[index] & bitboard.b_pawn != 0 {
                        b_feature_map.passed_pawn_point += self.params.candidate_passer_base_val + (passer_rank - 1) * self.params.candidate_passer_rank_bonus;
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
    
        let wk_ring_mask = bitmask.k_attack_masks[state.wk_index];
        let bk_ring_mask = bitmask.k_attack_masks[state.bk_index];
    
        let w_attack_without_king_mask = wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask | wq_attack_mask;
        let w_attack_mask = w_attack_without_king_mask | wk_ring_mask;
    
        let b_attack_without_king_mask = bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask | bq_attack_mask;
        let b_attack_mask = b_attack_without_king_mask | bk_ring_mask;

        if bitmask.index_masks[state.wk_index] & WK_K_SIDE_MASK != 0 {
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
        } else if bitmask.index_masks[state.wk_index] & WK_Q_SIDE_MASK != 0 {
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
        }
    
        if bitmask.index_masks[state.bk_index] & BK_K_SIDE_MASK != 0 {
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
        } else if bitmask.index_masks[state.bk_index] & BK_Q_SIDE_MASK != 0 {
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
    
                    w_feature_map.king_attack_count += (bk_ring_mask & mov_mask).count_ones() as i32;
                },
                def::WN => {
                    w_feature_map.mg_sqr_point += SQR_TABLE_WN[index];
    
                    if index_mask & w_attack_mask == 0 {
                        if index_mask & b_attack_mask != 0 {
                            w_feature_map.threat_point -= threat_val;
                        }
                    } else if index_mask & bp_attack_mask != 0 {
                        w_feature_map.threat_point -= threat_val - self.val_of(def::BP);
                    }
    
                    let mobility_mask = mov_mask & !bp_attack_mask & !bitboard.w_all & !(b_attack_mask & !w_attack_mask);
                    if mobility_mask == 0 {
                        w_feature_map.mobility += self.params.b_mob_zero_pen;
                    } else {
                        w_feature_map.mobility += mobility_mask.count_ones() as i32 * self.params.n_mob_score;
                    }
    
                    w_feature_map.king_attack_count += (bk_ring_mask & mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask)).count_ones() as i32 * self.params.nk_attack_weight;
                },
                def::WB => {
                    w_feature_map.mg_sqr_point += SQR_TABLE_WB[index];
    
                    if index_mask & w_attack_mask == 0 {
                        if index_mask & b_attack_mask != 0 {
                            w_feature_map.threat_point -= threat_val;
                        }
                    } else if index_mask & bp_attack_mask != 0 {
                        w_feature_map.threat_point -= threat_val - self.val_of(def::BP);
                    }
    
                    let mobility_mask = mov_mask & !bp_attack_mask & !bitboard.w_all & !(b_attack_mask & !w_attack_mask);
                    if mobility_mask == 0 {
                        w_feature_map.mobility += self.params.b_mob_zero_pen;
                    } else {
                        w_feature_map.mobility += mobility_mask.count_ones() as i32 * self.params.b_mob_score;
                    }
    
                    w_feature_map.king_attack_count += (bk_ring_mask & mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask)).count_ones() as i32 * self.params.bk_attack_weight;
                },
                def::WR => {
                    w_feature_map.mg_sqr_point += SQR_TABLE_WR[index];
    
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
                        w_feature_map.mobility += self.params.r_mob_zero_pen;
                    } else {
                        w_feature_map.mobility += mobility_mask.count_ones() as i32 * self.params.r_mob_score;
                    }
    
                    w_feature_map.king_attack_count += (bk_ring_mask & mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask)).count_ones() as i32 * self.params.rk_attack_weight;
                },
                def::WQ => {
                    w_feature_map.mg_sqr_point += SQR_TABLE_WQ[index];
    
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
                        w_feature_map.mobility += self.params.q_mob_zero_pen;
                    } else {
                        w_feature_map.mobility += mobility_mask.count_ones() as i32 * self.params.q_mob_score;
                    }
    
                    w_feature_map.king_attack_count += (bk_ring_mask & mov_mask & !(bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask | bq_attack_mask)).count_ones() as i32 * self.params.qk_attack_weight;
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
    
                    b_feature_map.king_attack_count += (wk_ring_mask & mov_mask).count_ones() as i32;
                },
                def::BN => {
                    b_feature_map.mg_sqr_point += SQR_TABLE_BN[index];
    
                    if index_mask & b_attack_mask == 0 {
                        if index_mask & w_attack_mask != 0 {
                            b_feature_map.threat_point -= threat_val;
                        }
                    } else if index_mask & wp_attack_mask != 0 {
                        b_feature_map.threat_point -= threat_val - self.val_of(def::WP);
                    }
    
                    let mobility_mask = mov_mask & !wp_attack_mask & !bitboard.b_all & !(w_attack_mask & !b_attack_mask);
                    if mobility_mask == 0 {
                        b_feature_map.mobility += self.params.n_mob_zero_pen;
                    } else {
                        b_feature_map.mobility += mobility_mask.count_ones() as i32 * self.params.n_mob_score;
                    }
    
                    b_feature_map.king_attack_count += (wk_ring_mask & mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask)).count_ones() as i32 * self.params.nk_attack_weight;
                },
                def::BB => {
                    b_feature_map.mg_sqr_point += SQR_TABLE_BB[index];
    
                    if index_mask & b_attack_mask == 0 {
                        if index_mask & w_attack_mask != 0 {
                            b_feature_map.threat_point -= threat_val;
                        }
                    } else if index_mask & wp_attack_mask != 0 {
                        b_feature_map.threat_point -= threat_val - self.val_of(def::WP);
                    }
    
                    let mobility_mask = mov_mask & !wp_attack_mask & !bitboard.b_all & !(w_attack_mask & !b_attack_mask);
                    if mobility_mask == 0 {
                        b_feature_map.mobility += self.params.b_mob_zero_pen;
                    } else {
                        b_feature_map.mobility += mobility_mask.count_ones() as i32 * self.params.b_mob_score;
                    }
    
                    b_feature_map.king_attack_count += (wk_ring_mask & mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask)).count_ones() as i32 * self.params.bk_attack_weight;
                },
                def::BR => {
                    b_feature_map.mg_sqr_point += SQR_TABLE_BR[index];
    
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
                        b_feature_map.mobility += self.params.r_mob_zero_pen;
                    } else {
                        b_feature_map.mobility += mobility_mask.count_ones() as i32 * self.params.r_mob_score;
                    }
    
                    b_feature_map.king_attack_count += (wk_ring_mask & mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask)).count_ones() as i32 * self.params.rk_attack_weight;
                },
                def::BQ => {
                    b_feature_map.mg_sqr_point += SQR_TABLE_BQ[index];
    
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
                        b_feature_map.mobility += self.params.q_mob_zero_pen;
                    } else {
                        b_feature_map.mobility += mobility_mask.count_ones() as i32 * self.params.q_mob_score;
                    }
    
                    b_feature_map.king_attack_count += (wk_ring_mask & mov_mask & !(wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask | wq_attack_mask)).count_ones() as i32 * self.params.qk_attack_weight;
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
}

