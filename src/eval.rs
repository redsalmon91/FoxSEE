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

static KING_PROTECTED_VAL: i32 = 30;
static KING_COVERED_VAL: i32 = 30;
static KING_SAFE_SPOT_VAL: i32 = 30;
static KING_CAS_VAL: i32 = 30;
static KING_THREAT_VAL: i32 = 15;

static PASS_PAWN_VAL: i32 = 30;
static ENDGAME_PASS_PAWN_VAL: i32 = 10;
static DUP_PAWN_PEN: i32 = 20;
static ENDGAME_DUP_PAWN_PEN: i32 = 30;
static ISOLATE_PAWN_PEN: i32 = 20;

static ROOK_OPEN_LINE_VAL: i32 = 20;
static DOUBLE_BISHOP_VAL: i32 = 30;

static COMF_SQR_VAL: i32 = 10;
static PREF_SQR_VAL: i32 = 20;
static AVOID_SQR_PEN: i32 = 20;

static WIN_VAL: i32 = 500;
static ENDGAME_DYNAMIC_FACTOR: i32 = 30;

const W_FIFTH_RANK: usize = 63;
const B_FIFTH_RANK: usize = 56;

static WK_SAFE_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_11000011_11000110;
static BK_SAFE_MASK: u64 = 0b11000110_11000011_00000000_00000000_00000000_00000000_00000000_00000000;

static WQ_COMF_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_01111110_00111100_00000000;
static BQ_COMF_MASK: u64 = 0b00000000_00111100_01111110_00000000_00000000_00000000_00000000_00000000;
static WQ_AVOID_MASK: u64 = 0b11000011_11000011_11000011_10000001_10000001_10000001_11000011_11000011;
static BQ_AVOID_MASK: u64 = 0b11000011_11000011_10000001_10000001_10000001_11000011_11000011_11000011;

static WR_COMF_MASK: u64 = 0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00111100;
static BR_COMF_MASK: u64 = 0b00111100_00000000_00000000_00000000_00000000_00000000_11111111_11111111;
static WR_PREF_MASK: u64 = 0b00000000_00111100_00000000_00000000_00000000_00000000_00000000_00000000;
static BR_PREF_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00111100_00000000;
static WR_AVOID_MASK: u64 = 0b00000000_00000000_00000000_10000001_11000011_11000011_11100111_00000000;
static BR_AVOID_MASK: u64 = 0b00000000_11100111_11000011_11000011_10000001_00000000_00000000_00000000;

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

static WP_COMF_MASK: u64 = 0b00000000_01111110_01111110_01111110_00111100_11000011_11100111_00000000;
static BP_COMF_MASK: u64 = 0b00000000_11100111_11000011_00111100_01111110_01111110_01111110_00000000;
static WP_PREF_MASK: u64 = 0b00000000_01111110_01111110_00011000_00000000_00000000_00000000_00000000;
static BP_PREF_MASK: u64 = 0b00000000_00000000_00000000_00000000_00011000_01111110_01111110_00000000;

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
    let bitboard = state.bitboard;

    let w_pawn_count = bitboard.w_pawn.count_ones();
    let b_pawn_count = bitboard.b_pawn.count_ones();
    let w_piece_count = bitboard.w_all.count_ones() - w_pawn_count - 1;
    let b_piece_count = bitboard.b_all.count_ones() - b_pawn_count - 1;

    if (w_piece_count < 2 || b_piece_count < 2) || ((w_piece_count <= 3 && w_pawn_count < 5) || (b_piece_count <= 3 && b_pawn_count < 5)) {
        eval_endgame(state, w_piece_count, b_piece_count, w_pawn_count, b_pawn_count)
    } else {
        eval_midgame(state)
    }
}

