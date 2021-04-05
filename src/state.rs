/*
 * Copyright (C) 2020 Zixiao Han
 */

use crate::{
    bitboard::BitBoard,
    bitmask,
    def,
    util,
    zob_keys,
};

use std::fmt;

const FEN_SQRS_INDEX: usize = 0;
const FEN_PLAYER_INDEX: usize = 1;
const FEN_CAS_RIGHTS_INDEX: usize = 2;
const FEN_ENP_SQR_INDEX: usize = 3;
const FEN_HALF_MOV_INDEX: usize = 4;
const FEN_FULL_MOV_INDEX: usize = 5;

const LAST_MOV_POS_INDEX: usize = 4;
const MAX_NON_CAP_MOV_COUNT: usize = 100;

const K_CAS_SQR_SIZE: usize = 4;
const Q_CAS_SQR_SIZE: usize = 5;

const WK_BEFORE_CAS_SQRS: [u8; K_CAS_SQR_SIZE] = [def::WK, 0, 0, def::WR];
const WK_AFTER_CAS_SQRS: [u8; K_CAS_SQR_SIZE] = [0, def::WR, def::WK, 0];

const BK_BEFORE_CAS_SQRS: [u8; K_CAS_SQR_SIZE] = [def::BK, 0, 0, def::BR];
const BK_AFTER_CAS_SQRS: [u8; K_CAS_SQR_SIZE] = [0, def::BR, def::BK, 0];

const WQ_BEFORE_CAS_SQRS: [u8; Q_CAS_SQR_SIZE] = [def::WR, 0, 0, 0, def::WK];
const WQ_AFTER_CAS_SQRS: [u8; Q_CAS_SQR_SIZE] = [0, 0, def::WK, def::WR, 0];

const BQ_BEFORE_CAS_SQRS: [u8; Q_CAS_SQR_SIZE] = [def::BR, 0, 0, 0, def::BK];
const BQ_AFTER_CAS_SQRS: [u8; Q_CAS_SQR_SIZE] = [0, 0, def::BK, def::BR, 0];

const WK_CAS_ALL_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11110000;
const WK_CAS_R_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10100000;

const BK_CAS_ALL_MASK: u64 = 0b11110000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
const BK_CAS_R_MASK: u64 = 0b10100000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;

const WQ_CAS_ALL_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00011101;
const WQ_CAS_R_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001001;

const BQ_CAS_ALL_MASK: u64 = 0b00011101_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
const BQ_CAS_R_MASK: u64 = 0b00001001_00000000_00000000_00000000_00000000_00000000_00000000_00000000;

pub struct State {
    pub squares: [u8; def::BOARD_SIZE],
    pub player: u8,
    pub cas_rights: u8,
    pub enp_square: usize,
    pub non_cap_mov_count: u16,
    pub hash_key: u64,

    pub wk_index: usize,
    pub bk_index: usize,

    pub cas_history: u8,

    pub bitboard: BitBoard,

    pub taken_piece_stack: Vec<u8>,
    pub enp_sqr_stack: Vec<usize>,
    pub cas_rights_stack: Vec<u8>,
    pub history_pos_stack: Vec<(u64, u8)>,
    pub non_cap_mov_count_stack: Vec<u16>,
    pub king_index_stack: Vec<(usize, usize)>,

    pub full_mov_count: u16,
}

impl State {
    pub fn new(fen_string: &str) -> Self {
        let fen_segment_list: Vec<&str> = fen_string.split(" ").collect();
        let player = get_player_from_fen(fen_segment_list[FEN_PLAYER_INDEX]);
        let cas_rights = get_cas_rights_from_fen(fen_segment_list[FEN_CAS_RIGHTS_INDEX]);
        let enp_square = get_enp_sqr_from_fen(fen_segment_list[FEN_ENP_SQR_INDEX]);
        let non_cap_mov_count = fen_segment_list[FEN_HALF_MOV_INDEX].parse::<u16>().unwrap();
        let full_mov_count = fen_segment_list[FEN_FULL_MOV_INDEX].parse::<u16>().unwrap();

        let bitmask = bitmask::get_bitmask();

        let mut squares = [0; def::BOARD_SIZE];
        let mut hash_key = 0;
        let mut wk_index = 0;
        let mut bk_index = 0;
        let mut bitboard = BitBoard::new();

        let rank_string_list: Vec<&str> = fen_segment_list[FEN_SQRS_INDEX].split("/").collect();

        let mut index = 56;
        for rank_index in 0..def::DIM_SIZE {
            let rank_string = rank_string_list[rank_index];

            for char_code in rank_string.chars() {
                if char_code.is_numeric() {
                    index += char_code.to_digit(10).unwrap() as usize;
                    continue
                }

                if char_code.is_alphabetic() {
                    let piece = util::map_piece_char_to_code(char_code);
                    squares[index] = piece;
                    hash_key ^= zob_keys::get_board_zob_key(index, piece);

                    match piece {
                        def::WP => {
                            bitboard.w_pawn ^= bitmask.index_masks[index];
                        },
                        def::BP => {
                            bitboard.b_pawn ^= bitmask.index_masks[index];
                        },
                        def::WN => {
                            bitboard.w_knight ^= bitmask.index_masks[index];
                        },
                        def::BN => {
                            bitboard.b_knight ^= bitmask.index_masks[index];
                        },
                        def::WB => {
                            bitboard.w_bishop ^= bitmask.index_masks[index];
                        },
                        def::BB => {
                            bitboard.b_bishop ^= bitmask.index_masks[index];
                        },
                        def::WR => {
                            bitboard.w_rook ^= bitmask.index_masks[index];
                        },
                        def::BR => {
                            bitboard.b_rook ^= bitmask.index_masks[index];
                        },
                        def::WQ => {
                            bitboard.w_queen ^= bitmask.index_masks[index];
                        },
                        def::BQ => {
                            bitboard.b_queen ^= bitmask.index_masks[index];
                        },
                        def::WK => {
                            wk_index = index;
                        },
                        def::BK => {
                            bk_index = index;
                        },
                        _ => ()
                    }

                    if def::on_same_side(def::PLAYER_W, piece) {
                        bitboard.w_all ^= bitmask.index_masks[index];
                    } else {
                        bitboard.b_all ^= bitmask.index_masks[index];
                    }

                    index += 1;
                }
            }

            if index == def::DIM_SIZE {
                break
            }

            index -= 16;
        }

        State {
            squares,
            player,
            cas_rights,
            enp_square,
            non_cap_mov_count,
            hash_key,

            wk_index,
            bk_index,

            cas_history: 0,

            bitboard,

            taken_piece_stack: Vec::new(),
            enp_sqr_stack: Vec::new(),
            cas_rights_stack: Vec::new(),
            history_pos_stack: Vec::new(),
            non_cap_mov_count_stack: Vec::new(),
            king_index_stack: Vec::new(),

            full_mov_count,
        }
    }

