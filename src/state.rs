use crate::{
    bitboard::{BitMask, BitBoard},
    def,
    util,
};

use std::fmt;

const FEN_SQRS_INDEX: usize = 0;
const FEN_PLAYER_INDEX: usize = 1;
const FEN_CAS_RIGHTS_INDEX: usize = 2;
const FEN_ENP_SQR_INDEX: usize = 3;
const FEN_HALF_MOV_INDEX: usize = 4;

const LAST_DUP_POS_INDEX: usize = 4;
const MIN_POS_COUNT_FOR_REP: usize = 6;

pub struct State<'state> {
    pub squares: [u8; def::BOARD_SIZE],
    pub player: u8,
    pub cas_rights: u8,
    pub enp_square: usize,
    pub non_cap_mov_count: u16,
    pub hash_key: u64,

    pub wk_index: usize,
    pub bk_index: usize,

    pub bitboard: BitBoard,
    pub bitboard_stack: Vec<BitBoard>,
    pub bitmask: &'state BitMask,

    pub taken_piece_stack: Vec<u8>,
    pub enp_sqr_stack: Vec<usize>,
    pub cas_rights_stack: Vec<u8>,
    pub history_mov_stack: Vec<(usize, u8)>,
    pub history_pos_stack: Vec<u64>,
    pub non_cap_mov_count_stack: Vec<u16>,
    pub wk_index_stack: Vec<usize>,
    pub bk_index_stack: Vec<usize>,

    zob_keys: &'state Vec<Vec<u64>>,
}

impl <'state> State<'state> {
    pub fn new(fen_string: &str, zob_keys: &'state Vec<Vec<u64>>, bitmask: &'state BitMask) -> Self {
        let fen_segment_list: Vec<&str> = fen_string.split(" ").collect();
        let (squares, hash_key, wk_index, bk_index, bitboard) = get_board_info_from_fen(fen_segment_list[FEN_SQRS_INDEX], zob_keys, bitmask);
        let player = get_player_from_fen(fen_segment_list[FEN_PLAYER_INDEX]);
        let cas_rights = get_cas_rights_from_fen(fen_segment_list[FEN_CAS_RIGHTS_INDEX]);
        let enp_sqr = get_enp_sqr_from_fen(fen_segment_list[FEN_ENP_SQR_INDEX]);
        let non_cap_mov_count = fen_segment_list[FEN_HALF_MOV_INDEX].parse::<u16>().unwrap();

        State {
            squares: squares,
            player: player,
            cas_rights: cas_rights,
            enp_square: enp_sqr,
            non_cap_mov_count: non_cap_mov_count,
            hash_key: hash_key,

            wk_index: wk_index,
            bk_index: bk_index,

            bitboard: bitboard,
            bitboard_stack: Vec::new(),

            taken_piece_stack: Vec::new(),
            enp_sqr_stack: Vec::new(),
            cas_rights_stack: Vec::new(),
            history_mov_stack: Vec::new(),
            history_pos_stack: Vec::new(),
            non_cap_mov_count_stack: Vec::new(),
            wk_index_stack: Vec::new(),
            bk_index_stack: Vec::new(),

            zob_keys: zob_keys,
            bitmask: bitmask,
        }
    }

    pub fn is_draw(&self) -> bool {
        let history_len = self.history_pos_stack.len();
        let check_range = history_len.min(self.non_cap_mov_count as usize);

        if check_range < MIN_POS_COUNT_FOR_REP {
            return false
        }

        if self.history_pos_stack[history_len - LAST_DUP_POS_INDEX] == self.hash_key {
            return true
        }

        let mut dup_count = 0;
        for check_index in 1..=check_range {
            if self.history_pos_stack[history_len-check_index] == self.hash_key {
                dup_count += 1;
            }

            if dup_count > 1 {
                return true
            }
        }

        false
    }

