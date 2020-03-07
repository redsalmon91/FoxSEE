/*
 * Copyright (C) 2020 Zixiao Han
 */

use crate::{
    def,
    state::State,
    util,
};

static CAS_WK_OCCUPY_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10010000;
static CAS_WK_R_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000000;
static CAS_WK_EMPTY_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01100000;

static CAS_WQ_OCCUPY_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010001;
static CAS_WQ_R_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001;
static CAS_WQ_EMPTY_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001110;

static CAS_BK_OCCUPY_MASK: u64 = 0b10010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
static CAS_BK_R_MASK: u64 = 0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
static CAS_BK_EMPTY_MASK: u64 = 0b01100000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;

static CAS_BQ_OCCUPY_MASK: u64 = 0b00010001_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
static CAS_BQ_R_MASK: u64 = 0b00000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
static CAS_BQ_EMPTY_MASK: u64 = 0b00001110_00000000_00000000_00000000_00000000_00000000_00000000_00000000;

static ENP_SQRS_MASK: u64 = 0b00000000_00000000_11111111_00000000_00000000_11111111_00000000_00000000;

pub fn gen_castle_mov_list(state: &State, cas_list: &mut [u32; def::MAX_CAS_COUNT]) {
    let cas_rights = state.cas_rights;
    let bitboard = state.bitboard;
    let all_mask = bitboard.w_all | bitboard.b_all;

    let mut cas_count = 0;

    if state.player == def::PLAYER_W {
        if cas_rights & 0b1000 != 0 {
            if bitboard.w_all & CAS_WK_OCCUPY_MASK == CAS_WK_OCCUPY_MASK
            && bitboard.w_rook & CAS_WK_R_MASK == CAS_WK_R_MASK
            && all_mask & CAS_WK_EMPTY_MASK == 0
            && !is_under_attack(state, def::CAS_SQUARE_WK, def::PLAYER_W)
            && !is_under_attack(state, def::CAS_SQUARE_WK - 1, def::PLAYER_W)
            && !is_under_attack(state, def::CAS_SQUARE_WK - 2, def::PLAYER_W) {
                cas_list[cas_count] = util::encode_u32_mov(def::CAS_SQUARE_WK - 2, def::CAS_SQUARE_WK, def::MOV_CAS, 0);
                cas_count += 1;
            }
        }

        if cas_rights & 0b0100 != 0 {
            if bitboard.w_all & CAS_WQ_OCCUPY_MASK == CAS_WQ_OCCUPY_MASK
            && bitboard.w_rook & CAS_WQ_R_MASK == CAS_WQ_R_MASK
            && all_mask & CAS_WQ_EMPTY_MASK == 0
            && !is_under_attack(state, def::CAS_SQUARE_WQ, def::PLAYER_W)
            && !is_under_attack(state, def::CAS_SQUARE_WQ + 1, def::PLAYER_W)
            && !is_under_attack(state, def::CAS_SQUARE_WQ + 2, def::PLAYER_W) {
                cas_list[cas_count] = util::encode_u32_mov(def::CAS_SQUARE_WQ + 2, def::CAS_SQUARE_WQ, def::MOV_CAS, 0);
            }
        }
    } else {
        if cas_rights & 0b0010 != 0 {
            if bitboard.b_all & CAS_BK_OCCUPY_MASK == CAS_BK_OCCUPY_MASK
            && bitboard.b_rook & CAS_BK_R_MASK == CAS_BK_R_MASK
            && all_mask & CAS_BK_EMPTY_MASK == 0
            && !is_under_attack(state, def::CAS_SQUARE_BK, def::PLAYER_B)
            && !is_under_attack(state, def::CAS_SQUARE_BK - 1, def::PLAYER_B)
            && !is_under_attack(state, def::CAS_SQUARE_BK - 2, def::PLAYER_B) {
                cas_list[cas_count] = util::encode_u32_mov(def::CAS_SQUARE_BK - 2, def::CAS_SQUARE_BK, def::MOV_CAS, 0);
                cas_count += 1;
            }
        }

        if cas_rights & 0b0001 != 0 {
            if bitboard.b_all & CAS_BQ_OCCUPY_MASK == CAS_BQ_OCCUPY_MASK
            && bitboard.b_rook & CAS_BQ_R_MASK == CAS_BQ_R_MASK
            && all_mask & CAS_BQ_EMPTY_MASK == 0
            && !is_under_attack(state, def::CAS_SQUARE_BQ, def::PLAYER_B)
            && !is_under_attack(state, def::CAS_SQUARE_BQ + 1, def::PLAYER_B)
            && !is_under_attack(state, def::CAS_SQUARE_BQ + 2, def::PLAYER_B) {
                cas_list[cas_count] = util::encode_u32_mov(def::CAS_SQUARE_BQ + 2, def::CAS_SQUARE_BQ, def::MOV_CAS, 0);
            }
        }
    }
}