    pub fn is_draw(&self) -> bool {
        let history_len = self.history_pos_stack.len();
        let check_range = history_len.min(self.non_cap_mov_count as usize + 1);

        if check_range < LAST_MOV_POS_INDEX {
            return false
        }

        if check_range >= MAX_NON_CAP_MOV_COUNT {
            return true
        }

        let (pos_hash, _player) = self.history_pos_stack[history_len - LAST_MOV_POS_INDEX];
        if pos_hash == self.hash_key {
            return true
        }

        let mut dup_count = 0;
        for check_index in 1..=check_range {
            let (pos_hash, player) = self.history_pos_stack[history_len-check_index];
            if pos_hash == self.hash_key && player == self.player {
                dup_count += 1;
            }

            if dup_count > 1 {
                return true
            }
        }

        false
    }

    pub fn do_null_mov(&mut self) {
        self.player = def::get_opposite_player(self.player);
        self.enp_sqr_stack.push(self.enp_square);
        self.enp_square = 0;
    }

    pub fn undo_null_mov(&mut self) {
        self.player = def::get_opposite_player(self.player);
        self.enp_square = self.enp_sqr_stack.pop().unwrap();
    }

    pub fn do_mov(&mut self, from: usize, to: usize, mov_type: u8, promo: u8) {
        self.cas_rights_stack.push(self.cas_rights);
        self.enp_sqr_stack.push(self.enp_square);
        self.history_pos_stack.push((self.hash_key, self.player));
        self.non_cap_mov_count_stack.push(self.non_cap_mov_count);
        self.king_index_stack.push((self.wk_index, self.bk_index));
        self.enp_square = 0;

        match mov_type {
            def::MOV_REG => self.do_reg_mov(from, to),
            def::MOV_PROMO => self.do_promo_mov(from, to, promo),
            def::MOV_CAS => self.do_cas_mov(to),
            def::MOV_ENP => self.do_enp_mov(from, to),
            def::MOV_CR_ENP => self.do_cr_enp_mov(from, to),
            _ => panic!("invalid mov type {}", mov_type),
        }

        self.player = def::get_opposite_player(self.player);
    }

    pub fn undo_mov(&mut self, from: usize, to: usize, mov_type: u8) {
        self.cas_rights = self.cas_rights_stack.pop().unwrap();
        self.enp_square = self.enp_sqr_stack.pop().unwrap();
        self.non_cap_mov_count = self.non_cap_mov_count_stack.pop().unwrap();
        let (wk_index, bk_index) = self.king_index_stack.pop().unwrap();
        self.wk_index = wk_index;
        self.bk_index = bk_index;
        self.hash_key = self.history_pos_stack.pop().unwrap().0;

        self.player = def::get_opposite_player(self.player);

        match mov_type {
            def::MOV_REG => self.undo_reg_mov(from, to),
            def::MOV_PROMO => self.undo_promo_mov(from, to),
            def::MOV_CAS => self.undo_cas_mov(to),
            def::MOV_ENP => self.undo_enp_mov(from, to),
            def::MOV_CR_ENP => self.undo_cr_enp_mov(from, to),
            _ => panic!("invalid mov type {}", mov_type),
        }
    }

    fn do_reg_mov(&mut self, from: usize, to: usize) {
        let moving_piece = self.squares[from];
        let taken_piece = self.squares[to];
        let from_index_mask = bitmask::get_bitmask().index_masks[from];
        let to_index_mask = bitmask::get_bitmask().index_masks[to];
        let move_index_mask = from_index_mask ^ to_index_mask;

        self.hash_key ^= zob_keys::get_board_zob_key(from, moving_piece) ^ zob_keys::get_board_zob_key(to, moving_piece);

        if def::on_same_side(def::PLAYER_W, moving_piece) {
            self.bitboard.w_all ^= move_index_mask;
        } else {
            self.bitboard.b_all ^= move_index_mask;
        }

        match moving_piece {
            def::WP => {
                self.bitboard.w_pawn ^= move_index_mask;
            },
            def::WN => {
                self.bitboard.w_knight ^= move_index_mask;
            },
            def::WB => {
                self.bitboard.w_bishop ^= move_index_mask;
            },
            def::WR => {
                self.bitboard.w_rook ^= move_index_mask;

                if from == 0 {
                    self.cas_rights &= 0b1011;
                } else if from == 7 {
                    self.cas_rights &= 0b0111;
                }
            },
            def::WQ => {
                self.bitboard.w_queen ^= move_index_mask;
            },
            def::BP => {
                self.bitboard.b_pawn ^= move_index_mask;
            },
            def::BN => {
                self.bitboard.b_knight ^= move_index_mask;
            },
            def::BB => {
                self.bitboard.b_bishop ^= move_index_mask;
            },
            def::BR => {
                self.bitboard.b_rook ^= move_index_mask;

                if from == 56 {
                    self.cas_rights &= 0b1110;
                } else if from == 63 {
                    self.cas_rights &= 0b1101;
                }
            },
            def::BQ => {
                self.bitboard.b_queen ^= move_index_mask;
            },
            def::WK => {
                if from == 4 {
                    self.cas_rights &= 0b0011;
                }

                self.wk_index = to;
            },
            def::BK => {
                if from == 60 {
                    self.cas_rights &= 0b1100;
                }

                self.bk_index = to;
            },
            _ => (),
        }

        if taken_piece == 0 {
            if def::is_p(moving_piece) {
                self.non_cap_mov_count = 0;
            } else {
                self.non_cap_mov_count += 1;
            }
        } else {
            self.non_cap_mov_count = 0;
            self.hash_key ^= zob_keys::get_board_zob_key(to, taken_piece);

            if def::on_same_side(def::PLAYER_W, taken_piece) {
                self.bitboard.w_all ^= to_index_mask;
            } else {
                self.bitboard.b_all ^= to_index_mask;
            }

            match taken_piece {
                def::WP => {
                    self.bitboard.w_pawn ^= to_index_mask;
                },
                def::WN => {
                    self.bitboard.w_knight ^= to_index_mask;
                },
                def::WB => {
                    self.bitboard.w_bishop ^= to_index_mask;
                },
                def::WR => {
                    self.bitboard.w_rook ^= to_index_mask;
                },
                def::WQ => {
                    self.bitboard.w_queen ^= to_index_mask;
                },
                def::BP => {
                    self.bitboard.b_pawn ^= to_index_mask;
                },
                def::BN => {
                    self.bitboard.b_knight ^= to_index_mask;
                },
                def::BB => {
                    self.bitboard.b_bishop ^= to_index_mask;
                },
                def::BR => {
                    self.bitboard.b_rook ^= to_index_mask;
                },
                def::BQ => {
                    self.bitboard.b_queen ^= to_index_mask;
                },
                _ => (),
            }
        }

        self.taken_piece_stack.push(taken_piece);
        self.squares[to] = moving_piece;
        self.squares[from] = 0;
    }

