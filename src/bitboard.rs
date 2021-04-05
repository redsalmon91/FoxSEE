/*
 * Copyright (C) 2020 Zixiao Han
 */

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