pub fn gen_reg_mov_list(state: &State, cap_list: &mut [u32; def::MAX_CAP_COUNT], mov_list: &mut [u32; def::MAX_MOV_COUNT]) {
    let player = state.player;
    let bitboard = state.bitboard;
    let bitmask = state.bitmask;

    let occupy_mask = bitboard.w_all | bitboard.b_all;
    let empty_mask = !occupy_mask;

    let mut cap_count = 0;
    let mut mov_count = 0;

    let mut add_mov = |from: usize, to: usize, tp: u8, promo: u8| {
        mov_list[mov_count] = util::encode_u32_mov(from, to, tp, promo);
        mov_count += 1;
    };

    let mut add_cap = |from: usize, to: usize, tp: u8, promo: u8| {
        cap_list[cap_count] = util::encode_u32_mov(from, to, tp, promo);
        cap_count += 1;
    };

    let (opponent_mask, self_mask) = if player == def::PLAYER_W {
        (bitboard.b_all, bitboard.w_all)
    } else {
        (bitboard.w_all, bitboard.b_all)
    };

    let start_index = self_mask.trailing_zeros() as usize;
    let end_index = def::BOARD_SIZE - self_mask.leading_zeros() as usize;

    for from_index in start_index..end_index {
        let moving_piece = state.squares[from_index];

        if !def::on_same_side(player, moving_piece) {
            continue
        }

        if def::is_p(moving_piece) {
            if player == def::PLAYER_W {
                if bitmask.wp_mov_masks[from_index] & empty_mask != 0 {
                    let to_index = from_index + 8;
                    if to_index > 55 {
                        add_mov(from_index, to_index, def::MOV_PROMO, def::WQ);
                        add_mov(from_index, to_index, def::MOV_PROMO, def::WR);
                        add_mov(from_index, to_index, def::MOV_PROMO, def::WB);
                        add_mov(from_index, to_index, def::MOV_PROMO, def::WN);
                    } else {
                        add_mov(from_index, to_index, def::MOV_REG, 0);

                        if bitmask.wp_init_mov_masks[from_index] & empty_mask != 0 {
                            add_mov(from_index, to_index + 8, def::MOV_CR_ENP, 0);
                        }
                    }
                }

                let enp_square_index = state.enp_square;
                let mut attack_mask = bitmask.wp_attack_masks[from_index] & (bitboard.b_all | bitmask.index_masks[enp_square_index]);

                let start_index = get_lowest_index(attack_mask);
                attack_mask >>= start_index;

                let mut attack_index = start_index;
                while attack_mask != 0 {
                    if attack_mask & 1u64 != 0 {
                        if attack_index > 55 {
                            add_cap(from_index, attack_index, def::MOV_PROMO, def::WQ);
                            add_cap(from_index, attack_index, def::MOV_PROMO, def::WR);
                            add_cap(from_index, attack_index, def::MOV_PROMO, def::WB);
                            add_cap(from_index, attack_index, def::MOV_PROMO, def::WN);
                        } else {
                            if attack_index == enp_square_index {
                                add_cap(from_index, attack_index, def::MOV_ENP, 0);
                            } else {
                                add_cap(from_index, attack_index, def::MOV_REG, 0);
                            }
                        }
                    }

                    attack_mask >>= 1;
                    attack_index += 1;
                }
            } else {
                if bitmask.bp_mov_masks[from_index] & empty_mask != 0 {
                    let to_index = from_index - 8;
                    if to_index < 8 {
                        add_mov(from_index, to_index, def::MOV_PROMO, def::BQ);
                        add_mov(from_index, to_index, def::MOV_PROMO, def::BR);
                        add_mov(from_index, to_index, def::MOV_PROMO, def::BB);
                        add_mov(from_index, to_index, def::MOV_PROMO, def::BN);
                    } else {
                        add_mov(from_index, to_index, def::MOV_REG, 0);

                        if bitmask.bp_init_mov_masks[from_index] & empty_mask != 0 {
                            add_mov(from_index, to_index - 8, def::MOV_CR_ENP, 0);
                        }
                    }
                }

                let enp_square_index = state.enp_square;
                let mut attack_mask = bitmask.bp_attack_masks[from_index] & (bitboard.w_all | (bitmask.index_masks[enp_square_index] & ENP_SQRS_MASK));

                let start_index = get_lowest_index(attack_mask);
                attack_mask >>= start_index;

                let mut attack_index = start_index;
                while attack_mask != 0 {
                    if attack_mask & 1u64 != 0 {
                        if attack_index < 8 {
                            add_cap(from_index, attack_index, def::MOV_PROMO, def::BQ);
                            add_cap(from_index, attack_index, def::MOV_PROMO, def::BR);
                            add_cap(from_index, attack_index, def::MOV_PROMO, def::BB);
                            add_cap(from_index, attack_index, def::MOV_PROMO, def::BN);
                        } else {
                            if attack_index == enp_square_index {
                                add_cap(from_index, attack_index, def::MOV_ENP, 0);
                            } else {
                                add_cap(from_index, attack_index, def::MOV_REG, 0);
                            }
                        }
                    }

                    attack_mask >>= 1;
                    attack_index += 1;
                }
            }
        } else if def::is_n(moving_piece) {
            let reach_mask = bitmask.n_attack_masks[from_index] & !self_mask;
            let mut attack_mask = reach_mask & opponent_mask;
            let mut mov_mask = reach_mask ^ attack_mask;

            let start_index = get_lowest_index(attack_mask);
            attack_mask >>= start_index;

            let mut attack_index = start_index;
            while attack_mask != 0 {
                if attack_mask & 1u64 != 0 {
                    add_cap(from_index, attack_index, def::MOV_REG, 0);
                }

                attack_mask >>= 1;
                attack_index += 1;
            }

            let start_index = get_lowest_index(mov_mask);
            mov_mask >>= start_index;

            let mut mov_index = start_index;
            while mov_mask != 0 {
                if mov_mask & 1u64 != 0 {
                    add_mov(from_index, mov_index, def::MOV_REG, 0);
                }

                mov_mask >>= 1;
                mov_index += 1;
            }
        } else if def::is_b(moving_piece) {
            let mut reach_mask = 0;

            let up_left_attack_mask = bitmask.up_left_attack_masks[from_index];
            reach_mask ^= up_left_attack_mask;
            if up_left_attack_mask & occupy_mask != 0 {
                let lowest_blocker_index = get_lowest_index(up_left_attack_mask & occupy_mask);
                reach_mask &= !bitmask.up_left_attack_masks[lowest_blocker_index];
            }

            let up_right_attack_mask = bitmask.up_right_attack_masks[from_index];
            reach_mask ^= up_right_attack_mask;
            if up_right_attack_mask & occupy_mask != 0 {
                let lowest_blocker_index = get_lowest_index(up_right_attack_mask & occupy_mask);
                reach_mask &= !bitmask.up_right_attack_masks[lowest_blocker_index];
            }

            let down_left_attack_mask = bitmask.down_left_attack_masks[from_index];
            reach_mask ^= down_left_attack_mask;
            if down_left_attack_mask & occupy_mask != 0 {
                let highest_blocker_index = get_highest_index(down_left_attack_mask & occupy_mask);
                reach_mask &= !bitmask.down_left_attack_masks[highest_blocker_index];
            }

            let down_right_attack_mask = bitmask.down_right_attack_masks[from_index];
            reach_mask ^= down_right_attack_mask;
            if down_right_attack_mask & occupy_mask != 0 {
                let highest_blocker_index = get_highest_index(down_right_attack_mask & occupy_mask);
                reach_mask &= !bitmask.down_right_attack_masks[highest_blocker_index];
            }

            reach_mask &= !self_mask;

            let mut attack_mask = reach_mask & opponent_mask;
            let mut mov_mask = reach_mask ^ attack_mask;

            let start_index = get_lowest_index(attack_mask);
            attack_mask >>= start_index;

            let mut attack_index = start_index;
            while attack_mask != 0 {
                if attack_mask & 1u64 != 0 {
                    add_cap(from_index, attack_index, def::MOV_REG, 0);
                }

                attack_mask >>= 1;
                attack_index += 1;
            }

            let start_index = get_lowest_index(mov_mask);
            mov_mask >>= start_index;

            let mut mov_index = start_index;
            while mov_mask != 0 {
                if mov_mask & 1u64 != 0 {
                    add_mov(from_index, mov_index, def::MOV_REG, 0);
                }

                mov_mask >>= 1;
                mov_index += 1;
            }
        } else if def::is_r(moving_piece) {
            let mut reach_mask = 0;

            let up_attack_mask = bitmask.up_attack_masks[from_index];
            reach_mask ^= up_attack_mask;
            if up_attack_mask & occupy_mask != 0 {
                let lowest_blocker_index = get_lowest_index(up_attack_mask & occupy_mask);
                reach_mask &= !bitmask.up_attack_masks[lowest_blocker_index];
            }

            let right_attack_mask = bitmask.right_attack_masks[from_index];
            reach_mask ^= right_attack_mask;
            if right_attack_mask & occupy_mask != 0 {
                let lowest_blocker_index = get_lowest_index(right_attack_mask & occupy_mask);
                reach_mask &= !bitmask.right_attack_masks[lowest_blocker_index];
            }

            let down_attack_mask = bitmask.down_attack_masks[from_index];
            reach_mask ^= down_attack_mask;
            if down_attack_mask & occupy_mask != 0 {
                let highest_blocker_index = get_highest_index(down_attack_mask & occupy_mask);
                reach_mask &= !bitmask.down_attack_masks[highest_blocker_index];
            }

            let left_attack_mask = bitmask.left_attack_masks[from_index];
            reach_mask ^= left_attack_mask;
            if left_attack_mask & occupy_mask != 0 {
                let highest_blocker_index = get_highest_index(left_attack_mask & occupy_mask);
                reach_mask &= !bitmask.left_attack_masks[highest_blocker_index];
            }

            reach_mask &= !self_mask;

            let mut attack_mask = reach_mask & opponent_mask;
            let mut mov_mask = reach_mask ^ attack_mask;

            let start_index = get_lowest_index(attack_mask);
            attack_mask >>= start_index;

            let mut attack_index = start_index;
            while attack_mask != 0 {
                if attack_mask & 1u64 != 0 {
                    add_cap(from_index, attack_index, def::MOV_REG, 0);
                }

                attack_mask >>= 1;
                attack_index += 1;
            }

            let start_index = get_lowest_index(mov_mask);
            mov_mask >>= start_index;

            let mut mov_index = start_index;
            while mov_mask != 0 {
                if mov_mask & 1u64 != 0 {
                    add_mov(from_index, mov_index, def::MOV_REG, 0);
                }

                mov_mask >>= 1;
                mov_index += 1;
            }
        } else if def::is_q(moving_piece) {
            let mut reach_mask = 0;

            let up_left_attack_mask = bitmask.up_left_attack_masks[from_index];
            reach_mask ^= up_left_attack_mask;
            if up_left_attack_mask & occupy_mask != 0 {
                let lowest_blocker_index = get_lowest_index(up_left_attack_mask & occupy_mask);
                reach_mask &= !bitmask.up_left_attack_masks[lowest_blocker_index];
            }

            let up_right_attack_mask = bitmask.up_right_attack_masks[from_index];
            reach_mask ^= up_right_attack_mask;
            if up_right_attack_mask & occupy_mask != 0 {
                let lowest_blocker_index = get_lowest_index(up_right_attack_mask & occupy_mask);
                reach_mask &= !bitmask.up_right_attack_masks[lowest_blocker_index];
            }

            let down_left_attack_mask = bitmask.down_left_attack_masks[from_index];
            reach_mask ^= down_left_attack_mask;
            if down_left_attack_mask & occupy_mask != 0 {
                let highest_blocker_index = get_highest_index(down_left_attack_mask & occupy_mask);
                reach_mask &= !bitmask.down_left_attack_masks[highest_blocker_index];
            }

            let down_right_attack_mask = bitmask.down_right_attack_masks[from_index];
            reach_mask ^= down_right_attack_mask;
            if down_right_attack_mask & occupy_mask != 0 {
                let highest_blocker_index = get_highest_index(down_right_attack_mask & occupy_mask);
                reach_mask &= !bitmask.down_right_attack_masks[highest_blocker_index];
            }

            let up_attack_mask = bitmask.up_attack_masks[from_index];
            reach_mask ^= up_attack_mask;
            if up_attack_mask & occupy_mask != 0 {
                let lowest_blocker_index = get_lowest_index(up_attack_mask & occupy_mask);
                reach_mask &= !bitmask.up_attack_masks[lowest_blocker_index];
            }

            let right_attack_mask = bitmask.right_attack_masks[from_index];
            reach_mask ^= right_attack_mask;
            if right_attack_mask & occupy_mask != 0 {
                let lowest_blocker_index = get_lowest_index(right_attack_mask & occupy_mask);
                reach_mask &= !bitmask.right_attack_masks[lowest_blocker_index];
            }

            let down_attack_mask = bitmask.down_attack_masks[from_index];
            reach_mask ^= down_attack_mask;
            if down_attack_mask & occupy_mask != 0 {
                let highest_blocker_index = get_highest_index(down_attack_mask & occupy_mask);
                reach_mask &= !bitmask.down_attack_masks[highest_blocker_index];
            }

            let left_attack_mask = bitmask.left_attack_masks[from_index];
            reach_mask ^= left_attack_mask;
            if left_attack_mask & occupy_mask != 0 {
                let highest_blocker_index = get_highest_index(left_attack_mask & occupy_mask);
                reach_mask &= !bitmask.left_attack_masks[highest_blocker_index];
            }

            reach_mask &= !self_mask;

            let mut attack_mask = reach_mask & opponent_mask;
            let mut mov_mask = reach_mask ^ attack_mask;

            let start_index = get_lowest_index(attack_mask);
            attack_mask >>= start_index;

            let mut attack_index = start_index;
            while attack_mask != 0 {
                if attack_mask & 1u64 != 0 {
                    add_cap(from_index, attack_index, def::MOV_REG, 0);
                }

                attack_mask >>= 1;
                attack_index += 1;
            }

            let start_index = get_lowest_index(mov_mask);
            mov_mask >>= start_index;

            let mut mov_index = start_index;
            while mov_mask != 0 {
                if mov_mask & 1u64 != 0 {
                    add_mov(from_index, mov_index, def::MOV_REG, 0);
                }

                mov_mask >>= 1;
                mov_index += 1;
            }
        } else if def::is_k(moving_piece) {
            let reach_mask = bitmask.k_attack_masks[from_index] & !self_mask;
            let mut attack_mask = reach_mask & opponent_mask;
            let mut mov_mask = reach_mask ^ attack_mask;

            let start_index = get_lowest_index(attack_mask);
            attack_mask >>= start_index;

            let mut attack_index = start_index;
            while attack_mask != 0 {
                if attack_mask & 1u64 != 0 {
                    add_cap(from_index, attack_index, def::MOV_REG, 0);
                }

                attack_mask >>= 1;
                attack_index += 1;
            }

            let start_index = get_lowest_index(mov_mask);
            mov_mask >>= start_index;

            let mut mov_index = start_index;
            while mov_mask != 0 {
                if mov_mask & 1u64 != 0 {
                    add_mov(from_index, mov_index, def::MOV_REG, 0);
                }

                mov_mask >>= 1;
                mov_index += 1;
            }
        }
    }
}

