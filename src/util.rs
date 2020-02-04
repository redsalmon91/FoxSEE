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
    let rank = index / 16 + 1;
    let file = index % 16;

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

    (rank_index * 16 + file_index) as usize
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        def,
        util,
    };

    #[test]
    fn test_map_sqr_notation_to_index() {
        assert_eq!(0, map_sqr_notation_to_index("a1"));
        assert_eq!(16, map_sqr_notation_to_index("a2"));
        assert_eq!(49, map_sqr_notation_to_index("b4"));
        assert_eq!(119, map_sqr_notation_to_index("h8"));
        assert_eq!(112, map_sqr_notation_to_index("a8"));
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
