use crate::{
    def,
    state::State,
    mov_gen::MoveGenerator,
};

pub static TERM_VAL: i32 = 10000;
pub static K_VAL: i32 = 20000;

static Q_VAL: i32 = 1000;
static R_VAL: i32 = 525;
static B_VAL: i32 = 350;
static N_VAL: i32 = 350;
static P_VAL: i32 = 100;

static KING_COVERED_BASE_VAL: i32 = 10;
static KING_SAFE_SPOT_BASE_VAL: i32 = 20;

static MIDGAME_PASS_PAWN_VAL: i32 = 10;
static ENDGMAE_PASS_PAWN_VAL: i32 = 20;
static ENDGMAE_SEMI_PASS_PAWN_VAL: i32 = 10;

static DUP_PAWN_PEN: i32 = 20;
static ISOLATE_PAWN_PEN: i32 = 20;

static ROOK_OPEN_LINE_VAL: i32 = 20;
static DOUBLE_BISHOP_VAL: i32 = 30;

static COMF_SQR_VAL: i32 = 10;
static PREF_SQR_VAL: i32 = 20;
static DANGER_SQR_VAL: i32 = 20;
static AVOID_SQR_PEN: i32 = 30;

static WK_SAFE_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_11000011_11000110;
static BK_SAFE_MASK: u64 = 0b11000110_11000011_00000000_00000000_00000000_00000000_00000000_00000000;

static WQ_COMF_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_01111110_00111100_00000000;
static BQ_COMF_MASK: u64 = 0b00000000_00111100_01111110_00000000_00000000_00000000_00000000_00000000;
static WQ_AVOID_MASK: u64 = 0b11000011_11000011_11000011_10011001_10011001_10000001_11000011_11000011;
static BQ_AVOID_MASK: u64 = 0b11000011_11000011_10000001_10011001_10011001_11000011_11000011_11000011;

static WR_COMF_MASK: u64 = 0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00111100;
static BR_COMF_MASK: u64 = 0b00111100_00000000_00000000_00000000_00000000_00000000_11111111_11111111;
static WR_PREF_MASK: u64 = 0b00000000_00111100_00000000_00000000_00000000_00000000_00000000_00000000;
static BR_PREF_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00111100_00000000;
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

static WK_ENDGAME_PREF_MASK: u64 = 0b00000000_00000000_00011000_00111100_00111100_00011000_00000000_00000000;
static BK_ENDGAME_PREF_MASK: u64 = 0b00000000_00000000_00011000_00111100_00111100_00011000_00000000_00000000;

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

pub fn eval_state(state: &State, mov_generator: &MoveGenerator) -> i32 {
    let bitboard = state.bitboard;

    let w_pawn_count = bitboard.w_pawn.count_ones();
    let b_pawn_count = bitboard.b_pawn.count_ones();
    let w_piece_count = bitboard.w_all.count_ones() - w_pawn_count - 1;
    let b_piece_count = bitboard.b_all.count_ones() - b_pawn_count - 1;

    if (w_piece_count < 2 || b_piece_count < 2) || ((w_piece_count <= 3 && w_pawn_count < 5) || (b_piece_count <= 3 && b_pawn_count < 5)) {
        eval_endgame(state, mov_generator, w_piece_count, b_piece_count, w_pawn_count, b_pawn_count)
    } else {
        eval_midgame(state, mov_generator, w_piece_count, b_piece_count)
    }
}

