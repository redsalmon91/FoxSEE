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
    pub w_rook: u64,
    pub b_rook: u64,
    pub w_pawn: u64,
    pub b_pawn: u64,
}

pub struct BitMask {
    pub index_masks: [u64; def::BOARD_SIZE],
    pub file_masks: [u64; def::BOARD_SIZE],
    pub rank_masks: [u64; def::BOARD_SIZE],
    pub wk_protect_masks: [u64; def::BOARD_SIZE],
    pub bk_protect_masks: [u64; def::BOARD_SIZE],
    pub wp_forward_masks: [u64; def::BOARD_SIZE],
    pub bp_forward_masks: [u64; def::BOARD_SIZE],
    pub wp_nearby_masks: [u64; def::BOARD_SIZE],
    pub bp_nearby_masks: [u64; def::BOARD_SIZE],
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
            wp_nearby_masks,
            bp_nearby_masks,
        ) = gen_masks();

        BitMask {
            index_masks,
            file_masks,
            rank_masks,
            wk_protect_masks,
            bk_protect_masks,
            wp_forward_masks,
            bp_forward_masks,
            wp_nearby_masks,
            bp_nearby_masks,
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
    [u64; def::BOARD_SIZE]) {

    let mut index_masks = [0; def::BOARD_SIZE];
    let mut file_masks = [0; def::BOARD_SIZE];
    let mut rank_masks = [0; def::BOARD_SIZE];
    let mut wk_protect_masks = [0; def::BOARD_SIZE];
    let mut bk_protect_masks = [0; def::BOARD_SIZE];
    let mut wp_forward_masks = [0; def::BOARD_SIZE];
    let mut bp_forward_masks = [0; def::BOARD_SIZE];
    let mut wp_nearby_masks = [0; def::BOARD_SIZE];
    let mut bp_nearby_masks = [0; def::BOARD_SIZE];

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

        let mut wp_forward_mask = file_masks[index];
        let mut bp_forward_mask = file_masks[index];
        let mut nearby_mask = 0;

        if index as isize - 1 >= 16 && def::is_index_valid(index - 1) {
            wp_forward_mask |= file_masks[index - 1];
            bp_forward_mask |= file_masks[index - 1];
            nearby_mask |= file_masks[index - 1];
        }

        if index + 1 <= 103 && def::is_index_valid(index + 1) {
            wp_forward_mask |= file_masks[index + 1];
            bp_forward_mask |= file_masks[index + 1];
            nearby_mask |= file_masks[index + 1];
        }

        wp_forward_masks[index] = wp_forward_mask;
        bp_forward_masks[index] = bp_forward_mask;
        wp_nearby_masks[index] = nearby_mask;
        bp_nearby_masks[index] = nearby_mask;

        index += 1;
    }

    index = 0;
    while index < def::BOARD_SIZE {
        if !def::is_index_valid(index) {
            index += 8;
        }

        let mut wp_forward_mask = wp_forward_masks[index];
        let mut bp_forward_mask = bp_forward_masks[index];
        let mut wp_nearby_mask = wp_nearby_masks[index];
        let mut bp_nearby_mask = bp_nearby_masks[index];

        let mut mask_index = 0;
        while mask_index < def::BOARD_SIZE {
            if !def::is_index_valid(mask_index) {
                mask_index += 8;
            }

            if mask_index > index + 1 {
                wp_nearby_mask &= !index_masks[mask_index];
            }

            if index > 1 && mask_index < index - 1 {
                bp_nearby_mask &= !index_masks[mask_index];
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
        wp_nearby_masks[index] = wp_nearby_mask;
        bp_nearby_masks[index] = bp_nearby_mask;

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
        wp_nearby_masks, 
        bp_nearby_masks,
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
            wp_nearby_masks,
            bp_nearby_masks,
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

        assert_eq!(0b00000000_00000000_00000000_00000000_00101000_00101000_00101000_00101000, wp_nearby_masks[52]);
        assert_eq!(0b00101000_00101000_00101000_00101000_00000000_00000000_00000000_00000000, bp_nearby_masks[68]);
    }
}