    fn undo_reg_mov(&mut self, from: usize, to: usize) {
        let moving_piece = self.squares[to];
        let taken_piece = self.taken_piece_stack.pop().unwrap();

        self.squares[to] = taken_piece;
        self.squares[from] = moving_piece;

        let from_index_mask = bitmask::get_bitmask().index_masks[from];
        let to_index_mask = bitmask::get_bitmask().index_masks[to];
        let move_index_mask = from_index_mask ^ to_index_mask;

        if def::on_same_side(def::PLAYER_W, moving_piece) {
            self.bitboard.w_all ^= move_index_mask;
        } else {
            self.bitboard.b_all ^= move_index_mask;
        }

        match moving_piece {
            def::WP => {
                self.bitboard.w_pawn ^= move_index_mask;
            },
            def::WN => {
                self.bitboard.w_knight ^= move_index_mask;
            },
            def::WB => {
                self.bitboard.w_bishop ^= move_index_mask;
            },
            def::WR => {
                self.bitboard.w_rook ^= move_index_mask;
            },
            def::WQ => {
                self.bitboard.w_queen ^= move_index_mask;
            },
            def::BP => {
                self.bitboard.b_pawn ^= move_index_mask;
            },
            def::BN => {
                self.bitboard.b_knight ^= move_index_mask;
            },
            def::BB => {
                self.bitboard.b_bishop ^= move_index_mask;
            },
            def::BR => {
                self.bitboard.b_rook ^= move_index_mask;
            },
            def::BQ => {
                self.bitboard.b_queen ^= move_index_mask;
            },
            _ => (),
        }

        if taken_piece != 0 {
            if def::on_same_side(def::PLAYER_W, taken_piece) {
                self.bitboard.w_all ^= to_index_mask;
            } else {
                self.bitboard.b_all ^= to_index_mask;
            }

            match taken_piece {
                def::WP => {
                    self.bitboard.w_pawn ^= to_index_mask;
                },
                def::WN => {
                    self.bitboard.w_knight ^= to_index_mask;
                },
                def::WB => {
                    self.bitboard.w_bishop ^= to_index_mask;
                },
                def::WR => {
                    self.bitboard.w_rook ^= to_index_mask;
                },
                def::WQ => {
                    self.bitboard.w_queen ^= to_index_mask;
                },
                def::BP => {
                    self.bitboard.b_pawn ^= to_index_mask;
                },
                def::BN => {
                    self.bitboard.b_knight ^= to_index_mask;
                },
                def::BB => {
                    self.bitboard.b_bishop ^= to_index_mask;
                },
                def::BR => {
                    self.bitboard.b_rook ^= to_index_mask;
                },
                def::BQ => {
                    self.bitboard.b_queen ^= to_index_mask;
                },
                _ => (),
            }
        }
    }

    fn do_promo_mov(&mut self, from: usize, to: usize, promo: u8) {
        let moving_piece = self.squares[from];
        let taken_piece = self.squares[to];

        let from_index_mask = bitmask::get_bitmask().index_masks[from];
        let to_index_mask = bitmask::get_bitmask().index_masks[to];

        if moving_piece == def::WP {
            self.bitboard.w_pawn ^= from_index_mask;
            self.bitboard.w_all ^= from_index_mask;
            self.bitboard.w_all ^= to_index_mask;
        } else {
            self.bitboard.b_pawn ^= from_index_mask;
            self.bitboard.b_all ^= from_index_mask;
            self.bitboard.b_all ^= to_index_mask;
        }

        match promo {
            def::WN => {
                self.bitboard.w_knight ^= to_index_mask;
            },
            def::WB => {
                self.bitboard.w_bishop ^= to_index_mask;
            },
            def::WR => {
                self.bitboard.w_rook ^= to_index_mask;
            },
            def::WQ => {
                self.bitboard.w_queen ^= to_index_mask;
            },
            def::BN => {
                self.bitboard.b_knight ^= to_index_mask;
            },
            def::BB => {
                self.bitboard.b_bishop ^= to_index_mask;
            },
            def::BR => {
                self.bitboard.b_rook ^= to_index_mask;
            },
            def::BQ => {
                self.bitboard.b_queen ^= to_index_mask;
            },
            _ => (),
        }

        self.hash_key ^= zob_keys::get_board_zob_key(from, moving_piece) ^ zob_keys::get_board_zob_key(to, promo);

        if taken_piece != 0 {
            self.hash_key ^= zob_keys::get_board_zob_key(to, taken_piece);

            if def::on_same_side(def::PLAYER_W, taken_piece) {
                self.bitboard.w_all ^= to_index_mask;
            } else {
                self.bitboard.b_all ^= to_index_mask;
            }

            match taken_piece {
                def::WN => {
                    self.bitboard.w_knight ^= to_index_mask;
                },
                def::WB => {
                    self.bitboard.w_bishop ^= to_index_mask;
                },
                def::WR => {
                    self.bitboard.w_rook ^= to_index_mask;
                },
                def::WQ => {
                    self.bitboard.w_queen ^= to_index_mask;
                },
                def::BN => {
                    self.bitboard.b_knight ^= to_index_mask;
                },
                def::BB => {
                    self.bitboard.b_bishop ^= to_index_mask;
                },
                def::BR => {
                    self.bitboard.b_rook ^= to_index_mask;
                },
                def::BQ => {
                    self.bitboard.b_queen ^= to_index_mask;
                },
                _ => (),
            }
        }

        self.taken_piece_stack.push(taken_piece);
        self.squares[to] = promo;
        self.squares[from] = 0;

        self.non_cap_mov_count = 0;
    }

    fn undo_promo_mov(&mut self, from: usize, to: usize) {
        let moving_piece = if self.player == def::PLAYER_W {
            def::WP
        } else {
            def::BP
        };

        let promo = self.squares[to];
        let taken_piece = self.taken_piece_stack.pop().unwrap();

        self.squares[to] = taken_piece;
        self.squares[from] = moving_piece;

        let from_index_mask = bitmask::get_bitmask().index_masks[from];
        let to_index_mask = bitmask::get_bitmask().index_masks[to];

        if moving_piece == def::WP {
            self.bitboard.w_pawn ^= from_index_mask;
            self.bitboard.w_all ^= from_index_mask;
            self.bitboard.w_all ^= to_index_mask;
        } else {
            self.bitboard.b_pawn ^= from_index_mask;
            self.bitboard.b_all ^= from_index_mask;
            self.bitboard.b_all ^= to_index_mask;
        }

        match promo {
            def::WN => {
                self.bitboard.w_knight ^= to_index_mask;
            },
            def::WB => {
                self.bitboard.w_bishop ^= to_index_mask;
            },
            def::WR => {
                self.bitboard.w_rook ^= to_index_mask;
            },
            def::WQ => {
                self.bitboard.w_queen ^= to_index_mask;
            },
            def::BN => {
                self.bitboard.b_knight ^= to_index_mask;
            },
            def::BB => {
                self.bitboard.b_bishop ^= to_index_mask;
            },
            def::BR => {
                self.bitboard.b_rook ^= to_index_mask;
            },
            def::BQ => {
                self.bitboard.b_queen ^= to_index_mask;
            },
            _ => (),
        }

        if taken_piece != 0 {
            if def::on_same_side(def::PLAYER_W, taken_piece) {
                self.bitboard.w_all ^= to_index_mask;
            } else {
                self.bitboard.b_all ^= to_index_mask;
            }

            match taken_piece {
                def::WN => {
                    self.bitboard.w_knight ^= to_index_mask;
                },
                def::WB => {
                    self.bitboard.w_bishop ^= to_index_mask;
                },
                def::WR => {
                    self.bitboard.w_rook ^= to_index_mask;
                },
                def::WQ => {
                    self.bitboard.w_queen ^= to_index_mask;
                },
                def::BN => {
                    self.bitboard.b_knight ^= to_index_mask;
                },
                def::BB => {
                    self.bitboard.b_bishop ^= to_index_mask;
                },
                def::BR => {
                    self.bitboard.b_rook ^= to_index_mask;
                },
                def::BQ => {
                    self.bitboard.b_queen ^= to_index_mask;
                },
                _ => (),
            }
        }
    }