pub fn gen_capture_list(state: &State, cap_list: &mut [u32; def::MAX_CAP_COUNT]) {
    let player = state.player;
    let bitboard = state.bitboard;
    let bitmask = state.bitmask;

    let occupy_mask = bitboard.w_all | bitboard.b_all;

    let mut cap_count = 0;

    let mut add_cap = |from: usize, to: usize, tp: u8, promo: u8| {
        cap_list[cap_count] = util::encode_u32_mov(from, to, tp, promo);
        cap_count += 1;
    };

    let (self_mask, opponent_mask) = if player == def::PLAYER_W {
        (bitboard.w_all, bitboard.b_all)
    } else {
        (bitboard.b_all, bitboard.w_all)
    };

    let start_index = self_mask.trailing_zeros() as usize;
    let end_index = def::BOARD_SIZE - self_mask.leading_zeros() as usize;

    for from_index in start_index..end_index {
        let moving_piece = state.squares[from_index];

        if !def::on_same_side(player, moving_piece) {
            continue
        }

        if def::is_p(moving_piece) {
            if player == def::PLAYER_W {
                let enp_square_index = state.enp_square;
                let mut attack_mask = bitmask.wp_attack_masks[from_index] & (bitboard.b_all | bitmask.index_masks[enp_square_index]);

                let start_index = get_lowest_index(attack_mask);
                attack_mask >>= start_index;

                let mut attack_index = start_index;
                while attack_mask != 0 {
                    if attack_mask & 1u64 != 0 {
                        if attack_index > 55 {
                            add_cap(from_index, attack_index, def::MOV_PROMO, def::WQ);
                            add_cap(from_index, attack_index, def::MOV_PROMO, def::WR);
                            add_cap(from_index, attack_index, def::MOV_PROMO, def::WB);
                            add_cap(from_index, attack_index, def::MOV_PROMO, def::WN);
                        } else {
                            if attack_index == enp_square_index {
                                add_cap(from_index, attack_index, def::MOV_ENP, 0);
                            } else {
                                add_cap(from_index, attack_index, def::MOV_REG, 0);
                            }
                        }
                    }

                    attack_mask >>= 1;
                    attack_index += 1;
                }
            } else {
                let enp_square_index = state.enp_square;
                let mut attack_mask = bitmask.bp_attack_masks[from_index] & (bitboard.w_all | (bitmask.index_masks[enp_square_index] & ENP_SQRS_MASK));

                let start_index = get_lowest_index(attack_mask);
                attack_mask >>= start_index;

                let mut attack_index = start_index;
                while attack_mask != 0 {
                    if attack_mask & 1u64 != 0 {
                        if attack_index < 8 {
                            add_cap(from_index, attack_index, def::MOV_PROMO, def::BQ);
                            add_cap(from_index, attack_index, def::MOV_PROMO, def::BR);
                            add_cap(from_index, attack_index, def::MOV_PROMO, def::BB);
                            add_cap(from_index, attack_index, def::MOV_PROMO, def::BN);
                        } else {
                            if attack_index == enp_square_index {
                                add_cap(from_index, attack_index, def::MOV_ENP, 0);
                            } else {
                                add_cap(from_index, attack_index, def::MOV_REG, 0);
                            }
                        }
                    }

                    attack_mask >>= 1;
                    attack_index += 1;
                }
            }
        } else if def::is_n(moving_piece) {
            let mut attack_mask = bitmask.n_attack_masks[from_index] & opponent_mask;

            let start_index = get_lowest_index(attack_mask);
            attack_mask >>= start_index;

            let mut attack_index = start_index;
            while attack_mask != 0 {
                if attack_mask & 1u64 != 0 {
                    add_cap(from_index, attack_index, def::MOV_REG, 0);
                }

                attack_mask >>= 1;
                attack_index += 1;
            }
        } else if def::is_b(moving_piece) {
            let mut reach_mask = 0;

            let up_left_attack_mask = bitmask.up_left_attack_masks[from_index];
            reach_mask ^= up_left_attack_mask;
            if up_left_attack_mask & occupy_mask != 0 {
                let lowest_blocker_index = get_lowest_index(up_left_attack_mask & occupy_mask);
                reach_mask &= !bitmask.up_left_attack_masks[lowest_blocker_index];
            }

            let up_right_attack_mask = bitmask.up_right_attack_masks[from_index];
            reach_mask ^= up_right_attack_mask;
            if up_right_attack_mask & occupy_mask != 0 {
                let lowest_blocker_index = get_lowest_index(up_right_attack_mask & occupy_mask);
                reach_mask &= !bitmask.up_right_attack_masks[lowest_blocker_index];
            }

            let down_left_attack_mask = bitmask.down_left_attack_masks[from_index];
            reach_mask ^= down_left_attack_mask;
            if down_left_attack_mask & occupy_mask != 0 {
                let highest_blocker_index = get_highest_index(down_left_attack_mask & occupy_mask);
                reach_mask &= !bitmask.down_left_attack_masks[highest_blocker_index];
            }

            let down_right_attack_mask = bitmask.down_right_attack_masks[from_index];
            reach_mask ^= down_right_attack_mask;
            if down_right_attack_mask & occupy_mask != 0 {
                let highest_blocker_index = get_highest_index(down_right_attack_mask & occupy_mask);
                reach_mask &= !bitmask.down_right_attack_masks[highest_blocker_index];
            }

            let mut attack_mask = reach_mask & opponent_mask;

            let start_index = get_lowest_index(attack_mask);
            attack_mask >>= start_index;

            let mut attack_index = start_index;
            while attack_mask != 0 {
                if attack_mask & 1u64 != 0 {
                    add_cap(from_index, attack_index, def::MOV_REG, 0);
                }

                attack_mask >>= 1;
                attack_index += 1;
            }
        } else if def::is_r(moving_piece) {
            let mut reach_mask = 0;

            let up_attack_mask = bitmask.up_attack_masks[from_index];
            reach_mask ^= up_attack_mask;
            if up_attack_mask & occupy_mask != 0 {
                let lowest_blocker_index = get_lowest_index(up_attack_mask & occupy_mask);
                reach_mask &= !bitmask.up_attack_masks[lowest_blocker_index];
            }

            let right_attack_mask = bitmask.right_attack_masks[from_index];
            reach_mask ^= right_attack_mask;
            if right_attack_mask & occupy_mask != 0 {
                let lowest_blocker_index = get_lowest_index(right_attack_mask & occupy_mask);
                reach_mask &= !bitmask.right_attack_masks[lowest_blocker_index];
            }

            let down_attack_mask = bitmask.down_attack_masks[from_index];
            reach_mask ^= down_attack_mask;
            if down_attack_mask & occupy_mask != 0 {
                let highest_blocker_index = get_highest_index(down_attack_mask & occupy_mask);
                reach_mask &= !bitmask.down_attack_masks[highest_blocker_index];
            }

            let left_attack_mask = bitmask.left_attack_masks[from_index];
            reach_mask ^= left_attack_mask;
            if left_attack_mask & occupy_mask != 0 {
                let highest_blocker_index = get_highest_index(left_attack_mask & occupy_mask);
                reach_mask &= !bitmask.left_attack_masks[highest_blocker_index];
            }

            let mut attack_mask = reach_mask & opponent_mask;

            let start_index = get_lowest_index(attack_mask);
            attack_mask >>= start_index;

            let mut attack_index = start_index;
            while attack_mask != 0 {
                if attack_mask & 1u64 != 0 {
                    add_cap(from_index, attack_index, def::MOV_REG, 0);
                }

                attack_mask >>= 1;
                attack_index += 1;
            }
        } else if def::is_q(moving_piece) {
            let mut reach_mask = 0;

            let up_left_attack_mask = bitmask.up_left_attack_masks[from_index];
            reach_mask ^= up_left_attack_mask;
            if up_left_attack_mask & occupy_mask != 0 {
                let lowest_blocker_index = get_lowest_index(up_left_attack_mask & occupy_mask);
                reach_mask &= !bitmask.up_left_attack_masks[lowest_blocker_index];
            }

            let up_right_attack_mask = bitmask.up_right_attack_masks[from_index];
            reach_mask ^= up_right_attack_mask;
            if up_right_attack_mask & occupy_mask != 0 {
                let lowest_blocker_index = get_lowest_index(up_right_attack_mask & occupy_mask);
                reach_mask &= !bitmask.up_right_attack_masks[lowest_blocker_index];
            }

            let down_left_attack_mask = bitmask.down_left_attack_masks[from_index];
            reach_mask ^= down_left_attack_mask;
            if down_left_attack_mask & occupy_mask != 0 {
                let highest_blocker_index = get_highest_index(down_left_attack_mask & occupy_mask);
                reach_mask &= !bitmask.down_left_attack_masks[highest_blocker_index];
            }

            let down_right_attack_mask = bitmask.down_right_attack_masks[from_index];
            reach_mask ^= down_right_attack_mask;
            if down_right_attack_mask & occupy_mask != 0 {
                let highest_blocker_index = get_highest_index(down_right_attack_mask & occupy_mask);
                reach_mask &= !bitmask.down_right_attack_masks[highest_blocker_index];
            }

            let up_attack_mask = bitmask.up_attack_masks[from_index];
            reach_mask ^= up_attack_mask;
            if up_attack_mask & occupy_mask != 0 {
                let lowest_blocker_index = get_lowest_index(up_attack_mask & occupy_mask);
                reach_mask &= !bitmask.up_attack_masks[lowest_blocker_index];
            }

            let right_attack_mask = bitmask.right_attack_masks[from_index];
            reach_mask ^= right_attack_mask;
            if right_attack_mask & occupy_mask != 0 {
                let lowest_blocker_index = get_lowest_index(right_attack_mask & occupy_mask);
                reach_mask &= !bitmask.right_attack_masks[lowest_blocker_index];
            }

            let down_attack_mask = bitmask.down_attack_masks[from_index];
            reach_mask ^= down_attack_mask;
            if down_attack_mask & occupy_mask != 0 {
                let highest_blocker_index = get_highest_index(down_attack_mask & occupy_mask);
                reach_mask &= !bitmask.down_attack_masks[highest_blocker_index];
            }

            let left_attack_mask = bitmask.left_attack_masks[from_index];
            reach_mask ^= left_attack_mask;
            if left_attack_mask & occupy_mask != 0 {
                let highest_blocker_index = get_highest_index(left_attack_mask & occupy_mask);
                reach_mask &= !bitmask.left_attack_masks[highest_blocker_index];
            }

            let mut attack_mask = reach_mask & opponent_mask;

            let start_index = get_lowest_index(attack_mask);
            attack_mask >>= start_index;

            let mut attack_index = start_index;
            while attack_mask != 0 {
                if attack_mask & 1u64 != 0 {
                    add_cap(from_index, attack_index, def::MOV_REG, 0);
                }

                attack_mask >>= 1;
                attack_index += 1;
            }
        } else if def::is_k(moving_piece) {
            let mut attack_mask = bitmask.k_attack_masks[from_index] & opponent_mask;

            let start_index = get_lowest_index(attack_mask);
            attack_mask >>= start_index;

            let mut attack_index = start_index;
            while attack_mask != 0 {
                if attack_mask & 1u64 != 0 {
                    add_cap(from_index, attack_index, def::MOV_REG, 0);
                }

                attack_mask >>= 1;
                attack_index += 1;
            }
        }
    }
}

