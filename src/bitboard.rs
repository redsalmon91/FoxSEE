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
    pub surround_masks: [u64; def::BOARD_SIZE],
}

impl BitMask {
    pub fn new() -> Self {
        let (index_masks, file_masks, rank_masks, surround_masks) = gen_masks();

        BitMask {
            index_masks,
            file_masks,
            rank_masks,
            surround_masks,
        }
    }
}

pub fn gen_masks() -> ([u64; def::BOARD_SIZE], [u64; def::BOARD_SIZE], [u64; def::BOARD_SIZE], [u64; def::BOARD_SIZE]) {
    let mut index_masks = [0; def::BOARD_SIZE];
    let mut file_masks = [0; def::BOARD_SIZE];
    let mut rank_masks = [0; def::BOARD_SIZE];
    let mut surround_masks = [0; def::BOARD_SIZE];

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

        let mut surround_mask = 0;
        for index_change in vec![1, 15, 16, 17, -1, -15, -16, -17] {
            let surround_index = index as isize + index_change;
            if surround_index >= 0 {
                let surround_index = surround_index as usize;
                if def::is_index_valid(surround_index) {
                    surround_mask ^= index_masks[surround_index];
                }
            }
        }

        surround_masks[index] = surround_mask;

        index += 1;
    }

    (index_masks, file_masks, rank_masks, surround_masks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_masks() {
        let (index_masks, file_masks, rank_masks, surround_masks) = gen_masks();

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

        assert_eq!(0b00000000_00000000_00000000_00000000_00000000_00000000_00000011_00000010, surround_masks[0]);
        assert_eq!(0b01000000_11000000_00000000_00000000_00000000_00000000_00000000_00000000, surround_masks[119]);
        assert_eq!(0b00000000_00000000_00000000_01110000_01010000_01110000_00000000_00000000, surround_masks[53]);
    }
}
