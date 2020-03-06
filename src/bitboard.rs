/*
 * Copyright (C) 2020 Zixiao Han
 */

use crate::def;

pub static WP_PROMO_PAWNS_MASK: u64 = 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
pub static BP_PROMO_PAWNS_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;

#[derive(Copy, Clone)]
pub struct BitBoard {
    pub w_all: u64,
    pub b_all: u64,

    pub w_pawn: u64,
    pub b_pawn: u64,

    pub w_knight: u64,
    pub b_knight: u64,

    pub w_bishop: u64,
    pub b_bishop: u64,

    pub w_rook: u64,
    pub b_rook: u64,

    pub w_queen: u64,
    pub b_queen: u64,
}

impl BitBoard {
    pub fn new() -> Self {
        BitBoard {
            w_all: 0,
            w_pawn: 0,
            w_knight: 0,
            w_bishop: 0,
            w_rook: 0,
            w_queen: 0,

            b_all: 0,
            b_pawn: 0,
            b_knight: 0,
            b_bishop: 0,
            b_rook: 0,
            b_queen: 0,
        }
    }
}

pub struct BitMask {
    pub index_masks: [u64; def::BOARD_SIZE],
    pub file_masks: [u64; def::BOARD_SIZE],

    pub wk_protect_masks: [u64; def::BOARD_SIZE],
    pub bk_protect_masks: [u64; def::BOARD_SIZE],

    pub wp_forward_masks: [u64; def::BOARD_SIZE],
    pub bp_forward_masks: [u64; def::BOARD_SIZE],
    pub wp_behind_masks: [u64; def::BOARD_SIZE],
    pub bp_behind_masks: [u64; def::BOARD_SIZE],

    pub wp_attack_masks: [u64; def::BOARD_SIZE],
    pub bp_attack_masks: [u64; def::BOARD_SIZE],

    pub wp_mov_masks: [u64; def::BOARD_SIZE],
    pub bp_mov_masks: [u64; def::BOARD_SIZE],

    pub wp_init_mov_masks: [u64; def::BOARD_SIZE],
    pub bp_init_mov_masks: [u64; def::BOARD_SIZE],

    pub n_attack_masks: [u64; def::BOARD_SIZE],
    pub k_attack_masks: [u64; def::BOARD_SIZE],
    pub b_attack_masks: [u64; def::BOARD_SIZE],
    pub r_attack_masks: [u64; def::BOARD_SIZE],

    pub left_attack_masks: [u64; def::BOARD_SIZE],
    pub right_attack_masks: [u64; def::BOARD_SIZE],
    pub up_attack_masks: [u64; def::BOARD_SIZE],
    pub down_attack_masks: [u64; def::BOARD_SIZE],

    pub up_left_attack_masks: [u64; def::BOARD_SIZE],
    pub up_right_attack_masks: [u64; def::BOARD_SIZE],
    pub down_left_attack_masks: [u64; def::BOARD_SIZE],
    pub down_right_attack_masks: [u64; def::BOARD_SIZE],
}

