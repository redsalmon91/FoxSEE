/*
 * Copyright (C) 2020-2022 Zixiao Han
 */

use crate::{
    bitmask,
    def,
    state::State,
    util::{self, get_lowest_index},
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

pub fn gen_reg_mov_list(state: &State, mov_list: &mut [u32; def::MAX_MOV_COUNT]) {
    let player = state.player;
    let bitboard = state.bitboard;
    let bitmask = bitmask::get_bitmask();

    let occupy_mask = bitboard.w_all | bitboard.b_all;
    let empty_mask = !occupy_mask;

    let mut mov_count = 0;

    let mut add_mov = |from: usize, to: usize, tp: u8, promo: u8| {
        mov_list[mov_count] = util::encode_u32_mov(from, to, tp, promo);
        mov_count += 1;
    };

    let self_mask = if player == def::PLAYER_W {
        bitboard.w_all
    } else {
        bitboard.b_all
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

                if attack_mask != 0 {
                    let start_index = get_lowest_index(attack_mask);
                    attack_mask >>= start_index;

                    let mut attack_index = start_index;
                    while attack_mask != 0 {
                        if attack_mask & 1u64 != 0 {
                            if attack_index > 55 {
                                add_mov(from_index, attack_index, def::MOV_PROMO, def::WQ);
                                add_mov(from_index, attack_index, def::MOV_PROMO, def::WR);
                                add_mov(from_index, attack_index, def::MOV_PROMO, def::WB);
                                add_mov(from_index, attack_index, def::MOV_PROMO, def::WN);
                            } else {
                                if attack_index == enp_square_index {
                                    add_mov(from_index, attack_index, def::MOV_ENP, 0);
                                } else {
                                    add_mov(from_index, attack_index, def::MOV_REG, 0);
                                }
                            }
                        }

                        attack_mask >>= 1;
                        attack_index += 1;
                    }
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

                if attack_mask != 0 {
                    let start_index = get_lowest_index(attack_mask);
                    attack_mask >>= start_index;

                    let mut attack_index = start_index;
                    while attack_mask != 0 {
                        if attack_mask & 1u64 != 0 {
                            if attack_index < 8 {
                                add_mov(from_index, attack_index, def::MOV_PROMO, def::BQ);
                                add_mov(from_index, attack_index, def::MOV_PROMO, def::BR);
                                add_mov(from_index, attack_index, def::MOV_PROMO, def::BB);
                                add_mov(from_index, attack_index, def::MOV_PROMO, def::BN);
                            } else {
                                if attack_index == enp_square_index {
                                    add_mov(from_index, attack_index, def::MOV_ENP, 0);
                                } else {
                                    add_mov(from_index, attack_index, def::MOV_REG, 0);
                                }
                            }
                        }

                        attack_mask >>= 1;
                        attack_index += 1;
                    }
                }
            }
        } else if def::is_n(moving_piece) {
            let mut mov_mask = bitmask.n_attack_masks[from_index] & !self_mask;

            if mov_mask != 0 {
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
        } else if def::is_b(moving_piece) {
            let mut mov_mask = 0;

            mov_mask |= bitmask.diag_up_attack_masks[from_index][util::kindergarten_transform_rank_diag(occupy_mask & bitmask.diag_up_masks[from_index]) as usize];
            mov_mask |= bitmask.diag_down_attack_masks[from_index][util::kindergarten_transform_rank_diag(occupy_mask & bitmask.diag_down_masks[from_index]) as usize];

            mov_mask &= !self_mask;

            if mov_mask != 0 {
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
        } else if def::is_r(moving_piece) {
            let mut mov_mask = 0;

            mov_mask |= bitmask.horizontal_attack_masks[from_index][util::kindergarten_transform_rank_diag(occupy_mask & bitmask.rank_masks[from_index]) as usize];
            mov_mask |= bitmask.vertical_attack_masks[from_index][util::kindergarten_transform_file(occupy_mask & bitmask.file_masks[from_index], from_index) as usize];

            mov_mask &= !self_mask;

            if mov_mask != 0 {
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
        } else if def::is_q(moving_piece) {
            let mut mov_mask = 0;
            mov_mask |= bitmask.horizontal_attack_masks[from_index][util::kindergarten_transform_rank_diag(occupy_mask & bitmask.rank_masks[from_index]) as usize];
            mov_mask |= bitmask.vertical_attack_masks[from_index][util::kindergarten_transform_file(occupy_mask & bitmask.file_masks[from_index], from_index) as usize];
            mov_mask |= bitmask.diag_up_attack_masks[from_index][util::kindergarten_transform_rank_diag(occupy_mask & bitmask.diag_up_masks[from_index]) as usize];
            mov_mask |= bitmask.diag_down_attack_masks[from_index][util::kindergarten_transform_rank_diag(occupy_mask & bitmask.diag_down_masks[from_index]) as usize];

            mov_mask &= !self_mask;

            if mov_mask != 0 {
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
        } else if def::is_k(moving_piece) {
            let mut mov_mask = bitmask.k_attack_masks[from_index] & !self_mask;

            if mov_mask != 0 {
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

    let cas_rights = state.cas_rights;
    let bitboard = state.bitboard;
    let all_mask = bitboard.w_all | bitboard.b_all;

    if state.player == def::PLAYER_W {
        if cas_rights & 0b1000 != 0 {
            if bitboard.w_all & CAS_WK_OCCUPY_MASK == CAS_WK_OCCUPY_MASK
            && bitboard.w_rook & CAS_WK_R_MASK == CAS_WK_R_MASK
            && all_mask & CAS_WK_EMPTY_MASK == 0
            && !is_under_attack(state, def::CAS_SQUARE_WK, def::PLAYER_W)
            && !is_under_attack(state, def::CAS_SQUARE_WK - 1, def::PLAYER_W)
            && !is_under_attack(state, def::CAS_SQUARE_WK - 2, def::PLAYER_W) {
                add_mov(def::CAS_SQUARE_WK - 2, def::CAS_SQUARE_WK, def::MOV_CAS, 0);
            }
        }

        if cas_rights & 0b0100 != 0 {
            if bitboard.w_all & CAS_WQ_OCCUPY_MASK == CAS_WQ_OCCUPY_MASK
            && bitboard.w_rook & CAS_WQ_R_MASK == CAS_WQ_R_MASK
            && all_mask & CAS_WQ_EMPTY_MASK == 0
            && !is_under_attack(state, def::CAS_SQUARE_WQ, def::PLAYER_W)
            && !is_under_attack(state, def::CAS_SQUARE_WQ + 1, def::PLAYER_W)
            && !is_under_attack(state, def::CAS_SQUARE_WQ + 2, def::PLAYER_W) {
                add_mov(def::CAS_SQUARE_WQ + 2, def::CAS_SQUARE_WQ, def::MOV_CAS, 0);
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
                add_mov(def::CAS_SQUARE_BK - 2, def::CAS_SQUARE_BK, def::MOV_CAS, 0);
            }
        }

        if cas_rights & 0b0001 != 0 {
            if bitboard.b_all & CAS_BQ_OCCUPY_MASK == CAS_BQ_OCCUPY_MASK
            && bitboard.b_rook & CAS_BQ_R_MASK == CAS_BQ_R_MASK
            && all_mask & CAS_BQ_EMPTY_MASK == 0
            && !is_under_attack(state, def::CAS_SQUARE_BQ, def::PLAYER_B)
            && !is_under_attack(state, def::CAS_SQUARE_BQ + 1, def::PLAYER_B)
            && !is_under_attack(state, def::CAS_SQUARE_BQ + 2, def::PLAYER_B) {
                add_mov(def::CAS_SQUARE_BQ + 2, def::CAS_SQUARE_BQ, def::MOV_CAS, 0);
            }
        }
    }
}

pub fn gen_capture_and_promo_list(state: &State, cap_list: &mut [u32; def::MAX_CAP_COUNT]) {
    let player = state.player;
    let bitboard = state.bitboard;
    let bitmask = bitmask::get_bitmask();

    let occupy_mask = bitboard.w_all | bitboard.b_all;
    let empty_mask = !occupy_mask;

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
                if bitmask.wp_mov_masks[from_index] & empty_mask != 0 {
                    let to_index = from_index + 8;
                    if to_index > 55 {
                        add_cap(from_index, to_index, def::MOV_PROMO, def::WQ);
                        add_cap(from_index, to_index, def::MOV_PROMO, def::WR);
                        add_cap(from_index, to_index, def::MOV_PROMO, def::WB);
                        add_cap(from_index, to_index, def::MOV_PROMO, def::WN);
                    }
                }

                let enp_square_index = state.enp_square;
                let mut attack_mask = bitmask.wp_attack_masks[from_index] & (bitboard.b_all | bitmask.index_masks[enp_square_index]);

                if attack_mask != 0 {
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
                }
            } else {
                if bitmask.bp_mov_masks[from_index] & empty_mask != 0 {
                    let to_index = from_index - 8;
                    if to_index < 8 {
                        add_cap(from_index, to_index, def::MOV_PROMO, def::BQ);
                        add_cap(from_index, to_index, def::MOV_PROMO, def::BR);
                        add_cap(from_index, to_index, def::MOV_PROMO, def::BB);
                        add_cap(from_index, to_index, def::MOV_PROMO, def::BN);
                    }
                }

                let enp_square_index = state.enp_square;
                let mut attack_mask = bitmask.bp_attack_masks[from_index] & (bitboard.w_all | (bitmask.index_masks[enp_square_index] & ENP_SQRS_MASK));

                if attack_mask != 0 {
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
            }
        } else if def::is_n(moving_piece) {
            let mut attack_mask = bitmask.n_attack_masks[from_index] & opponent_mask;

            if attack_mask != 0 {
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
        } else if def::is_b(moving_piece) {
            let mut mov_mask = 0;

            mov_mask |= bitmask.diag_up_attack_masks[from_index][util::kindergarten_transform_rank_diag(occupy_mask & bitmask.diag_up_masks[from_index]) as usize];
            mov_mask |= bitmask.diag_down_attack_masks[from_index][util::kindergarten_transform_rank_diag(occupy_mask & bitmask.diag_down_masks[from_index]) as usize];

            let mut attack_mask = mov_mask & opponent_mask;

            if attack_mask != 0 {
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
        } else if def::is_r(moving_piece) {
            let mut mov_mask = 0;

            mov_mask |= bitmask.horizontal_attack_masks[from_index][util::kindergarten_transform_rank_diag(occupy_mask & bitmask.rank_masks[from_index]) as usize];
            mov_mask |= bitmask.vertical_attack_masks[from_index][util::kindergarten_transform_file(occupy_mask & bitmask.file_masks[from_index], from_index) as usize];

            let mut attack_mask = mov_mask & opponent_mask;

            if attack_mask != 0 {
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
        } else if def::is_q(moving_piece) {
            let mut mov_mask = 0;

            mov_mask |= bitmask.horizontal_attack_masks[from_index][util::kindergarten_transform_rank_diag(occupy_mask & bitmask.rank_masks[from_index]) as usize];
            mov_mask |= bitmask.vertical_attack_masks[from_index][util::kindergarten_transform_file(occupy_mask & bitmask.file_masks[from_index], from_index) as usize];
            mov_mask |= bitmask.diag_up_attack_masks[from_index][util::kindergarten_transform_rank_diag(occupy_mask & bitmask.diag_up_masks[from_index]) as usize];
            mov_mask |= bitmask.diag_down_attack_masks[from_index][util::kindergarten_transform_rank_diag(occupy_mask & bitmask.diag_down_masks[from_index]) as usize];

            let mut attack_mask = mov_mask & opponent_mask;

            if attack_mask != 0 {
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
        } else if def::is_k(moving_piece) {
            let mut attack_mask = bitmask.k_attack_masks[from_index] & opponent_mask;

            if attack_mask != 0 {
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
    let bitmask = bitmask::get_bitmask();

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
        let horizontal_attack_mask = bitmask.horizontal_attack_masks[index][util::kindergarten_transform_rank_diag(blocker_mask & bitmask.rank_masks[index]) as usize];
        if horizontal_attack_mask & opponent_rq_mask != 0 {
            return true;
        }

        let vertical_attack_mask = bitmask.vertical_attack_masks[index][util::kindergarten_transform_file(blocker_mask & bitmask.file_masks[index], index) as usize];
        if vertical_attack_mask & opponent_rq_mask != 0 {
            return true;
        }
    }

    let opponent_bq_mask = if player == def::PLAYER_W {
        bitboard.b_bishop | bitboard.b_queen
    } else {
        bitboard.w_bishop | bitboard.w_queen
    };

    if opponent_bq_mask != 0 {
        let diag_up_attack_mask = bitmask.diag_up_attack_masks[index][util::kindergarten_transform_rank_diag(blocker_mask & bitmask.diag_up_masks[index]) as usize];
        if diag_up_attack_mask & opponent_bq_mask != 0 {
            return true;
        }

        let diag_down_attack_mask = bitmask.diag_down_attack_masks[index][util::kindergarten_transform_rank_diag(blocker_mask & bitmask.diag_down_masks[index]) as usize];
        if diag_down_attack_mask & opponent_bq_mask != 0 {
            return true;
        }
    }

    false
}

pub fn get_smallest_attacker_index(state: &mut State, index: usize) -> (u8, u8, u8, usize) {
    let bitmask = bitmask::get_bitmask();
    let bitboard = state.bitboard;
    let player = state.player;

    if player == def::PLAYER_W {
        let mut attack_mask = bitmask.bp_attack_masks[index] & bitboard.w_pawn;
        let mut attack_index = 0;
        while attack_mask != 0 {
            if attack_mask & 1u64 != 0 {
                if index > 55 {
                    if is_valid_attacker(state, attack_index, index) {
                        return (def::WP, def::MOV_PROMO, def::WQ, attack_index)
                    }
                } else {
                    if is_valid_attacker(state, attack_index, index) {
                        return (def::WP, def::MOV_REG, 0, attack_index)
                    }
                }
            }

            attack_mask >>= 1;
            attack_index += 1;
        }

        let n_attack_mask = bitmask.n_attack_masks[index];

        let mut attack_mask = n_attack_mask & bitboard.w_knight;
        let mut attack_index = 0;
        while attack_mask != 0 {
            if attack_mask & 1u64 != 0 {
                if is_valid_attacker(state, attack_index, index) {
                    return (def::WN, def::MOV_REG, 0, attack_index)
                }
            }

            attack_mask >>= 1;
            attack_index += 1;
        }

        let bq_mask = bitboard.w_bishop | bitboard.w_queen;
        let mut bq_attack_mask = 0;

        bq_attack_mask |= bitmask.diag_up_attack_masks[index][util::kindergarten_transform_rank_diag(bq_mask & bitmask.diag_up_masks[index]) as usize];
        bq_attack_mask |= bitmask.diag_down_attack_masks[index][util::kindergarten_transform_rank_diag(bq_mask & bitmask.diag_down_masks[index]) as usize];

        let mut attack_mask = bq_attack_mask & bitboard.w_bishop;
        let mut attack_index = 0;
        while attack_mask != 0 {
            if attack_mask & 1u64 != 0 {
                if is_valid_attacker(state, attack_index, index) {
                    return (def::WB, def::MOV_REG, 0, attack_index)
                }
            }

            attack_mask >>= 1;
            attack_index += 1;
        }

        let rq_mask = bitboard.w_rook | bitboard.w_queen;
        let mut rq_attack_mask = 0;

        rq_attack_mask |= bitmask.horizontal_attack_masks[index][util::kindergarten_transform_rank_diag(rq_mask & bitmask.rank_masks[index]) as usize];
        rq_attack_mask |= bitmask.vertical_attack_masks[index][util::kindergarten_transform_file(rq_mask & bitmask.file_masks[index], index) as usize];

        let mut attack_mask = rq_attack_mask & bitboard.w_rook;
        let mut attack_index = 0;
        while attack_mask != 0 {
            if attack_mask & 1u64 != 0 {
                if is_valid_attacker(state, attack_index, index) {
                    return (def::WR, def::MOV_REG, 0, attack_index)
                }
            }

            attack_mask >>= 1;
            attack_index += 1;
        }

        let mut attack_mask = (bq_attack_mask | rq_attack_mask) & bitboard.w_queen;
        let mut attack_index = 0;
        while attack_mask != 0 {
            if attack_mask & 1u64 != 0 {
                if is_valid_attacker(state, attack_index, index) {
                    return (def::WQ, def::MOV_REG, 0, attack_index)
                }
            }

            attack_mask >>= 1;
            attack_index += 1;
        }

        let k_attack_mask = bitmask.k_attack_masks[index];

        if k_attack_mask & bitmask.index_masks[state.wk_index] != 0 {
            return (def::WK, def::MOV_REG, 0, state.wk_index)
        }
    } else {
        let mut attack_mask = bitmask.wp_attack_masks[index] & bitboard.b_pawn;
        let mut attack_index = 0;
        while attack_mask != 0 {
            if attack_mask & 1u64 != 0 {
                if index < 8 {
                    if is_valid_attacker(state, attack_index, index) {
                        return (def::BP, def::MOV_PROMO, def::BQ, attack_index)
                    }
                } else {
                    if is_valid_attacker(state, attack_index, index) {
                        return (def::BP, def::MOV_REG, 0, attack_index)
                    }
                }
            }

            attack_mask >>= 1;
            attack_index += 1;
        }

        let n_attack_mask = bitmask.n_attack_masks[index];

        let mut attack_mask = n_attack_mask & bitboard.b_knight;
        let mut attack_index = 0;
        while attack_mask != 0 {
            if attack_mask & 1u64 != 0 {
                if is_valid_attacker(state, attack_index, index) {
                    return (def::BN, def::MOV_REG, 0, attack_index)
                }
            }

            attack_mask >>= 1;
            attack_index += 1;
        }

        let bq_mask = bitboard.b_bishop | bitboard.b_queen;
        let mut bq_attack_mask = 0;

        bq_attack_mask |= bitmask.diag_up_attack_masks[index][util::kindergarten_transform_rank_diag(bq_mask & bitmask.diag_up_masks[index]) as usize];
        bq_attack_mask |= bitmask.diag_down_attack_masks[index][util::kindergarten_transform_rank_diag(bq_mask & bitmask.diag_down_masks[index]) as usize];

        let mut attack_mask = bq_attack_mask & bitboard.b_bishop;
        let mut attack_index = 0;
        while attack_mask != 0 {
            if attack_mask & 1u64 != 0 {
                if is_valid_attacker(state, attack_index, index) {
                    return (def::BB, def::MOV_REG, 0, attack_index)
                }
            }

            attack_mask >>= 1;
            attack_index += 1;
        }

        let rq_mask = bitboard.b_rook | bitboard.b_queen;
        let mut rq_attack_mask = 0;

        rq_attack_mask |= bitmask.horizontal_attack_masks[index][util::kindergarten_transform_rank_diag(rq_mask & bitmask.rank_masks[index]) as usize];
        rq_attack_mask |= bitmask.vertical_attack_masks[index][util::kindergarten_transform_file(rq_mask & bitmask.file_masks[index], index) as usize];

        let mut attack_mask = rq_attack_mask & bitboard.b_rook;
        let mut attack_index = 0;
        while attack_mask != 0 {
            if attack_mask & 1u64 != 0 {
                if is_valid_attacker(state, attack_index, index) {
                    return (def::BR, def::MOV_REG, 0, attack_index)
                }
            }

            attack_mask >>= 1;
            attack_index += 1;
        }

        let mut attack_mask = (bq_attack_mask | rq_attack_mask) & bitboard.b_queen;
        let mut attack_index = 0;
        while attack_mask != 0 {
            if attack_mask & 1u64 != 0 {
                if is_valid_attacker(state, attack_index, index) {
                    return (def::BQ, def::MOV_REG, 0, attack_index)
                }
            }

            attack_mask >>= 1;
            attack_index += 1;
        }

        let k_attack_mask = bitmask.k_attack_masks[index];

        if k_attack_mask & bitmask.index_masks[state.bk_index] != 0 {
            return (def::BK, def::MOV_REG, 0, state.bk_index)
        }
    }

    (0, 0, 0, 0)
}

fn is_valid_attacker(state: &mut State, from_index: usize, to_index: usize) -> bool {
    state.do_mov(from_index, to_index, def::MOV_REG, 0);

    if state.player == def::PLAYER_W {
        if is_in_check(state, def::PLAYER_B) {
            state.undo_mov(from_index, to_index, def::MOV_REG);
            return false;
        } else {
            state.undo_mov(from_index, to_index, def::MOV_REG);
            return true;
        }
    } else {
        if is_in_check(state, def::PLAYER_W) {
            state.undo_mov(from_index, to_index, def::MOV_REG);
            return false;
        } else {
            state.undo_mov(from_index, to_index, def::MOV_REG);
            return true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        def,
        state::State,
        util,
        zob_keys,
    };

    fn gen_reg_movs_test_helper(fen: &str, expected_mov_list: Vec<&str>, debug: bool) {
        zob_keys::init();
        bitmask::init();

        let state = State::new(fen);

        let mut mov_list = [0; def::MAX_MOV_COUNT];

        gen_reg_mov_list(&state, &mut mov_list);

        if debug {
            println!("Moves:");
            for mov_index in 0..def::MAX_MOV_COUNT {
                let mov = mov_list[mov_index];
                if mov == 0 {
                    break
                }

                println!("{}", util::format_mov(mov));
            }
        }

        let mut mov_counter = 0;

        for mov_index in 0..def::MAX_MOV_COUNT {
            let mov = mov_list[mov_index];
            if mov == 0 {
                break
            }

            mov_counter += 1;

            let mov_str = util::format_mov(mov);
            if !expected_mov_list.contains(&&*mov_str) {
                assert!(false, "{} not matched", mov_str);
            }
        }

        assert_eq!(mov_counter, expected_mov_list.len(), "non-capture count do not match");
    }

    #[test]
    fn test_gen_movs_1() {
        gen_reg_movs_test_helper(
            "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1",
            vec![
                "e4d5",
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
            vec![
                "e7c5", "f7f1",
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
            vec![
                "a1a5", "e7g8", "h6g7", "f3f7",
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
            vec![
                "b7a8q", "b7c8q", "b7a8r", "b7c8r", "b7a8b", "b7c8b", "b7a8n", "b7c8n", "f3e5", "c4f7",
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
                "e1g1"
                ],
            false,
        );
    }

    #[test]
    fn test_gen_movs_5() {
        gen_reg_movs_test_helper(
            "2k2r2/pp2br2/1np1p3/2NpP2p/3P2p1/2PN4/PP2Q1PP/2qR1R1K b - - 8 27",
            vec![
                "e7c5", "f7f1", "c1d1", "c1b2", "c1c3",
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
        gen_reg_movs_test_helper(
            "4Q1k1/pp3ppp/3B4/q2p4/5P1P/P3PbPK/1P1r4/2R5 w - - 3 5",
            vec![
                "e8g8", "e8f7",
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
        zob_keys::init();
        bitmask::init();

        let state = State::new("3rr3/2pq2pk/p2p1pnp/8/2QBPP2/1P6/P5PP/4RRK1 b - - 0 1");

        assert!(is_under_attack(&state, util::map_sqr_notation_to_index("f6"), def::PLAYER_B));
        assert!(is_under_attack(&state, util::map_sqr_notation_to_index("c7"), def::PLAYER_B));
        assert!(is_under_attack(&state, util::map_sqr_notation_to_index("a6"), def::PLAYER_B));
        assert!(is_under_attack(&state, util::map_sqr_notation_to_index("e4"), def::PLAYER_W));
        assert!(!is_under_attack(&state, util::map_sqr_notation_to_index("d4"), def::PLAYER_W));
    }

    #[test]
    fn test_attack_check_2() {
        zob_keys::init();
        bitmask::init();

        let state = State::new("8/8/1k6/8/1R6/3K4/8/8 w - - 0 1");

        assert!(is_under_attack(&state, util::map_sqr_notation_to_index("b6"), def::PLAYER_B));
        assert!(is_under_attack(&state, util::map_sqr_notation_to_index("c7"), def::PLAYER_W));
    }

    #[test]
    fn test_king_check_1() {
        zob_keys::init();
        bitmask::init();

        let state = State::new("3rr1k1/2pq2p1/p2p1pnp/8/2BBPP2/1PQ5/P5PP/4RRK1 b - - 0 1");

        assert!(is_in_check(&state, def::PLAYER_B));
    }

    #[test]
    fn test_king_check_2() {
        zob_keys::init();
        bitmask::init();

        let state = State::new("3rr1k1/2pq2p1/p2pNpnp/8/2QBPP2/1P1B4/P5PP/4RRK1 b - - 0 1");

        assert!(!is_in_check(&state, def::PLAYER_B));
    }

    #[test]
    fn test_king_check_3() {
        zob_keys::init();
        bitmask::init();

        let state = State::new("r2qnkn1/p2b2br/1p1p1pp1/2pPpp2/1PP1P2K/PRNBB3/3QNPPP/5R2 w - - 0 1");

        assert!(is_in_check(&state, def::PLAYER_W));
        assert!(!is_in_check(&state, def::PLAYER_B));
    }

    #[test]
    fn test_king_check_4() {
        zob_keys::init();
        bitmask::init();

        let state = State::new("r2q1kn1/p2b1rb1/1p1p1pp1/2pPpn2/1PP1P3/PRNBB1K1/3QNPPP/5R2 w - - 0 1");

        assert!(is_in_check(&state, def::PLAYER_W));
        assert!(!is_in_check(&state, def::PLAYER_B));
    }

    #[test]
    fn test_king_check_5() {
        zob_keys::init();
        bitmask::init();

        let state = State::new("r2q1k2/p2bPrbR/1p1p1ppn/2pPpn2/1PP1P3/P1NBB3/3QNPPP/5RK1 b - - 0 1");

        assert!(is_in_check(&state, def::PLAYER_B));
        assert!(!is_in_check(&state, def::PLAYER_W));
    }

    #[test]
    fn test_king_check_6() {
        zob_keys::init();
        bitmask::init();

        let state = State::new("r1bqkbnr/pppppppp/3n1n2/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        assert!(!is_in_check(&state, def::PLAYER_W));
        assert!(!is_in_check(&state, def::PLAYER_B));
    }

    #[test]
    fn test_king_check_7() {
        zob_keys::init();
        bitmask::init();

        let state = State::new("8/1B6/8/3k3p/8/6K1/8/4b3 w - - 0 1");

        assert!(is_in_check(&state, def::PLAYER_W));
        assert!(is_in_check(&state, def::PLAYER_B));
    }

    #[test]
    fn test_is_valid_attacker() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("3r2k1/2pq2p1/p3npnp/8/2BBPPb1/1P3N2/P5PP/r1QKRR2 b - - 0 1");

        assert!(!is_valid_attacker(&mut state, util::map_sqr_notation_to_index("e6"), util::map_sqr_notation_to_index("d4")));
        assert!(is_valid_attacker(&mut state, util::map_sqr_notation_to_index("d7"), util::map_sqr_notation_to_index("d4")));
        assert!(is_valid_attacker(&mut state, util::map_sqr_notation_to_index("g4"), util::map_sqr_notation_to_index("f3")));
        assert!(is_valid_attacker(&mut state, util::map_sqr_notation_to_index("g6"), util::map_sqr_notation_to_index("f4")));
        assert!(is_valid_attacker(&mut state, util::map_sqr_notation_to_index("a1"), util::map_sqr_notation_to_index("a2")));

        state.player = def::get_opposite_player(state.player);
        assert!(!is_valid_attacker(&mut state, util::map_sqr_notation_to_index("f3"), util::map_sqr_notation_to_index("g5")));
        assert!(!is_valid_attacker(&mut state, util::map_sqr_notation_to_index("d4"), util::map_sqr_notation_to_index("f6")));
        assert!(!is_valid_attacker(&mut state, util::map_sqr_notation_to_index("c1"), util::map_sqr_notation_to_index("c2")));
        assert!(!is_valid_attacker(&mut state, util::map_sqr_notation_to_index("c1"), util::map_sqr_notation_to_index("d2")));
        assert!(is_valid_attacker(&mut state, util::map_sqr_notation_to_index("c1"), util::map_sqr_notation_to_index("a1")));
        assert!(is_valid_attacker(&mut state, util::map_sqr_notation_to_index("c1"), util::map_sqr_notation_to_index("b1")));
        assert!(is_valid_attacker(&mut state, util::map_sqr_notation_to_index("c4"), util::map_sqr_notation_to_index("e6")));
        assert!(is_valid_attacker(&mut state, util::map_sqr_notation_to_index("e1"), util::map_sqr_notation_to_index("e3")));
        assert!(is_valid_attacker(&mut state, util::map_sqr_notation_to_index("f1"), util::map_sqr_notation_to_index("f2")));
    }
}