pub fn is_in_check(state: &State, player: u8) -> bool {
    let k_index = if player == def::PLAYER_W {
        state.wk_index
    } else {
        state.bk_index
    };

    is_under_attack(state, k_index, player)
}

pub fn is_under_attack(state: &State, index: usize, player: u8) -> bool {
    let bitboard = state.bitboard;
    let bitmask = state.bitmask;

    let opponent_n_mask = if player == def::PLAYER_W {
        bitboard.b_knight
    } else {
        bitboard.w_knight
    };

    if opponent_n_mask & bitmask.n_attack_masks[index] != 0 {
        return true
    }

    let opponent_k_mask = if player == def::PLAYER_W {
        bitmask.index_masks[state.bk_index]
    } else {
        bitmask.index_masks[state.wk_index]
    };

    if opponent_k_mask & bitmask.k_attack_masks[index] != 0 {
        return true
    }

    if player == def::PLAYER_W && bitboard.b_pawn & bitmask.wp_attack_masks[index] != 0 {
        return true
    } else if player == def::PLAYER_B && bitboard.w_pawn & bitmask.bp_attack_masks[index] != 0 {
        return true
    }

    let blocker_mask = bitboard.w_all | bitboard.b_all;

    let opponent_rq_mask = if player == def::PLAYER_W {
        bitboard.b_rook | bitboard.b_queen
    } else {
        bitboard.w_rook | bitboard.w_queen
    };

    if opponent_rq_mask != 0 {
        let up_attack_mask = bitmask.up_attack_masks[index];
        if up_attack_mask & opponent_rq_mask != 0 {
            let lowest_blocker_index = get_lowest_index(up_attack_mask & blocker_mask);
            if (up_attack_mask ^ bitmask.up_attack_masks[lowest_blocker_index]) & opponent_rq_mask != 0 {
                return true
            }
        }

        let right_attack_mask = bitmask.right_attack_masks[index];
        if right_attack_mask & opponent_rq_mask != 0 {
            let lowest_blocker_index = get_lowest_index(right_attack_mask & blocker_mask);
            if (right_attack_mask ^ bitmask.right_attack_masks[lowest_blocker_index]) & opponent_rq_mask != 0 {
                return true
            }
        }

        let down_attack_mask = bitmask.down_attack_masks[index];
        if down_attack_mask & opponent_rq_mask != 0 {
            let highest_blocker_index = get_highest_index(down_attack_mask & blocker_mask);
            if (down_attack_mask ^ bitmask.down_attack_masks[highest_blocker_index]) & opponent_rq_mask != 0 {
                return true
            }
        }

        let left_attack_mask = bitmask.left_attack_masks[index];
        if left_attack_mask & opponent_rq_mask != 0 {
            let highest_blocker_index = get_highest_index(left_attack_mask & blocker_mask);
            if (left_attack_mask ^ bitmask.left_attack_masks[highest_blocker_index]) & opponent_rq_mask != 0 {
                return true
            }
        }
    }

    let opponent_bq_mask = if player == def::PLAYER_W {
        bitboard.b_bishop | bitboard.b_queen
    } else {
        bitboard.w_bishop | bitboard.w_queen
    };

    if opponent_bq_mask != 0 {
        let up_left_attack_mask = bitmask.up_left_attack_masks[index];
        if up_left_attack_mask & opponent_bq_mask != 0 {
            let lowest_blocker_index = get_lowest_index(up_left_attack_mask & blocker_mask);
            if (up_left_attack_mask ^ bitmask.up_left_attack_masks[lowest_blocker_index]) & opponent_bq_mask != 0 {
                return true
            }
        }

        let up_right_attack_mask = bitmask.up_right_attack_masks[index];
        if up_right_attack_mask & opponent_bq_mask != 0 {
            let lowest_blocker_index = get_lowest_index(up_right_attack_mask & blocker_mask);
            if (up_right_attack_mask ^ bitmask.up_right_attack_masks[lowest_blocker_index]) & opponent_bq_mask != 0 {
                return true
            }
        }

        let down_left_attack_mask = bitmask.down_left_attack_masks[index];
        if down_left_attack_mask & opponent_bq_mask != 0 {
            let highest_blocker_index = get_highest_index(down_left_attack_mask & blocker_mask);
            if (down_left_attack_mask ^ bitmask.down_left_attack_masks[highest_blocker_index]) & opponent_bq_mask != 0 {
                return true
            }
        }

        let down_right_attack_mask = bitmask.down_right_attack_masks[index];
        if down_right_attack_mask & opponent_bq_mask != 0 {
            let highest_blocker_index = get_highest_index(down_right_attack_mask & blocker_mask);
            if (down_right_attack_mask ^ bitmask.down_right_attack_masks[highest_blocker_index]) & opponent_bq_mask != 0 {
                return true
            }
        }
    }

    false
}