impl BitMask {
    pub fn new() -> Self {
        let mut bitmask = BitMask {
            index_masks: [0; def::BOARD_SIZE],
            file_masks: [0; def::BOARD_SIZE],

            wk_protect_masks: [0; def::BOARD_SIZE],
            bk_protect_masks: [0; def::BOARD_SIZE],

            wp_forward_masks: [0; def::BOARD_SIZE],
            bp_forward_masks: [0; def::BOARD_SIZE],
            wp_behind_masks: [0; def::BOARD_SIZE],
            bp_behind_masks: [0; def::BOARD_SIZE],

            wp_attack_masks: [0; def::BOARD_SIZE],
            bp_attack_masks: [0; def::BOARD_SIZE],

            wp_mov_masks: [0; def::BOARD_SIZE],
            bp_mov_masks: [0; def::BOARD_SIZE],

            wp_init_mov_masks: [0; def::BOARD_SIZE],
            bp_init_mov_masks: [0; def::BOARD_SIZE],

            n_attack_masks: [0; def::BOARD_SIZE],
            k_attack_masks: [0; def::BOARD_SIZE],
            b_attack_masks: [0; def::BOARD_SIZE],
            r_attack_masks: [0; def::BOARD_SIZE],

            left_attack_masks: [0; def::BOARD_SIZE],
            right_attack_masks: [0; def::BOARD_SIZE],
            up_attack_masks: [0; def::BOARD_SIZE],
            down_attack_masks: [0; def::BOARD_SIZE],

            up_left_attack_masks: [0; def::BOARD_SIZE],
            up_right_attack_masks: [0; def::BOARD_SIZE],
            down_left_attack_masks: [0; def::BOARD_SIZE],
            down_right_attack_masks: [0; def::BOARD_SIZE],
        };

        bitmask.init_base();

        bitmask.init_n_masks();
        bitmask.init_k_masks();

        bitmask.init_up_left_masks();
        bitmask.init_down_left_masks();
        bitmask.init_up_right_masks();
        bitmask.init_down_right_masks();

        bitmask.init_up_masks();
        bitmask.init_down_masks();
        bitmask.init_left_masks();
        bitmask.init_right_masks();

        bitmask.init_rb_attack_masks();

        bitmask.init_p_attack_masks();
        bitmask.init_p_mov_masks();

        bitmask.init_k_protect_masks();
        bitmask.init_p_misc_masks();

        bitmask
    }

    fn init_base(&mut self) {
        let mut file_masks = [0; def::DIM_SIZE];

        for index in 0..def::BOARD_SIZE {
            let index_mask = 1u64 << index;
            self.index_masks[index] = index_mask;

            let file = index % def::DIM_SIZE;
            file_masks[file] = file_masks[file] | index_mask;
        }

        for index in 0..def::BOARD_SIZE {
            let file = index % def::DIM_SIZE;
            self.file_masks[index] = file_masks[file];
        }
    }

    fn init_n_masks(&mut self) {
        for index in 0..def::BOARD_SIZE {
            if index + 6 < def::BOARD_SIZE
                && (self.index_masks[index] & self.file_masks[0] == 0)
                && (self.index_masks[index] & self.file_masks[1] == 0)
            {
                self.n_attack_masks[index] = self.n_attack_masks[index] | self.index_masks[index + 6];
            }

            if index >= 6
                && (self.index_masks[index] & self.file_masks[6] == 0)
                && (self.index_masks[index] & self.file_masks[7] == 0)
            {
                self.n_attack_masks[index] = self.n_attack_masks[index] | self.index_masks[index - 6];
            }

            if index + 10 < def::BOARD_SIZE
                && (self.index_masks[index] & self.file_masks[6] == 0)
                && (self.index_masks[index] & self.file_masks[7] == 0)
            {
                self.n_attack_masks[index] = self.n_attack_masks[index] | self.index_masks[index + 10];
            }

            if index >= 10
                && (self.index_masks[index] & self.file_masks[0] == 0)
                && (self.index_masks[index] & self.file_masks[1] == 0)
            {
                self.n_attack_masks[index] = self.n_attack_masks[index] | self.index_masks[index - 10];
            }

            if index + 15 < def::BOARD_SIZE && (self.index_masks[index] & self.file_masks[0] == 0) {
                self.n_attack_masks[index] = self.n_attack_masks[index] | self.index_masks[index + 15];
            }

            if index >= 15 && (self.index_masks[index] & self.file_masks[7] == 0) {
                self.n_attack_masks[index] = self.n_attack_masks[index] | self.index_masks[index - 15];
            }

            if index + 17 < def::BOARD_SIZE && (self.index_masks[index] & self.file_masks[7] == 0) {
                self.n_attack_masks[index] = self.n_attack_masks[index] | self.index_masks[index + 17];
            }

            if index >= 17 && (self.index_masks[index] & self.file_masks[0] == 0) {
                self.n_attack_masks[index] = self.n_attack_masks[index] | self.index_masks[index - 17];
            }
        }
    }