    pub fn do_mov(&mut self, from: usize, to: usize, mov_type: u8, promo: u8) {
        self.cas_rights_stack.push(self.cas_rights);
        self.enp_sqr_stack.push(self.enp_square);
        self.history_mov_stack.push((to, self.squares[to]));
        self.history_pos_stack.push(self.hash_key);
        self.non_cap_mov_count_stack.push(self.non_cap_mov_count);
        self.wk_index_stack.push(self.wk_index);
        self.bk_index_stack.push(self.bk_index);
        self.bitboard_stack.push(self.bitboard);
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
        self.wk_index = self.wk_index_stack.pop().unwrap();
        self.bk_index = self.bk_index_stack.pop().unwrap();
        self.hash_key = self.history_pos_stack.pop().unwrap();
        self.bitboard = self.bitboard_stack.pop().unwrap();
        self.history_mov_stack.pop();

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

        self.hash_key ^= self.zob_keys[from][moving_piece as usize];
        self.hash_key ^= self.zob_keys[to][moving_piece as usize];

        if def::on_same_side(def::PLAYER_W, moving_piece) {
            self.bitboard.w_all ^= self.bitmask.index_masks[from];
            self.bitboard.w_all ^= self.bitmask.index_masks[to];

            if def::is_p(moving_piece) {
                self.bitboard.w_pawn ^= self.bitmask.index_masks[from];
                self.bitboard.w_pawn ^= self.bitmask.index_masks[to];
            } else if def::is_r(moving_piece) {
                self.bitboard.w_rook ^= self.bitmask.index_masks[from];
                self.bitboard.w_rook ^= self.bitmask.index_masks[to];
            }
        } else {
            self.bitboard.b_all ^= self.bitmask.index_masks[from];
            self.bitboard.b_all ^= self.bitmask.index_masks[to];

            if def::is_p(moving_piece) {
                self.bitboard.b_pawn ^= self.bitmask.index_masks[from];
                self.bitboard.b_pawn ^= self.bitmask.index_masks[to];
            } else if def::is_r(moving_piece) {
                self.bitboard.b_rook ^= self.bitmask.index_masks[from];
                self.bitboard.b_rook ^= self.bitmask.index_masks[to];
            }
        }

        if taken_piece == 0 {
            if def::is_p(moving_piece) {
                self.non_cap_mov_count = 0;
            } else {
                self.non_cap_mov_count += 1;
            }
        } else {
            self.non_cap_mov_count = 0;
            self.hash_key ^= self.zob_keys[to][taken_piece as usize];

            if def::on_same_side(def::PLAYER_W, taken_piece) {
                self.bitboard.w_all ^= self.bitmask.index_masks[to];
    
                if def::is_p(taken_piece) {
                    self.bitboard.w_pawn ^= self.bitmask.index_masks[to];
                } else if def::is_r(taken_piece) {
                    self.bitboard.w_rook ^= self.bitmask.index_masks[to];
                }
            } else {
                self.bitboard.b_all ^= self.bitmask.index_masks[to];
    
                if def::is_p(taken_piece) {
                    self.bitboard.b_pawn ^= self.bitmask.index_masks[to];
                } else if def::is_r(taken_piece) {
                    self.bitboard.b_rook ^= self.bitmask.index_masks[to];
                }
            }
        }

        self.taken_piece_stack.push(taken_piece);
        self.squares[to] = moving_piece;
        self.squares[from] = 0;

        if moving_piece == def::WR {
            if from == 0 {
                self.cas_rights &= 0b1011;
            } else if from == 7 {
                self.cas_rights &= 0b0111;
            }
        } else if moving_piece == def::BR {
            if from == 112 {
                self.cas_rights &= 0b1110;
            } else if from == 119 {
                self.cas_rights &= 0b1101;
            }
        } else if moving_piece == def::WK {
            if from == 4 {
                self.cas_rights &= 0b0011;
            }

            self.wk_index = to;
        } else if moving_piece == def::BK {
            if from == 116 {
                self.cas_rights &= 0b1100;
            }

            self.bk_index = to;
        }
    }