pub fn eval_midgame(state: &State) -> i32 {
    let squares = state.squares;
    let index_masks = state.bitmask.index_masks;
    let file_masks = state.bitmask.file_masks;
    let wk_protect_masks = state.bitmask.wk_protect_masks;
    let bk_protect_masks = state.bitmask.bk_protect_masks;
    let wp_forward_masks = state.bitmask.wp_forward_masks;
    let bp_forward_masks = state.bitmask.bp_forward_masks;
    let nearby_masks = state.bitmask.nearby_masks;
    let bitboard = state.bitboard;

    let mut index = 0;
    let mut base_score = 0;
    let mut midgame_score = 0;
    let mut wk_safety_score = 0;
    let mut bk_safety_score = 0;

    let mut wq_count = 0;
    let mut bq_count = 0;

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

                if index > W_FIFTH_RANK && wp_forward_masks[index] & b_pawn_mask == 0 {
                    let pass_pawn_val = PASS_PAWN_VAL * (def::get_rank(index) as i32 - 4);

                    midgame_score += pass_pawn_val;

                    if nearby_masks[index] & w_pawn_mask != 0 {
                        midgame_score += pass_pawn_val;
                    }
                } else if nearby_masks[index] & w_pawn_mask == 0 {
                    midgame_score -= ISOLATE_PAWN_PEN;

                    if (file_masks[index] & w_pawn_mask).count_ones() > 1 {
                        midgame_score -= DUP_PAWN_PEN;
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

                if index < B_FIFTH_RANK && bp_forward_masks[index] & w_pawn_mask == 0 {
                    let pass_pawn_val = PASS_PAWN_VAL * (5 - def::get_rank(index) as i32);

                    midgame_score -= pass_pawn_val;

                    if nearby_masks[index] & b_pawn_mask != 0 {
                        midgame_score -= pass_pawn_val;
                    }
                } else if nearby_masks[index] & b_pawn_mask == 0 {
                    midgame_score += ISOLATE_PAWN_PEN;

                    if (file_masks[index] & b_pawn_mask).count_ones() > 1 {
                        midgame_score += DUP_PAWN_PEN;
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
                let file_mask = file_masks[index];

                if (wk_protect_masks[index] & bitboard.w_pawn).count_ones() < 2 {
                    wk_safety_score -= KING_PROTECTED_VAL;
                } else if index_mask & WK_SAFE_MASK != 0 {
                    wk_safety_score += KING_SAFE_SPOT_VAL;

                    if file_mask & bitboard.w_pawn != 0 {
                        wk_safety_score += KING_COVERED_VAL;
                    }
                }

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
                let file_mask = file_masks[index];

                if (bk_protect_masks[index] & bitboard.b_pawn).count_ones() < 2 {
                    bk_safety_score += KING_PROTECTED_VAL;
                } else if index_mask & BK_SAFE_MASK != 0 {
                    bk_safety_score -= KING_SAFE_SPOT_VAL;

                    if file_mask & bitboard.b_pawn != 0 {
                        bk_safety_score -= KING_COVERED_VAL;
                    }
                }

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

    if bq_count > 0 {
        if state.w_castled || state.cas_rights & 0b1100 != 0 {
            wk_safety_score += KING_CAS_VAL;
        }

        midgame_score += wk_safety_score;
    }

    if wq_count > 0 {
        if state.b_castled || state.cas_rights & 0b0011 != 0 {
            bk_safety_score -= KING_CAS_VAL;
        }

        midgame_score += bk_safety_score;
    }

    base_score + midgame_score
}

pub fn eval_endgame(state: &State, w_piece_count: u32, b_piece_count: u32, w_pawn_count: u32, b_pawn_count: u32) -> i32 {
    let squares = state.squares;
    let index_masks = state.bitmask.index_masks;
    let file_masks = state.bitmask.file_masks;
    let wk_protect_masks = state.bitmask.wk_protect_masks;
    let bk_protect_masks = state.bitmask.bk_protect_masks;
    let wp_forward_masks = state.bitmask.wp_forward_masks;
    let bp_forward_masks = state.bitmask.bp_forward_masks;
    let nearby_masks = state.bitmask.nearby_masks;
    let bitboard = state.bitboard;

    let mut index = 0;
    let mut base_score = 0;
    let mut endgame_score = 0;
    let mut wk_safety_score = 0;
    let mut bk_safety_score = 0;

    let mut wq_count = 0;
    let mut bq_count = 0;
    let mut wb_count = 0;
    let mut bb_count = 0;

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

                let w_pawn_mask = bitboard.w_pawn;
                let b_pawn_mask = bitboard.b_pawn;

                if wp_forward_masks[index] & b_pawn_mask == 0 {
                    let pass_pawn_val = ENDGAME_PASS_PAWN_VAL * (def::get_rank(index) as i32);

                    endgame_score += pass_pawn_val;

                    if nearby_masks[index] & w_pawn_mask != 0 {
                        endgame_score += pass_pawn_val;
                    }
                } else if nearby_masks[index] & w_pawn_mask == 0 {
                    if (file_masks[index] & w_pawn_mask).count_ones() > 1 {
                        endgame_score -= ENDGAME_DUP_PAWN_PEN;
                    }
                }
            },
            def::BP => {
                base_score -= P_VAL;

                let w_pawn_mask = bitboard.w_pawn;
                let b_pawn_mask = bitboard.b_pawn;

                if bp_forward_masks[index] & w_pawn_mask == 0 {
                    let pass_pawn_val = ENDGAME_PASS_PAWN_VAL * (8 - def::get_rank(index) as i32);

                    endgame_score -= pass_pawn_val;

                    if nearby_masks[index] & b_pawn_mask != 0 {
                        endgame_score -= pass_pawn_val;
                    }
                } else if nearby_masks[index] & b_pawn_mask == 0 {
                    if (file_masks[index] & b_pawn_mask).count_ones() > 1 {
                        endgame_score += ENDGAME_DUP_PAWN_PEN;
                    }
                }
            },

            def::WN => {
                base_score += N_VAL;
            },
            def::BN => {
                base_score -= N_VAL;
            },

            def::WB => {
                base_score += B_VAL;
                wb_count += 1;
            },
            def::BB => {
                base_score -= B_VAL;
                bb_count += 1;
            },

            def::WR => {
                base_score += R_VAL;
            },
            def::BR => {
                base_score -= R_VAL;
            },

            def::WQ => {
                base_score += Q_VAL;
                wq_count += 1;
            },
            def::BQ => {
                base_score -= Q_VAL;
                bq_count += 1;
            },

            def::WK => {
                base_score += K_VAL;
                let file_mask = file_masks[index];

                if index_mask & K_ENDGAME_COMF_MASK != 0 {
                    endgame_score += COMF_SQR_VAL;
                }

                if (wk_protect_masks[index] & bitboard.w_pawn).count_ones() < 2 {
                    wk_safety_score -= KING_PROTECTED_VAL;
                } else if index_mask & WK_SAFE_MASK != 0 {
                    wk_safety_score += KING_SAFE_SPOT_VAL;

                    if file_mask & bitboard.w_pawn != 0 {
                        wk_safety_score += KING_COVERED_VAL;
                    }
                }

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
                let file_mask = file_masks[index];

                if index_mask & K_ENDGAME_COMF_MASK != 0 {
                    endgame_score -= COMF_SQR_VAL;
                }

                if (bk_protect_masks[index] & bitboard.b_pawn).count_ones() < 2 {
                    bk_safety_score += KING_PROTECTED_VAL;
                } else if index_mask & BK_SAFE_MASK != 0 {
                    bk_safety_score -= KING_SAFE_SPOT_VAL;

                    if file_mask & bitboard.b_pawn != 0 {
                        bk_safety_score -= KING_COVERED_VAL;
                    }
                }

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

    if base_score > 0 {
        endgame_score -= b_piece_count as i32 * ENDGAME_DYNAMIC_FACTOR;
    } else if base_score < 0 {
        endgame_score += w_piece_count as i32 * ENDGAME_DYNAMIC_FACTOR;
    }

    if wb_count > 1 {
        endgame_score += DOUBLE_BISHOP_VAL;
    }

    if bb_count > 1 {
        endgame_score -= DOUBLE_BISHOP_VAL;
    }

    if bq_count > 0 {
        endgame_score += wk_safety_score;
    }

    if wq_count > 0 {
        endgame_score += bk_safety_score;
    }

    if w_pawn_count + b_pawn_count == 0 && base_score.abs() < WIN_VAL {
        return 0
    }

    base_score + endgame_score
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

        let state = State::new("5rk1/pbp1nppp/1bn2q2/3pp3/3P4/1BN1P3/PPP2PPP/R1BQ1RK1 b KQkq - 0 1", &zob_keys, &bitmask);
        assert_eq!(270, eval_state(&state));
    }

    #[test]
    fn test_eval_2() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("4k2r/pbppnppp/1bn5/4p3/2B5/2N1P3/PPPP1PPP/R1BQK2R b KQk - 0 1", &zob_keys, &bitmask);
        assert_eq!(1250, eval_state(&state));
    }

    #[test]
    fn test_eval_3() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("2q5/3p1rk1/3P1pp1/p3P2p/8/4Q3/5PPP/R5K1 w - - 0 1", &zob_keys, &bitmask);
        assert_eq!(170, eval_state(&state));
    }

    #[test]
    fn test_eval_4() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("1r2k3/8/3P4/p3P2p/8/8/1R2K3/8 w - - 0 1", &zob_keys, &bitmask);
        assert_eq!(160, eval_state(&state));
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

        let state = State::new("8/6k1/6b1/p5p1/P4pP1/1P1pp2P/8/2R3K1 b - - 1 39", &zob_keys, &bitmask);
        assert_eq!(-235, eval_state(&state));
    }

    #[test]
    fn test_eval_8() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("8/6k1/6b1/p3p1p1/P3p1P1/1P1p3P/8/2R3K1 b - - 1 39", &zob_keys, &bitmask);
        assert_eq!(-165, eval_state(&state));
    }

    #[test]
    fn test_eval_9() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("8/p6k/p4K1P/P4PP1/8/3b4/8/8 w - - 11 61", &zob_keys, &bitmask);
        assert_eq!(240, eval_state(&state));
    }

    #[test]
    fn test_eval_10() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("r1b1r1k1/2p1bppp/p1P5/3nN3/3P4/1qN2Q2/1P3PPP/R1B1R1K1 b - - 2 19", &zob_keys, &bitmask);
        assert_eq!(185, eval_state(&state));
    }

    #[test]
    fn test_eval_11() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("r6k/2R4p/p1P1rp1q/4p3/8/3Q4/5PPP/5RK1 b - - 1 32", &zob_keys, &bitmask);
        assert_eq!(130, eval_state(&state));
    }

    #[test]
    fn test_eval_12() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("rnbqkbnr/p1pppppp/8/8/8/2P5/P1P1PPPP/RNBQKBNR w KQkq - 0 1", &zob_keys, &bitmask);
        assert_eq!(-80, eval_state(&state));
    }

    #[test]
    fn test_eval_13() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("rnbqkbnr/p1pppppp/8/2P5/2P5/8/P3PPPP/RNBQKBNR w KQkq - 0 1", &zob_keys, &bitmask);
        assert_eq!(-70, eval_state(&state));
    }

    #[test]
    fn test_eval_14() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("r1bq1rk1/p1pppppp/2n5/2PN4/2P5/8/P3PPPP/R1BQ1RK1 w Qq - 0 1", &zob_keys, &bitmask);
        assert_eq!(-50, eval_state(&state));
    }

    #[test]
    fn test_eval_15() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("r1bq1rk1/p1ppp1p1/2n5/8/8/2N5/P1PPPP1P/R1BQ1RK1 w Qq - 0 1", &zob_keys, &bitmask);
        assert_eq!(150, eval_state(&state));
    }

    #[test]
    fn test_eval_16() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("r1bq1r1k/3ppppp/P1n5/1P6/8/2N5/5PPP/R1BQ1RK1 w Qq - 0 1", &zob_keys, &bitmask);
        assert_eq!(170, eval_state(&state));
    }

    #[test]
    fn test_eval_17() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();

        let state = State::new("r1bq1r1k/5ppp/2n5/8/1p6/p1N5/3PPPPP/R1BQ1RK1 w Qq - 0 1", &zob_keys, &bitmask);
        assert_eq!(-170, eval_state(&state));
    }
}