    fn init_k_masks(&mut self) {
        for index in 0..def::BOARD_SIZE {
            if index + 1 < def::BOARD_SIZE && (self.index_masks[index] & self.file_masks[7] == 0) {
                self.k_attack_masks[index] = self.k_attack_masks[index] | self.index_masks[index + 1];
            }

            if index >= 1 && (self.index_masks[index] & self.file_masks[0] == 0) {
                self.k_attack_masks[index] = self.k_attack_masks[index] | self.index_masks[index - 1];
            }

            if index + 8 < def::BOARD_SIZE {
                self.k_attack_masks[index] = self.k_attack_masks[index] | self.index_masks[index + 8];
            }

            if index >= 8 {
                self.k_attack_masks[index] = self.k_attack_masks[index] | self.index_masks[index - 8];
            }

            if index + 7 < def::BOARD_SIZE && (self.index_masks[index] & self.file_masks[0] == 0) {
                self.k_attack_masks[index] = self.k_attack_masks[index] | self.index_masks[index + 7];
            }

            if index >= 7 && (self.index_masks[index] & self.file_masks[7] == 0) {
                self.k_attack_masks[index] = self.k_attack_masks[index] | self.index_masks[index - 7];
            }

            if index + 9 < def::BOARD_SIZE && (self.index_masks[index] & self.file_masks[7] == 0) {
                self.k_attack_masks[index] = self.k_attack_masks[index] | self.index_masks[index + 9];
            }

            if index >= 9 && (self.index_masks[index] & self.file_masks[0] == 0) {
                self.k_attack_masks[index] = self.k_attack_masks[index] | self.index_masks[index - 9];
            }
        }
    }

    fn init_up_masks(&mut self) {
        for index in 0..def::BOARD_SIZE {
            let mut up_index = index;
            while up_index < 56 {
                up_index += 8;

                self.up_attack_masks[index] ^= self.index_masks[up_index];
            }
        }
    }

    fn init_down_masks(&mut self) {
        for index in 0..def::BOARD_SIZE {
            let mut down_index = index;
            while down_index > 7 {
                down_index -= 8;

                self.down_attack_masks[index] ^= self.index_masks[down_index];
            }
        }
    }

    fn init_left_masks(&mut self) {
        for index in 0..def::BOARD_SIZE {
            let mut left_index = index;
            while left_index % 8 > 0 {
                left_index -= 1;

                self.left_attack_masks[index] ^= self.index_masks[left_index];
            }
        }
    }

    fn init_right_masks(&mut self) {
        for index in 0..def::BOARD_SIZE {
            let mut right_index = index;
            while right_index % 8 < 7 {
                right_index += 1;

                self.right_attack_masks[index] ^= self.index_masks[right_index];
            }
        }
    }

    fn init_up_left_masks(&mut self) {
        for index in 0..def::BOARD_SIZE {
            let mut up_left_index = index;
            while up_left_index < 56 && up_left_index % 8 > 0 {
                up_left_index += 7;

                self.up_left_attack_masks[index] ^= self.index_masks[up_left_index];
            }
        }
    }

    fn init_up_right_masks(&mut self) {
        for index in 0..def::BOARD_SIZE {
            let mut up_right_index = index;
            while up_right_index < 56 && up_right_index % 8 < 7 {
                up_right_index += 9;

                self.up_right_attack_masks[index] ^= self.index_masks[up_right_index];
            }
        }
    }

    fn init_down_left_masks(&mut self) {
        for index in 0..def::BOARD_SIZE {
            let mut down_left_index = index;
            while down_left_index > 7 && down_left_index % 8 > 0 {
                down_left_index -= 9;

                self.down_left_attack_masks[index] ^= self.index_masks[down_left_index];
            }
        }
    }

    fn init_down_right_masks(&mut self) {
        for index in 0..def::BOARD_SIZE {
            let mut down_right_index = index;
            while down_right_index > 7 && down_right_index % 8 < 7 {
                down_right_index -= 7;

                self.down_right_attack_masks[index] ^= self.index_masks[down_right_index];
            }
        }
    }