pub fn get_attackers(state: &State, initial_attacker_index: usize, index: usize, w_attacker_list: &mut [u8; def::MAX_ATTACKERS_COUNT], b_attacker_list: &mut [u8; def::MAX_ATTACKERS_COUNT]) {
    let bitmask = state.bitmask;
    let bitboard = state.bitboard;

    let surround_wp_mask = bitmask.bp_attack_masks[index] & bitboard.w_pawn;
    let surround_bp_mask = bitmask.wp_attack_masks[index] & bitboard.b_pawn;

    let all_mask = (bitboard.w_all | bitboard.b_all) ^ surround_wp_mask ^ surround_bp_mask;

    let initial_attacker_mask = bitmask.index_masks[initial_attacker_index];

    let mut w_attacker_index = 0;
    let mut b_attacker_index = 0;

    let mut add_w_attacker = |attacker: u8| {
        w_attacker_list[w_attacker_index] = attacker;
        w_attacker_index += 1;
    };

    let mut add_b_attacker = |attacker: u8| {
        b_attacker_list[b_attacker_index] = attacker;
        b_attacker_index += 1;
    };

    let n_attack_mask = bitmask.n_attack_masks[index] & !initial_attacker_mask;

    let wn_attack_mask = n_attack_mask & bitboard.w_knight;
    if wn_attack_mask != 0 {
        let wn_count = wn_attack_mask.count_ones();

        for _ in 0..wn_count {
            add_w_attacker(def::WN);
        }
    }

    let bn_attack_mask = n_attack_mask & bitboard.b_knight;
    if bn_attack_mask != 0 {
        let bn_count = bn_attack_mask.count_ones();

        for _ in 0..bn_count {
            add_b_attacker(def::BN);
        }
    }

    let k_attack_mask = bitmask.k_attack_masks[index] & !initial_attacker_mask;

    if k_attack_mask & bitmask.index_masks[state.wk_index] != 0 {
        add_w_attacker(def::WK);
    }

    if k_attack_mask & bitmask.index_masks[state.bk_index] != 0 {
        add_b_attacker(def::BK);
    }

    let r_mask = bitboard.w_rook | bitboard.b_rook;
    let b_mask = bitboard.w_bishop | bitboard.b_bishop;
    let q_mask = bitboard.w_queen | bitboard.b_queen;

    let rq_mask = (r_mask | q_mask) & !initial_attacker_mask;
    if rq_mask != 0 {
        let mut rq_attack_mask = 0;

        let mut up_attack_mask = bitmask.up_attack_masks[index];
        if up_attack_mask != 0 && up_attack_mask & rq_mask != 0 {    
            if up_attack_mask & all_mask != 0 {
                let lowest_blocker_index = get_lowest_index(up_attack_mask & all_mask);
                up_attack_mask &= !bitmask.up_attack_masks[lowest_blocker_index];
            }

            rq_attack_mask ^= up_attack_mask;
        }

        let mut right_attack_mask = bitmask.right_attack_masks[index];
        if right_attack_mask != 0 && right_attack_mask & rq_mask != 0 {    
            if right_attack_mask & all_mask != 0 {
                let lowest_blocker_index = get_lowest_index(right_attack_mask & all_mask);
                right_attack_mask &= !bitmask.right_attack_masks[lowest_blocker_index];
            }

            rq_attack_mask ^= right_attack_mask;
        }

        let mut down_attack_mask = bitmask.down_attack_masks[index];
        if down_attack_mask != 0 && down_attack_mask & rq_mask != 0 {    
            if down_attack_mask & all_mask != 0 {
                let highest_blocker_index = get_highest_index(down_attack_mask & all_mask);
                down_attack_mask &= !bitmask.down_attack_masks[highest_blocker_index];
            }

            rq_attack_mask ^= down_attack_mask;
        }

        let mut left_attack_mask = bitmask.left_attack_masks[index];
        if left_attack_mask != 0 && left_attack_mask & rq_mask != 0 {    
            if left_attack_mask & all_mask != 0 {
                let highest_blocker_index = get_highest_index(left_attack_mask & all_mask);
                left_attack_mask &= !bitmask.left_attack_masks[highest_blocker_index];
            }

            rq_attack_mask ^= left_attack_mask;
        }

        let wr_attacker_count = (rq_attack_mask & bitboard.w_rook).count_ones();
        for _ in 0..wr_attacker_count {
            add_w_attacker(def::WR);
        }

        let wq_attacker_count = (rq_attack_mask & bitboard.w_queen).count_ones();
        for _ in 0..wq_attacker_count {
            add_w_attacker(def::WQ);
        }

        let br_attacker_count = (rq_attack_mask & bitboard.b_rook).count_ones();
        for _ in 0..br_attacker_count {
            add_b_attacker(def::BR);
        }

        let bq_attacker_count = (rq_attack_mask & bitboard.b_queen).count_ones();
        for _ in 0..bq_attacker_count {
            add_b_attacker(def::BQ);
        }
    }

    let bq_mask = (b_mask | q_mask) & !initial_attacker_mask;
    if bq_mask != 0 {
        let mut bq_attack_mask = 0;

        let mut up_left_attack_mask = bitmask.up_left_attack_masks[index];
        if up_left_attack_mask != 0 && up_left_attack_mask & bq_mask != 0 {    
            if up_left_attack_mask & all_mask != 0 {
                let lowest_blocker_index = get_lowest_index(up_left_attack_mask & all_mask);
                up_left_attack_mask &= !bitmask.up_left_attack_masks[lowest_blocker_index];
            }

            bq_attack_mask ^= up_left_attack_mask;
        }

        let mut up_right_attack_mask = bitmask.up_right_attack_masks[index];
        if up_right_attack_mask != 0 && up_right_attack_mask & bq_mask != 0 {  
            if up_right_attack_mask & all_mask != 0 {
                let lowest_blocker_index = get_lowest_index(up_right_attack_mask & all_mask);
                up_right_attack_mask &= !bitmask.up_right_attack_masks[lowest_blocker_index];
            }

            bq_attack_mask ^= up_right_attack_mask;
        }

        let mut down_left_attack_mask = bitmask.down_left_attack_masks[index];
        if down_left_attack_mask != 0 && down_left_attack_mask & bq_mask != 0 {    
            if down_left_attack_mask & all_mask != 0 {
                let highest_blocker_index = get_highest_index(down_left_attack_mask & all_mask);
                down_left_attack_mask &= !bitmask.down_left_attack_masks[highest_blocker_index];
            }

            bq_attack_mask ^= down_left_attack_mask;
        }

        let mut down_right_attack_mask = bitmask.down_right_attack_masks[index];
        if down_right_attack_mask != 0 && down_right_attack_mask & bq_mask != 0 {    
            if down_right_attack_mask & all_mask != 0 {
                let highest_blocker_index = get_highest_index(down_right_attack_mask & all_mask);
                down_right_attack_mask &= !bitmask.down_right_attack_masks[highest_blocker_index];
            }

            bq_attack_mask ^= down_right_attack_mask;
        }

        let wb_attacker_count = (bq_attack_mask & bitboard.w_bishop).count_ones();
        for _ in 0..wb_attacker_count {
            add_w_attacker(def::WB);
        }

        let wq_attacker_count = (bq_attack_mask & bitboard.w_queen).count_ones();
        for _ in 0..wq_attacker_count {
            add_w_attacker(def::WQ);
        }

        let bb_attacker_count = (bq_attack_mask & bitboard.b_bishop).count_ones();
        for _ in 0..bb_attacker_count {
            add_b_attacker(def::BB);
        }

        let bq_attacker_count = (bq_attack_mask & bitboard.b_queen).count_ones();
        for _ in 0..bq_attacker_count {
            add_b_attacker(def::BQ);
        }
    }

    let wp_attack_mask = bitmask.bp_attack_masks[index] & bitboard.w_pawn & !initial_attacker_mask;
    if wp_attack_mask != 0 {
        let wp_count = wp_attack_mask.count_ones();

        for _ in 0..wp_count {
            add_w_attacker(def::WP);
        }
    }

    let bp_attack_mask = bitmask.wp_attack_masks[index] & bitboard.b_pawn & !initial_attacker_mask;
    if bp_attack_mask != 0 {
        let bp_count = bp_attack_mask.count_ones();

        for _ in 0..bp_count {
            add_b_attacker(def::BP);
        }
    }
}