    fn undo_reg_mov(&mut self, from: usize, to: usize) {
        let moving_piece = self.squares[to];
        let taken_piece = self.taken_piece_stack.pop().unwrap();

        self.squares[to] = taken_piece;
        self.squares[from] = moving_piece;
    }

    fn do_promo_mov(&mut self, from: usize, to: usize, promo: u8) {
        let moving_piece = self.squares[from];
        let taken_piece = self.squares[to];

        self.hash_key ^= self.zob_keys[from][moving_piece as usize];
        self.hash_key ^= self.zob_keys[to][promo as usize];

        if def::on_same_side(def::PLAYER_W, moving_piece) {
            self.bitboard.w_all ^= self.bitmask.index_masks[from];
            self.bitboard.w_pawn ^= self.bitmask.index_masks[from];

            if def::is_r(promo) {
                self.bitboard.w_rook ^= self.bitmask.index_masks[to];
            }
        } else {
            self.bitboard.b_all ^= self.bitmask.index_masks[from];
            self.bitboard.b_pawn ^= self.bitmask.index_masks[from];

            if def::is_r(promo) {
                self.bitboard.b_rook ^= self.bitmask.index_masks[to];
            }
        }

        if taken_piece == 0 {
            self.non_cap_mov_count += 1;
        } else {
            self.non_cap_mov_count = 0;
            self.hash_key ^= self.zob_keys[to][taken_piece as usize];

            if def::on_same_side(def::PLAYER_W, taken_piece) {
                self.bitboard.w_all ^= self.bitmask.index_masks[to];
    
                if def::is_r(taken_piece) {
                    self.bitboard.w_rook ^= self.bitmask.index_masks[to];
                }
            } else {
                self.bitboard.b_all ^= self.bitmask.index_masks[to];
    
                if def::is_r(taken_piece) {
                    self.bitboard.b_rook ^= self.bitmask.index_masks[to];
                }
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
        let taken_piece = self.taken_piece_stack.pop().unwrap();

        self.squares[to] = taken_piece;
        self.squares[from] = moving_piece;
    }

    fn do_cas_mov(&mut self, to: usize) {
        self.non_cap_mov_count = 0;

        if to == def::CAS_SQUARE_WK {
            self.cas_rights &= 0b0111;
            self.wk_index = to;

            let k_index = def::CAS_SQUARE_WK-2;
            let r_index = def::CAS_SQUARE_WK+1;
            let r_to_index = def::CAS_SQUARE_WK-1;

            self.squares[k_index] = 0;
            self.squares[r_index] = 0;
            self.squares[r_to_index] = def::WR;
            self.squares[def::CAS_SQUARE_WK] = def::WK;

            self.hash_key ^= self.zob_keys[k_index][def::WK as usize];
            self.hash_key ^= self.zob_keys[r_index][def::WR as usize];
            self.hash_key ^= self.zob_keys[def::CAS_SQUARE_WK][def::WK as usize];
            self.hash_key ^= self.zob_keys[r_to_index][def::WR as usize];

            self.bitboard.w_all ^= self.bitmask.index_masks[k_index];
            self.bitboard.w_all ^= self.bitmask.index_masks[r_index];
            self.bitboard.w_all ^= self.bitmask.index_masks[def::CAS_SQUARE_WK];
            self.bitboard.w_all ^= self.bitmask.index_masks[r_to_index];
            self.bitboard.w_rook ^= self.bitmask.index_masks[r_index];
            self.bitboard.w_rook ^= self.bitmask.index_masks[r_to_index];
        } else if to == def::CAS_SQUARE_BK {
            self.cas_rights &= 0b1101;
            self.bk_index = to;

            let k_index = def::CAS_SQUARE_BK-2;
            let r_index = def::CAS_SQUARE_BK+1;
            let r_to_index = def::CAS_SQUARE_BK-1;

            self.squares[k_index] = 0;
            self.squares[r_index] = 0;
            self.squares[r_to_index] = def::BR;
            self.squares[def::CAS_SQUARE_BK] = def::BK;

            self.hash_key ^= self.zob_keys[k_index][def::BK as usize];
            self.hash_key ^= self.zob_keys[r_index][def::BR as usize];
            self.hash_key ^= self.zob_keys[def::CAS_SQUARE_BK][def::BK as usize];
            self.hash_key ^= self.zob_keys[r_to_index][def::BR as usize];

            self.bitboard.b_all ^= self.bitmask.index_masks[k_index];
            self.bitboard.b_all ^= self.bitmask.index_masks[r_index];
            self.bitboard.b_all ^= self.bitmask.index_masks[def::CAS_SQUARE_BK];
            self.bitboard.b_all ^= self.bitmask.index_masks[r_to_index];
            self.bitboard.b_rook ^= self.bitmask.index_masks[r_index];
            self.bitboard.b_rook ^= self.bitmask.index_masks[r_to_index];
        } else if to == def::CAS_SQUARE_WQ {
            self.cas_rights &= 0b1011;
            self.wk_index = to;

            let k_index = def::CAS_SQUARE_WQ+2;
            let r_index = def::CAS_SQUARE_WQ-2;
            let r_to_index = def::CAS_SQUARE_WQ+1;

            self.squares[k_index] = 0;
            self.squares[r_index] = 0;
            self.squares[r_to_index] = def::WR;
            self.squares[def::CAS_SQUARE_WQ] = def::WK;

            self.hash_key ^= self.zob_keys[k_index][def::WK as usize];
            self.hash_key ^= self.zob_keys[r_index][def::WR as usize];
            self.hash_key ^= self.zob_keys[def::CAS_SQUARE_WQ][def::WK as usize];
            self.hash_key ^= self.zob_keys[r_to_index][def::WR as usize];

            self.bitboard.w_all ^= self.bitmask.index_masks[k_index];
            self.bitboard.w_all ^= self.bitmask.index_masks[r_index];
            self.bitboard.w_all ^= self.bitmask.index_masks[def::CAS_SQUARE_WQ];
            self.bitboard.w_all ^= self.bitmask.index_masks[r_to_index];
            self.bitboard.w_rook ^= self.bitmask.index_masks[r_index];
            self.bitboard.w_rook ^= self.bitmask.index_masks[r_to_index];
        } else if to == def::CAS_SQUARE_BQ {
            self.cas_rights &= 0b1110;
            self.bk_index = to;

            let k_index = def::CAS_SQUARE_BQ+2;
            let r_index = def::CAS_SQUARE_BQ-2;
            let r_to_index = def::CAS_SQUARE_BQ+1;

            self.squares[k_index] = 0;
            self.squares[r_index] = 0;
            self.squares[r_to_index] = def::BR;
            self.squares[def::CAS_SQUARE_BQ] = def::BK;

            self.hash_key ^= self.zob_keys[k_index][def::BK as usize];
            self.hash_key ^= self.zob_keys[r_index][def::BR as usize];
            self.hash_key ^= self.zob_keys[def::CAS_SQUARE_BQ][def::BK as usize];
            self.hash_key ^= self.zob_keys[r_to_index][def::BR as usize];

            self.bitboard.b_all ^= self.bitmask.index_masks[k_index];
            self.bitboard.b_all ^= self.bitmask.index_masks[r_index];
            self.bitboard.b_all ^= self.bitmask.index_masks[def::CAS_SQUARE_BQ];
            self.bitboard.b_all ^= self.bitmask.index_masks[r_to_index];
            self.bitboard.b_rook ^= self.bitmask.index_masks[r_index];
            self.bitboard.b_rook ^= self.bitmask.index_masks[r_to_index];
        }
    }

    fn undo_cas_mov(&mut self, to: usize) {
        if to == def::CAS_SQUARE_WK {
            self.squares[def::CAS_SQUARE_WK-2] = def::WK;
            self.squares[def::CAS_SQUARE_WK+1] = def::WR;
            self.squares[def::CAS_SQUARE_WK-1] = 0;
            self.squares[def::CAS_SQUARE_WK] = 0;
        } else if to == def::CAS_SQUARE_BK {
            self.squares[def::CAS_SQUARE_BK-2] = def::BK;
            self.squares[def::CAS_SQUARE_BK+1] = def::BR;
            self.squares[def::CAS_SQUARE_BK-1] = 0;
            self.squares[def::CAS_SQUARE_BK] = 0;
        } else if to == def::CAS_SQUARE_WQ {
            self.squares[def::CAS_SQUARE_WQ+2] = def::WK;
            self.squares[def::CAS_SQUARE_WQ-2] = def::WR;
            self.squares[def::CAS_SQUARE_WQ+1] = 0;
            self.squares[def::CAS_SQUARE_WQ] = 0;
        } else if to == def::CAS_SQUARE_BQ {
            self.squares[def::CAS_SQUARE_BQ+2] = def::BK;
            self.squares[def::CAS_SQUARE_BQ-2] = def::BR;
            self.squares[def::CAS_SQUARE_BQ+1] = 0;
            self.squares[def::CAS_SQUARE_BQ] = 0;
        }
    }

    fn do_enp_mov(&mut self, from: usize, to: usize) {
        let taken_index = if self.player == def::PLAYER_W {
            to - 16
        } else {
            to + 16
        };

        self.non_cap_mov_count = 0;

        let moving_piece = self.squares[from];
        let taken_piece = self.squares[taken_index];

        self.hash_key ^= self.zob_keys[from][moving_piece as usize];
        self.hash_key ^= self.zob_keys[to][moving_piece as usize];
        self.hash_key ^= self.zob_keys[taken_index][taken_piece as usize];

        if def::on_same_side(def::PLAYER_W, moving_piece) {
            self.bitboard.w_all ^= self.bitmask.index_masks[from];
            self.bitboard.w_all ^= self.bitmask.index_masks[to];
            self.bitboard.w_pawn ^= self.bitmask.index_masks[from];
            self.bitboard.w_pawn ^= self.bitmask.index_masks[to];

            self.bitboard.b_all ^= self.bitmask.index_masks[taken_index];
            self.bitboard.b_pawn ^= self.bitmask.index_masks[taken_index];
        } else {
            self.bitboard.b_all ^= self.bitmask.index_masks[from];
            self.bitboard.b_all ^= self.bitmask.index_masks[to];
            self.bitboard.b_pawn ^= self.bitmask.index_masks[from];
            self.bitboard.b_pawn ^= self.bitmask.index_masks[to];

            self.bitboard.w_all ^= self.bitmask.index_masks[taken_index];
            self.bitboard.w_pawn ^= self.bitmask.index_masks[taken_index];
        }

        self.taken_piece_stack.push(taken_piece);
        self.squares[to] = moving_piece;
        self.squares[from] = 0;
        self.squares[taken_index] = 0;
    }

    fn undo_enp_mov(&mut self, from: usize, to: usize) {
        let taken_index = if self.player == def::PLAYER_W {
            to - 16
        } else {
            to + 16
        };

        let moving_piece = self.squares[to];
        let taken_piece = self.taken_piece_stack.pop().unwrap();

        self.squares[taken_index] = taken_piece;
        self.squares[from] = moving_piece;
        self.squares[to] = 0;
    }

    fn do_cr_enp_mov(&mut self, from: usize, to: usize) {
        self.enp_square = if self.player == def::PLAYER_W {
            to - 16
        } else {
            to + 16
        };

        self.non_cap_mov_count += 1;

        let moving_piece = self.squares[from];

        self.hash_key ^= self.zob_keys[from][moving_piece as usize];
        self.hash_key ^= self.zob_keys[to][moving_piece as usize];

        if def::on_same_side(def::PLAYER_W, moving_piece) {
            self.bitboard.w_all ^= self.bitmask.index_masks[from];
            self.bitboard.w_all ^= self.bitmask.index_masks[to];
            self.bitboard.w_pawn ^= self.bitmask.index_masks[from];
            self.bitboard.w_pawn ^= self.bitmask.index_masks[to];
        } else {
            self.bitboard.b_all ^= self.bitmask.index_masks[from];
            self.bitboard.b_all ^= self.bitmask.index_masks[to];
            self.bitboard.b_pawn ^= self.bitmask.index_masks[from];
            self.bitboard.b_pawn ^= self.bitmask.index_masks[to];
        }

        self.squares[to] = moving_piece;
        self.squares[from] = 0;
    }

    fn undo_cr_enp_mov(&mut self, from: usize, to: usize) {
        let moving_piece = self.squares[to];

        self.squares[from] = moving_piece;
        self.squares[to] = 0;
    }
}

impl <'state> fmt::Display for State <'state> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut display_string = String::new();

        let mut rank_left_index = 112;
        loop {
            for file_index in 0..def::DIM_SIZE {
                display_string.push(util::map_piece_code_to_char(self.squares[rank_left_index + file_index]));
            }

            display_string.push('\n');

            if rank_left_index == 0 {
                break
            }

            rank_left_index -= 16;
        }

        write!(formatter, "{}", display_string)
    }
}

