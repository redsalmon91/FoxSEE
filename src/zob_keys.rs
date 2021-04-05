use crate::{
    def,
    XorshiftPrng
};

pub static mut BOARD_ZOB_KEYS: Vec<Vec<u64>> = Vec::new();

pub fn init() {
    unsafe {
        BOARD_ZOB_KEYS = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE)
    }
}

pub fn get_board_zob_key(index: usize, moving_piece: u8) -> u64 {
    unsafe {
        return BOARD_ZOB_KEYS[index][moving_piece as usize];
    }
}