pub fn eval_midgame(state: &State, mov_generator: &MoveGenerator, w_piece_count: u32, b_piece_count: u32) -> i32 {
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
    let mut pawn_score = 0;
    let mut wk_safety_score = 0;
    let mut bk_safety_score = 0;
    let mut mob_score = 0;

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
                    pawn_score += COMF_SQR_VAL;

                    if index_mask & WP_PREF_MASK != 0 {
                        pawn_score += PREF_SQR_VAL;

                        if index_mask & WP_DANGER_MASK != 0 {
                            pawn_score += DANGER_SQR_VAL;
                        }
                    }
                }

                let w_pawn_mask = bitboard.w_pawn;
                let rank = def::get_w_rank(index);

                if wp_forward_masks[index] & bitboard.b_pawn == 0 {
                    pawn_score += MIDGAME_PASS_PAWN_VAL * rank as i32;
                } else if nearby_masks[index] & w_pawn_mask == 0 {
                    pawn_score -= ISOLATE_PAWN_PEN;

                    if (file_masks[index] & w_pawn_mask).count_ones() > 1 {
                        pawn_score -= DUP_PAWN_PEN;
                    }
                }
            },
            def::BP => {
                base_score -= P_VAL;

                if index_mask & BP_COMF_MASK != 0 {
                    pawn_score -= COMF_SQR_VAL;

                    if index_mask & BP_PREF_MASK != 0 {
                        pawn_score -= PREF_SQR_VAL;

                        if index_mask & BP_DANGER_MASK != 0 {
                            pawn_score -= DANGER_SQR_VAL;
                        }
                    }
                }

                let b_pawn_mask = bitboard.b_pawn;
                let rank = def::get_b_rank(index);

                if bp_forward_masks[index] & bitboard.w_pawn == 0 {
                    pawn_score -= MIDGAME_PASS_PAWN_VAL * rank as i32;
                } else if nearby_masks[index] & b_pawn_mask == 0 {
                    pawn_score += ISOLATE_PAWN_PEN;

                    if (file_masks[index] & b_pawn_mask).count_ones() > 1 {
                        pawn_score += DUP_PAWN_PEN;
                    }
                }
            },

            def::WN => {
                base_score += N_VAL;
                mob_score += mov_generator.count_knight_mobility(state, index, def::PLAYER_W);

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
                mob_score -= mov_generator.count_knight_mobility(state, index, def::PLAYER_B);

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
                mob_score += mov_generator.count_bishop_mobility(state, index, def::PLAYER_W);

                if index_mask & WB_COMF_MASK != 0 {
                    midgame_score += COMF_SQR_VAL;
                } else if index_mask & WB_AVOID_MASK != 0 {
                    midgame_score -= AVOID_SQR_PEN;
                }
            },
            def::BB => {
                base_score -= B_VAL;
                mob_score -= mov_generator.count_bishop_mobility(state, index, def::PLAYER_B);

                if index_mask & BB_COMF_MASK != 0 {
                    midgame_score -= COMF_SQR_VAL;
                } else if index_mask & BB_AVOID_MASK != 0 {
                    midgame_score += AVOID_SQR_PEN;
                }
            },

            def::WR => {
                base_score += R_VAL;
                mob_score += mov_generator.count_rook_mobility(state, index, def::PLAYER_W);

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
                mob_score -= mov_generator.count_rook_mobility(state, index, def::PLAYER_B);

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
                let protect_mask = wk_protect_masks[index];

                if index_mask & WK_SAFE_MASK == 0 && state.cas_rights & 0b1100 == 0 {
                    wk_safety_score -= KING_SAFE_SPOT_BASE_VAL * b_piece_count as i32;
                }

                if file_mask & protect_mask & bitboard.w_pawn == 0 {
                    wk_safety_score -= KING_COVERED_BASE_VAL * b_piece_count as i32;
                }

                if protect_mask.count_ones() < 2 {
                    wk_safety_score -= KING_COVERED_BASE_VAL * b_piece_count as i32;
                }
            },
            def::BK => {
                base_score -= K_VAL;
                let file_mask = file_masks[index];
                let protect_mask = bk_protect_masks[index];

                if index_mask & BK_SAFE_MASK == 0 && state.cas_rights & 0b0011 == 0 {
                    bk_safety_score += KING_SAFE_SPOT_BASE_VAL * w_piece_count as i32;
                }

                if file_mask & protect_mask & bitboard.b_pawn == 0 {
                    bk_safety_score += KING_COVERED_BASE_VAL * w_piece_count as i32;
                }

                if protect_mask.count_ones() < 2 {
                    bk_safety_score += KING_COVERED_BASE_VAL * w_piece_count as i32;
                }
            },
            _ => {},
        }

        index += 1;
    }

    if bq_count > 0 {
        midgame_score += wk_safety_score;
    }

    if wq_count > 0 {
        midgame_score += bk_safety_score;
    }

    base_score + midgame_score + pawn_score + mob_score
}

