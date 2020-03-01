use crate::{
    def,
};

static FILE_MASKS: [u64; def::DIM_SIZE] = [
    0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000,
    0b01000000_01000000_01000000_01000000_01000000_01000000_01000000_01000000,
    0b00100000_00100000_00100000_00100000_00100000_00100000_00100000_00100000,
    0b00010000_00010000_00010000_00010000_00010000_00010000_00010000_00010000,
    0b00001000_00001000_00001000_00001000_00001000_00001000_00001000_00001000,
    0b00000100_00000100_00000100_00000100_00000100_00000100_00000100_00000100,
    0b00000010_00000010_00000010_00000010_00000010_00000010_00000010_00000010,
    0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001,
];

static RANK_MASKS: [u64; def::DIM_SIZE] = [
    0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
    0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
    0b00000000_00000000_11111111_00000000_00000000_00000000_00000000_00000000,
    0b00000000_00000000_00000000_11111111_00000000_00000000_00000000_00000000,
    0b00000000_00000000_00000000_00000000_11111111_00000000_00000000_00000000,
    0b00000000_00000000_00000000_00000000_00000000_11111111_00000000_00000000,
    0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000,
    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111,
];

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
    pub rank_masks: [u64; def::BOARD_SIZE],

    pub wk_protect_masks: [u64; def::BOARD_SIZE],
    pub bk_protect_masks: [u64; def::BOARD_SIZE],

    pub wp_forward_masks: [u64; def::BOARD_SIZE],
    pub bp_forward_masks: [u64; def::BOARD_SIZE],
    pub wp_behind_masks: [u64; def::BOARD_SIZE],
    pub bp_behind_masks: [u64; def::BOARD_SIZE],

    pub wp_attack_masks: [u64; def::BOARD_SIZE],
    pub bp_attack_masks: [u64; def::BOARD_SIZE],
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
        let (
            index_masks,
            file_masks,
            rank_masks,
            wk_protect_masks,
            bk_protect_masks,
            wp_forward_masks,
            bp_forward_masks,
            wp_behind_masks,
            bp_behind_masks,
            wp_attack_masks,
            bp_attack_masks,
            n_attack_masks,
            k_attack_masks,
            b_attack_masks,
            r_attack_masks,
            left_attack_masks,
            right_attack_masks,
            up_attack_masks,
            down_attack_masks,
            up_left_attack_masks,
            up_right_attack_masks,
            down_left_attack_masks,
            down_right_attack_masks,
        ) = gen_masks();

        BitMask {
            index_masks,
            file_masks,
            rank_masks,
            wk_protect_masks,
            bk_protect_masks,
            wp_forward_masks,
            bp_forward_masks,
            wp_behind_masks,
            bp_behind_masks,
            wp_attack_masks,
            bp_attack_masks,
            n_attack_masks,
            k_attack_masks,
            b_attack_masks,
            r_attack_masks,
            left_attack_masks,
            right_attack_masks,
            up_attack_masks,
            down_attack_masks,
            up_left_attack_masks,
            up_right_attack_masks,
            down_left_attack_masks,
            down_right_attack_masks,
        }
    }
}

