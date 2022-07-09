/*
 * Copyright (C) 2020-2022 Zixiao Han
 */

use crate::{
    def,
    util::{self, get_lowest_index, get_highest_index},
};

const SLIDE_ATTACK_PERM_COUNT: usize = 256;

pub struct BitMask {
    pub index_masks: [u64; def::BOARD_SIZE],
    pub rank_masks: [u64; def::BOARD_SIZE],
    pub file_masks: [u64; def::BOARD_SIZE],
    pub diag_up_masks: [u64; def::BOARD_SIZE],
    pub diag_down_masks: [u64; def::BOARD_SIZE],

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

    pub wp_front_control_sqr_masks: [u64; def::BOARD_SIZE],
    pub bp_front_control_sqr_masks: [u64; def::BOARD_SIZE],
    pub wp_connected_sqr_masks: [u64; def::BOARD_SIZE],
    pub bp_connected_sqr_masks: [u64; def::BOARD_SIZE],

    pub n_attack_masks: [u64; def::BOARD_SIZE],
    pub k_attack_masks: [u64; def::BOARD_SIZE],

    up_attack_masks: [u64; def::BOARD_SIZE],
    down_attack_masks: [u64; def::BOARD_SIZE],
    left_attack_masks: [u64; def::BOARD_SIZE],
    right_attack_masks: [u64; def::BOARD_SIZE],

    up_left_attack_masks: [u64; def::BOARD_SIZE],
    up_right_attack_masks: [u64; def::BOARD_SIZE],
    down_left_attack_masks: [u64; def::BOARD_SIZE],
    down_right_attack_masks: [u64; def::BOARD_SIZE],

    pub horizontal_attack_masks: [[u64; SLIDE_ATTACK_PERM_COUNT]; def::BOARD_SIZE],
    pub vertical_attack_masks: [[u64; SLIDE_ATTACK_PERM_COUNT]; def::BOARD_SIZE],
    pub diag_up_attack_masks: [[u64; SLIDE_ATTACK_PERM_COUNT]; def::BOARD_SIZE],
    pub diag_down_attack_masks: [[u64; SLIDE_ATTACK_PERM_COUNT]; def::BOARD_SIZE],

    pub wk_attack_zone_masks: [u64; def::BOARD_SIZE],
    pub bk_attack_zone_masks: [u64; def::BOARD_SIZE],

    pub b_cover_masks: [u64; def::BOARD_SIZE],
}