    fn init_rb_attack_masks(&mut self) {
        for index in 0..def::BOARD_SIZE {
            self.b_attack_masks[index] = self.up_left_attack_masks[index] ^ self.up_right_attack_masks[index] ^ self.down_left_attack_masks[index] ^ self.down_right_attack_masks[index];
            self.r_attack_masks[index] = self.up_attack_masks[index] ^ self.down_attack_masks[index] ^ self.left_attack_masks[index] ^ self.right_attack_masks[index];
        }
    }

    fn init_p_attack_masks(&mut self) {
        for index in 0..def::BOARD_SIZE {
            if self.index_masks[index] & self.file_masks[0] == 0 {
                if index < 56 {
                    self.wp_attack_masks[index] = self.wp_attack_masks[index] | self.index_masks[index + 7];
                }

                if index > 7 {
                    self.bp_attack_masks[index] = self.bp_attack_masks[index] | self.index_masks[index - 9];
                }
            }

            if self.index_masks[index] & self.file_masks[7] == 0 {
                if index < 56 {
                    self.wp_attack_masks[index] = self.wp_attack_masks[index] | self.index_masks[index + 9];
                }

                if index > 7 {
                    self.bp_attack_masks[index] = self.bp_attack_masks[index] | self.index_masks[index - 7];
                }
            }
        }
    }

    fn init_p_mov_masks(&mut self) {
        for index in 8..def::BOARD_SIZE - 8 {
            self.wp_mov_masks[index] = self.index_masks[index + 8];
            self.bp_mov_masks[index] = self.index_masks[index - 8];

            if index < 16 {
                self.wp_init_mov_masks[index] = self.index_masks[index + 16];
            }

            if index > 47 {
                self.bp_init_mov_masks[index] = self.index_masks[index - 16];
            }
        }
    }

    fn init_k_protect_masks(&mut self) {
        for index in 0..16 {
            self.wk_protect_masks[index] ^= self.index_masks[index + 8] ^ self.index_masks[index + 16];

            if index % 8 < 7 {
                self.wk_protect_masks[index] ^= self.index_masks[index + 9] ^ self.index_masks[index + 17];
            }

            if index % 8 > 0 {
                self.wk_protect_masks[index] ^= self.index_masks[index + 7] ^ self.index_masks[index + 15];
            }
        }

        for index in 48..def::BOARD_SIZE {
            self.bk_protect_masks[index] ^= self.index_masks[index - 8] ^ self.index_masks[index - 16];

            if index % 8 < 7 {
                self.bk_protect_masks[index] ^= self.index_masks[index - 7] ^ self.index_masks[index - 15];
            }

            if index % 8 > 0 {
                self.bk_protect_masks[index] ^= self.index_masks[index - 9] ^ self.index_masks[index - 17];
            }
        }
    }

    fn init_p_misc_masks(&mut self) {
        for index in 0..def::BOARD_SIZE {
            self.wp_forward_masks[index] = self.up_attack_masks[index];
            self.bp_forward_masks[index] = self.down_attack_masks[index];

            if index % 8 > 0 {
                self.wp_forward_masks[index] ^= self.up_attack_masks[index - 1];
                self.wp_behind_masks[index] ^= self.down_attack_masks[index - 1] ^ self.index_masks[index - 1];

                self.bp_forward_masks[index] ^= self.down_attack_masks[index - 1];
                self.bp_behind_masks[index] ^= self.up_attack_masks[index - 1] ^ self.index_masks[index - 1];
            }

            if index % 8 < 7 {
                self.wp_forward_masks[index] ^= self.up_attack_masks[index + 1];
                self.wp_behind_masks[index] ^= self.down_attack_masks[index + 1] ^ self.index_masks[index + 1];

                self.bp_forward_masks[index] ^= self.down_attack_masks[index + 1];
                self.bp_behind_masks[index] ^= self.up_attack_masks[index + 1] ^ self.index_masks[index + 1];
            }
        }
    }
}
