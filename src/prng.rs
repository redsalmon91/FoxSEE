/*
 * Copyright (C) 2020 Zixiao Han
 */

use crate::{
    def,
};

static SEED_C89: u64 = 0b10110110_00101111_10100100_01011000_00001000_01100100_11010111_11111010;
static SEED_A86: u64 = 0b10111001_11010011_00111100_00010100_00110000_00100110_11001111_10110110;

const fn rotate(x: u64, k: usize) -> u64 {
    (x << k) | (x >> (64 - k))
}

pub struct XorshiftPrng {
    state: [u64; 2],
}

impl XorshiftPrng {
    pub fn new() -> XorshiftPrng {
        XorshiftPrng {
            state: [SEED_C89, SEED_A86],
        }
    }

    fn gen_rand(&mut self) -> u64 {
        let s0 = self.state[0];
        let mut s1 = self.state[1];

        let next_rand = rotate(s0 * 5, 7) * 9;
        s1 ^= s0;

        self.state[0] = rotate(s0, 24) ^ s1 ^ (s1 << 16);
        self.state[1] = rotate(s1, 37);

        next_rand
    }

    pub fn create_prn_table(&mut self, fst_dim: usize, snd_dim: usize) -> Vec<Vec<u64>> {
        let mut prn_table = vec![vec![0; snd_dim]; fst_dim];

        for i in 0..def::BOARD_SIZE {
            for j in 0..def::PIECE_CODE_RANGE {
                let mut prn = self.gen_rand();

                'trail_error: loop {
                    for ii in 0..i {
                        for jj in 0..j {
                            if prn_table[ii][jj] == prn {
                                prn = self.gen_rand();
                                continue 'trail_error
                            }
                        }
                    }

                    break
                }

                prn_table[i][j] = prn;
            }
        }

        prn_table
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::def;

    #[test]
    fn test_gen_rand() {
        let mut rand = XorshiftPrng::new();
        let rand_table = rand.create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);

        let mut hash_key = 0;
        for i in 0..def::BOARD_SIZE {
            for j in 0..def::PIECE_CODE_RANGE {
                hash_key ^= rand_table[i][j];

                if hash_key == 0 {
                    panic!("bad hash");
                }
            }
        }
    }
}