    fn do_cas_mov(&mut self, to: usize) {
        self.non_cap_mov_count = 0;

        if to == def::CAS_SQUARE_WK {
            self.cas_rights &= 0b0011;
            self.cas_history |= 0b1100;
            self.wk_index = to;

            self.squares[def::CAS_SQUARE_WK-2..=def::CAS_SQUARE_WK+1].copy_from_slice(&WK_AFTER_CAS_SQRS);
            self.hash_key ^= zob_keys::get_board_zob_key(def::CAS_SQUARE_WK-2, def::WK)
                ^ zob_keys::get_board_zob_key(def::CAS_SQUARE_WK, def::WK)
                ^ zob_keys::get_board_zob_key(def::CAS_SQUARE_WK-1, def::WR)
                ^ zob_keys::get_board_zob_key(def::CAS_SQUARE_WK+1, def::WR);
            self.bitboard.w_all ^= WK_CAS_ALL_MASK;
            self.bitboard.w_rook ^= WK_CAS_R_MASK;
        } else if to == def::CAS_SQUARE_BK {
            self.cas_rights &= 0b1100;
            self.cas_history |= 0b0011;
            self.bk_index = to;

            self.squares[def::CAS_SQUARE_BK-2..=def::CAS_SQUARE_BK+1].copy_from_slice(&BK_AFTER_CAS_SQRS);
            self.hash_key ^= zob_keys::get_board_zob_key(def::CAS_SQUARE_BK-2, def::BK)
            ^ zob_keys::get_board_zob_key(def::CAS_SQUARE_BK, def::BK)
            ^ zob_keys::get_board_zob_key(def::CAS_SQUARE_BK-1, def::BR)
            ^ zob_keys::get_board_zob_key(def::CAS_SQUARE_BK+1, def::BR);

            self.bitboard.b_all ^= BK_CAS_ALL_MASK;
            self.bitboard.b_rook ^= BK_CAS_R_MASK;
        } else if to == def::CAS_SQUARE_WQ {
            self.cas_rights &= 0b0011;
            self.cas_history |= 0b1100;
            self.wk_index = to;

            self.squares[def::CAS_SQUARE_WQ-2..=def::CAS_SQUARE_WQ+2].copy_from_slice(&WQ_AFTER_CAS_SQRS);
            self.hash_key ^= zob_keys::get_board_zob_key(def::CAS_SQUARE_WQ+2, def::WK)
                ^ zob_keys::get_board_zob_key(def::CAS_SQUARE_WQ, def::WK)
                ^ zob_keys::get_board_zob_key(def::CAS_SQUARE_WQ-2, def::WR)
                ^ zob_keys::get_board_zob_key(def::CAS_SQUARE_WQ+1, def::WR);

            self.bitboard.w_all ^= WQ_CAS_ALL_MASK;
            self.bitboard.w_rook ^= WQ_CAS_R_MASK;
        } else if to == def::CAS_SQUARE_BQ {
            self.cas_rights &= 0b1100;
            self.cas_history |= 0b0011;
            self.bk_index = to;

            self.squares[def::CAS_SQUARE_BQ-2..=def::CAS_SQUARE_BQ+2].copy_from_slice(&BQ_AFTER_CAS_SQRS);
            self.hash_key ^= zob_keys::get_board_zob_key(def::CAS_SQUARE_BQ+2, def::BK)
            ^ zob_keys::get_board_zob_key(def::CAS_SQUARE_BQ, def::BK)
            ^ zob_keys::get_board_zob_key(def::CAS_SQUARE_BQ-2, def::BR)
            ^ zob_keys::get_board_zob_key(def::CAS_SQUARE_BQ+1, def::BR);

            self.bitboard.b_all ^= BQ_CAS_ALL_MASK;
            self.bitboard.b_rook ^= BQ_CAS_R_MASK;
        }
    }

    fn undo_cas_mov(&mut self, to: usize) {
        if to == def::CAS_SQUARE_WK {
            self.squares[def::CAS_SQUARE_WK-2..=def::CAS_SQUARE_WK+1].copy_from_slice(&WK_BEFORE_CAS_SQRS);
            self.bitboard.w_all ^= WK_CAS_ALL_MASK;
            self.bitboard.w_rook ^= WK_CAS_R_MASK;
            self.cas_history &= 0b0011;
        } else if to == def::CAS_SQUARE_BK {
            self.squares[def::CAS_SQUARE_BK-2..=def::CAS_SQUARE_BK+1].copy_from_slice(&BK_BEFORE_CAS_SQRS);
            self.bitboard.b_all ^= BK_CAS_ALL_MASK;
            self.bitboard.b_rook ^= BK_CAS_R_MASK;
            self.cas_history &= 0b1100;
        } else if to == def::CAS_SQUARE_WQ {
            self.squares[def::CAS_SQUARE_WQ-2..=def::CAS_SQUARE_WQ+2].copy_from_slice(&WQ_BEFORE_CAS_SQRS);
            self.bitboard.w_all ^= WQ_CAS_ALL_MASK;
            self.bitboard.w_rook ^= WQ_CAS_R_MASK;
            self.cas_history &= 0b0011;
        } else if to == def::CAS_SQUARE_BQ {
            self.squares[def::CAS_SQUARE_BQ-2..=def::CAS_SQUARE_BQ+2].copy_from_slice(&BQ_BEFORE_CAS_SQRS);
            self.bitboard.b_all ^= BQ_CAS_ALL_MASK;
            self.bitboard.b_rook ^= BQ_CAS_R_MASK;
            self.cas_history &= 0b1100;
        }
    }

