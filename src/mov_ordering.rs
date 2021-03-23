use crate::{
    def
};

static SQR_TABLE_BN: [i32; def::BOARD_SIZE] = [
  -8, -4, -4, -4, -4, -4, -4, -8,
  -6, -6,  4,  2,  2,  4, -6, -6,
  -4,  0,  3,  4,  4,  3,  0, -4,
  -4,  1,  2,  5,  5,  2,  1, -4,
  -4,  0,  2,  4,  4,  2,  0, -4,
  -4,  1,  2,  0,  0,  2,  1, -4,
  -6, -6,  0,  0,  0,  0, -6, -6,
  -8, -4, -4, -4, -4, -4, -4, -8,
];

static SQR_TABLE_WN: [i32; def::BOARD_SIZE] = [
  -8, -4, -4, -4, -4, -4, -4, -8,
  -6, -6,  0,  0,  0,  0, -6, -6,
  -4,  1,  2,  0,  0,  2,  1, -4,
  -4,  0,  2,  4,  4,  2,  0, -4,
  -4,  1,  2,  5,  5,  2,  1, -4,
  -4,  0,  3,  4,  4,  3,  0, -4,
  -6, -6,  4,  2,  2,  4, -6, -6,
  -8, -4, -4, -4, -4, -4, -4, -8,
];

static SQR_TABLE_BB: [i32; def::BOARD_SIZE] = [
  -7, -2, -2, -2, -2, -2, -2, -7,
  -4,  0,  2,  0,  0,  2,  0, -4,
  -2,  0,  1,  2,  2,  1,  0, -2,
  -2,  1,  1,  2,  2,  1,  1, -2,
  -2,  0,  2,  2,  2,  2,  0, -2,
  -2,  2,  2,  1,  1,  2,  2, -2,
  -2,  1,  0,  0,  0,  0,  1, -2,
  -7, -2, -2, -2, -2, -2, -2, -7,
];

static SQR_TABLE_WB: [i32; def::BOARD_SIZE] = [
  -7, -2, -2, -2, -2, -2, -2, -7,
  -2,  1,  0,  0,  0,  0,  1, -2,
  -2,  2,  2,  1,  1,  2,  2, -2,
  -2,  0,  2,  2,  2,  2,  0, -2,
  -2,  1,  1,  2,  2,  1,  1, -2,
  -2,  0,  1,  2,  2,  1,  0, -2,
  -4,  0,  2,  0,  0,  2,  0, -4,
  -7, -2, -2, -2, -2, -2, -2, -7,
];

static SQR_TABLE_BR: [i32; def::BOARD_SIZE] = [
    2,  2,  4,  4,  4,  4,  2,  2,
    2,  4,  6,  6,  6,  6,  4,  2,
   -1,  0,  0,  0,  0,  0,  0, -1,
   -1,  0,  0,  0,  0,  0,  0, -1,
   -1,  0,  0,  0,  0,  0,  0, -1,
   -1,  0,  0,  0,  0,  0,  0, -1,
   -2, -1,  0,  0,  0,  0, -1, -2,
   -1,  0,  0,  0,  0,  0,  0, -1,
];

static SQR_TABLE_WR: [i32; def::BOARD_SIZE] = [
   -1,  0,  0,  0,  0,  0,  0, -1,
   -2, -1,  0,  0,  0,  0, -1, -2,
   -1,  0,  0,  0,  0,  0,  0, -1,
   -1,  0,  0,  0,  0,  0,  0, -1,
   -1,  0,  0,  0,  0,  0,  0, -1,
   -1,  0,  0,  0,  0,  0,  0, -1,
    2,  4,  6,  6,  6,  6,  4,  2,
    2,  2,  4,  4,  4,  4,  2,  2,
];

static SQR_TABLE_BQ: [i32; def::BOARD_SIZE] = [
   -4, -2, -2, -1, -1, -2, -2, -4,
   -2, -2,  0,  0,  0,  0, -2, -2,
    0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,
   -2, -2,  0,  0,  0,  0, -2, -2,
   -4, -2, -2, -1, -1, -2, -2, -4,
];

static SQR_TABLE_WQ: [i32; def::BOARD_SIZE] = [
   -4, -2, -2, -1, -1, -2, -2, -4,
   -2, -2,  0,  0,  0,  0, -2, -2,
    0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,
   -2, -2,  0,  0,  0,  0, -2, -2,
   -4, -2, -2, -1, -1, -2, -2, -4,
];


pub fn get_square_ordering_val(moving_piece: u8, from_index: usize, to_index: usize) -> i32 {
    match moving_piece {
        def::WN => SQR_TABLE_WN[to_index] - SQR_TABLE_WN[from_index],
        def::WB => SQR_TABLE_WB[to_index] - SQR_TABLE_WB[from_index],
        def::WR => SQR_TABLE_WR[to_index] - SQR_TABLE_WR[from_index],
        def::WQ => SQR_TABLE_WQ[to_index] - SQR_TABLE_WQ[from_index],

        def::BN => SQR_TABLE_BN[to_index] - SQR_TABLE_BN[from_index],
        def::BB => SQR_TABLE_BB[to_index] - SQR_TABLE_BB[from_index],
        def::BR => SQR_TABLE_BR[to_index] - SQR_TABLE_BR[from_index],
        def::BQ => SQR_TABLE_BQ[to_index] - SQR_TABLE_BQ[from_index],

        _ => 0,
    }
}
