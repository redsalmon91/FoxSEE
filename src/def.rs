/*
 * Copyright (C) 2020 Zixiao Han
 */

pub static ENGINE_NAME: &str = "FoxSEE";
pub static VERSION: &str = "7.8.0";
pub static AUTHOR: &str = "Zixiao Han";

pub const DEFAULT_HASH_SIZE_MB: usize = 128;
pub const DEFAULT_HASH_SIZE_UNIT: usize = 4194304;
pub const MIN_HASH_SIZE_MB: usize = 1;
pub const MIN_HASH_SIZE_UNIT: usize = 32768;
pub const MAX_HASH_SIZE_MB: usize = 512;

pub const BOARD_SIZE: usize = 64;
pub const DIM_SIZE: usize = 8;
pub const PIECE_CODE_RANGE: usize = 131;

pub const MAX_CAP_COUNT: usize = 64;
pub const MAX_MOV_COUNT: usize = 128;

pub const MOV_REG: u8 = 1;
pub const MOV_PROMO: u8 = 2;
pub const MOV_CAS: u8 = 3;
pub const MOV_ENP: u8 = 4;
pub const MOV_CR_ENP: u8 = 5;

pub const CAS_SQUARE_WK: usize = 6;
pub const CAS_SQUARE_WQ: usize = 2;
pub const CAS_SQUARE_BK: usize = 62;
pub const CAS_SQUARE_BQ: usize = 58;

pub const PLAYER_W: u8 = 0b10;
pub const PLAYER_B: u8 = 0b01;
pub const PLAYER_SWITCH: u8 = 0b11;

pub const WP: u8 = 6;
pub const WN: u8 = 10;
pub const WB: u8 = 18;
pub const WR: u8 = 34;
pub const WQ: u8 = 66;
pub const WK: u8 = 130;

pub const BP: u8 = 5;
pub const BN: u8 = 9;
pub const BB: u8 = 17;
pub const BR: u8 = 33;
pub const BQ: u8 = 65;
pub const BK: u8 = 129;

pub const P: u8 = 0b100;
pub const N: u8 = 0b1000;
pub const B: u8 = 0b10000;
pub const R: u8 = 0b100000;
pub const Q: u8 = 0b1000000;
pub const K: u8 = 0b10000000;

#[inline]
pub const fn get_opposite_player(player: u8) -> u8 {
    player ^ PLAYER_SWITCH
}

#[inline]
pub const fn on_same_side(player: u8, piece_code: u8) -> bool {
    player & piece_code == player
}

#[inline]
pub fn get_rank(player: u8, index: usize) -> usize {
    if player == PLAYER_W {
        index / 8
    } else {
        7 - index / 8
    }
}

#[inline]
pub fn get_file_distance(index1: usize, index2: usize) -> i32 {
    ((index1 % DIM_SIZE) as isize - (index2 % DIM_SIZE) as isize).abs() as i32
}

#[inline]
pub const fn is_k(piece_code: u8) -> bool {
    piece_code & K != 0
}

#[inline]
pub const fn is_q(piece_code: u8) -> bool {
    piece_code & Q != 0
}

#[inline]
pub const fn is_r(piece_code: u8) -> bool {
    piece_code & R != 0
}

#[inline]
pub const fn is_b(piece_code: u8) -> bool {
    piece_code & B != 0
}

#[inline]
pub const fn is_n(piece_code: u8) -> bool {
    piece_code & N != 0
}

#[inline]
pub const fn is_p(piece_code: u8) -> bool {
    piece_code & P != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_player() {
        let p = PLAYER_W;
        assert_eq!(PLAYER_B, get_opposite_player(p));

        let p = PLAYER_B;
        assert_eq!(PLAYER_W, get_opposite_player(p));
    }

    #[test]
    fn test_get_rank() {
        assert_eq!(0, get_rank(PLAYER_W, 6));
        assert_eq!(1, get_rank(PLAYER_W, 9));
        assert_eq!(7, get_rank(PLAYER_W, 62));
        assert_eq!(7, get_rank(PLAYER_B, 2));
        assert_eq!(3, get_rank(PLAYER_B, 35));
        assert_eq!(7, get_rank(PLAYER_W, 58));
    }

    #[test]
    fn test_get_file_distance() {
        assert_eq!(3, get_file_distance(0, 27));
        assert_eq!(3, get_file_distance(1, 12));
    }

    #[test]
    fn test_piece_type() {
        assert!(is_k(WK));
        assert!(is_k(BK));

        assert!(is_q(WQ));
        assert!(is_q(BQ));

        assert!(is_r(WR));
        assert!(is_r(BR));

        assert!(is_b(WB));
        assert!(is_b(BB));

        assert!(is_n(WN));
        assert!(is_n(BN));

        assert!(is_p(WP));
        assert!(is_p(BP));

        assert!(!is_q(BK));
        assert!(!is_n(WK));
        assert!(!is_b(BR));
        assert!(!is_k(WP));
        assert!(!is_q(BB));
        assert!(!is_p(WN));

        assert!(!is_k(WQ));
        assert!(!is_k(BQ));
        assert!(!is_k(WR));
        assert!(!is_k(BR));
        assert!(!is_k(WB));
        assert!(!is_k(BB));
        assert!(!is_k(WN));
        assert!(!is_k(BN));
        assert!(!is_k(WP));
        assert!(!is_k(BP));
    }

    #[test]
    fn test_checkside() {
        assert!(!on_same_side(PLAYER_W, 0));
        assert!(!on_same_side(PLAYER_B, 0));

        assert!(on_same_side(PLAYER_W, WK));
        assert!(on_same_side(PLAYER_W, WQ));
        assert!(on_same_side(PLAYER_W, WR));
        assert!(on_same_side(PLAYER_W, WB));
        assert!(on_same_side(PLAYER_W, WN));
        assert!(on_same_side(PLAYER_W, WP));

        assert!(on_same_side(PLAYER_B, BK));
        assert!(on_same_side(PLAYER_B, BQ));
        assert!(on_same_side(PLAYER_B, BR));
        assert!(on_same_side(PLAYER_B, BB));
        assert!(on_same_side(PLAYER_B, BN));
        assert!(on_same_side(PLAYER_B, BP));

        assert!(!on_same_side(PLAYER_W, BK));
        assert!(!on_same_side(PLAYER_W, BQ));
        assert!(!on_same_side(PLAYER_W, BR));
        assert!(!on_same_side(PLAYER_W, BB));
        assert!(!on_same_side(PLAYER_W, BN));
        assert!(!on_same_side(PLAYER_W, BP));

        assert!(!on_same_side(PLAYER_B, WK));
        assert!(!on_same_side(PLAYER_B, WQ));
        assert!(!on_same_side(PLAYER_B, WR));
        assert!(!on_same_side(PLAYER_B, WB));
        assert!(!on_same_side(PLAYER_B, WN));
        assert!(!on_same_side(PLAYER_B, WP));
    }
}
