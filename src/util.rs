/*
 * Copyright (C) 2020-2022 Zixiao Han
 */

use crate::def;

const MOV_ENCODE_BIT_MASK: u32 = 0b11111111;

#[inline]
pub fn map_piece_char_to_code(piece_char: char) -> u8 {
    match piece_char {
        'K' => def::WK,
        'Q' => def::WQ,
        'R' => def::WR,
        'B' => def::WB,
        'N' => def::WN,
        'P' => def::WP,

        'k' => def::BK,
        'q' => def::BQ,
        'r' => def::BR,
        'b' => def::BB,
        'n' => def::BN,
        'p' => def::BP,

        _ => panic!("invalid piece char {}", piece_char),
    }
}

#[inline]
pub fn map_piece_code_to_char(piece_code: u8) -> char {
    match piece_code {
        def::WK => '♔',
        def::WQ => '♕',
        def::WR => '♖',
        def::WB => '♗',
        def::WN => '♘',
        def::WP => '♙',

        def::BK => '♚',
        def::BQ => '♛',
        def::BR => '♜',
        def::BB => '♝',
        def::BN => '♞',
        def::BP => '♟',
        _ => '-',
    }
}

#[inline]
pub fn map_promo_piece_to_char(piece: u8) -> char {
    match piece {
        def::WQ => 'q',
        def::WR => 'r',
        def::WB => 'b',
        def::WN => 'n',

        def::BQ => 'q',
        def::BR => 'r',
        def::BB => 'b',
        def::BN => 'n',

        _ => '\0',
    }
}

#[inline]
pub fn map_index_to_sqr_notation(index: usize) -> String {
    let rank = index / 8 + 1;
    let file = index % 8;

    let file_str = match file {
        0 => "a",
        1 => "b",
        2 => "c",
        3 => "d",
        4 => "e",
        5 => "f",
        6 => "g",
        7 => "h",
        _ => panic!("invalid file {}", file),
    };

    format!("{}{}", file_str, rank)
}

#[inline]
pub fn map_sqr_notation_to_index(sqr_notation: &str) -> usize {
    let sqr_notation_chars: Vec<char> = sqr_notation.chars().collect();
    let file_index = match sqr_notation_chars[0] {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        r => panic!("invalid file {}", r),
    };

    let rank_index = sqr_notation_chars[1].to_digit(10).unwrap() - 1;

    (rank_index * 8 + file_index) as usize
}

#[inline]
pub fn encode_u32_mov(from: usize, to: usize, tp: u8, promo: u8) -> u32 {
    from as u32 | (to as u32) << 8 | (tp as u32) << 16 | (promo as u32) << 24
}

#[inline]
pub fn decode_u32_mov(rep: u32) -> (usize, usize, u8, u8) {
    (
        (rep & MOV_ENCODE_BIT_MASK) as usize,
        ((rep >> 8) & MOV_ENCODE_BIT_MASK) as usize,
        ((rep >> 16) & MOV_ENCODE_BIT_MASK) as u8,
        ((rep >> 24) & MOV_ENCODE_BIT_MASK) as u8,
    )
}

#[inline]
pub fn format_mov(mov: u32) -> String {
    let (from, to, _tp, promo) = decode_u32_mov(mov);

    if promo != 0 {
        format!("{}{}{}", map_index_to_sqr_notation(from), map_index_to_sqr_notation(to), map_promo_piece_to_char(promo))
    } else {
        format!("{}{}", map_index_to_sqr_notation(from), map_index_to_sqr_notation(to))
    }
}

#[inline]
pub fn format_pv(pv_table: &[u32]) -> String {
    let mut pv_line = String::new();

    for mov in pv_table {
        if *mov == 0 {
            break
        }

        pv_line.push_str(&&*format!("{} ", format_mov(*mov)));
    }

    pv_line
}