    fn do_enp_mov(&mut self, from: usize, to: usize) {
        let taken_index = if self.player == def::PLAYER_W {
            to - 8
        } else {
            to + 8
        };

        self.non_cap_mov_count = 0;

        let moving_piece = self.squares[from];
        let taken_piece = self.squares[taken_index];

        let from_index_mask = bitmask::get_bitmask().index_masks[from];
        let to_index_mask = bitmask::get_bitmask().index_masks[to];
        let taken_index_mask = bitmask::get_bitmask().index_masks[taken_index];

        match taken_piece {
            def::WP => {
                self.bitboard.w_pawn ^= taken_index_mask;
                self.bitboard.w_all ^= taken_index_mask;

                self.bitboard.b_pawn ^= from_index_mask;
                self.bitboard.b_all ^= from_index_mask;
                self.bitboard.b_pawn ^= to_index_mask;
                self.bitboard.b_all ^= to_index_mask;
            },
            def::BP => {
                self.bitboard.b_pawn ^= taken_index_mask;
                self.bitboard.b_all ^= taken_index_mask;

                self.bitboard.w_pawn ^= from_index_mask;
                self.bitboard.w_all ^= from_index_mask;
                self.bitboard.w_pawn ^= to_index_mask;
                self.bitboard.w_all ^= to_index_mask;
            },
            _ => ()
        }

        self.hash_key ^= zob_keys::get_board_zob_key(from, moving_piece) ^ zob_keys::get_board_zob_key(to, moving_piece) ^ zob_keys::get_board_zob_key(taken_index, taken_piece);

        self.taken_piece_stack.push(taken_piece);
        self.squares[to] = moving_piece;
        self.squares[from] = 0;
        self.squares[taken_index] = 0;
    }

    fn undo_enp_mov(&mut self, from: usize, to: usize) {
        let taken_index = if self.player == def::PLAYER_W {
            to - 8
        } else {
            to + 8
        };

        let moving_piece = self.squares[to];
        let taken_piece = self.taken_piece_stack.pop().unwrap();

        self.squares[taken_index] = taken_piece;
        self.squares[from] = moving_piece;
        self.squares[to] = 0;

        let from_index_mask = bitmask::get_bitmask().index_masks[from];
        let to_index_mask = bitmask::get_bitmask().index_masks[to];
        let taken_index_mask = bitmask::get_bitmask().index_masks[taken_index];

        match taken_piece {
            def::WP => {
                self.bitboard.w_pawn ^= taken_index_mask;
                self.bitboard.w_all ^= taken_index_mask;

                self.bitboard.b_pawn ^= from_index_mask;
                self.bitboard.b_all ^= from_index_mask;
                self.bitboard.b_pawn ^= to_index_mask;
                self.bitboard.b_all ^= to_index_mask;
            },
            def::BP => {
                self.bitboard.b_pawn ^= taken_index_mask;
                self.bitboard.b_all ^= taken_index_mask;

                self.bitboard.w_pawn ^= from_index_mask;
                self.bitboard.w_all ^= from_index_mask;
                self.bitboard.w_pawn ^= to_index_mask;
                self.bitboard.w_all ^= to_index_mask;
            },
            _ => ()
        }
    }

    fn do_cr_enp_mov(&mut self, from: usize, to: usize) {
        self.enp_square = if self.player == def::PLAYER_W {
            to - 8
        } else {
            to + 8
        };

        self.non_cap_mov_count = 0;

        let moving_piece = self.squares[from];
        let move_index_mask = bitmask::get_bitmask().index_masks[from] ^ bitmask::get_bitmask().index_masks[to];

        self.hash_key ^= zob_keys::get_board_zob_key(from, moving_piece) ^ zob_keys::get_board_zob_key(to, moving_piece);

        if def::on_same_side(def::PLAYER_W, moving_piece) {
            self.bitboard.w_all ^= move_index_mask;
            self.bitboard.w_pawn ^= move_index_mask;
        } else {
            self.bitboard.b_all ^= move_index_mask;
            self.bitboard.b_pawn ^= move_index_mask;
        }

        self.squares[to] = moving_piece;
        self.squares[from] = 0;
    }

    fn undo_cr_enp_mov(&mut self, from: usize, to: usize) {
        let moving_piece = self.squares[to];

        self.squares[from] = moving_piece;
        self.squares[to] = 0;

        let move_index_mask = bitmask::get_bitmask().index_masks[from] ^ bitmask::get_bitmask().index_masks[to];

        if def::on_same_side(def::PLAYER_W, moving_piece) {
            self.bitboard.w_all ^= move_index_mask;
            self.bitboard.w_pawn ^= move_index_mask;
        } else {
            self.bitboard.b_all ^= move_index_mask;
            self.bitboard.b_pawn ^= move_index_mask;
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut display_string = String::new();

        let mut rank_left_index = 56;
        loop {
            for file_index in 0..def::DIM_SIZE {
                display_string.push(util::map_piece_code_to_char(self.squares[rank_left_index + file_index]));
                display_string.push_str(" ");
            }

            display_string.push('\n');

            if rank_left_index == 0 {
                break
            }

            rank_left_index -= 8;
        }

        write!(formatter, "{}", display_string)
    }
}

fn get_player_from_fen(fen_player_string: &str) -> u8 {
    match fen_player_string {
        "w" => def::PLAYER_W,
        "b" => def::PLAYER_B,
        _ => panic!("invalid player {}", fen_player_string),
    }
}

fn get_cas_rights_from_fen(fen_cas_rights_player: &str) -> u8 {
    if fen_cas_rights_player == "-" {
        return 0
    }

    let mut cas_rights = 0;

    if fen_cas_rights_player.contains("K") {
        cas_rights |= 0b1000;
    }

    if fen_cas_rights_player.contains("Q") {
        cas_rights |= 0b0100;
    }

    if fen_cas_rights_player.contains("k") {
        cas_rights |= 0b0010;
    }

    if fen_cas_rights_player.contains("q") {
        cas_rights |= 0b0001;
    }

    cas_rights
}

fn get_enp_sqr_from_fen(fen_enp_sqr_string: &str) -> usize {
    if fen_enp_sqr_string == "-" {
        return 0
    }

    util::map_sqr_notation_to_index(fen_enp_sqr_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        def,
    };

    #[test]
    fn test_new_startpos() {
        zob_keys::init();
        bitmask::init();

        let state = State::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        assert_eq!(0b1111, state.cas_rights);
        assert_eq!(0, state.enp_square);
        assert_eq!(def::PLAYER_W, state.player);
    }

    #[test]
    fn test_do_move_1() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(0b1111, state.cas_rights);
        assert_eq!(0, state.enp_square);
        assert_eq!(def::PLAYER_W, state.player);

        let w_all = state.bitboard.w_all;
        let b_all = state.bitboard.b_all;

        state.do_mov(util::map_sqr_notation_to_index("e2"), util::map_sqr_notation_to_index("e4"), def::MOV_CR_ENP, 0);
        assert_eq!(0b1111, state.cas_rights);
        assert_eq!(util::map_sqr_notation_to_index("e3"), state.enp_square);
        assert_eq!(def::PLAYER_B, state.player);

        state.undo_mov(util::map_sqr_notation_to_index("e2"), util::map_sqr_notation_to_index("e4"), def::MOV_CR_ENP);
        assert_eq!(0b1111, state.cas_rights);
        assert_eq!(0, state.enp_square);
        assert_eq!(def::PLAYER_W, state.player);

        assert_eq!(w_all, state.bitboard.w_all);
        assert_eq!(b_all, state.bitboard.b_all);
    }