pub fn is_mov_valid(state: &State, from: usize, to: usize) -> bool {
    let squares = state.squares;

    if !def::on_same_side(state.player, squares[from]) {
        return false
    }

    if def::on_same_side(state.player, squares[to]) {
        return false
    }

    let moving_piece = squares[from];
    let bitboard = state.bitboard;
    let bitmask = state.bitmask;
    let to_index_mask = bitmask.index_masks[to];

    if def::is_n(moving_piece) {
        if bitmask.n_attack_masks[from] & to_index_mask != 0 {
            return true
        }
    } else if def::is_b(moving_piece) {
        if bitmask.b_attack_masks[from] & to_index_mask != 0 {
            return true
        }
    } else if def::is_r(moving_piece) {
        if bitmask.r_attack_masks[from] & to_index_mask != 0 {
            return true
        }
    } else if def::is_q(moving_piece) {
        if (bitmask.b_attack_masks[from] | bitmask.r_attack_masks[from]) & to_index_mask != 0 {
            return true
        }
    } else if def::is_k(moving_piece) {
        if bitmask.k_attack_masks[from] & to_index_mask != 0 {
            return true
        }
    } else if def::is_p(moving_piece) {
        if moving_piece == def::WP {
            if bitmask.wp_attack_masks[from] & to_index_mask & (bitboard.b_all | bitmask.index_masks[state.enp_square]) != 0 {
                return true
            }

            if bitmask.wp_mov_masks[from] & to_index_mask & !(bitboard.w_all | bitboard.b_all) != 0 {
                return true
            }

            if bitmask.wp_init_mov_masks[from] == to_index_mask && (bitmask.wp_mov_masks[from] | bitmask.wp_init_mov_masks[from]) & (bitboard.w_all | bitboard.b_all) == 0 {
                return true
            }
        } else {
            if bitmask.bp_attack_masks[from] & to_index_mask & (bitboard.w_all | (bitmask.index_masks[state.enp_square] & ENP_SQRS_MASK)) != 0 {
                return true
            }

            if bitmask.bp_mov_masks[from] & to_index_mask & !(bitboard.w_all | bitboard.b_all) != 0 {
                return true
            }

            if bitmask.bp_init_mov_masks[from] == to_index_mask && (bitmask.bp_mov_masks[from] | bitmask.bp_init_mov_masks[from]) & (bitboard.w_all | bitboard.b_all) == 0 {
                return true
            }
        }
    }

    false
}

pub fn count_rook_mobility(state: &State, index: usize, player: u8) -> i32 {
    let bitboard = state.bitboard;
    let bitmask = state.bitmask;

    let (opponent_mask, self_mask) = if player == def::PLAYER_W {
        (bitboard.b_all, bitboard.w_all)
    } else {
        (bitboard.w_all, bitboard.b_all)
    };

    let occupy_mask = opponent_mask | self_mask;

    let mut reach_mask = 0;

    let up_attack_mask = bitmask.up_attack_masks[index];
    reach_mask ^= up_attack_mask;
    if up_attack_mask & occupy_mask != 0 {
        let lowest_blocker_index = get_lowest_index(up_attack_mask & occupy_mask);
        reach_mask &= !bitmask.up_attack_masks[lowest_blocker_index];
    }

    let right_attack_mask = bitmask.right_attack_masks[index];
    reach_mask ^= right_attack_mask;
    if right_attack_mask & occupy_mask != 0 {
        let lowest_blocker_index = get_lowest_index(right_attack_mask & occupy_mask);
        reach_mask &= !bitmask.right_attack_masks[lowest_blocker_index];
    }

    let down_attack_mask = bitmask.down_attack_masks[index];
    reach_mask ^= down_attack_mask;
    if down_attack_mask & occupy_mask != 0 {
        let highest_blocker_index = get_highest_index(down_attack_mask & occupy_mask);
        reach_mask &= !bitmask.down_attack_masks[highest_blocker_index];
    }

    let left_attack_mask = bitmask.left_attack_masks[index];
    reach_mask ^= left_attack_mask;
    if left_attack_mask & occupy_mask != 0 {
        let highest_blocker_index = get_highest_index(left_attack_mask & occupy_mask);
        reach_mask &= !bitmask.left_attack_masks[highest_blocker_index];
    }

    (reach_mask & !self_mask).count_ones() as i32
}