impl BitMask {
    fn new() -> Self {
        let mut bitmask = BitMask {
            index_masks: [0; def::BOARD_SIZE],
            rank_masks: [0; def::BOARD_SIZE],
            file_masks: [0; def::BOARD_SIZE],
            diag_up_masks: [0; def::BOARD_SIZE],
            diag_down_masks: [0; def::BOARD_SIZE],

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

            wp_front_control_sqr_masks: [0; def::BOARD_SIZE],
            bp_front_control_sqr_masks: [0; def::BOARD_SIZE],
            wp_connected_sqr_masks: [0; def::BOARD_SIZE],
            bp_connected_sqr_masks: [0; def::BOARD_SIZE],

            n_attack_masks: [0; def::BOARD_SIZE],
            k_attack_masks: [0; def::BOARD_SIZE],

            left_attack_masks: [0; def::BOARD_SIZE],
            right_attack_masks: [0; def::BOARD_SIZE],
            up_attack_masks: [0; def::BOARD_SIZE],
            down_attack_masks: [0; def::BOARD_SIZE],

            up_left_attack_masks: [0; def::BOARD_SIZE],
            up_right_attack_masks: [0; def::BOARD_SIZE],
            down_left_attack_masks: [0; def::BOARD_SIZE],
            down_right_attack_masks: [0; def::BOARD_SIZE],

            horizontal_attack_masks: [[0; SLIDE_ATTACK_PERM_COUNT]; def::BOARD_SIZE],
            vertical_attack_masks: [[0; SLIDE_ATTACK_PERM_COUNT]; def::BOARD_SIZE],
            diag_up_attack_masks: [[0; SLIDE_ATTACK_PERM_COUNT]; def::BOARD_SIZE],
            diag_down_attack_masks: [[0; SLIDE_ATTACK_PERM_COUNT]; def::BOARD_SIZE],

            wk_attack_zone_masks: [0; def::BOARD_SIZE],
            bk_attack_zone_masks: [0; def::BOARD_SIZE],

            b_cover_masks: [0; def::BOARD_SIZE],
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

        bitmask.init_horizontal_attack_masks();
        bitmask.init_vertical_attack_masks();
        bitmask.init_diag_up_attack_masks();
        bitmask.init_diag_down_attack_masks();

        bitmask.init_p_attack_masks();
        bitmask.init_p_mov_masks();
        bitmask.init_p_misc_masks();
        bitmask.init_p_endgame_masks();

        bitmask.init_k_safety_masks();

        bitmask.init_b_cover_masks();

        bitmask
    }

    fn init_base(&mut self) {
        let mut file_masks = [0; def::DIM_SIZE];
        let mut rank_masks = [0; def::DIM_SIZE];
        let mut diag_up_masks = [0; def::DIM_SIZE << 1];
        let mut diag_down_masks = [0; def::DIM_SIZE << 1];

        for index in 0..def::BOARD_SIZE {
            let index_mask = 1u64 << index;
            self.index_masks[index] = index_mask;

            let file = index % def::DIM_SIZE;
            file_masks[file] = file_masks[file] | index_mask;

            let rank = index / def::DIM_SIZE;
            rank_masks[rank] = rank_masks[rank] | index_mask;

            let diag_up = (rank - file) & 15;
            diag_up_masks[diag_up] = diag_up_masks[diag_up] | index_mask;

            let diag_down = (rank + file) ^ 7;
            diag_down_masks[diag_down] = diag_down_masks[diag_down] | index_mask;
        }

        for index in 0..def::BOARD_SIZE {
            let file = index % def::DIM_SIZE;
            self.file_masks[index] = file_masks[file];

            let rank = index / def::DIM_SIZE;
            self.rank_masks[index] = rank_masks[rank];

            let diag_up = (rank - file) & 15;
            self.diag_up_masks[index] = diag_up_masks[diag_up];

            let diag_down = (rank + file) ^ 7;
            self.diag_down_masks[index] = diag_down_masks[diag_down];
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

    fn init_horizontal_attack_masks(&mut self) {
        let horizontal_masks = util::gen_all_perms_1st_rank();

        for index in 0..def::DIM_SIZE {
            let right_attack_mask = self.right_attack_masks[index];
            let left_attack_mask = self.left_attack_masks[index];

            for occupy_mask in &horizontal_masks {
                let mut left_mov_mask = 0;
                let mut right_mov_mask = 0;

                right_mov_mask ^= right_attack_mask;
                if right_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(right_attack_mask & occupy_mask);
                    right_mov_mask &= !self.right_attack_masks[lowest_blocker_index];
                }

                left_mov_mask ^= left_attack_mask;
                if left_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(left_attack_mask & occupy_mask);
                    left_mov_mask &= !self.left_attack_masks[highest_blocker_index];
                }

                let mov_mask = left_mov_mask | right_mov_mask;
                self.horizontal_attack_masks[index][*occupy_mask as usize] = mov_mask;

                for rank in 1..def::DIM_SIZE {
                    self.horizontal_attack_masks[rank * def::DIM_SIZE + index][*occupy_mask as usize] = mov_mask << rank * def::DIM_SIZE;
                }
            }
        }
    }

    fn init_vertical_attack_masks(&mut self) {
        let vertical_masks = util::gen_all_perms_1st_file();

        for rank in 0..def::DIM_SIZE {
            let index = rank * def::DIM_SIZE;
            let up_attack_mask = self.up_attack_masks[index];
            let down_attack_mask = self.down_attack_masks[index];

            for occupy_mask in &vertical_masks {
                let mut up_mov_mask = 0;
                let mut down_mov_mask = 0;

                up_mov_mask ^= up_attack_mask;
                if up_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_attack_mask & occupy_mask);
                    up_mov_mask &= !self.up_attack_masks[lowest_blocker_index];
                }

                down_mov_mask ^= down_attack_mask;
                if down_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_attack_mask & occupy_mask);
                    down_mov_mask &= !self.down_attack_masks[highest_blocker_index];
                }

                let mov_mask = up_mov_mask | down_mov_mask;
                let mapped_mask = util::kindergarten_transform_file(*occupy_mask, index);
                self.vertical_attack_masks[index][mapped_mask as usize] = mov_mask;