#[allow(dead_code)]
#[inline]
pub fn print_bitboard(bitboard: u64) {
    let mut index = 56;

    loop {
        print!(" {} ", get_bit(bitboard, index));
        index += 1;

        if index % def::DIM_SIZE == 0 {
            print!("\n");

            if index == def::DIM_SIZE {
                break;
            }

            index -= def::DIM_SIZE * 2;
        }
    }
}

#[allow(dead_code)]
#[inline]
pub fn get_bit(bitboard: u64, index: usize) -> u64 {
    if bitboard & 1u64 << index == 0 {
        0
    } else {
        1
    }
}

#[inline]
pub fn get_lowest_index(mask: u64) -> usize {
    mask.trailing_zeros() as usize
}

#[inline]
pub fn get_highest_index(mask: u64) -> usize {
    63 - mask.leading_zeros() as usize
}

#[inline]
pub fn kindergarten_transform_rank_diag(bitboard: u64) -> u64 {
    (bitboard * 0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001) >> 56
}

pub fn kindergarten_transform_file(bitboard: u64, index: usize) -> u64 {
    ((bitboard >> (index & 7)) * 0b10000000_01000000_00100000_00010000_00001000_00000100_00000010_00000001) >> 56
}

pub fn gen_all_perms_1st_rank() -> Vec<u64> {
    let mut perms = Vec::new();
    gen_rank_perms(0, 0, &mut perms);
    return perms;
}

fn gen_rank_perms(mask: u64, index: usize, perms: &mut Vec<u64>) {
    if index == def::DIM_SIZE - 1 {
        perms.push(mask);
        perms.push(mask | 0b1 << index);
        return;
    }

    gen_rank_perms(mask, index + 1, perms);
    gen_rank_perms(mask | 0b1 << index, index + 1, perms);
}

pub fn gen_all_perms_1st_file() -> Vec<u64> {
    let mut perms = Vec::new();
    gen_file_perms(0, 0, &mut perms);
    return perms;
}

fn gen_file_perms(mask: u64, index: usize, perms: &mut Vec<u64>) {
    if index == def::BOARD_SIZE - def::DIM_SIZE {
        perms.push(mask);
        perms.push(mask | 0b1 << index);
        return;
    }

    gen_file_perms(mask, index + def::DIM_SIZE, perms);
    gen_file_perms(mask | 0b1 << index, index + def::DIM_SIZE, perms);
}

pub fn gen_all_perms_diag_up() -> Vec<u64> {
    let mut perms = Vec::new();
    gen_diag_up_perms(0, 0, &mut perms);
    return perms;
}

fn gen_diag_up_perms(mask: u64, index: usize, perms: &mut Vec<u64>) {
    if index == def::BOARD_SIZE - 1 {
        perms.push(mask);
        perms.push(mask | 0b1 << index);
        return;
    }

    gen_diag_up_perms(mask, index + def::DIM_SIZE + 1, perms);
    gen_diag_up_perms(mask | 0b1 << index, index + def::DIM_SIZE + 1, perms);
}

pub fn gen_all_perms_diag_down() -> Vec<u64> {
    let mut perms = Vec::new();
    gen_diag_down_perms(0, 7, &mut perms);
    return perms;
}

