use crate::def;

pub struct BitMask {
    pub index_masks: [u64; def::BOARD_SIZE],
    pub file_masks: [u64; def::BOARD_SIZE],

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

    pub wk_attack_zone_masks: [u64; def::BOARD_SIZE],
    pub bk_attack_zone_masks: [u64; def::BOARD_SIZE],
}

impl BitMask {
    fn new() -> Self {
        let mut bitmask = BitMask {
            index_masks: [0; def::BOARD_SIZE],
            file_masks: [0; def::BOARD_SIZE],

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

            wk_attack_zone_masks: [0; def::BOARD_SIZE],
            bk_attack_zone_masks: [0; def::BOARD_SIZE],
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
        bitmask.init_p_misc_masks();
        bitmask.init_p_endgame_masks();

        bitmask.init_k_safety_masks();

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

        for index in 8..def::BOARD_SIZE - 16 {
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

        for index in 16..def::BOARD_SIZE - 8 {
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
    file_masks: [0; def::BOARD_SIZE],

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

    wk_attack_zone_masks: [0; def::BOARD_SIZE],
    bk_attack_zone_masks: [0; def::BOARD_SIZE],
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
