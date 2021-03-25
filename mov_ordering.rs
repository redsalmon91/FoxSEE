use crate::{
    def,
    state::State,
    util::{get_lowest_index, get_highest_index}
};

const P_THREAT: i32 = 10;
const NB_THREAT: i32 = 30;
const R_THREAT: i32 = 50;
const Q_THREAT: i32 = 100;
const K_THREAT: i32 = 200;

const NBP_THREAT: i32 = 20;
const RP_THREAT: i32 = 40;
const QP_THREAT: i32 = 90;

const RNB_THREAT: i32 = 15;
const QNB_THREAT: i32 = 65;
const QR_THREAT: i32 = 50;

pub fn get_threat_val(state: &State) -> (bool, i32) {
    let squares = state.squares;
    let bitboard = state.bitboard;
    let bitmask = state.bitmask;

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

    let occupy_mask = bitboard.w_all | bitboard.b_all;
    let start_index = occupy_mask.trailing_zeros() as usize;
    let end_index = def::BOARD_SIZE - occupy_mask.leading_zeros() as usize;

    for index in start_index..end_index {
        let moving_piece = squares[index];

        if moving_piece == 0 {
            continue
        }
 
        match moving_piece {
            def::WP => {
                wp_attack_mask |= bitmask.wp_attack_masks[index];
            },
            def::BP => {
                bp_attack_mask |= bitmask.bp_attack_masks[index];
            },

            def::WN => {
                let mov_mask = bitmask.n_attack_masks[index];
                wn_attack_mask |= mov_mask;
            },
            def::BN => {
                let mov_mask = bitmask.n_attack_masks[index];
                bn_attack_mask |= mov_mask;
            },

            def::WB => {
                let mut mov_mask = 0;

                let up_left_attack_mask = bitmask.up_left_attack_masks[index];
                mov_mask ^= up_left_attack_mask;
                if up_left_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_left_attack_masks[lowest_blocker_index];
                }

                let up_right_attack_mask = bitmask.up_right_attack_masks[index];
                mov_mask ^= up_right_attack_mask;
                if up_right_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_right_attack_masks[lowest_blocker_index];
                }

                let down_left_attack_mask = bitmask.down_left_attack_masks[index];
                mov_mask ^= down_left_attack_mask;
                if down_left_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_left_attack_masks[highest_blocker_index];
                }

                let down_right_attack_mask = bitmask.down_right_attack_masks[index];
                mov_mask ^= down_right_attack_mask;
                if down_right_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_right_attack_masks[highest_blocker_index];
                }

                wb_attack_mask |= mov_mask;
            },
            def::BB => {
                let mut mov_mask = 0;

                let up_left_attack_mask = bitmask.up_left_attack_masks[index];
                mov_mask ^= up_left_attack_mask;
                if up_left_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_left_attack_masks[lowest_blocker_index];
                }

                let up_right_attack_mask = bitmask.up_right_attack_masks[index];
                mov_mask ^= up_right_attack_mask;
                if up_right_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_right_attack_masks[lowest_blocker_index];
                }

                let down_left_attack_mask = bitmask.down_left_attack_masks[index];
                mov_mask ^= down_left_attack_mask;
                if down_left_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_left_attack_masks[highest_blocker_index];
                }

                let down_right_attack_mask = bitmask.down_right_attack_masks[index];
                mov_mask ^= down_right_attack_mask;
                if down_right_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_right_attack_masks[highest_blocker_index];
                }

                bb_attack_mask |= mov_mask;
            },

            def::WR => {
                let mut mov_mask = 0;

                let up_attack_mask = bitmask.up_attack_masks[index];
                mov_mask ^= up_attack_mask;
                if up_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_attack_masks[lowest_blocker_index];
                }

                let right_attack_mask = bitmask.right_attack_masks[index];
                mov_mask ^= right_attack_mask;
                if right_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.right_attack_masks[lowest_blocker_index];
                }

                let down_attack_mask = bitmask.down_attack_masks[index];
                mov_mask ^= down_attack_mask;
                if down_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_attack_masks[highest_blocker_index];
                }

                let left_attack_mask = bitmask.left_attack_masks[index];
                mov_mask ^= left_attack_mask;
                if left_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.left_attack_masks[highest_blocker_index];
                }

                wr_attack_mask |= mov_mask;
            },
            def::BR => {
                let mut mov_mask = 0;

                let up_attack_mask = bitmask.up_attack_masks[index];
                mov_mask ^= up_attack_mask;
                if up_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_attack_masks[lowest_blocker_index];
                }

                let right_attack_mask = bitmask.right_attack_masks[index];
                mov_mask ^= right_attack_mask;
                if right_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.right_attack_masks[lowest_blocker_index];
                }

                let down_attack_mask = bitmask.down_attack_masks[index];
                mov_mask ^= down_attack_mask;
                if down_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_attack_masks[highest_blocker_index];
                }

                let left_attack_mask = bitmask.left_attack_masks[index];
                mov_mask ^= left_attack_mask;
                if left_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.left_attack_masks[highest_blocker_index];
                }

                br_attack_mask |= mov_mask;
            },

            def::WQ => {
                let mut mov_mask = 0;

                let up_left_attack_mask = bitmask.up_left_attack_masks[index];
                mov_mask ^= up_left_attack_mask;
                if up_left_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_left_attack_masks[lowest_blocker_index];
                }

                let up_right_attack_mask = bitmask.up_right_attack_masks[index];
                mov_mask ^= up_right_attack_mask;
                if up_right_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_right_attack_masks[lowest_blocker_index];
                }

                let down_left_attack_mask = bitmask.down_left_attack_masks[index];
                mov_mask ^= down_left_attack_mask;
                if down_left_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_left_attack_masks[highest_blocker_index];
                }

                let down_right_attack_mask = bitmask.down_right_attack_masks[index];
                mov_mask ^= down_right_attack_mask;
                if down_right_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_right_attack_masks[highest_blocker_index];
                }

                let up_attack_mask = bitmask.up_attack_masks[index];
                mov_mask ^= up_attack_mask;
                if up_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_attack_masks[lowest_blocker_index];
                }

                let right_attack_mask = bitmask.right_attack_masks[index];
                mov_mask ^= right_attack_mask;
                if right_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.right_attack_masks[lowest_blocker_index];
                }

                let down_attack_mask = bitmask.down_attack_masks[index];
                mov_mask ^= down_attack_mask;
                if down_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_attack_masks[highest_blocker_index];
                }

                let left_attack_mask = bitmask.left_attack_masks[index];
                mov_mask ^= left_attack_mask;
                if left_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.left_attack_masks[highest_blocker_index];
                }

                wq_attack_mask |= mov_mask;
            },
            def::BQ => {
                let mut mov_mask = 0;

                let up_left_attack_mask = bitmask.up_left_attack_masks[index];
                mov_mask ^= up_left_attack_mask;
                if up_left_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_left_attack_masks[lowest_blocker_index];
                }

                let up_right_attack_mask = bitmask.up_right_attack_masks[index];
                mov_mask ^= up_right_attack_mask;
                if up_right_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_right_attack_masks[lowest_blocker_index];
                }

                let down_left_attack_mask = bitmask.down_left_attack_masks[index];
                mov_mask ^= down_left_attack_mask;
                if down_left_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_left_attack_masks[highest_blocker_index];
                }

                let down_right_attack_mask = bitmask.down_right_attack_masks[index];
                mov_mask ^= down_right_attack_mask;
                if down_right_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_right_attack_masks[highest_blocker_index];
                }

                let up_attack_mask = bitmask.up_attack_masks[index];
                mov_mask ^= up_attack_mask;
                if up_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.up_attack_masks[lowest_blocker_index];
                }

                let right_attack_mask = bitmask.right_attack_masks[index];
                mov_mask ^= right_attack_mask;
                if right_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(right_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.right_attack_masks[lowest_blocker_index];
                }

                let down_attack_mask = bitmask.down_attack_masks[index];
                mov_mask ^= down_attack_mask;
                if down_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.down_attack_masks[highest_blocker_index];
                }

                let left_attack_mask = bitmask.left_attack_masks[index];
                mov_mask ^= left_attack_mask;
                if left_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(left_attack_mask & occupy_mask);
                    mov_mask &= !bitmask.left_attack_masks[highest_blocker_index];
                }

                bq_attack_mask |= mov_mask;
            },
            _ => {}
        }
    }

    let w_attack_mask = wp_attack_mask | wn_attack_mask | wb_attack_mask | wr_attack_mask | wq_attack_mask;
    let wk_ring_mask = bitmask.k_attack_masks[state.wk_index];
    let w_defense_mask = w_attack_mask | wk_ring_mask;

    let b_attack_mask = bp_attack_mask | bn_attack_mask | bb_attack_mask | br_attack_mask | bq_attack_mask;
    let bk_ring_mask = bitmask.k_attack_masks[state.bk_index];
    let b_defense_mask = b_attack_mask | bk_ring_mask;

    let (in_check, score_sign) = if state.player == def::PLAYER_W {
      (bitmask.index_masks[state.wk_index] & b_attack_mask != 0, -1)
    } else {
      (bitmask.index_masks[state.bk_index] & w_attack_mask != 0, 1)
    };

    if in_check {
      return (true, K_THREAT * score_sign)
    }

    let mut threat_point = 0;

    threat_point += (bitboard.b_pawn & w_attack_mask & !b_defense_mask).count_ones() as i32 * P_THREAT;

    threat_point += (bitboard.b_knight & (w_attack_mask & !b_defense_mask)).count_ones() as i32 * NB_THREAT;
    threat_point += (bitboard.b_knight & wp_attack_mask).count_ones() as i32 * NBP_THREAT;

    threat_point += (bitboard.b_bishop & (w_attack_mask & !b_defense_mask)).count_ones() as i32 * NB_THREAT;
    threat_point += (bitboard.b_bishop & wp_attack_mask).count_ones() as i32 * NBP_THREAT;

    threat_point += (bitboard.b_rook & (w_attack_mask & !b_defense_mask)).count_ones() as i32 * R_THREAT;
    threat_point += (bitboard.b_rook & wp_attack_mask).count_ones() as i32 * RP_THREAT;
    threat_point += (bitboard.b_rook & (wn_attack_mask | wb_attack_mask)).count_ones() as i32 * RNB_THREAT;
    
    threat_point += (bitboard.b_queen & (w_attack_mask & !b_defense_mask)).count_ones() as i32 * Q_THREAT;
    threat_point += (bitboard.b_queen & wp_attack_mask).count_ones() as i32 * QP_THREAT;
    threat_point += (bitboard.b_queen & (wn_attack_mask | wb_attack_mask)).count_ones() as i32 * QNB_THREAT;
    threat_point += (bitboard.b_queen & wr_attack_mask).count_ones() as i32 * QR_THREAT;

    threat_point -= (bitboard.w_pawn & b_attack_mask & !w_defense_mask).count_ones() as i32 * P_THREAT;

    threat_point -= (bitboard.w_knight & (b_attack_mask & !w_defense_mask)).count_ones() as i32 * NB_THREAT;
    threat_point -= (bitboard.w_knight & bp_attack_mask).count_ones() as i32 * NBP_THREAT;

    threat_point -= (bitboard.w_bishop & (b_attack_mask & !w_defense_mask)).count_ones() as i32 * NB_THREAT;
    threat_point -= (bitboard.w_bishop & bp_attack_mask).count_ones() as i32 * NBP_THREAT;

    threat_point -= (bitboard.w_rook & (b_attack_mask & !w_defense_mask)).count_ones() as i32 * R_THREAT;
    threat_point -= (bitboard.w_rook & bp_attack_mask).count_ones() as i32 * RP_THREAT;
    threat_point -= (bitboard.w_rook & (bn_attack_mask | bb_attack_mask)).count_ones() as i32 * RNB_THREAT;
    
    threat_point -= (bitboard.w_queen & (b_attack_mask & !w_defense_mask)).count_ones() as i32 * Q_THREAT;
    threat_point -= (bitboard.w_queen & bp_attack_mask).count_ones() as i32 * QP_THREAT;
    threat_point -= (bitboard.w_queen & (bn_attack_mask | bb_attack_mask)).count_ones() as i32 * QNB_THREAT;
    threat_point -= (bitboard.w_queen & br_attack_mask).count_ones() as i32 * QR_THREAT;

    (in_check, threat_point * score_sign)
}
