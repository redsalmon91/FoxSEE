use crate::{
    def,
    state::State,
    mov_gen::MoveGenerator,
};

pub static MATE_VAL: i32 = 20000;
pub static TERM_VAL: i32 = 10000;

static Q_VAL: i32 = 1000;
static R_VAL: i32 = 525;
static B_VAL: i32 = 350;
static N_VAL: i32 = 345;
static P_VAL: i32 = 100;

static KING_EXPOSED_BASE_PEN: i32 = -5;
static KING_SAFE_SPOT_BASE_PEN: i32 = -10;

static MIDGAME_PASS_PAWN_VAL: i32 = 10;
static ENDGMAE_PASS_PAWN_VAL: i32 = 20;

static DUP_PAWN_PEN: i32 = -20;
static ISOLATE_PAWN_PEN: i32 = -20;

static SEMI_ROOK_OPEN_LINE_VAL: i32 = 20;
static ROOK_OPEN_LINE_VAL: i32 = 30;
static EXPOSED_ROOK_PEN: i32 = -10;

static COMF_SQR_VAL: i32 = 10;
static PREF_SQR_VAL: i32 = 20;
static DANGER_SQR_VAL: i32 = 20;
static AVOID_SQR_PEN: i32 = -10;

static ROOK_MOB_BASE_VAL: i32 = 2;
static BISHOP_MOB_BASE_VAL: i32 = 2;
static KNIGHT_MOB_BASE_VAL: i32 = 1;

static MIN_PROTECTOR_PAWN_COUNT: i32 = 2;

static WK_SAFE_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_11000011_11000110;
static BK_SAFE_MASK: u64 = 0b11000110_11000011_00000000_00000000_00000000_00000000_00000000_00000000;

static WQ_AVOID_MASK: u64 = 0b11000011_11000011_11000011_00011000_00011000_10000001_11000011_11000011;
static BQ_AVOID_MASK: u64 = 0b11000011_11000011_10000001_00011000_00011000_11000011_11000011_11000011;

static WR_COMF_MASK: u64 = 0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00111100;
static BR_COMF_MASK: u64 = 0b00111100_00000000_00000000_00000000_00000000_00000000_11111111_11111111;
static WR_PREF_MASK: u64 = 0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
static BR_PREF_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111;
static WR_DANGER_MASK: u64 = 0b00111100_00111100_00000000_00000000_00000000_00000000_00000000_00000000;
static BR_DANGER_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00111100_00111100;
static WR_AVOID_MASK: u64 = 0b00000000_00000000_00000000_11111111_11111111_11000011_11100111_11000011;
static BR_AVOID_MASK: u64 = 0b11000011_11100111_11000011_11111111_11111111_00000000_00000000_00000000;

static WB_COMF_MASK: u64 = 0b00000000_00000000_01111110_01111110_00111100_01011010_01000010_00000000;
static BB_COMF_MASK: u64 = 0b00000000_01000010_01011010_00111100_01111110_01111110_00000000_00000000;
static WB_AVOID_MASK: u64 = 0b11111111_10000001_10000001_10000001_10000001_10000001_10000001_11111111;
static BB_AVOID_MASK: u64 = 0b11111111_10000001_10000001_10000001_10000001_10000001_10000001_11111111;

static WN_COMF_MASK: u64 = 0b00000000_00111100_01111110_01111110_00111100_01100110_00011000_00000000;
static BN_COMF_MASK: u64 = 0b00000000_00011000_01100110_00111100_01111110_01111110_00111100_00000000;
static WN_PREF_MASK: u64 = 0b00000000_00000000_00011000_00011000_00000000_00000000_00000000_00000000;
static BN_PREF_MASK: u64 = 0b00000000_00000000_00000000_00000000_00011000_00011000_00000000_00000000;
static WN_AVOID_MASK: u64 = 0b11000011_11000011_10000001_10000001_10000001_10000001_11100111_11111111;
static BN_AVOID_MASK: u64 = 0b11111111_11100111_10000001_10000001_10000001_10000001_11000011_11000011;