    #[test]
    fn test_do_move_2() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("r1bqk1nr/pPpp1ppp/2n5/2b1p3/2B1P3/2N2N2/P1PP1PPP/R1BQK2R w KQkq - 0 1");
        assert_eq!(0b1111, state.cas_rights);
        assert_eq!(0, state.enp_square);
        assert_eq!(def::PLAYER_W, state.player);
        assert_eq!(def::BR, state.squares[util::map_sqr_notation_to_index("a8")]);

        let w_all = state.bitboard.w_all;
        let b_all = state.bitboard.b_all;

        state.do_mov(util::map_sqr_notation_to_index("b7"), util::map_sqr_notation_to_index("a8"), def::MOV_PROMO, def::WQ);
        assert_eq!(0b1111, state.cas_rights);
        assert_eq!(0, state.enp_square);
        assert_eq!(def::PLAYER_B, state.player);
        assert_eq!(def::WQ, state.squares[util::map_sqr_notation_to_index("a8")]);

        state.undo_mov(util::map_sqr_notation_to_index("b7"), util::map_sqr_notation_to_index("a8"), def::MOV_PROMO);
        assert_eq!(0b1111, state.cas_rights);
        assert_eq!(0, state.enp_square);
        assert_eq!(def::PLAYER_W, state.player);
        assert_eq!(def::BR, state.squares[util::map_sqr_notation_to_index("a8")]);

