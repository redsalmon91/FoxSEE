use crate::{
    def,
    state::State,
};

pub static TERM_VAL: i32 = 10000;
pub static K_VAL: i32 = 20000;

static Q_VAL: i32 = 1000;
static R_VAL: i32 = 525;
static B_VAL: i32 = 350;
static N_VAL: i32 = 345;
static P_VAL: i32 = 100;

static KING_PROTECTED_VAL: i32 = 50;
static KING_COVERED_VAL: i32 = 30;
static KING_SAFE_SPOT_VAL: i32 = 30;
static KING_THREAT_VAL: i32 = 15;

static PASS_PAWN_VAL: i32 = 10;
static DUP_PAWN_PEN: i32 = 20;
static ISOLATE_PAWN_PEN: i32 = 20;
static ENDGAME_PAWN_VAL: i32 = 5;

static ROOK_OPEN_LINE_VAL: i32 = 20;

static COMF_SQR_VAL: i32 = 10;
static PREF_SQR_VAL: i32 = 20;
static AVOID_SQR_PEN: i32 = 20;

const W_FIFTH_RANK: usize = 63;
const B_FIFTH_RANK: usize = 56;

static WK_SAFE_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_11000011_11000111;
static BK_SAFE_MASK: u64 = 0b11000111_11000011_00000000_00000000_00000000_00000000_00000000_00000000;

static WQ_COMF_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_01111110_00111100_00000000;
static BQ_COMF_MASK: u64 = 0b00000000_00111100_01111110_00000000_00000000_00000000_00000000_00000000;
static WQ_AVOID_MASK: u64 = 0b11000011_11000011_10000001_10000001_10000001_10000001_11000011_11111111;
static BQ_AVOID_MASK: u64 = 0b11111111_11000011_10000001_10000001_10000001_10000001_11000011_11000011;

static WR_COMF_MASK: u64 = 0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00111100;
static BR_COMF_MASK: u64 = 0b00111100_00000000_00000000_00000000_00000000_00000000_11111111_11111111;
static WR_PREF_MASK: u64 = 0b00000000_00111100_00000000_00000000_00000000_00000000_00000000_00000000;
static BR_PREF_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00111100_00000000;
static WR_AVOID_MASK: u64 = 0b00000000_00000000_00000000_10000001_11000011_11000011_11111111_00000000;
static BR_AVOID_MASK: u64 = 0b00000000_11111111_11000011_11000011_10000001_00000000_00000000_00000000;

static WB_COMF_MASK: u64 = 0b00000000_00000000_11111111_01111110_00111100_01011010_01000010_00000000;
static BB_COMF_MASK: u64 = 0b00000000_01000010_01011010_00111100_01111110_11111111_00000000_00000000;
static WB_AVOID_MASK: u64 = 0b11111111_10000001_00000000_10000001_10000001_10000001_10000001_11111111;
static BB_AVOID_MASK: u64 = 0b11111111_10000001_10000001_10000001_10000001_00000000_10000001_11111111;

static WN_COMF_MASK: u64 = 0b00000000_00111100_01111110_01111110_00111100_01100110_00011000_00000000;
static BN_COMF_MASK: u64 = 0b00000000_00011000_01100110_00111100_01111110_01111110_00111100_00000000;
static WN_PREF_MASK: u64 = 0b00000000_00000000_00011000_00011000_00000000_00000000_00000000_00000000;
static BN_PREF_MASK: u64 = 0b00000000_00000000_00000000_00000000_00011000_00011000_00000000_00000000;
static WN_AVOID_MASK: u64 = 0b11111111_10000001_00000000_10000001_10000001_10000001_11000011_11111111;
static BN_AVOID_MASK: u64 = 0b11111111_11000011_10000001_10000001_10000001_00000000_10000001_11111111;

static WP_COMF_MASK: u64 = 0b00000000_01111110_01111110_01111110_00111100_11000011_11100111_00000000;
static BP_COMF_MASK: u64 = 0b00000000_11100111_11000011_00111100_01111110_01111110_01111110_00000000;
static WP_PREF_MASK: u64 = 0b00000000_01111110_00111100_00011000_00000000_00000000_00000000_00000000;
static BP_PREF_MASK: u64 = 0b00000000_00000000_00000000_00000000_00011000_00111100_01111110_00000000;

static K_ENDGAME_COMF_MASK: u64 = 0b00000000_00000000_01111110_01111110_01111110_01111110_00000000_00000000;

pub fn val_of(piece: u8) -> i32 {
    match piece {
        0 => 0,
        def::WK => K_VAL,
        def::WQ => Q_VAL,
        def::WR => R_VAL,
        def::WB => B_VAL,
        def::WN => N_VAL,
        def::WP => P_VAL,

        def::BK => K_VAL,
        def::BQ => Q_VAL,
        def::BR => R_VAL,
        def::BB => B_VAL,
        def::BN => N_VAL,
        def::BP => P_VAL,

        _ => 0,
    }
}