fn get_board_info_from_fen(fen_squares_string: &str, zob_keys: &Vec<Vec<u64>>, bitmask: &BitMask) -> ([u8; def::BOARD_SIZE], u64, usize, usize, BitBoard) {
    let mut squares = [0; def::BOARD_SIZE];
    let mut hash_key = 0;
    let mut wk_index = 0;
    let mut bk_index = 0;
    let mut bitboard = BitBoard {
        w_all: 0,
        w_rook: 0,
        w_pawn: 0,
        b_all: 0,
        b_rook: 0,
        b_pawn: 0,
    };

    let rank_string_list: Vec<&str> = fen_squares_string.split("/").collect();
    assert_eq!(def::DIM_SIZE, rank_string_list.len());

    let mut index = 112;
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
                hash_key ^= zob_keys[index][piece as usize];

                if piece == def::WP {
                    bitboard.w_pawn ^= bitmask.index_masks[index];
                } else if piece == def::BP {
                    bitboard.b_pawn ^= bitmask.index_masks[index];
                } else if piece == def::WR {
                    bitboard.w_rook ^= bitmask.index_masks[index];
                } else if piece == def::BR {
                    bitboard.b_rook ^= bitmask.index_masks[index];
                } else if piece == def::WK {
                    wk_index = index;
                } else if piece == def::BK {
                    bk_index = index;
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

        index -= 24;
    }

    (squares, hash_key, wk_index, bk_index, bitboard)
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
        bitboard::BitMask,
        def,
        prng::XorshiftPrng,
    };

    #[test]
    fn test_new_startpos() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &zob_keys, &bitmask);

        assert_eq!(0b1111, state.cas_rights);
        assert_eq!(0, state.enp_square);
        assert_eq!(def::PLAYER_W, state.player);
    }

    #[test]
    fn test_do_move_1() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &zob_keys, &bitmask);
        assert_eq!(0b1111, state.cas_rights);
        assert_eq!(0, state.enp_square);
        assert_eq!(def::PLAYER_W, state.player);

        state.do_mov(util::map_sqr_notation_to_index("e2"), util::map_sqr_notation_to_index("e4"), def::MOV_CR_ENP, 0);
        assert_eq!(0b1111, state.cas_rights);
        assert_eq!(util::map_sqr_notation_to_index("e3"), state.enp_square);
        assert_eq!(def::PLAYER_B, state.player);

        state.undo_mov(util::map_sqr_notation_to_index("e2"), util::map_sqr_notation_to_index("e4"), def::MOV_CR_ENP);
        assert_eq!(0b1111, state.cas_rights);
        assert_eq!(0, state.enp_square);
        assert_eq!(def::PLAYER_W, state.player);
    }

    #[test]
    fn test_do_move_2() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("r1bqk1nr/pPpp1ppp/2n5/2b1p3/2B1P3/2N2N2/P1PP1PPP/R1BQK2R w KQkq - 0 1", &zob_keys, &bitmask);
        assert_eq!(0b1111, state.cas_rights);
        assert_eq!(0, state.enp_square);
        assert_eq!(def::PLAYER_W, state.player);
        assert_eq!(def::BR, state.squares[util::map_sqr_notation_to_index("a8")]);

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
    }

    #[test]
    fn test_do_move_3() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("r3k2r/pbppnppp/1bn2q2/4p3/2B5/2N1PN2/PPPP1PPP/R1BQK2R b Qkq - 0 1", &zob_keys, &bitmask);
        assert_eq!(0b0111, state.cas_rights);
        assert_eq!(0, state.enp_square);
        assert_eq!(def::PLAYER_B, state.player);
        assert_eq!(def::BK, state.squares[util::map_sqr_notation_to_index("e8")]);
        assert_eq!(def::BR, state.squares[util::map_sqr_notation_to_index("a8")]);
        assert_eq!(0, state.squares[util::map_sqr_notation_to_index("c8")]);

        state.do_mov(util::map_sqr_notation_to_index("e8"), util::map_sqr_notation_to_index("c8"), def::MOV_CAS, 0);
        assert_eq!(0b0110, state.cas_rights);
        assert_eq!(0, state.enp_square);
        assert_eq!(def::PLAYER_W, state.player);
        assert_eq!(0, state.squares[util::map_sqr_notation_to_index("e8")]);
        assert_eq!(0, state.squares[util::map_sqr_notation_to_index("a8")]);
        assert_eq!(def::BK, state.squares[util::map_sqr_notation_to_index("c8")]);
        assert_eq!(def::BR, state.squares[util::map_sqr_notation_to_index("d8")]);

        state.undo_mov(util::map_sqr_notation_to_index("e8"), util::map_sqr_notation_to_index("c8"), def::MOV_CAS);
        assert_eq!(0b0111, state.cas_rights);
        assert_eq!(0, state.enp_square);
        assert_eq!(def::PLAYER_B, state.player);
        assert_eq!(def::BK, state.squares[util::map_sqr_notation_to_index("e8")]);
        assert_eq!(def::BR, state.squares[util::map_sqr_notation_to_index("a8")]);
        assert_eq!(0, state.squares[util::map_sqr_notation_to_index("c8")]);
        assert_eq!(0, state.squares[util::map_sqr_notation_to_index("d8")]);
    }

    #[test]
    fn test_do_move_4() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("4r1k1/pp1Q1ppp/3B4/q2p4/5P1P/P3PbPK/1P1r4/2R5 b - - 3 5", &zob_keys, &bitmask);
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
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("r1bqkbnr/ppp1p1pp/2n5/3pPp2/3P4/8/PPP2PPP/RNBQKBNR w KQkq f6 0 1", &zob_keys, &bitmask);
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
    fn test_zob_hash_1() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 1", &zob_keys, &bitmask);
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
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("r3kb1r/ppp2ppp/2np1n2/4p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R b KQkq - 0 1", &zob_keys, &bitmask);
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
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let mut state = State::new("r3kb1r/ppp2ppp/2np1n2/4p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R w KQkq - 0 1", &zob_keys, &bitmask);

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
}