pub fn count_bishop_mobility(state: &State, index: usize, player: u8) -> i32 {
    let bitboard = state.bitboard;
    let bitmask = state.bitmask;

    let (opponent_mask, self_mask) = if player == def::PLAYER_W {
        (bitboard.b_all, bitboard.w_all)
    } else {
        (bitboard.w_all, bitboard.b_all)
    };

    let occupy_mask = opponent_mask | self_mask;

    let mut reach_mask = 0;

    let up_left_attack_mask = bitmask.up_left_attack_masks[index];
    reach_mask ^= up_left_attack_mask;
    if up_left_attack_mask & occupy_mask != 0 {
        let lowest_blocker_index = get_lowest_index(up_left_attack_mask & occupy_mask);
        reach_mask &= !bitmask.up_left_attack_masks[lowest_blocker_index];
    }

    let up_right_attack_mask = bitmask.up_right_attack_masks[index];
    reach_mask ^= up_right_attack_mask;
    if up_right_attack_mask & occupy_mask != 0 {
        let lowest_blocker_index = get_lowest_index(up_right_attack_mask & occupy_mask);
        reach_mask &= !bitmask.up_right_attack_masks[lowest_blocker_index];
    }

    let down_left_attack_mask = bitmask.down_left_attack_masks[index];
    reach_mask ^= down_left_attack_mask;
    if down_left_attack_mask & occupy_mask != 0 {
        let highest_blocker_index = get_highest_index(down_left_attack_mask & occupy_mask);
        reach_mask &= !bitmask.down_left_attack_masks[highest_blocker_index];
    }

    let down_right_attack_mask = bitmask.down_right_attack_masks[index];
    reach_mask ^= down_right_attack_mask;
    if down_right_attack_mask & occupy_mask != 0 {
        let highest_blocker_index = get_highest_index(down_right_attack_mask & occupy_mask);
        reach_mask &= !bitmask.down_right_attack_masks[highest_blocker_index];
    }

    (reach_mask & !self_mask).count_ones() as i32
}

pub fn count_knight_mobility(state: &State, index: usize, player: u8) -> i32 {
    let bitboard = state.bitboard;
    let bitmask = state.bitmask;

    let self_mask = if player == def::PLAYER_W {
        bitboard.w_all
    } else {
        bitboard.b_all
    };

    (bitmask.n_attack_masks[index] & !self_mask).count_ones() as i32
}

#[inline]
fn get_lowest_index(mask: u64) -> usize {
    mask.trailing_zeros() as usize
}