                for file in 1..def::DIM_SIZE {
                    self.vertical_attack_masks[index + file][mapped_mask as usize] = mov_mask << file;
                }
            }
        }
    }

    fn init_diag_up_attack_masks(&mut self) {
        let diag_up_masks = util::gen_all_perms_diag_up();

        let mut index = 0;
        loop {
            if index > def:: BOARD_SIZE - 1 {
                break;
            }

            let up_right_attack_mask = self.up_right_attack_masks[index];
            let down_left_attack_mask = self.down_left_attack_masks[index];

            for occupy_mask in &diag_up_masks {
                let mut down_left_mov_mask = 0;
                let mut up_right_mov_mask = 0;

                up_right_mov_mask ^= up_right_attack_mask;
                if up_right_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_right_attack_mask & occupy_mask);
                    up_right_mov_mask &= !self.up_right_attack_masks[lowest_blocker_index];
                }

                down_left_mov_mask ^= down_left_attack_mask;
                if down_left_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_left_attack_mask & occupy_mask);
                    down_left_mov_mask &= !self.down_left_attack_masks[highest_blocker_index];
                }

                let mov_mask = down_left_mov_mask | up_right_mov_mask;
                let mapped_mask = util::kindergarten_transform_rank_diag(*occupy_mask);
                self.diag_up_attack_masks[index][mapped_mask as usize] = mov_mask;

                for up in 1..def::DIM_SIZE {
                    if index + up * def::DIM_SIZE >= def::BOARD_SIZE - 1 {
                        continue;
                    }

                    let mapped_mask = util::kindergarten_transform_rank_diag(*occupy_mask << up * def::DIM_SIZE);
                    self.diag_up_attack_masks[index + up * def::DIM_SIZE][mapped_mask as usize] = mov_mask << up * def::DIM_SIZE;
                }

                for down in 1..def::DIM_SIZE {
                    if index <= down * def::DIM_SIZE {
                        continue;
                    }

                    let mapped_mask = util::kindergarten_transform_rank_diag(*occupy_mask >> down * def::DIM_SIZE);
                    self.diag_up_attack_masks[index - down * def::DIM_SIZE][mapped_mask as usize] = mov_mask >> down * def::DIM_SIZE;
                }
            }

            index = index + 9;
        }
    }

    fn init_diag_down_attack_masks(&mut self) {
        let diag_down_masks = util::gen_all_perms_diag_down();

        let mut index = 7;
        loop {
            if index > def:: BOARD_SIZE - def:: DIM_SIZE {
                break;
            }

            let up_left_attack_mask = self.up_left_attack_masks[index];
            let down_right_attack_mask = self.down_right_attack_masks[index];

            for occupy_mask in &diag_down_masks {
                let mut up_left_mov_mask = 0;
                let mut down_right_mov_mask = 0;

                up_left_mov_mask ^= up_left_attack_mask;
                if up_left_attack_mask & occupy_mask != 0 {
                    let lowest_blocker_index = get_lowest_index(up_left_attack_mask & occupy_mask);
                    up_left_mov_mask &= !self.up_left_attack_masks[lowest_blocker_index];
                }

                down_right_mov_mask ^= down_right_attack_mask;
                if down_right_attack_mask & occupy_mask != 0 {
                    let highest_blocker_index = get_highest_index(down_right_attack_mask & occupy_mask);
                    down_right_mov_mask &= !self.down_right_attack_masks[highest_blocker_index];
                }

                let mov_mask = up_left_mov_mask | down_right_mov_mask;
                let mapped_mask = util::kindergarten_transform_rank_diag(*occupy_mask);
                self.diag_down_attack_masks[index][mapped_mask as usize] = mov_mask;

                for up in 1..def::DIM_SIZE {
                    if index + up * def::DIM_SIZE >= def::BOARD_SIZE - 1 {
                        continue;
                    }

                    let mapped_mask = util::kindergarten_transform_rank_diag(*occupy_mask << up * def::DIM_SIZE);
                    self.diag_down_attack_masks[index + up * def::DIM_SIZE][mapped_mask as usize] = mov_mask << up * def::DIM_SIZE;
                }

                for down in 1..def::DIM_SIZE {
                    if index <= down * def::DIM_SIZE {
                        continue;
                    }

                    let mapped_mask = util::kindergarten_transform_rank_diag(*occupy_mask >> down * def::DIM_SIZE);
                    self.diag_down_attack_masks[index - down * def::DIM_SIZE][mapped_mask as usize] = mov_mask >> down * def::DIM_SIZE;
                }
            }

            index = index + 7;
        }
    }

    fn init_b_cover_masks(&mut self) {
        for index in 0..def::BOARD_SIZE {
            self.b_cover_masks[index] = self.up_left_attack_masks[index] ^ self.up_right_attack_masks[index] ^ self.down_left_attack_masks[index] ^ self.down_right_attack_masks[index];
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

        for index in 8..def::BOARD_SIZE - 8 {
            let mut connected_mask = 0;

            if index % 8 != 0 {
                connected_mask |= self.index_masks[index - 1];
                connected_mask |= self.index_masks[index - 9];
            }

            if index % 8 != 7 {
                connected_mask |= self.index_masks[index + 1];
                connected_mask |= self.index_masks[index - 7];
            }

            self.wp_connected_sqr_masks[index] = connected_mask;
        }

        for index in 8..def::BOARD_SIZE - 8 {
            let mut connected_mask = 0;

            if index % 8 != 0 {
                connected_mask |= self.index_masks[index - 1];
                connected_mask |= self.index_masks[index + 7];
            }

            if index % 8 != 7 {
                connected_mask |= self.index_masks[index + 1];
                connected_mask |= self.index_masks[index + 9];
            }

            self.bp_connected_sqr_masks[index] = connected_mask;
        }
    }

    fn init_p_endgame_masks(&mut self) {
        for index in 8..def::BOARD_SIZE - 16 {
            let mut front_control_mask = 0;

            front_control_mask |= self.index_masks[index + 16];

            if index % 8 != 0 {
                front_control_mask |= self.index_masks[index + 15];
            }

            if index % 8 != 7 {
                front_control_mask |= self.index_masks[index + 17];
            }

            self.wp_front_control_sqr_masks[index] = front_control_mask;
        }

        for index in 16..def::BOARD_SIZE - 8 {
            let mut front_control_mask = 0;

            front_control_mask |= self.index_masks[index - 16];

            if index % 8 != 0 {
                front_control_mask |= self.index_masks[index - 17];
            }

            if index % 8 != 7 {
                front_control_mask |= self.index_masks[index - 15];
            }

            self.bp_front_control_sqr_masks[index] = front_control_mask;
        }
    }

    fn init_k_safety_masks(&mut self) {
        for index in 0..def::BOARD_SIZE - 16 {
            let mut attack_zone_mask = self.k_attack_masks[index];

            attack_zone_mask |= self.index_masks[index + 8];
            attack_zone_mask |= self.index_masks[index + 16];

            if index % 8 != 0 {
                attack_zone_mask |= self.index_masks[index + 7];
                attack_zone_mask |= self.index_masks[index + 15];
            }

            if index % 8 != 7 {
                attack_zone_mask |= self.index_masks[index + 9];
                attack_zone_mask |= self.index_masks[index + 17];
            }

            self.wk_attack_zone_masks[index] = attack_zone_mask;
        }

        for index in 16..def::BOARD_SIZE {
            let mut attack_zone_mask = self.k_attack_masks[index];

            attack_zone_mask |= self.index_masks[index - 8];
            attack_zone_mask |= self.index_masks[index - 16];

            if index % 8 != 0 {
                attack_zone_mask |= self.index_masks[index - 9];
                attack_zone_mask |= self.index_masks[index - 17];
            }

            if index % 8 != 7 {
                attack_zone_mask |= self.index_masks[index - 7];
                attack_zone_mask |= self.index_masks[index - 15];
            }

            self.bk_attack_zone_masks[index] = attack_zone_mask;
        }
    }
}