        assert_eq!(w_all, state.bitboard.w_all);
        assert_eq!(b_all, state.bitboard.b_all);
    }

    #[test]
    fn test_do_move_3() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("r3k2r/pbppnppp/1bn2q2/4p3/2B5/2N1PN2/PPPP1PPP/R1BQK2R b Qkq - 0 1");
        assert_eq!(0b0111, state.cas_rights);
        assert_eq!(0b0000, state.cas_history);
        assert_eq!(0, state.enp_square);
        assert_eq!(def::PLAYER_B, state.player);
        assert_eq!(def::BK, state.squares[util::map_sqr_notation_to_index("e8")]);
        assert_eq!(def::BR, state.squares[util::map_sqr_notation_to_index("a8")]);
        assert_eq!(0, state.squares[util::map_sqr_notation_to_index("c8")]);

        let w_all = state.bitboard.w_all;
        let b_all = state.bitboard.b_all;

        state.do_mov(util::map_sqr_notation_to_index("e8"), util::map_sqr_notation_to_index("c8"), def::MOV_CAS, 0);
        assert_eq!(0b0100, state.cas_rights);
        assert_eq!(0b0011, state.cas_history);
        assert_eq!(0, state.enp_square);
        assert_eq!(def::PLAYER_W, state.player);
        assert_eq!(0, state.squares[util::map_sqr_notation_to_index("e8")]);
        assert_eq!(0, state.squares[util::map_sqr_notation_to_index("a8")]);
        assert_eq!(def::BK, state.squares[util::map_sqr_notation_to_index("c8")]);
        assert_eq!(def::BR, state.squares[util::map_sqr_notation_to_index("d8")]);

        state.undo_mov(util::map_sqr_notation_to_index("e8"), util::map_sqr_notation_to_index("c8"), def::MOV_CAS);
        assert_eq!(0b0111, state.cas_rights);
        assert_eq!(0b0000, state.cas_history);
        assert_eq!(0, state.enp_square);
        assert_eq!(def::PLAYER_B, state.player);
        assert_eq!(def::BK, state.squares[util::map_sqr_notation_to_index("e8")]);
        assert_eq!(def::BR, state.squares[util::map_sqr_notation_to_index("a8")]);
        assert_eq!(0, state.squares[util::map_sqr_notation_to_index("c8")]);
        assert_eq!(0, state.squares[util::map_sqr_notation_to_index("d8")]);

        assert_eq!(w_all, state.bitboard.w_all);
        assert_eq!(b_all, state.bitboard.b_all);
    }

    #[test]
    fn test_do_move_4() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("4r1k1/pp1Q1ppp/3B4/q2p4/5P1P/P3PbPK/1P1r4/2R5 b - - 3 5");
        assert_eq!(0b0000, state.cas_rights);
        assert_eq!(0, state.enp_square);
        assert_eq!(def::PLAYER_B, state.player);
        assert_eq!(0, state.squares[util::map_sqr_notation_to_index("e2")]);
        assert_eq!(def::BR, state.squares[util::map_sqr_notation_to_index("d2")]);

        state.do_mov(util::map_sqr_notation_to_index("d2"), util::map_sqr_notation_to_index("e2"), def::MOV_REG, 0);
        assert_eq!(0b0000, state.cas_rights);
        assert_eq!(0, state.enp_square);
        assert_eq!(def::PLAYER_W, state.player);
        assert_eq!(0, state.squares[util::map_sqr_notation_to_index("d2")]);
        assert_eq!(def::BR, state.squares[util::map_sqr_notation_to_index("e2")]);

        state.do_mov(util::map_sqr_notation_to_index("d7"), util::map_sqr_notation_to_index("e8"), def::MOV_REG, 0);
        state.do_mov(util::map_sqr_notation_to_index("e2"), util::map_sqr_notation_to_index("h2"), def::MOV_REG, 0);

        state.undo_mov(util::map_sqr_notation_to_index("e2"), util::map_sqr_notation_to_index("h2"), def::MOV_REG);
        state.undo_mov(util::map_sqr_notation_to_index("d7"), util::map_sqr_notation_to_index("e8"), def::MOV_REG);

        state.undo_mov(util::map_sqr_notation_to_index("d2"), util::map_sqr_notation_to_index("e2"), def::MOV_REG);
        assert_eq!(0b0000, state.cas_rights);
        assert_eq!(0, state.enp_square);
        assert_eq!(def::PLAYER_B, state.player);
        assert_eq!(0, state.squares[util::map_sqr_notation_to_index("e2")]);
        assert_eq!(def::BR, state.squares[util::map_sqr_notation_to_index("d2")]);
    }

    #[test]
    fn test_do_move_5() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("r1bqkbnr/ppp1p1pp/2n5/3pPp2/3P4/8/PPP2PPP/RNBQKBNR w KQkq f6 0 1");
        assert_eq!(0b1111, state.cas_rights);
        assert_eq!(util::map_sqr_notation_to_index("f6"), state.enp_square);
        assert_eq!(def::PLAYER_W, state.player);
        assert_eq!(0, state.squares[util::map_sqr_notation_to_index("f6")]);
        assert_eq!(def::BP, state.squares[util::map_sqr_notation_to_index("f5")]);
        assert_eq!(def::WP, state.squares[util::map_sqr_notation_to_index("e5")]);

        state.do_mov(util::map_sqr_notation_to_index("e5"), util::map_sqr_notation_to_index("f6"), def::MOV_ENP, 0);
        assert_eq!(0b1111, state.cas_rights);
        assert_eq!(0, state.enp_square);
        assert_eq!(def::PLAYER_B, state.player);
        assert_eq!(def::WP, state.squares[util::map_sqr_notation_to_index("f6")]);
        assert_eq!(0, state.squares[util::map_sqr_notation_to_index("f5")]);
        assert_eq!(0, state.squares[util::map_sqr_notation_to_index("e5")]);

        state.undo_mov(util::map_sqr_notation_to_index("e5"), util::map_sqr_notation_to_index("f6"), def::MOV_ENP);
        assert_eq!(0b1111, state.cas_rights);
        assert_eq!(util::map_sqr_notation_to_index("f6"), state.enp_square);
        assert_eq!(def::PLAYER_W, state.player);
        assert_eq!(0, state.squares[util::map_sqr_notation_to_index("f6")]);
        assert_eq!(def::BP, state.squares[util::map_sqr_notation_to_index("f5")]);
        assert_eq!(def::WP, state.squares[util::map_sqr_notation_to_index("e5")]);
    }

    #[test]
    fn test_do_move_6() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B1P1NBn/pPP2PPP/R3K2R b KQ - 0 1");

        state.do_mov(util::map_sqr_notation_to_index("e5"), util::map_sqr_notation_to_index("e4"), def::MOV_REG, 0);
        state.do_mov(util::map_sqr_notation_to_index("f3"), util::map_sqr_notation_to_index("d2"), def::MOV_REG, 0);
        state.do_mov(util::map_sqr_notation_to_index("d7"), util::map_sqr_notation_to_index("d5"), def::MOV_REG, 0);
        state.do_mov(util::map_sqr_notation_to_index("f2"), util::map_sqr_notation_to_index("f4"), def::MOV_CR_ENP, 0);

        assert_eq!(util::map_sqr_notation_to_index("f3"), state.enp_square);
        assert_eq!(def::PLAYER_B, state.player);
        assert_eq!(0, state.squares[util::map_sqr_notation_to_index("f3")]);
        assert_eq!(def::WP, state.squares[util::map_sqr_notation_to_index("f4")]);
        assert_eq!(def::BP, state.squares[util::map_sqr_notation_to_index("e4")]);

        let w_all = state.bitboard.w_all;
        let b_all = state.bitboard.b_all;

        state.do_mov(util::map_sqr_notation_to_index("e4"), util::map_sqr_notation_to_index("f3"), def::MOV_ENP, 0);
        assert_eq!(0, state.enp_square);
        assert_eq!(def::PLAYER_W, state.player);
        assert_eq!(def::BP, state.squares[util::map_sqr_notation_to_index("f3")]);
        assert_eq!(0, state.squares[util::map_sqr_notation_to_index("f4")]);
        assert_eq!(0, state.squares[util::map_sqr_notation_to_index("e4")]);

        state.do_mov(util::map_sqr_notation_to_index("g2"), util::map_sqr_notation_to_index("f3"), def::MOV_REG, 0);
        assert_eq!(0, state.enp_square);
        assert_eq!(def::PLAYER_B, state.player);
        assert_eq!(def::WP, state.squares[util::map_sqr_notation_to_index("f3")]);
        assert_eq!(0, state.squares[util::map_sqr_notation_to_index("f4")]);
        assert_eq!(0, state.squares[util::map_sqr_notation_to_index("e4")]);

        state.undo_mov(util::map_sqr_notation_to_index("g2"), util::map_sqr_notation_to_index("f3"), def::MOV_REG);
        state.undo_mov(util::map_sqr_notation_to_index("e4"), util::map_sqr_notation_to_index("f3"), def::MOV_ENP);
        assert_eq!(util::map_sqr_notation_to_index("f3"), state.enp_square);
        assert_eq!(def::PLAYER_B, state.player);
        assert_eq!(0, state.squares[util::map_sqr_notation_to_index("f3")]);
        assert_eq!(def::WP, state.squares[util::map_sqr_notation_to_index("f4")]);
        assert_eq!(def::BP, state.squares[util::map_sqr_notation_to_index("e4")]);

        assert_eq!(w_all, state.bitboard.w_all);
        assert_eq!(b_all, state.bitboard.b_all);
    }

    #[test]
    fn test_do_move_7() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 1");
        assert_eq!(0b1111, state.cas_rights);
        assert_eq!(0b0000, state.cas_history);

        state.do_mov(util::map_sqr_notation_to_index("e1"), util::map_sqr_notation_to_index("e2"), def::MOV_REG, 0);
        assert_eq!(0b0011, state.cas_rights);
        assert_eq!(0b0000, state.cas_history);

        state.undo_mov(util::map_sqr_notation_to_index("e2"), util::map_sqr_notation_to_index("e1"), def::MOV_REG);
        assert_eq!(0b1111, state.cas_rights);
        assert_eq!(0b0000, state.cas_history);
    }

    #[test]
    fn test_zob_hash_1() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 1");
        let original_hash = state.hash_key;

        state.do_mov(util::map_sqr_notation_to_index("e1"), util::map_sqr_notation_to_index("g1"), def::MOV_CAS, 0);
        let hash_after_castle = state.hash_key;

        state.undo_mov(util::map_sqr_notation_to_index("e1"), util::map_sqr_notation_to_index("g1"), def::MOV_CAS);

        assert_eq!(state.hash_key, original_hash);

        state.do_mov(util::map_sqr_notation_to_index("e1"), util::map_sqr_notation_to_index("e2"), def::MOV_REG, 0);
        state.do_mov(util::map_sqr_notation_to_index("f8"), util::map_sqr_notation_to_index("e7"), def::MOV_REG, 0);

        state.do_mov(util::map_sqr_notation_to_index("h1"), util::map_sqr_notation_to_index("e1"), def::MOV_REG, 0);
        state.do_mov(util::map_sqr_notation_to_index("e7"), util::map_sqr_notation_to_index("f8"), def::MOV_REG, 0);

        state.do_mov(util::map_sqr_notation_to_index("e2"), util::map_sqr_notation_to_index("f1"), def::MOV_REG, 0);
        state.do_mov(util::map_sqr_notation_to_index("f8"), util::map_sqr_notation_to_index("e7"), def::MOV_REG, 0);

        state.do_mov(util::map_sqr_notation_to_index("f1"), util::map_sqr_notation_to_index("g1"), def::MOV_REG, 0);
        state.do_mov(util::map_sqr_notation_to_index("e7"), util::map_sqr_notation_to_index("f8"), def::MOV_REG, 0);

        state.do_mov(util::map_sqr_notation_to_index("e1"), util::map_sqr_notation_to_index("f1"), def::MOV_REG, 0);

        assert_eq!(state.hash_key, hash_after_castle);
    }

    #[test]
    fn test_zob_hash_2() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("r3kb1r/ppp2ppp/2np1n2/4p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R b KQkq - 0 1");
        let original_hash = state.hash_key;

        state.do_mov(util::map_sqr_notation_to_index("e8"), util::map_sqr_notation_to_index("c8"), def::MOV_CAS, 0);
        let hash_after_castle = state.hash_key;

        state.undo_mov(util::map_sqr_notation_to_index("e8"), util::map_sqr_notation_to_index("c8"), def::MOV_CAS);

        assert_eq!(state.hash_key, original_hash);

        state.do_mov(util::map_sqr_notation_to_index("e8"), util::map_sqr_notation_to_index("d7"), def::MOV_REG, 0);
        state.do_mov(util::map_sqr_notation_to_index("b1"), util::map_sqr_notation_to_index("c3"), def::MOV_REG, 0);

        state.do_mov(util::map_sqr_notation_to_index("a8"), util::map_sqr_notation_to_index("d8"), def::MOV_REG, 0);
        state.do_mov(util::map_sqr_notation_to_index("c3"), util::map_sqr_notation_to_index("b1"), def::MOV_REG, 0);

        state.do_mov(util::map_sqr_notation_to_index("d7"), util::map_sqr_notation_to_index("c8"), def::MOV_REG, 0);

        assert_eq!(state.hash_key, hash_after_castle);
    }

    #[test]
    fn test_bitboard_1() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("r3kb1r/ppp2ppp/2np1n2/4p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R w KQkq - 0 1");

        assert_eq!(0b00000000_00000000_00000000_00000000_00010000_00001000_11100111_00000000, state.bitboard.w_pawn);
        assert_eq!(0b00000000_11100111_00001000_00010000_00000000_00000000_00000000_00000000, state.bitboard.b_pawn);

        assert_eq!(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001, state.bitboard.w_rook);
        assert_eq!(0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000, state.bitboard.b_rook);

        state.do_mov(util::map_sqr_notation_to_index("e1"), util::map_sqr_notation_to_index("g1"), def::MOV_CAS, 0);

        assert_eq!(0b00000000_00000000_00000000_00000000_00010000_00001000_11100111_00000000, state.bitboard.w_pawn);
        assert_eq!(0b00000000_11100111_00001000_00010000_00000000_00000000_00000000_00000000, state.bitboard.b_pawn);

        assert_eq!(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100001, state.bitboard.w_rook);
        assert_eq!(0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000, state.bitboard.b_rook);

        state.do_mov(util::map_sqr_notation_to_index("e8"), util::map_sqr_notation_to_index("c8"), def::MOV_CAS, 0);

        assert_eq!(0b00000000_00000000_00000000_00000000_00010000_00001000_11100111_00000000, state.bitboard.w_pawn);
        assert_eq!(0b00000000_11100111_00001000_00010000_00000000_00000000_00000000_00000000, state.bitboard.b_pawn);

        assert_eq!(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100001, state.bitboard.w_rook);
        assert_eq!(0b10001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000, state.bitboard.b_rook);

        state.do_mov(util::map_sqr_notation_to_index("b2"), util::map_sqr_notation_to_index("b4"), def::MOV_CR_ENP, 0);

        assert_eq!(0b00000000_00000000_00000000_00000000_00010010_00001000_11100101_00000000, state.bitboard.w_pawn);
        assert_eq!(0b00000000_11100111_00001000_00010000_00000000_00000000_00000000_00000000, state.bitboard.b_pawn);

        assert_eq!(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100001, state.bitboard.w_rook);
        assert_eq!(0b10001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000, state.bitboard.b_rook);

        state.do_mov(util::map_sqr_notation_to_index("b7"), util::map_sqr_notation_to_index("b6"), def::MOV_REG, 0);
        state.do_mov(util::map_sqr_notation_to_index("b4"), util::map_sqr_notation_to_index("b5"), def::MOV_REG, 0);
        state.do_mov(util::map_sqr_notation_to_index("a7"), util::map_sqr_notation_to_index("a5"), def::MOV_CR_ENP, 0);
        state.do_mov(util::map_sqr_notation_to_index("b5"), util::map_sqr_notation_to_index("a6"), def::MOV_ENP, 0);

        assert_eq!(0b00000000_00000000_00000001_00000000_00010000_00001000_11100101_00000000, state.bitboard.w_pawn);
        assert_eq!(0b00000000_11100100_00001010_00010000_00000000_00000000_00000000_00000000, state.bitboard.b_pawn);

        assert_eq!(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100001, state.bitboard.w_rook);
        assert_eq!(0b10001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000, state.bitboard.b_rook);
    }

    #[test]
    fn test_is_draw() {
        zob_keys::init();
        bitmask::init();

        let mut state = State::new("8/p2rrpk1/R2p1p1p/1P1P1P1P/R2KP1P1/8/8/8 b - - 12 57");

        assert!(!state.is_draw());

        state.do_mov(util::map_sqr_notation_to_index("g7"), util::map_sqr_notation_to_index("g8"), def::MOV_REG, 0);
        assert!(!state.is_draw());

        state.do_mov(util::map_sqr_notation_to_index("a4"), util::map_sqr_notation_to_index("a5"), def::MOV_REG, 0);
        assert!(!state.is_draw());

        state.do_mov(util::map_sqr_notation_to_index("g8"), util::map_sqr_notation_to_index("f8"), def::MOV_REG, 0);
        assert!(!state.is_draw());

        state.do_mov(util::map_sqr_notation_to_index("a5"), util::map_sqr_notation_to_index("a2"), def::MOV_REG, 0);
        assert!(!state.is_draw());

        state.do_mov(util::map_sqr_notation_to_index("f8"), util::map_sqr_notation_to_index("g8"), def::MOV_REG, 0);
        assert!(!state.is_draw());

        state.do_mov(util::map_sqr_notation_to_index("a2"), util::map_sqr_notation_to_index("a1"), def::MOV_REG, 0);
        assert!(!state.is_draw());

        state.do_mov(util::map_sqr_notation_to_index("g8"), util::map_sqr_notation_to_index("g7"), def::MOV_REG, 0);
        assert!(!state.is_draw());

        state.do_mov(util::map_sqr_notation_to_index("a1"), util::map_sqr_notation_to_index("a4"), def::MOV_REG, 0);
        assert!(!state.is_draw());

        state.do_mov(util::map_sqr_notation_to_index("g7"), util::map_sqr_notation_to_index("g8"), def::MOV_REG, 0);
        assert!(!state.is_draw());

        state.do_mov(util::map_sqr_notation_to_index("a4"), util::map_sqr_notation_to_index("a5"), def::MOV_REG, 0);
        assert!(!state.is_draw());

        state.do_mov(util::map_sqr_notation_to_index("g8"), util::map_sqr_notation_to_index("f8"), def::MOV_REG, 0);
        assert!(!state.is_draw());

        state.do_mov(util::map_sqr_notation_to_index("a5"), util::map_sqr_notation_to_index("a2"), def::MOV_REG, 0);
        assert!(!state.is_draw());

        state.do_mov(util::map_sqr_notation_to_index("f8"), util::map_sqr_notation_to_index("g8"), def::MOV_REG, 0);
        assert!(!state.is_draw());

        state.do_mov(util::map_sqr_notation_to_index("a2"), util::map_sqr_notation_to_index("a1"), def::MOV_REG, 0);
        assert!(!state.is_draw());

        state.do_mov(util::map_sqr_notation_to_index("g8"), util::map_sqr_notation_to_index("g7"), def::MOV_REG, 0);
        assert!(!state.is_draw());

        state.do_mov(util::map_sqr_notation_to_index("a1"), util::map_sqr_notation_to_index("a4"), def::MOV_REG, 0);
        assert!(state.is_draw());
    }
}