pub fn gen_masks() -> (
    [u64; def::BOARD_SIZE], 
    [u64; def::BOARD_SIZE], 
    [u64; def::BOARD_SIZE], 
    [u64; def::BOARD_SIZE], 
    [u64; def::BOARD_SIZE], 
    [u64; def::BOARD_SIZE], 
    [u64; def::BOARD_SIZE], 
    [u64; def::BOARD_SIZE],
    [u64; def::BOARD_SIZE],
    [u64; def::BOARD_SIZE],
    [u64; def::BOARD_SIZE],
    [u64; def::BOARD_SIZE],
    [u64; def::BOARD_SIZE],
    [u64; def::BOARD_SIZE],
    [u64; def::BOARD_SIZE],
    [u64; def::BOARD_SIZE],
    [u64; def::BOARD_SIZE],
    [u64; def::BOARD_SIZE],
    [u64; def::BOARD_SIZE],
    [u64; def::BOARD_SIZE],
    [u64; def::BOARD_SIZE],
    [u64; def::BOARD_SIZE],
    [u64; def::BOARD_SIZE]) {

    let mut index_masks = [0; def::BOARD_SIZE];
    let mut file_masks = [0; def::BOARD_SIZE];
    let mut rank_masks = [0; def::BOARD_SIZE];

    let mut wk_protect_masks = [0; def::BOARD_SIZE];
    let mut bk_protect_masks = [0; def::BOARD_SIZE];
    let mut wp_forward_masks = [0; def::BOARD_SIZE];
    let mut bp_forward_masks = [0; def::BOARD_SIZE];
    let mut wp_behind_masks = [0; def::BOARD_SIZE];
    let mut bp_behind_masks = [0; def::BOARD_SIZE];

    let mut wp_attack_masks = [0; def::BOARD_SIZE];
    let mut bp_attack_masks = [0; def::BOARD_SIZE];
    let mut n_attack_masks = [0; def::BOARD_SIZE];
    let mut k_attack_masks = [0; def::BOARD_SIZE];
    let mut b_attack_masks = [0; def::BOARD_SIZE];
    let mut r_attack_masks = [0; def::BOARD_SIZE];

    let mut left_attack_masks = [0; def::BOARD_SIZE];
    let mut right_attack_masks = [0; def::BOARD_SIZE];
    let mut up_attack_masks = [0; def::BOARD_SIZE];
    let mut down_attack_masks = [0; def::BOARD_SIZE];

    let mut up_left_attack_masks = [0; def::BOARD_SIZE];
    let mut up_right_attack_masks = [0; def::BOARD_SIZE];
    let mut down_left_attack_masks = [0; def::BOARD_SIZE];
    let mut down_right_attack_masks = [0; def::BOARD_SIZE];

    let mut index = 0;

    while index < def::BOARD_SIZE {
        if !def::is_index_valid(index) {
            index += 8;
        }

        let mapped_bit_index = (index + (index & 7)) >> 1;
        let index_mask = 1 << mapped_bit_index;

        index_masks[index] = index_mask;

        for dim_index in 0..def::DIM_SIZE {
            let file_mask = FILE_MASKS[dim_index];
            if file_mask & index_mask != 0 {
                file_masks[index] = file_mask;
            }

            let rank_mask = RANK_MASKS[dim_index];
            if rank_mask & index_mask != 0 {
                rank_masks[index] = rank_mask;
            }
        }

        index += 1;
    }

    index = 0;
    while index < def::BOARD_SIZE {
        if !def::is_index_valid(index) {
            index += 8;
        }

        let mut wk_protect_mask = 0;
        for index_change in vec![15, 16, 17, 31, 32, 33] {
            let protect_index = index + index_change;
            if def::is_index_valid(protect_index) {
                wk_protect_mask ^= index_masks[protect_index];
            }
        }

        wk_protect_masks[index] = wk_protect_mask;

        let mut bk_protect_mask = 0;
        for index_change in vec![-15, -16, -17, -31, -32, -33] {
            let protect_index = index as isize + index_change;
            if protect_index >= 0 {
                let protect_index = protect_index as usize;
                if def::is_index_valid(protect_index) {
                    bk_protect_mask ^= index_masks[protect_index];
                }
            }
        }

        bk_protect_masks[index] = bk_protect_mask;

        index += 1;
    }

    index = 0;
    while index < def::BOARD_SIZE {
        if !def::is_index_valid(index) {
            index += 8;
        }

        let mut wp_attack_mask = 0;
        for index_change in vec![15, 17] {
            let attack_index = index + index_change;
            if def::is_index_valid(attack_index) {
                wp_attack_mask ^= index_masks[attack_index];
            }
        }

        wp_attack_masks[index] = wp_attack_mask;

        let mut bp_attack_mask = 0;
        for index_change in vec![-15, -17] {
            let attack_index = index as isize + index_change;
            if attack_index >= 0 {
                let attack_index = attack_index as usize;
                if def::is_index_valid(attack_index) {
                    bp_attack_mask ^= index_masks[attack_index];
                }
            }
        }

        bp_attack_masks[index] = bp_attack_mask;

        let mut n_attack_mask = 0;
        for index_change in vec![14, 18, 31, 33, -14, -18, -31, -33] {
            let attack_index = index as isize + index_change;
            if attack_index >= 0 {
                let attack_index = attack_index as usize;
                if def::is_index_valid(attack_index) {
                    n_attack_mask ^= index_masks[attack_index];
                }
            }
        }

        n_attack_masks[index] = n_attack_mask;

        let mut k_attack_mask = 0;
        for index_change in vec![1, 15, 16, 17, -1, -15, -16, -17] {
            let attack_index = index as isize + index_change;
            if attack_index >= 0 {
                let attack_index = attack_index as usize;
                if def::is_index_valid(attack_index) {
                    k_attack_mask ^= index_masks[attack_index];
                }
            }
        }

        k_attack_masks[index] = k_attack_mask;

        let mut left_attack_mask = 0;
        for index_change in vec![-1, -2, -3, -4, -5, -6, -7] {
            let attack_index = index as isize + index_change;
            if attack_index >= 0 {
                let attack_index = attack_index as usize;
                if def::is_index_valid(attack_index) {
                    left_attack_mask ^= index_masks[attack_index];
                }
            }
        }

        left_attack_masks[index] = left_attack_mask;

        let mut right_attack_mask = 0;
        for index_change in vec![1, 2, 3, 4, 5, 6, 7] {
            let attack_index = index + index_change;
            if def::is_index_valid(attack_index) {
                right_attack_mask ^= index_masks[attack_index];
            }
        }

        right_attack_masks[index] = right_attack_mask;

        let mut up_attack_mask = 0;
        for index_change in vec![16, 32, 48, 64, 80, 96, 112] {
            let attack_index = index + index_change;
            if def::is_index_valid(attack_index) {
                up_attack_mask ^= index_masks[attack_index];
            }
        }

        up_attack_masks[index] = up_attack_mask;

        let mut down_attack_mask = 0;
        for index_change in vec![-16, -32, -48, -64, -80, -96, -112] {
            let attack_index = index as isize + index_change;
            if attack_index >= 0 {
                let attack_index = attack_index as usize;
                if def::is_index_valid(attack_index) {
                    down_attack_mask ^= index_masks[attack_index];
                }
            }
        }

        down_attack_masks[index] = down_attack_mask;

        let mut down_left_attack_mask = 0;
        for index_change in vec![-17, -34, -51, -68, -85, -102, -119] {
            let attack_index = index as isize + index_change;
            if attack_index >= 0 {
                let attack_index = attack_index as usize;
                if def::is_index_valid(attack_index) {
                    down_left_attack_mask ^= index_masks[attack_index];
                }
            }
        }

        down_left_attack_masks[index] = down_left_attack_mask;

        let mut down_right_attack_mask = 0;
        for index_change in vec![-15, -30, -45, -60, -75, -90, -105] {
            let attack_index = index as isize + index_change;
            if attack_index >= 0 {
                let attack_index = attack_index as usize;
                if def::is_index_valid(attack_index) {
                    down_right_attack_mask ^= index_masks[attack_index];
                }
            }
        }

        down_right_attack_masks[index] = down_right_attack_mask;

        let mut up_left_attack_mask = 0;
        for index_change in vec![15, 30, 45, 60, 75, 90, 105] {
            let attack_index = index + index_change;
            if def::is_index_valid(attack_index) {
                up_left_attack_mask ^= index_masks[attack_index];
            }
        }

        up_left_attack_masks[index] = up_left_attack_mask;

        let mut up_right_attack_mask = 0;
        for index_change in vec![17, 34, 51, 68, 85, 102, 119] {
            let attack_index = index + index_change;
            if def::is_index_valid(attack_index) {
                up_right_attack_mask ^= index_masks[attack_index];
            }
        }

        up_right_attack_masks[index] = up_right_attack_mask;

        b_attack_masks[index] = up_left_attack_mask | up_right_attack_mask | down_left_attack_mask | down_right_attack_mask;
        r_attack_masks[index] = left_attack_mask | right_attack_mask | up_attack_mask | down_attack_mask;

        index += 1;
    }

    let mut surround_files_masks = [0; def::BOARD_SIZE];

    index = 0;
    while index < def::BOARD_SIZE {
        if !def::is_index_valid(index) {
            index += 8;
        }

        let mut surround_files_mask = file_masks[index];

        if index as isize - 1 >= 16 && def::is_index_valid(index - 1) {
            surround_files_mask |= file_masks[index - 1];
        }

        if index + 1 <= 103 && def::is_index_valid(index + 1) {
            surround_files_mask |= file_masks[index + 1];
        }

        surround_files_masks[index] = surround_files_mask;

        index += 1;
    }

    index = 0;
    while index < def::BOARD_SIZE {
        if !def::is_index_valid(index) {
            index += 8;
        }

        let surround_files_mask = surround_files_masks[index];
        let mut wp_forward_mask = surround_files_mask;
        let mut bp_forward_mask = surround_files_mask;

        let mut mask_index = 0;
        while mask_index < def::BOARD_SIZE {
            if !def::is_index_valid(mask_index) {
                mask_index += 8;
            }

            if mask_index < index || mask_index - index < 15 {
                wp_forward_mask &= !index_masks[mask_index]; 
            }

            if mask_index > index || index - mask_index < 15 {
                bp_forward_mask &= !index_masks[mask_index]; 
            }

            mask_index += 1;
        }

        wp_forward_masks[index] = wp_forward_mask;
        bp_forward_masks[index] = bp_forward_mask;
        wp_behind_masks[index] = (surround_files_mask ^ wp_forward_mask) & !file_masks[index];
        bp_behind_masks[index] = (surround_files_mask ^ bp_forward_mask) & !file_masks[index];

        index += 1;
    }

    (
        index_masks, 
        file_masks, 
        rank_masks, 
        wk_protect_masks, 
        bk_protect_masks, 
        wp_forward_masks, 
        bp_forward_masks,
        wp_behind_masks,
        bp_behind_masks,
        wp_attack_masks,
        bp_attack_masks,
        n_attack_masks,
        k_attack_masks,
        b_attack_masks,
        r_attack_masks,
        left_attack_masks,
        right_attack_masks,
        up_attack_masks,
        down_attack_masks,
        up_left_attack_masks,
        up_right_attack_masks,
        down_left_attack_masks,
        down_right_attack_masks,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_masks() {
        let (
            index_masks, 
            file_masks, 
            rank_masks, 
            wk_protect_masks, 
            bk_protect_masks, 
            wp_forward_masks, 
            bp_forward_masks,
            wp_behind_masks,
            bp_behind_masks,
            wp_attack_masks,
            bp_attack_masks,
            n_attack_masks,
            k_attack_masks,
            _b_attack_masks,
            _r_attack_masks,
            _left_attack_masks,
            _right_attack_masks,
            _up_attack_masks,
            _down_attack_masks,
            _up_left_attack_masks,
            _up_right_attack_masks,
            _down_left_attack_masks,
            _down_right_attack_masks,
        ) = gen_masks();

        assert_eq!(0b1, index_masks[0]);
        assert_eq!(0b10, index_masks[1]);
        assert_eq!(0b100, index_masks[2]);
        assert_eq!(0b10000000_00000000, index_masks[23]);

        assert_eq!(0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000, file_masks[7]);
        assert_eq!(0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000, file_masks[119]);
        assert_eq!(0b00000010_00000010_00000010_00000010_00000010_00000010_00000010_00000010, file_masks[1]);
        assert_eq!(0b00000010_00000010_00000010_00000010_00000010_00000010_00000010_00000010, file_masks[33]);

        assert_eq!(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111, rank_masks[0]);
        assert_eq!(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111, rank_masks[7]);
        assert_eq!(0b00000000_00000000_00000000_00000000_00000000_11111111_00000000_00000000, rank_masks[33]);

        assert_eq!(0b00000000_00000000_00000000_00000000_00000000_11100000_11100000_00000000, wk_protect_masks[6]);
        assert_eq!(0b00000000_11000000_11000000_00000000_00000000_00000000_00000000_00000000, bk_protect_masks[119]);
        assert_eq!(0b00000000_00011100_00011100_00000000_00000000_00000000_00000000_00000000, bk_protect_masks[115]);

        assert_eq!(0b00000011_00000011_00000011_00000011_00000011_00000000_00000000_00000000, wp_forward_masks[32]);
        assert_eq!(0b00000000_00000000_00000000_00000000_00000000_00000000_00000111_00000111, bp_forward_masks[33]);

        assert_eq!(0b00000000_00000000_00000000_00000000_00101000_00101000_00101000_00101000, wp_behind_masks[52]);
        assert_eq!(0b00000010_00000010_00000010_00000000_00000000_00000000_00000000_00000000, bp_behind_masks[80]);

        assert_eq!(0b00000000_00010100_00000000_00000000_00000000_00000000_00000000_00000000, bp_attack_masks[115]);
        assert_eq!(0b00000000_00000000_00000000_00000000_00000000_00000000_10100000_00000000, wp_attack_masks[6]);

        assert_eq!(0b00000000_00000000_00000000_00000000_00000000_10100000_00010000_00000000, n_attack_masks[6]);
        assert_eq!(0b10100000_11100000_00000000_00000000_00000000_00000000_00000000_00000000, k_attack_masks[118]);
    }
}