fn gen_diag_down_perms(mask: u64, index: usize, perms: &mut Vec<u64>) {
    if index == def::BOARD_SIZE - def::DIM_SIZE {
        perms.push(mask);
        perms.push(mask | 0b1 << index);
        return;
    }

    gen_diag_down_perms(mask, index + def::DIM_SIZE - 1, perms);
    gen_diag_down_perms(mask | 0b1 << index, index + def::DIM_SIZE - 1, perms);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        def,
        util,
    };

    #[test]
    fn test_rank_shift() {
        let original_board: u64 = 0b00000000_10110111_00000000_00000000_00000000_00000000_00000000_00000000;
        let shifted_board = original_board >> 6 * 8;
        assert_eq!(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10110111, shifted_board);
    }

    #[test]
    fn test_file_shift() {
        let original_board: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000001_00000001_00000001;
        let shifted_board = original_board << 1;
        assert_eq!(0b00000000_00000000_00000000_00000000_00000000_00000010_00000010_00000010, shifted_board);
    }

    #[test]
    #[allow(arithmetic_overflow)]
    fn test_kindergarten_ranks() {
        let original_board: u64 = 0b00000000_00000000_00000000_10000011_00000000_00000000_00000000_00000000;
        let kindergarten_board = kindergarten_transform_rank_diag(original_board);
        assert_eq!(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000011, kindergarten_board);
    }

    #[test]
    #[allow(arithmetic_overflow)]
    fn test_kindergarten_up_diagnol() {
        let original_board: u64 = 0b10000000_01000000_00100000_00010000_00001000_00000100_00000010_00000001;
        let kindergarten_board = kindergarten_transform_rank_diag(original_board);
        assert_eq!(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111, kindergarten_board);
    }

    #[test]
    #[allow(arithmetic_overflow)]
    fn test_kindergarten_down_diagnol() {
        let original_board: u64 = 0b00000001_00000010_00000100_00001000_00010000_00100000_01000000_10000000;
        let kindergarten_board = kindergarten_transform_rank_diag(original_board);
        assert_eq!(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111, kindergarten_board);
    }

    #[test]
    #[allow(arithmetic_overflow)]
    fn test_kindergarten_down_diagnol1() {
        let original_board: u64 = 0b00000010_00000100_00001000_00010000_00100000_01000000_10000000_00000000;
        let kindergarten_board = kindergarten_transform_rank_diag(original_board);
        assert_eq!(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111110, kindergarten_board);
    }

    #[test]
    #[allow(arithmetic_overflow)]
    fn test_kindergarten_file() {
        let original_board: u64 = 0b00000010_00000000_00000000_00000000_00000000_00000000_00000010_00000010;
        let kindergarten_board = kindergarten_transform_file(original_board, util::map_sqr_notation_to_index("b2"));
        assert_eq!(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11000001, kindergarten_board);
    }

    #[test]
    fn test_gen_perms() {
        let rank_perms = gen_all_perms_1st_rank();
        assert_eq!(256, rank_perms.len());

        let file_perms = gen_all_perms_1st_file();
        assert_eq!(256, file_perms.len());

        let diag_up_perms = gen_all_perms_diag_up();
        assert_eq!(256, diag_up_perms.len());

        let diag_down_perms = gen_all_perms_diag_down();
        assert_eq!(256, diag_down_perms.len());
    }

    #[test]
    fn test_map_sqr_notation_to_index() {
        assert_eq!(0, map_sqr_notation_to_index("a1"));
        assert_eq!(8, map_sqr_notation_to_index("a2"));
        assert_eq!(25, map_sqr_notation_to_index("b4"));
        assert_eq!(63, map_sqr_notation_to_index("h8"));
        assert_eq!(56, map_sqr_notation_to_index("a8"));
    }

    #[test]
    fn test_encode_decode_mov_1() {
        let encoded = encode_u32_mov(util::map_sqr_notation_to_index("e6"), util::map_sqr_notation_to_index("a8"), def::MOV_REG, 0);
        let (from, to, tp, promo) = util::decode_u32_mov(encoded);
        assert_eq!(from, util::map_sqr_notation_to_index("e6"));
        assert_eq!(to, util::map_sqr_notation_to_index("a8"));
        assert_eq!(def::MOV_REG, tp);
        assert_eq!(0, promo);
    }

    #[test]
    fn test_encode_decode_mov_2() {
        let encoded = encode_u32_mov(util::map_sqr_notation_to_index("c8"), util::map_sqr_notation_to_index("h3"), def::MOV_ENP, 6);
        let (from, to, tp, promo) = util::decode_u32_mov(encoded);
        assert_eq!(from, util::map_sqr_notation_to_index("c8"));
        assert_eq!(to, util::map_sqr_notation_to_index("h3"));
        assert_eq!(def::MOV_ENP, tp);
        assert_eq!(6, promo);
    }
}