static mut BITMASK: BitMask = BitMask {
    index_masks: [0; def::BOARD_SIZE],
    rank_masks: [0; def::BOARD_SIZE],
    file_masks: [0; def::BOARD_SIZE],
    diag_up_masks: [0; def::BOARD_SIZE],
    diag_down_masks: [0; def::BOARD_SIZE],

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

    wp_front_control_sqr_masks: [0; def::BOARD_SIZE],
    bp_front_control_sqr_masks: [0; def::BOARD_SIZE],
    wp_connected_sqr_masks: [0; def::BOARD_SIZE],
    bp_connected_sqr_masks: [0; def::BOARD_SIZE],

    n_attack_masks: [0; def::BOARD_SIZE],
    k_attack_masks: [0; def::BOARD_SIZE],

    left_attack_masks: [0; def::BOARD_SIZE],
    right_attack_masks: [0; def::BOARD_SIZE],
    up_attack_masks: [0; def::BOARD_SIZE],
    down_attack_masks: [0; def::BOARD_SIZE],

    up_left_attack_masks: [0; def::BOARD_SIZE],
    up_right_attack_masks: [0; def::BOARD_SIZE],
    down_left_attack_masks: [0; def::BOARD_SIZE],
    down_right_attack_masks: [0; def::BOARD_SIZE],

    horizontal_attack_masks: [[0; SLIDE_ATTACK_PERM_COUNT]; def::BOARD_SIZE],
    vertical_attack_masks: [[0; SLIDE_ATTACK_PERM_COUNT]; def::BOARD_SIZE],
    diag_up_attack_masks: [[0; SLIDE_ATTACK_PERM_COUNT]; def::BOARD_SIZE],
    diag_down_attack_masks: [[0; SLIDE_ATTACK_PERM_COUNT]; def::BOARD_SIZE],

    wk_attack_zone_masks: [0; def::BOARD_SIZE],
    bk_attack_zone_masks: [0; def::BOARD_SIZE],

    b_cover_masks: [0; def::BOARD_SIZE],
};