pub fn eval_state(state: &State) -> i32 {
    let squares = state.squares;
    let index_masks = state.bitmask.index_masks;
    let file_masks = state.bitmask.file_masks;
    let wk_protect_masks = state.bitmask.wk_protect_masks;
    let bk_protect_masks = state.bitmask.bk_protect_masks;
    let wp_forward_masks = state.bitmask.wp_forward_masks;
    let bp_forward_masks = state.bitmask.bp_forward_masks;
    let wp_nearby_masks = state.bitmask.wp_nearby_masks;
    let bp_nearby_masks = state.bitmask.bp_nearby_masks;
    let bitboard = state.bitboard;

    let mut index = 0;
    let mut base_score = 0;
    let mut midgame_score = 0;
    let mut endgame_score = 0;
    let mut wk_safety_score = 0;
    let mut bk_safety_score = 0;

    let mut wq_count = 0;
    let mut bq_count = 0;

    let w_pawn_count = bitboard.w_pawn.count_ones();
    let b_pawn_count = bitboard.b_pawn.count_ones();
    let w_piece_count = bitboard.w_all.count_ones() - w_pawn_count - 1;
    let b_piece_count = bitboard.b_all.count_ones() - b_pawn_count - 1;

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
                base_score += P_VAL;

                if index_mask & WP_COMF_MASK != 0 {
                    midgame_score += COMF_SQR_VAL;

                    if index_mask & WP_PREF_MASK != 0 {
                        midgame_score += PREF_SQR_VAL;
                    }
                }

                let w_pawn_mask = bitboard.w_pawn;
                let b_pawn_mask = bitboard.b_pawn;
                let rank = def::get_rank(index) as i32;

                endgame_score += rank * ENDGAME_PAWN_VAL;

                if index > W_FIFTH_RANK && wp_forward_masks[index] & b_pawn_mask == 0 {
                    let pass_pawn_weight = rank - 4;
                    let pass_pawn_val =  PASS_PAWN_VAL * pass_pawn_weight * pass_pawn_weight;

                    midgame_score += pass_pawn_val;
                    endgame_score += pass_pawn_val;

                    if wp_nearby_masks[index] & w_pawn_mask != 0 {
                        midgame_score += pass_pawn_val;
                        endgame_score += pass_pawn_val;
                    }
                } else if wp_nearby_masks[index] & w_pawn_mask == 0 && wp_forward_masks[index] & w_pawn_mask == 0 {
                    midgame_score -= ISOLATE_PAWN_PEN;

                    if (file_masks[index] & w_pawn_mask).count_ones() > 1 {
                        midgame_score -= DUP_PAWN_PEN;
                        endgame_score -= DUP_PAWN_PEN;
                    }
                }
            },
            def::BP => {
                base_score -= P_VAL;

                if index_mask & BP_COMF_MASK != 0 {
                    midgame_score -= COMF_SQR_VAL;

                    if index_mask & BP_PREF_MASK != 0 {
                        midgame_score -= PREF_SQR_VAL;
                    }
                }

                let w_pawn_mask = bitboard.w_pawn;
                let b_pawn_mask = bitboard.b_pawn;
                let rank = def::get_rank(index) as i32;

                endgame_score -= rank * ENDGAME_PAWN_VAL;

                if index < B_FIFTH_RANK && bp_forward_masks[index] & w_pawn_mask == 0 {
                    let pass_pawn_weight = 5 - rank;
                    let pass_pawn_val =  PASS_PAWN_VAL * pass_pawn_weight * pass_pawn_weight;

                    midgame_score -= pass_pawn_val;
                    endgame_score -= pass_pawn_val;

                    if bp_nearby_masks[index] & b_pawn_mask != 0 {
                        midgame_score -= pass_pawn_val;
                        endgame_score -= pass_pawn_val;
                    }
                } else if bp_nearby_masks[index] & b_pawn_mask == 0 && bp_forward_masks[index] & b_pawn_mask == 0 {
                    midgame_score += ISOLATE_PAWN_PEN;

                    if (file_masks[index] & b_pawn_mask).count_ones() > 1 {
                        midgame_score += DUP_PAWN_PEN;
                        endgame_score += DUP_PAWN_PEN;
                    }
                }
            },

            def::WN => {
                base_score += N_VAL;

                if index_mask & WN_COMF_MASK != 0 {
                    midgame_score += COMF_SQR_VAL;

                    if index_mask & WN_PREF_MASK != 0 {
                        midgame_score += PREF_SQR_VAL;
                    }
                } else if index_mask & WN_AVOID_MASK != 0 {
                    midgame_score -= AVOID_SQR_PEN;
                }
            },
            def::BN => {
                base_score -= N_VAL;

                if index_mask & BN_COMF_MASK != 0 {
                    midgame_score -= COMF_SQR_VAL;

                    if index_mask & BN_PREF_MASK != 0 {
                        midgame_score -= PREF_SQR_VAL;
                    }
                } else if index_mask & BN_AVOID_MASK != 0 {
                    midgame_score += AVOID_SQR_PEN;
                }
            },

            def::WB => {
                base_score += B_VAL;

                if index_mask & WB_COMF_MASK != 0 {
                    midgame_score += COMF_SQR_VAL;
                } else if index_mask & WB_AVOID_MASK != 0 {
                    midgame_score -= AVOID_SQR_PEN;
                }
            },
            def::BB => {
                base_score -= B_VAL;

                if index_mask & BB_COMF_MASK != 0 {
                    midgame_score -= COMF_SQR_VAL;
                } else if index_mask & BB_AVOID_MASK != 0 {
                    midgame_score += AVOID_SQR_PEN;
                }
            },

            def::WR => {
                base_score += R_VAL;

                if index_mask & WR_COMF_MASK != 0 {
                    midgame_score += COMF_SQR_VAL;

                    if index_mask & WR_PREF_MASK != 0 {
                        midgame_score += PREF_SQR_VAL;
                    }
                } else if index_mask & WR_AVOID_MASK != 0 {
                    midgame_score -= AVOID_SQR_PEN;
                }

                let file_mask = file_masks[index];
                if file_mask & bitboard.w_pawn == 0 {
                    midgame_score += ROOK_OPEN_LINE_VAL;

                    if file_mask & bitboard.b_pawn == 0
                    && file_mask & bitboard.b_rook == 0 {
                        midgame_score += ROOK_OPEN_LINE_VAL;
                    }
                }
            },
            def::BR => {
                base_score -= R_VAL;

                if index_mask & BR_COMF_MASK != 0 {
                    midgame_score -= COMF_SQR_VAL;

                    if index_mask & BR_PREF_MASK != 0 {
                        midgame_score -= PREF_SQR_VAL;
                    }
                } else if index_mask & BR_AVOID_MASK != 0 {
                    midgame_score += AVOID_SQR_PEN;
                }

                let file_mask = file_masks[index];
                if file_mask & bitboard.b_pawn == 0 {
                    midgame_score -= ROOK_OPEN_LINE_VAL;

                    if file_mask & bitboard.w_pawn == 0
                    && file_mask & bitboard.w_rook == 0 {
                        midgame_score -= ROOK_OPEN_LINE_VAL;
                    }
                }
            },

            def::WQ => {
                base_score += Q_VAL;

                if index_mask & WQ_COMF_MASK != 0 {
                    midgame_score += COMF_SQR_VAL;
                } else if index_mask & WQ_AVOID_MASK != 0 {
                    midgame_score -= AVOID_SQR_PEN;
                }

                wq_count += 1;
            },
            def::BQ => {
                base_score -= Q_VAL;

                if index_mask & BQ_COMF_MASK != 0 {
                    midgame_score -= COMF_SQR_VAL;
                } else if index_mask & BQ_AVOID_MASK != 0 {
                    midgame_score += AVOID_SQR_PEN;
                }

                bq_count += 1;
            },

            def::WK => {
                base_score += K_VAL;

                if index_mask & K_ENDGAME_COMF_MASK != 0 {
                    endgame_score += COMF_SQR_VAL;
                }

                if (wk_protect_masks[index] & bitboard.w_pawn).count_ones() < 2 {
                    wk_safety_score -= KING_PROTECTED_VAL;
                } else if index_mask & WK_SAFE_MASK != 0 {
                    wk_safety_score += KING_SAFE_SPOT_VAL;

                    if file_masks[index] & bitboard.w_pawn != 0 {
                        wk_safety_score += KING_COVERED_VAL;
                    }
                }

                let file_mask = file_masks[index];
                if file_mask & bitboard.b_rook != 0 && file_mask & bitboard.b_pawn == 0 {
                    wk_safety_score -= KING_THREAT_VAL;
                }

                if index > 0 && def::is_index_valid(index - 1) {
                    let file_mask = file_masks[index - 1];
                    if file_mask & bitboard.b_rook != 0 && file_mask & bitboard.b_pawn == 0 && file_mask & bitboard.w_rook == 0 {
                        wk_safety_score -= KING_THREAT_VAL;
                    }
                }

                if def::is_index_valid(index + 1) {
                    let file_mask = file_masks[index + 1];
                    if file_mask & bitboard.b_rook != 0 && file_mask & bitboard.b_pawn == 0 && file_mask & bitboard.w_rook == 0 {
                        wk_safety_score -= KING_THREAT_VAL;
                    }
                }
            },
            def::BK => {
                base_score -= K_VAL;

                if index_mask & K_ENDGAME_COMF_MASK != 0 {
                    endgame_score -= COMF_SQR_VAL;
                }

                if (bk_protect_masks[index] & bitboard.b_pawn).count_ones() < 2 {
                    bk_safety_score += KING_PROTECTED_VAL;
                } else if index_mask & BK_SAFE_MASK != 0 {
                    bk_safety_score -= KING_SAFE_SPOT_VAL;

                    if file_masks[index] & bitboard.b_pawn != 0 {
                        bk_safety_score -= KING_COVERED_VAL;
                    }
                }

                let file_mask = file_masks[index];
                if file_mask & bitboard.w_rook != 0 && file_mask & bitboard.w_pawn == 0 {
                    bk_safety_score += KING_THREAT_VAL;
                }

                if index > 0 && def::is_index_valid(index - 1) {
                    let file_mask = file_masks[index - 1];
                    if file_mask & bitboard.w_rook != 0 && file_mask & bitboard.w_pawn == 0 && file_mask & bitboard.b_rook == 0 {
                        bk_safety_score += KING_THREAT_VAL;
                    }
                }

                if def::is_index_valid(index + 1) {
                    let file_mask = file_masks[index + 1];
                    if file_mask & bitboard.w_rook != 0 && file_mask & bitboard.w_pawn == 0 && file_mask & bitboard.b_rook == 0 {
                        bk_safety_score += KING_THREAT_VAL;
                    }
                }
            },
            _ => {},
        }

        index += 1;
    }

    let is_endgame =
        (w_piece_count < 3 || b_piece_count < 3)
        || ((wq_count == 0 || bq_count == 0) && ((w_piece_count <= 3 || b_piece_count <= 3) && (w_pawn_count <= 3 || b_pawn_count <= 3)));

    if is_endgame {
        if w_pawn_count + b_pawn_count == 0 && base_score.abs() < R_VAL {
            return 0
        }

        return base_score + endgame_score
    }

    if bq_count > 0 {
        midgame_score += wk_safety_score;
    }

    if wq_count > 0 {
        midgame_score += bk_safety_score;
    }

    base_score + midgame_score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bitboard::BitMask,
        state::State,
        prng::XorshiftPrng,
    };

    #[test]
    fn test_eval_1() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("5rk1/pbp1nppp/1bn2q2/3pp3/3P4/1BN1P3/PPP2PPP/R1BQ1RK1 b Q - 0 1", &zob_keys, &bitmask);
        assert_eq!(230, eval_state(&state));
    }

    #[test]
    fn test_eval_2() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("4k2r/pbppnppp/1bn5/4p3/2B5/2N1P3/PPPP1PPP/R1BQK2R b KQk - 0 1", &zob_keys, &bitmask);
        assert_eq!(1240, eval_state(&state));
    }

    #[test]
    fn test_eval_3() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("2q5/3p1rk1/3P1pp1/p3P2p/8/4Q3/5PPP/R5K1 w - - 0 1", &zob_keys, &bitmask);
        assert_eq!(-60, eval_state(&state));
    }

    #[test]
    fn test_eval_4() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("1r2k3/8/3P4/p3P2p/8/8/1R2K3/8 w - - 0 1", &zob_keys, &bitmask);
        assert_eq!(95, eval_state(&state));
    }

    #[test]
    fn test_eval_5() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("rnbqr1k1/pppp1ppp/5nb1/8/8/5NB1/PPPP1PPP/RNBQ1RK1 w Qq - 0 1", &zob_keys, &bitmask);
        assert_eq!(-40, eval_state(&state));
    }

    #[test]
    fn test_eval_6() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("bnrqnrkb/1pp2pp1/8/8/3pp3/8/PPP2PPP/BNRQNRKB w - - 0 1", &zob_keys, &bitmask);
        assert_eq!(-40, eval_state(&state));
    }

    #[test]
    fn test_eval_7() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("rnbqr1k1/pppppppp/5nb1/8/8/5NB1/PPPP1PPP/RNBQR1K1 w Qq - 0 1", &zob_keys, &bitmask);
        assert_eq!(-80, eval_state(&state));
    }

    #[test]
    fn test_eval_8() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("8/6k1/6b1/p3p1p1/P3p1P1/1P1p3P/8/2R3K1 b - - 1 39", &zob_keys, &bitmask);
        assert_eq!(-55, eval_state(&state));
    }

    #[test]
    fn test_eval_9() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("8/p6k/p4K1P/P4PP1/8/3b4/8/8 w - - 11 61", &zob_keys, &bitmask);
        assert_eq!(40, eval_state(&state));
    }
}