static WP_COMF_MASK: u64 = 0b00000000_11111111_01111110_01111110_00111100_11000011_11100111_00000000;
static BP_COMF_MASK: u64 = 0b00000000_11100111_11000011_00111100_01111110_01111110_11111111_00000000;
static WP_PREF_MASK: u64 = 0b00000000_11111111_01111110_00111100_00011000_00000000_00000000_00000000;
static BP_PREF_MASK: u64 = 0b00000000_00000000_00000000_00011000_00111100_01111110_11111111_00000000;
static WP_DANGER_MASK: u64 = 0b00000000_11111111_01111110_00000000_00000000_00000000_00000000_00000000;
static BP_DANGER_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_01111110_11111111_00000000;

static WK_ENDGAME_COMF_MASK: u64 = 0b00000000_00000000_00111100_00111100_00111100_00111100_00000000_00000000;
static BK_ENDGAME_COMF_MASK: u64 = 0b00000000_00000000_00111100_00111100_00111100_00111100_00000000_00000000;

#[derive(PartialEq, Debug)]
pub struct FeatureMap {
    pawn_count: i32,
    queen_count: i32,
    rook_count: i32,
    bishop_count: i32,
    knight_count: i32,

    dup_pawn_count: i32,
    isolate_pawn_count: i32,
    passed_pawn_count: i32,

    knight_mobility: i32,
    bishop_mobility: i32,
    rook_mobility: i32,

    semi_open_rook_count: i32,
    open_rook_count: i32,
    exposed_rook_count: i32,

    comf_sqr_occupied: i32,
    pref_sqr_occupied: i32,
    danger_sqr_occupied: i32,
    avoid_sqr_occupied: i32,

    king_expose_count: i32,
    king_protector_count: i32,
    king_in_safe_spot: bool,
}