pub fn init() {
    unsafe {
        BITMASK = BitMask::new();
    }
}

#[inline]
pub fn get_bitmask() -> &'static BitMask {
    unsafe {
        &BITMASK
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        util,
    };

    #[test]
    fn test_horizontal_masks() {
        init();
        let bitmask = get_bitmask();
        let index = util::map_sqr_notation_to_index("e4");
        let rank_mask = bitmask.rank_masks[index];
        let occupy_mask = 0b00000010_00000001_00000000_00000000_01001100_00000000_00000000_00000001 & rank_mask;
        
        assert_eq!(0b00000000_00000000_00000000_00000000_01101000_00000000_00000000_00000000,
            bitmask.horizontal_attack_masks[index][util::kindergarten_transform_rank_diag(occupy_mask) as usize]);
    }

    #[test]
    fn test_diagup_masks() {
        init();
        let bitmask = get_bitmask();
        let index = util::map_sqr_notation_to_index("d4");
        let diag_up_mask = bitmask.diag_up_masks[index];
        let occupy_mask = 0b00000010_00000001_00000000_00010000_00001000_00000000_00000010_00000010 & diag_up_mask;

        assert_eq!(0b00000000_00000000_00000000_00010000_00000000_00000100_00000010_00000000,
            bitmask.diag_up_attack_masks[index][util::kindergarten_transform_rank_diag(occupy_mask) as usize]);
    }

    #[test]
    fn test_diagup_masks1() {
        init();
        let bitmask = get_bitmask();
        let index = util::map_sqr_notation_to_index("f4");
        let diag_up_mask = bitmask.diag_up_masks[index];
        let occupy_mask = 0b00000000_00000000_10000000_00000000_00100000_00010000_00000000_00000000 & diag_up_mask;

        assert_eq!(0b00000000_00000000_10000000_01000000_00000000_00010000_00000000_00000000,
            bitmask.diag_up_attack_masks[index][util::kindergarten_transform_rank_diag(occupy_mask) as usize]);
    }

    #[test]
    fn test_diagup_masks2() {
        init();
        let bitmask = get_bitmask();
        let index = util::map_sqr_notation_to_index("a5");
        let diag_up_mask = bitmask.diag_up_masks[index];
        let occupy_mask = 0b00000000_00000100_00000000_00000001_00000000_00000000_00000000_00000000 & diag_up_mask;

        assert_eq!(0b00000000_00000100_00000010_00000000_00000000_00000000_00000000_00000000,
            bitmask.diag_up_attack_masks[index][util::kindergarten_transform_rank_diag(occupy_mask) as usize]);
    }

    #[test]
    fn test_diagdown_masks() {
        init();
        let bitmask = get_bitmask();
        let index = util::map_sqr_notation_to_index("g2");
        let diag_down_mask = bitmask.diag_down_masks[index];
        let occupy_mask = 0b00000000_00000000_00000000_00000000_00010000_00000000_01000000_00000000 & diag_down_mask;

        assert_eq!(0b00000000_00000000_00000000_00000000_00010000_00100000_00000000_10000000,
            bitmask.diag_down_attack_masks[index][util::kindergarten_transform_rank_diag(occupy_mask) as usize]);
    }

    #[test]
    fn test_vertical_masks() {
        init();
        let bitmask = get_bitmask();
        let index = util::map_sqr_notation_to_index("e4");
        let occupy_mask = 0b00000000_00000000_00010000_00000000_00010000_00000000_00000000_00010000;

        assert_eq!(0b00000000_00000000_00010000_00010000_00000000_00010000_00010000_00010000,
            bitmask.vertical_attack_masks[index][util::kindergarten_transform_file(occupy_mask, index) as usize]);
    }

    #[test]
    fn test_vertical_masks1() {
        init();
        let bitmask = get_bitmask();
        let index = util::map_sqr_notation_to_index("h2");
        let occupy_mask = 0b00000000_00000000_00000000_00000000_10000000_00000000_10000000_00000000;

        assert_eq!(0b00000000_00000000_00000000_00000000_10000000_10000000_00000000_10000000,
            bitmask.vertical_attack_masks[index][util::kindergarten_transform_file(occupy_mask, index) as usize]);
    }
}