pub fn eval_endgame(state: &State, mov_generator: &MoveGenerator, w_piece_count: u32, b_piece_count: u32, w_pawn_count: u32, b_pawn_count: u32) -> i32 {
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
    let mut pawn_score = 0;
    let mut wk_safety_score = 0;
    let mut bk_safety_score = 0;
    let mut mob_score = 0;

    let mut wq_count = 0;
    let mut bq_count = 0;
    let mut wr_count = 0;
    let mut br_count = 0;
    let mut wb_count = 0;
    let mut bb_count = 0;
    let mut wn_count = 0;
    let mut bn_count = 0;

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
                let rank = def::get_w_rank(index);

                let pawn_blocker_mask = wp_forward_masks[index] & bitboard.b_pawn;

                if pawn_blocker_mask == 0 {
                    pawn_score += ENDGMAE_PASS_PAWN_VAL * rank as i32;
                } else {
                    if pawn_blocker_mask.count_ones() == 1 {
                        pawn_score += ENDGMAE_SEMI_PASS_PAWN_VAL;
                    }

                    if nearby_masks[index] & w_pawn_mask == 0 {
                        pawn_score -= ISOLATE_PAWN_PEN;
    
                        if (file_masks[index] & w_pawn_mask).count_ones() > 1 {
                            pawn_score -= DUP_PAWN_PEN;
                        }
                    }
                }
            },
            def::BP => {
                base_score -= P_VAL;

                let b_pawn_mask = bitboard.b_pawn;
                let rank = def::get_b_rank(index);

                let pawn_blocker_mask = bp_forward_masks[index] & bitboard.w_pawn;

                if pawn_blocker_mask == 0 {
                    pawn_score -= ENDGMAE_PASS_PAWN_VAL * rank as i32;
                } else {
                    if pawn_blocker_mask.count_ones() == 1 {
                        pawn_score -= ENDGMAE_SEMI_PASS_PAWN_VAL;
                    }

                    if nearby_masks[index] & b_pawn_mask == 0 {
                        pawn_score += ISOLATE_PAWN_PEN;
    
                        if (file_masks[index] & b_pawn_mask).count_ones() > 1 {
                            pawn_score += DUP_PAWN_PEN;
                        }
                    }
                }
            },

            def::WN => {
                base_score += N_VAL;
                mob_score += mov_generator.count_knight_mobility(state, index, def::PLAYER_W);

                wn_count += 1;
            },
            def::BN => {
                base_score -= N_VAL;
                mob_score -= mov_generator.count_knight_mobility(state, index, def::PLAYER_B);

                bn_count += 1;
            },

            def::WB => {
                base_score += B_VAL;
                mob_score += mov_generator.count_bishop_mobility(state, index, def::PLAYER_W);

                wb_count += 1;
            },
            def::BB => {
                base_score -= B_VAL;
                mob_score -= mov_generator.count_bishop_mobility(state, index, def::PLAYER_B);

                bb_count += 1;
            },

            def::WR => {
                base_score += R_VAL;
                mob_score += mov_generator.count_rook_mobility(state, index, def::PLAYER_W);

                wr_count += 1;
            },
            def::BR => {
                base_score -= R_VAL;
                mob_score -= mov_generator.count_rook_mobility(state, index, def::PLAYER_B);

                br_count += 1;
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
                let protect_mask = wk_protect_masks[index];

                if index_mask & WK_ENDGAME_COMF_MASK != 0 {
                    endgame_score += COMF_SQR_VAL;

                    if index_mask & WK_ENDGAME_PREF_MASK != 0 {
                        endgame_score += PREF_SQR_VAL;
                    }
                }

                if index_mask & WK_SAFE_MASK == 0 {
                    wk_safety_score -= KING_SAFE_SPOT_BASE_VAL * b_piece_count as i32;
                }

                if file_mask & protect_mask & bitboard.w_pawn == 0 {
                    wk_safety_score -= KING_COVERED_BASE_VAL * b_piece_count as i32;
                }

                if protect_mask.count_ones() < 2 {
                    wk_safety_score -= KING_COVERED_BASE_VAL * b_piece_count as i32;
                }
            },
            def::BK => {
                base_score -= K_VAL;
                let file_mask = file_masks[index];
                let protect_mask = bk_protect_masks[index];

                if index_mask & BK_ENDGAME_COMF_MASK != 0 {
                    endgame_score -= COMF_SQR_VAL;

                    if index_mask & BK_ENDGAME_PREF_MASK != 0 {
                        endgame_score -= PREF_SQR_VAL;
                    }
                }

                if index_mask & BK_SAFE_MASK == 0 {
                    bk_safety_score += KING_SAFE_SPOT_BASE_VAL * w_piece_count as i32;
                }

                if file_mask & protect_mask & bitboard.b_pawn == 0 {
                    bk_safety_score += KING_COVERED_BASE_VAL * w_piece_count as i32;
                }

                if protect_mask.count_ones() < 2 {
                    bk_safety_score += KING_COVERED_BASE_VAL * w_piece_count as i32;
                }
            },
            _ => {},
        }

        index += 1;
    }

    if base_score > 0 {
        if w_pawn_count + b_pawn_count == 0 && wq_count == 0 && wr_count == 0 && wn_count + wb_count < 2 {
            return 0
        }
    } else if base_score < 0 {
        if b_pawn_count + w_pawn_count == 0 && bq_count == 0 && br_count == 0 && bn_count + bb_count < 2 {
            return 0
        }
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

    base_score + endgame_score + pawn_score + mob_score
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
    fn test_eval_midgame() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mov_generator = MoveGenerator::new();

        let state = State::new("r4rk1/pp2nppp/1bn2q2/3pp3/3P4/1BN1P3/PPP1NP1P/R2Q1RK1 b Qf - 0 1", &zob_keys, &bitmask);
        assert_eq!(-127, eval_state(&state, &mov_generator));
    }

    #[test]
    fn test_eval_endgame() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mov_generator = MoveGenerator::new();

        let state = State::new("5rk1/1bp2ppp/1b6/3p4/3PpP2/8/1PP3PP/R4RK1 b Qq - 0 1", &zob_keys, &bitmask);
        assert_eq!(-261, eval_state(&state, &mov_generator));
    }

    #[test]
    fn test_eval_king_safety() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mov_generator = MoveGenerator::new();

        let state = State::new("5rk1/pp3ppp/5q2/3p4/3Pp3/2Q1P3/PPP2P1P/5RK1 b q - 0 1", &zob_keys, &bitmask);
        assert_eq!(-70, eval_state(&state, &mov_generator));
    }
}