impl FeatureMap {
    pub fn empty() -> Self {
        FeatureMap {
            pawn_count: 0,
            queen_count: 0,
            rook_count: 0,
            bishop_count: 0,
            knight_count: 0,

            dup_pawn_count: 0,
            isolate_pawn_count: 0,
            passed_pawn_count: 0,

            knight_mobility: 0,
            bishop_mobility: 0,
            rook_mobility: 0,

            semi_open_rook_count: 0,
            open_rook_count: 0,
            exposed_rook_count: 0,

            comf_sqr_occupied: 0,
            pref_sqr_occupied: 0,
            danger_sqr_occupied: 0,
            avoid_sqr_occupied: 0,

            king_expose_count: 0,
            king_protector_count: 0,
            king_in_safe_spot: false,
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

pub fn eval_state(state: &State, mov_generator: &MoveGenerator) -> i32 {
    let (w_features_map, b_features_map) = extract_features(state, mov_generator);
    let w_piece_count = w_features_map.queen_count + w_features_map.rook_count + w_features_map.bishop_count + w_features_map.knight_count;
    let b_piece_count = b_features_map.queen_count + b_features_map.rook_count + b_features_map.bishop_count + b_features_map.knight_count;

    let base_score = w_features_map.queen_count * Q_VAL
        + w_features_map.rook_count * R_VAL
        + w_features_map.bishop_count * B_VAL
        + w_features_map.knight_count * N_VAL
        + w_features_map.pawn_count * P_VAL
        - b_features_map.queen_count * Q_VAL
        - b_features_map.rook_count * R_VAL
        - b_features_map.bishop_count * B_VAL
        - b_features_map.knight_count * N_VAL
        - b_features_map.pawn_count * P_VAL;

    let is_endgame = (w_piece_count < 2 || b_piece_count < 2) || ((w_piece_count <= 3 && w_features_map.queen_count == 0 && w_features_map.pawn_count <= 5) || (b_piece_count <= 3 && b_features_map.queen_count == 0 && b_features_map.pawn_count <= 5));

    if is_endgame {
        let mut endgame_score = base_score
            + w_features_map.passed_pawn_count * ENDGMAE_PASS_PAWN_VAL
            + w_features_map.dup_pawn_count * DUP_PAWN_PEN
            + w_features_map.rook_mobility * ROOK_MOB_BASE_VAL
            + w_features_map.bishop_mobility * BISHOP_MOB_BASE_VAL
            + w_features_map.knight_mobility * KNIGHT_MOB_BASE_VAL
            - b_features_map.passed_pawn_count * ENDGMAE_PASS_PAWN_VAL
            - b_features_map.dup_pawn_count * DUP_PAWN_PEN
            - b_features_map.rook_mobility * ROOK_MOB_BASE_VAL
            - b_features_map.bishop_mobility * BISHOP_MOB_BASE_VAL
            - b_features_map.knight_mobility * KNIGHT_MOB_BASE_VAL;

        if b_features_map.queen_count == 0 {
            let wk_index_mask = state.bitmask.index_masks[state.wk_index];
            if wk_index_mask & WK_ENDGAME_COMF_MASK != 0 {
                endgame_score += COMF_SQR_VAL;

                if wk_index_mask & WK_ENDGAME_COMF_MASK != 0 {
                    endgame_score += PREF_SQR_VAL;
                }
            }
        }

        if w_features_map.queen_count == 0 {
            let bk_index_mask = state.bitmask.index_masks[state.bk_index];
            if bk_index_mask & BK_ENDGAME_COMF_MASK != 0 {
                endgame_score -= COMF_SQR_VAL;

                if bk_index_mask & BK_ENDGAME_COMF_MASK != 0 {
                    endgame_score -= PREF_SQR_VAL;
                }
            }
        }

        return endgame_score
    }

    let mut midgame_score = base_score
    + w_features_map.comf_sqr_occupied * COMF_SQR_VAL
    + w_features_map.pref_sqr_occupied * PREF_SQR_VAL
    + w_features_map.danger_sqr_occupied * DANGER_SQR_VAL
    + w_features_map.avoid_sqr_occupied * AVOID_SQR_PEN
    + w_features_map.passed_pawn_count * MIDGAME_PASS_PAWN_VAL
    + w_features_map.isolate_pawn_count * ISOLATE_PAWN_PEN
    + w_features_map.dup_pawn_count * DUP_PAWN_PEN
    + w_features_map.semi_open_rook_count * SEMI_ROOK_OPEN_LINE_VAL
    + w_features_map.open_rook_count * ROOK_OPEN_LINE_VAL
    + w_features_map.exposed_rook_count * EXPOSED_ROOK_PEN
    + w_features_map.rook_mobility * ROOK_MOB_BASE_VAL
    + w_features_map.bishop_mobility * BISHOP_MOB_BASE_VAL
    + w_features_map.knight_mobility * KNIGHT_MOB_BASE_VAL
    - b_features_map.comf_sqr_occupied * COMF_SQR_VAL
    - b_features_map.pref_sqr_occupied * PREF_SQR_VAL
    - b_features_map.danger_sqr_occupied * DANGER_SQR_VAL
    - b_features_map.avoid_sqr_occupied * AVOID_SQR_PEN
    - b_features_map.passed_pawn_count * MIDGAME_PASS_PAWN_VAL
    - b_features_map.isolate_pawn_count * ISOLATE_PAWN_PEN
    - b_features_map.dup_pawn_count * DUP_PAWN_PEN
    - b_features_map.semi_open_rook_count * SEMI_ROOK_OPEN_LINE_VAL
    - b_features_map.open_rook_count * ROOK_OPEN_LINE_VAL
    - b_features_map.exposed_rook_count * EXPOSED_ROOK_PEN
    - b_features_map.rook_mobility * ROOK_MOB_BASE_VAL
    - b_features_map.bishop_mobility * BISHOP_MOB_BASE_VAL
    - b_features_map.knight_mobility * KNIGHT_MOB_BASE_VAL;

    if b_features_map.queen_count != 0 {
        midgame_score += w_features_map.king_expose_count * b_piece_count * KING_EXPOSED_BASE_PEN;

        if w_features_map.king_protector_count < MIN_PROTECTOR_PAWN_COUNT {
            midgame_score += b_piece_count * (MIN_PROTECTOR_PAWN_COUNT - w_features_map.king_protector_count) * KING_EXPOSED_BASE_PEN;
        }

        if !w_features_map.king_in_safe_spot {
            midgame_score += b_piece_count * KING_SAFE_SPOT_BASE_PEN;
        }
    }

    if w_features_map.queen_count != 0 {
        midgame_score -= b_features_map.king_expose_count * w_piece_count * KING_EXPOSED_BASE_PEN;

        if b_features_map.king_protector_count < MIN_PROTECTOR_PAWN_COUNT {
            midgame_score -= w_piece_count * (MIN_PROTECTOR_PAWN_COUNT - b_features_map.king_protector_count) * KING_EXPOSED_BASE_PEN;
        }

        if !b_features_map.king_in_safe_spot {
            midgame_score -= w_piece_count * KING_SAFE_SPOT_BASE_PEN;
        }
    }

    return midgame_score
}

pub fn extract_features(state: &State, mov_generator: &MoveGenerator) -> (FeatureMap, FeatureMap) {
    let squares = state.squares;
    let index_masks = state.bitmask.index_masks;
    let file_masks = state.bitmask.file_masks;
    let wk_protect_masks = state.bitmask.wk_protect_masks;
    let bk_protect_masks = state.bitmask.bk_protect_masks;
    let wp_forward_masks = state.bitmask.wp_forward_masks;
    let bp_forward_masks = state.bitmask.bp_forward_masks;
    let nearby_masks = state.bitmask.nearby_masks;
    let bitboard = state.bitboard;

    let mut w_feature_map = FeatureMap::empty();
    let mut b_feature_map = FeatureMap::empty();

    let mut index = 0;

    while index < def::BOARD_SIZE {
        if !def::is_index_valid(index) {
            index += 8;
        }

        let moving_piece = squares[index];

        if moving_piece == 0 {
            index += 1;
            continue
        }

        let index_mask = index_masks[index];

        match moving_piece {
            def::WP => {
                w_feature_map.pawn_count += 1;

                if index_mask & WP_COMF_MASK != 0 {
                    w_feature_map.comf_sqr_occupied += 1;

                    if index_mask & WP_PREF_MASK != 0 {
                        w_feature_map.pref_sqr_occupied += 1;

                        if index_mask & WP_DANGER_MASK != 0 {
                            w_feature_map.danger_sqr_occupied += 1;
                        }
                    }
                }

                let w_pawn_mask = bitboard.w_pawn;
                let rank = def::get_w_rank(index) as i32;

                if wp_forward_masks[index] & bitboard.b_pawn == 0 {
                    w_feature_map.passed_pawn_count += rank;
                }

                if nearby_masks[index] & w_pawn_mask == 0 {
                    w_feature_map.isolate_pawn_count += 1;

                    if (file_masks[index] & w_pawn_mask).count_ones() > 1 {
                        w_feature_map.dup_pawn_count += 1;
                    }
                }
            },
            def::BP => {
                b_feature_map.pawn_count += 1;

                if index_mask & BP_COMF_MASK != 0 {
                    b_feature_map.comf_sqr_occupied += 1;

                    if index_mask & BP_PREF_MASK != 0 {
                        b_feature_map.pref_sqr_occupied += 1;

                        if index_mask & BP_DANGER_MASK != 0 {
                            b_feature_map.danger_sqr_occupied += 1;
                        }
                    }
                }

                let b_pawn_mask = bitboard.b_pawn;
                let rank = def::get_b_rank(index);

                if bp_forward_masks[index] & bitboard.w_pawn == 0 {
                    b_feature_map.passed_pawn_count += rank as i32;
                }

                if nearby_masks[index] & b_pawn_mask == 0 {
                    b_feature_map.isolate_pawn_count += 1;

                    if (file_masks[index] & b_pawn_mask).count_ones() > 1 {
                        b_feature_map.dup_pawn_count += 1;
                    }
                }
            },

            def::WN => {
                w_feature_map.knight_count += 1;
                w_feature_map.knight_mobility += mov_generator.count_knight_mobility(state, index, def::PLAYER_W);

                if index_mask & WN_COMF_MASK != 0 {
                    w_feature_map.comf_sqr_occupied += 1;

                    if index_mask & WN_PREF_MASK != 0 {
                        w_feature_map.pref_sqr_occupied += 1;
                    }
                } else if index_mask & WN_AVOID_MASK != 0 {
                    w_feature_map.avoid_sqr_occupied += 1;
                }
            },
            def::BN => {
                b_feature_map.knight_count += 1;
                b_feature_map.knight_mobility += mov_generator.count_knight_mobility(state, index, def::PLAYER_B);

                if index_mask & BN_COMF_MASK != 0 {
                    b_feature_map.comf_sqr_occupied += 1;

                    if index_mask & BN_PREF_MASK != 0 {
                        b_feature_map.pref_sqr_occupied += 1;
                    }
                } else if index_mask & BN_AVOID_MASK != 0 {
                    b_feature_map.avoid_sqr_occupied += 1;
                }
            },

            def::WB => {
                w_feature_map.bishop_count += 1;
                w_feature_map.bishop_mobility += mov_generator.count_bishop_mobility(state, index, def::PLAYER_W);

                if index_mask & WB_COMF_MASK != 0 {
                    w_feature_map.comf_sqr_occupied += 1;
                } else if index_mask & WB_AVOID_MASK != 0 {
                    w_feature_map.avoid_sqr_occupied += 1;
                }
            },
            def::BB => {
                b_feature_map.bishop_count += 1;
                b_feature_map.bishop_mobility += mov_generator.count_bishop_mobility(state, index, def::PLAYER_B);

                if index_mask & BB_COMF_MASK != 0 {
                    b_feature_map.comf_sqr_occupied += 1;
                } else if index_mask & BB_AVOID_MASK != 0 {
                    b_feature_map.avoid_sqr_occupied += 1;
                }
            },

            def::WR => {
                w_feature_map.rook_count += 1;
                w_feature_map.rook_mobility += mov_generator.count_rook_mobility(state, index, def::PLAYER_W);

                if index_mask & WR_COMF_MASK != 0 {
                    w_feature_map.comf_sqr_occupied += 1;

                    if index_mask & WR_PREF_MASK != 0 {
                        w_feature_map.pref_sqr_occupied += 1;

                        if index_mask & WR_DANGER_MASK != 0 {
                            w_feature_map.danger_sqr_occupied += 1;
                        }
                    }
                } else if index_mask & WR_AVOID_MASK != 0 {
                    w_feature_map.avoid_sqr_occupied += 1;
                }

                let file_mask = file_masks[index];
                if file_mask & bitboard.w_pawn == 0 {
                    if file_mask & bitboard.b_pawn == 0
                    && file_mask & bitboard.b_rook == 0 {
                        w_feature_map.open_rook_count += 1;
                    } else {
                        w_feature_map.semi_open_rook_count += 1;
                    }
                }

                if file_mask & bp_forward_masks[index] & bitboard.w_pawn != 0 {
                    w_feature_map.exposed_rook_count += 1;
                }
            },
            def::BR => {
                b_feature_map.rook_count += 1;
                b_feature_map.rook_mobility += mov_generator.count_rook_mobility(state, index, def::PLAYER_B);

                if index_mask & BR_COMF_MASK != 0 {
                    b_feature_map.comf_sqr_occupied += 1;

                    if index_mask & BR_PREF_MASK != 0 {
                        b_feature_map.pref_sqr_occupied += 1;

                        if index_mask & BR_DANGER_MASK != 0 {
                            b_feature_map.danger_sqr_occupied += 1;
                        }
                    }
                } else if index_mask & BR_AVOID_MASK != 0 {
                    b_feature_map.avoid_sqr_occupied += 1;
                }

                let file_mask = file_masks[index];
                if file_mask & bitboard.b_pawn == 0 {
                    if file_mask & bitboard.w_pawn == 0
                    && file_mask & bitboard.w_rook == 0 {
                        b_feature_map.open_rook_count += 1;
                    } else {
                        b_feature_map.semi_open_rook_count += 1;
                    }
                }

                if file_mask & wp_forward_masks[index] & bitboard.b_pawn != 0 {
                    b_feature_map.exposed_rook_count += 1;
                }
            },

            def::WQ => {
                w_feature_map.queen_count += 1;

                if index_mask & WQ_AVOID_MASK != 0 {
                    w_feature_map.avoid_sqr_occupied += 1;
                }
            },
            def::BQ => {
                b_feature_map.queen_count += 1;

                if index_mask & BQ_AVOID_MASK != 0 {
                    b_feature_map.avoid_sqr_occupied += 1;
                }
            },

            def::WK => {
                let file_mask = file_masks[index];
                let protect_mask = wk_protect_masks[index];

                if index_mask & WK_SAFE_MASK != 0 || state.cas_rights & 0b1100 != 0 {
                    w_feature_map.king_in_safe_spot = true;
                }

                if file_mask & protect_mask & bitboard.w_pawn == 0 {
                    w_feature_map.king_expose_count += 1;
                }

                if file_mask & bitboard.b_pawn == 0 {
                    w_feature_map.king_expose_count += 1;
                }

                w_feature_map.king_protector_count += (protect_mask & bitboard.w_pawn).count_ones() as i32;
            },
            def::BK => {
                let file_mask = file_masks[index];
                let protect_mask = bk_protect_masks[index];

                if index_mask & BK_SAFE_MASK != 0 || state.cas_rights & 0b0011 != 0 {
                    b_feature_map.king_in_safe_spot = true;
                }

                if file_mask & protect_mask & bitboard.b_pawn == 0 {
                    b_feature_map.king_expose_count += 1;
                }

                if file_mask & bitboard.w_pawn == 0 {
                    b_feature_map.king_expose_count += 1;
                }

                b_feature_map.king_protector_count += (protect_mask & bitboard.b_pawn).count_ones() as i32;
            },
            _ => {},
        }

        index += 1;
    }

    (w_feature_map, b_feature_map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bitboard::BitMask,
        mov_gen::MoveGenerator,
        state::State,
        prng::XorshiftPrng,
    };

    #[test]
    fn test_extract_features_1() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mov_generator = MoveGenerator::new();

        let state = State::new("1kr2r2/pp2nppp/1bn2q2/3pp3/3P4/1BN1P3/PPP1NP1P/R2Q1RK1 b Q - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state, &mov_generator);

        assert_eq!(FeatureMap {
            pawn_count: 7,
            queen_count: 1,
            rook_count: 2,
            bishop_count: 1,
            knight_count: 2,

            dup_pawn_count: 0,
            isolate_pawn_count: 1,
            passed_pawn_count: 0,

            knight_mobility: 8,
            bishop_mobility: 3,
            rook_mobility: 3,

            semi_open_rook_count: 0,
            open_rook_count: 0,
            exposed_rook_count: 0,

            comf_sqr_occupied: 10,
            pref_sqr_occupied: 1,
            danger_sqr_occupied: 0,
            avoid_sqr_occupied: 1,

            king_expose_count: 1,
            king_in_safe_spot: true,
            king_protector_count: 2,
        }, w_features);

        assert_eq!(FeatureMap {
            pawn_count: 7,
            queen_count: 1,
            rook_count: 2,
            bishop_count: 1,
            knight_count: 2,

            dup_pawn_count: 0,
            isolate_pawn_count: 0,
            passed_pawn_count: 0,

            knight_mobility: 7,
            bishop_mobility: 5,
            rook_mobility: 7,

            semi_open_rook_count: 1,
            open_rook_count: 0,
            exposed_rook_count: 0,

            comf_sqr_occupied: 12,
            pref_sqr_occupied: 2,
            danger_sqr_occupied: 0,
            avoid_sqr_occupied: 0,

            king_expose_count: 0,
            king_in_safe_spot: true,
            king_protector_count: 2,
        }, b_features);
    }

    #[test]
    fn test_extract_features_2() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mov_generator = MoveGenerator::new();

        let state = State::new("1kr2r2/1p4pp/1p1P1qn1/p2pp3/3P4/RB2P3/P1P1NP1P/3Q1RK1 b - - 0 1", &zob_keys, &bitmask);
        let (w_features, b_features) = extract_features(&state, &mov_generator);

        assert_eq!(FeatureMap {
            pawn_count: 7,
            queen_count: 1,
            rook_count: 2,
            bishop_count: 1,
            knight_count: 1,

            dup_pawn_count: 1,
            isolate_pawn_count: 4,
            passed_pawn_count: 5,

            knight_mobility: 4,
            bishop_mobility: 3,
            rook_mobility: 3,

            semi_open_rook_count: 0,
            open_rook_count: 0,
            exposed_rook_count: 1,

            comf_sqr_occupied: 9,
            pref_sqr_occupied: 2,
            danger_sqr_occupied: 1,
            avoid_sqr_occupied: 1,

            king_expose_count: 1,
            king_in_safe_spot: true,
            king_protector_count: 2,
        }, w_features);

        assert_eq!(FeatureMap {
            pawn_count: 7,
            queen_count: 1,
            rook_count: 2,
            bishop_count: 0,
            knight_count: 1,

            dup_pawn_count: 1,
            isolate_pawn_count: 1,
            passed_pawn_count: 0,

            knight_mobility: 4,
            bishop_mobility: 0,
            rook_mobility: 13,

            semi_open_rook_count: 2,
            open_rook_count: 0,
            exposed_rook_count: 0,

            comf_sqr_occupied: 9,
            pref_sqr_occupied: 2,
            danger_sqr_occupied: 0,
            avoid_sqr_occupied: 0,

            king_expose_count: 1,
            king_in_safe_spot: true,
            king_protector_count: 2,
        }, b_features);
    }
}