#[inline]
fn get_highest_index(mask: u64) -> usize {
    63 - mask.leading_zeros() as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        def,
        bitboard::BitMask,
        state::State,
        prng::XorshiftPrng,
        util,
    };

    fn gen_reg_movs_test_helper(fen: &str, expected_cap_list: Vec<&str>, expected_non_cap_list: Vec<&str>, debug: bool) {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new(fen, &zob_keys, &bitmask);

        let mut cap_list = [0; def::MAX_CAP_COUNT];
        let mut mov_list = [0; def::MAX_MOV_COUNT];

        gen_reg_mov_list(&state, &mut cap_list, &mut mov_list);

        if debug {
            println!("Captures:");
            for cap_index in 0..def::MAX_CAP_COUNT {
                let cap = cap_list[cap_index];
                if cap == 0 {
                    break
                }

                println!("{}", util::format_mov(cap));
            }

            println!("Moves:");
            for mov_index in 0..def::MAX_MOV_COUNT {
                let mov = mov_list[mov_index];
                if mov == 0 {
                    break
                }

                println!("{}", util::format_mov(mov));
            }
        }

        let mut cap_counter = 0;
        let mut mov_counter = 0;

        for cap_index in 0..def::MAX_CAP_COUNT {
            let cap = cap_list[cap_index];
            if cap == 0 {
                break
            }

            cap_counter += 1;

            let mov_str = util::format_mov(cap);
            if !expected_cap_list.contains(&&*mov_str) {
                assert!(false, "{} not matched", mov_str);
            }
        }

        for mov_index in 0..def::MAX_MOV_COUNT {
            let mov = mov_list[mov_index];
            if mov == 0 {
                break
            }

            mov_counter += 1;

            let mov_str = util::format_mov(mov);
            if !expected_non_cap_list.contains(&&*mov_str) {
                assert!(false, "{} not matched", mov_str);
            }
        }

        assert_eq!(cap_counter, expected_cap_list.len(), "capture count do not match");
        assert_eq!(mov_counter, expected_non_cap_list.len(), "non-capture count do not match");
    }

    fn gen_cas_movs_test_helper(fen: &str, expected_cas_mov_list: Vec<&str>, debug: bool) {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new(fen, &zob_keys, &bitmask);

        let mut cas_list = [0; def::MAX_CAS_COUNT];

        gen_castle_mov_list(&state, &mut cas_list);

        if debug {
            println!("Castles:");
            for cas_index in 0..def::MAX_CAS_COUNT {
                let cas = cas_list[cas_index];

                if cas == 0 {
                    break
                }

                println!("{}", util::format_mov(cas));
            }
        }

        let mut cas_count = 0;

        for cas_index in 0..def::MAX_CAS_COUNT {
            let cas = cas_list[cas_index];

            if cas == 0 {
                break
            }

            cas_count += 1;

            let mov_str = util::format_mov(cas);
            if !expected_cas_mov_list.contains(&&*mov_str) {
                assert!(false, "{} not matched", mov_str);
            }
        }

        assert_eq!(cas_count, expected_cas_mov_list.len(), "castle count do not match");
    }

    #[test]
    fn test_gen_movs_1() {
        gen_reg_movs_test_helper(
            "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1",
            vec!["e4d5"],
            vec![
                "e4e5", "a2a3", "a2a4", "b2b3", "b2b4", "c2c3", "c2c4", "d2d3", "d2d4", "f2f3", "f2f4", "g2g3", "g2g4", "h2h3", "h2h4",
                "b1a3", "b1c3", "g1e2", "g1f3", "g1h3", "f1e2", "f1d3", "f1c4", "f1b5", "f1a6", "d1e2", "d1f3", "d1g4", "d1h5", "e1e2",
                ],
            false,
        );
    }

    #[test]
    fn test_gen_movs_2() {
        gen_reg_movs_test_helper(
            "2k2r2/pp2br2/1np1p2q/2NpP2p/3P2p1/2PN4/PP2Q1PP/3R1R1K b - - 8 27",
            vec!["e7c5", "f7f1"],
            vec![
                "a7a6", "a7a5", "b6a4", "b6a8", "b6c4", "b6d7", "c8b8", "c8c7", "c8d7", "c8d8", "e7d8", "e7d6", "e7f6", "e7g5", "e7h4",
                "f8d8", "f8e8", "f8g8", "f8h8", "f7f6", "f7f5", "f7f4", "f7f3", "f7f2", "f7g7", "f7h7",
                "h6g6", "h6f6", "h6g7", "h6h7", "h6h8", "h6g5", "h6f4", "h6e3", "h6d2", "h6c1",
                "g4g3", "h5h4",
                ],
            false,
        );
    }

    #[test]
    fn test_gen_movs_3() {
        gen_reg_movs_test_helper(
            "r5rk/2p1Nppp/3p3P/pp2p1P1/4P3/2qnPQK1/8/R6R w - - 1 0",
            vec!["a1a5", "e7g8", "h6g7", "f3f7"],
            vec![
                "g5g6",
                "a1a2", "a1a3", "a1a4", "a1b1", "a1c1", "a1d1", "a1e1", "a1f1", "a1g1",
                "h1g1", "h1f1", "h1e1", "h1d1", "h1c1", "h1b1", "h1h2", "h1h3", "h1h4", "h1h5",
                "e7c8", "e7c6", "e7d5", "e7f5", "e7g6",
                "f3e2", "f3d1", "f3g4", "f3h5", "f3g2", "f3f2", "f3f1", "f3f4", "f3f5", "f3f6",
                "g3g2", "g3f2", "g3g4", "g3h2", "g3h3", "g3h4", "g3f4"
                ],
            false,
        );
    }

    #[test]
    fn test_gen_movs_4() {
        gen_reg_movs_test_helper(
            "r1bqk1nr/pPpp1ppp/2n5/2b1p3/2B1P3/2N2N2/P1PP1PPP/R1BQK2R w KQkq - 0 1",
            vec!["b7a8q", "b7c8q", "b7a8r", "b7c8r", "b7a8b", "b7c8b", "b7a8n", "b7c8n", "f3e5", "c4f7"],
            vec![
                "a2a3", "a2a4", "d2d3", "d2d4", "g2g3", "g2g4", "h2h3", "h2h4",
                "b7b8q", "b7b8r", "b7b8b", "b7b8n",
                "a1b1",
                "c1b2", "c1a3",
                "c3b1", "c3a4", "c3b5", "c3d5", "c3e2",
                "c4b3", "c4b5", "c4a6", "c4d3", "c4e2", "c4f1", "c4d5", "c4e6",
                "f3d4", "f3g5", "f3h4", "f3g1",
                "d1e2",
                "e1f1", "e1e2",
                "h1g1", "h1f1",
                ],
            false,
        );
    }

    #[test]
    fn test_gen_movs_5() {
        gen_reg_movs_test_helper(
            "2k2r2/pp2br2/1np1p3/2NpP2p/3P2p1/2PN4/PP2Q1PP/2qR1R1K b - - 8 27",
            vec!["e7c5", "f7f1", "c1d1", "c1b2", "c1c3"],
            vec![
                "a7a6", "a7a5", "b6a4", "b6a8", "b6c4", "b6d7", "c8b8", "c8c7", "c8d7", "c8d8", "e7d8", "e7d6", "e7f6", "e7g5", "e7h4",
                "f8d8", "f8e8", "f8g8", "f8h8", "f7f6", "f7f5", "f7f4", "f7f3", "f7f2", "f7g7", "f7h7",
                "c1a1", "c1b1", "c1c2", "c1d2", "c1e3", "c1f4", "c1g5", "c1h6",
                "g4g3", "h5h4",
                ],
            false,
        );
    }

    #[test]
    fn test_gen_movs_6() {
        gen_cas_movs_test_helper(
            "r1bqk1nr/pPpp1ppp/2n5/2b1p3/2B1P3/2N2N2/P1PP1PPP/R1BQK2R w KQkq - 0 1",
            vec!["e1g1"],
            false,
        );
    }

    #[test]
    fn test_gen_movs_7() {
        gen_cas_movs_test_helper(
            "r3k2r/pbppnppp/1bn2q2/4p3/2B5/2N1PN2/PPPP1PPP/R1BQK2R b KQkq - 0 1",
            vec!["e8g8", "e8c8"],
            false,
        );
    }

    #[test]
    fn test_gen_movs_8() {
        gen_cas_movs_test_helper(
            "1r2k2r/pbppnppp/1bn2q2/4p3/2B5/2N1PN2/PPPP1PPP/R1BQK2R b KQk - 0 1",
            vec!["e8g8"],
            false,
        );
    }

    #[test]
    fn test_gen_movs_9() {
        gen_reg_movs_test_helper(
            "4Q1k1/pp3ppp/3B4/q2p4/5P1P/P3PbPK/1P1r4/2R5 w - - 3 5",
            vec!["e8g8", "e8f7"],
            vec![
                "e8f8", "e8d8", "e8c8", "e8b8", "e8a8", "e8e7", "e8e6", "e8e5", "e8e4", "e8d7", "e8c6", "e8b5", "e8a4",
                "d6c7", "d6b8", "d6c5", "d6b4", "d6e5", "d6e7", "d6f8",
                "a3a4", "b2b3", "b2b4",
                "e3e4", "f4f5", "g3g4", "h4h5",
                "c1b1", "c1a1", "c1d1", "c1e1", "c1f1", "c1g1", "c1h1", "c1c2", "c1c3", "c1c4", "c1c5", "c1c6", "c1c7", "c1c8",
                "h3g2", "h3h2", "h3g4",
                ],
            false,
        );
    }

    #[test]
    fn test_attack_check_1() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("3rr3/2pq2pk/p2p1pnp/8/2QBPP2/1P6/P5PP/4RRK1 b - - 0 1", &zob_keys, &bitmask);

        assert!(is_under_attack(&state, util::map_sqr_notation_to_index("f6"), def::PLAYER_B));
        assert!(is_under_attack(&state, util::map_sqr_notation_to_index("c7"), def::PLAYER_B));
        assert!(is_under_attack(&state, util::map_sqr_notation_to_index("a6"), def::PLAYER_B));
        assert!(is_under_attack(&state, util::map_sqr_notation_to_index("e4"), def::PLAYER_W));
        assert!(!is_under_attack(&state, util::map_sqr_notation_to_index("d4"), def::PLAYER_W));
    }

    #[test]
    fn test_attack_check_2() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("8/8/1k6/8/1R6/3K4/8/8 w - - 0 1", &zob_keys, &bitmask);

        assert!(is_under_attack(&state, util::map_sqr_notation_to_index("b6"), def::PLAYER_B));
        assert!(is_under_attack(&state, util::map_sqr_notation_to_index("c7"), def::PLAYER_W));
    }

    #[test]
    fn test_king_check_1() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("3rr1k1/2pq2p1/p2p1pnp/8/2BBPP2/1PQ5/P5PP/4RRK1 b - - 0 1", &zob_keys, &bitmask);

        assert!(is_in_check(&state, def::PLAYER_B));
    }

    #[test]
    fn test_king_check_2() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("3rr1k1/2pq2p1/p2pNpnp/8/2QBPP2/1P1B4/P5PP/4RRK1 b - - 0 1", &zob_keys, &bitmask);

        assert!(!is_in_check(&state, def::PLAYER_B));
    }

    #[test]
    fn test_king_check_3() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("r2qnkn1/p2b2br/1p1p1pp1/2pPpp2/1PP1P2K/PRNBB3/3QNPPP/5R2 w - - 0 1", &zob_keys, &bitmask);

        assert!(is_in_check(&state, def::PLAYER_W));
        assert!(!is_in_check(&state, def::PLAYER_B));
    }

    #[test]
    fn test_king_check_4() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("r2q1kn1/p2b1rb1/1p1p1pp1/2pPpn2/1PP1P3/PRNBB1K1/3QNPPP/5R2 w - - 0 1", &zob_keys, &bitmask);

        assert!(is_in_check(&state, def::PLAYER_W));
        assert!(!is_in_check(&state, def::PLAYER_B));
    }

    #[test]
    fn test_king_check_5() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("r2q1k2/p2bPrbR/1p1p1ppn/2pPpn2/1PP1P3/P1NBB3/3QNPPP/5RK1 b - - 0 1", &zob_keys, &bitmask);

        assert!(is_in_check(&state, def::PLAYER_B));
        assert!(!is_in_check(&state, def::PLAYER_W));
    }

    #[test]
    fn test_king_check_6() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("r1bqkbnr/pppppppp/3n1n2/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &zob_keys, &bitmask);

        assert!(!is_in_check(&state, def::PLAYER_W));
        assert!(!is_in_check(&state, def::PLAYER_B));
    }

    #[test]
    fn test_king_check_7() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("8/1B6/8/3k3p/8/6K1/8/4b3 w - - 0 1", &zob_keys, &bitmask);

        assert!(is_in_check(&state, def::PLAYER_W));
        assert!(is_in_check(&state, def::PLAYER_B));
    }

    #[test]
    fn test_count_rook_mobility() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("r2q1k2/p2bPrb1/1p1p1ppn/2pP1n2/1PP1R3/P1NB4/3QNPPP/5RK1 b - - 0 1", &zob_keys, &bitmask);

        assert_eq!(7, count_rook_mobility(&state, util::map_sqr_notation_to_index("e4"), def::PLAYER_W));
        assert_eq!(1, count_rook_mobility(&state, util::map_sqr_notation_to_index("f7"), def::PLAYER_B));
    }

    #[test]
    fn test_count_bishop_mobility() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("r2q1k2/p2bPrb1/1p1p1ppn/2pP1n2/1PP1R3/P1NB4/3QNPPP/5RK1 b - - 0 1", &zob_keys, &bitmask);

        assert_eq!(2, count_bishop_mobility(&state, util::map_sqr_notation_to_index("d3"), def::PLAYER_W));
        assert_eq!(6, count_bishop_mobility(&state, util::map_sqr_notation_to_index("d7"), def::PLAYER_B));
    }

    #[test]
    fn test_count_knight_mobility() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("r2q1k2/p2bPrb1/1p1p1ppn/1NpP1n2/1PP1R3/P2B4/3QNPPP/5RK1 b - - 0 1", &zob_keys, &bitmask);

        assert_eq!(5, count_knight_mobility(&state, util::map_sqr_notation_to_index("b5"), def::PLAYER_W));
        assert_eq!(2, count_knight_mobility(&state, util::map_sqr_notation_to_index("h6"), def::PLAYER_B));
    }
}
