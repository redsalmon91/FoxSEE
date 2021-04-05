use crate::{
    def,
    XorshiftPrng
};

const CASTLING_RIGHTS_COMB_SIZE: usize = 16;

static mut BOARD_ZOB_KEYS: Vec<Vec<u64>> = Vec::new();
static mut B_PLAYER_ZOB_KEY: u64 = 0;
static mut ENP_SQR_ZOB_KEYS: Vec<u64> = Vec::new();
static mut CASTLING_RIGHTS_ZOB_KEYS: Vec<u64> = Vec::new();

pub fn init() {
    let mut prng = XorshiftPrng::new();

    unsafe {
        BOARD_ZOB_KEYS = prng.create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        B_PLAYER_ZOB_KEY = prng.gen_rand();
        ENP_SQR_ZOB_KEYS = prng.create_prn_list(def::BOARD_SIZE);
        CASTLING_RIGHTS_ZOB_KEYS = prng.create_prn_list(CASTLING_RIGHTS_COMB_SIZE);
    }
}

#[inline]
pub fn get_board_zob_key(index: usize, moving_piece: u8) -> u64 {
    unsafe {
        BOARD_ZOB_KEYS[index][moving_piece as usize]
    }
}

#[inline]
pub fn get_b_player_zob_key() -> u64 {
    unsafe {
        B_PLAYER_ZOB_KEY
    }
}

#[inline]
pub fn get_enp_sqr_zob_key(index: usize) -> u64 {
    unsafe {
        ENP_SQR_ZOB_KEYS[index]
    }
}

#[inline]
pub fn get_cas_rights_zob_key(cas_rights: u8) -> u64 {
    unsafe {
        CASTLING_RIGHTS_ZOB_KEYS[cas_rights as usize]
    }
}
