use crate::{
    def,
    state::State,
    util,
};

static CAS_WK_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01100000;
static CAS_WQ_MASK: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001110;
static CAS_BK_MASK: u64 = 0b01100000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
static CAS_BQ_MASK: u64 = 0b00001110_00000000_00000000_00000000_00000000_00000000_00000000_00000000;

const N_MOVS: [isize; 8] = [14, 18, 31, 33, -14, -18, -31, -33];
const K_MOVS: [isize; 8] = [1, 15, 16, 17, -1, -15, -16, -17];

const VERTICAL_SLIDE_MOVS: [isize; 7] = [16, 32, 48, 64, 80, 96, 112];
const HORIZONTAL_SLIDE_MOVS: [isize; 7] = [1, 2, 3, 4, 5, 6, 7];

const DESC_DIAGNOL_SLIDE_MOVS: [isize; 7] = [15, 30, 45, 60, 75, 90, 105];
const ASC_DIAGNOL_SLIDE_MOVS: [isize; 7] = [17, 34, 51, 68, 85, 102, 119];

fn gen_n_mov_table() -> Vec<Vec<usize>> {
    let mut mov_table = vec![vec![]; def::BOARD_SIZE];

    let mut from_index = 0;

    while from_index < def::BOARD_SIZE {
        if !def::is_index_valid(from_index) {
            from_index += 8;
        }

        let mut mov_list_on_index = Vec::new();

        for mov_index in 0..8 {
            let mov = N_MOVS[mov_index];
            let to_index = from_index as isize + mov;
            if to_index < 0 {
                continue
            }

            let to_index = to_index as usize;

            if def::is_index_valid(to_index) {
                mov_list_on_index.push(to_index);
            }
        }

        mov_table[from_index] = mov_list_on_index;
        from_index += 1;
    }

    mov_table
}

fn gen_k_mov_table() -> Vec<Vec<usize>> {
    let mut mov_table = vec![vec![]; def::BOARD_SIZE];

    let mut from_index = 0;

    while from_index < def::BOARD_SIZE {
        if !def::is_index_valid(from_index) {
            from_index += 8;
        }

        let mut mov_list_on_index = Vec::new();

        for mov_index in 0..8 {
            let mov = K_MOVS[mov_index];
            let to_index = from_index as isize + mov;
            if to_index < 0 {
                continue
            }

            let to_index = to_index as usize;

            if def::is_index_valid(to_index) {
                mov_list_on_index.push(to_index);
            }
        }

        mov_table[from_index] = mov_list_on_index;
        from_index += 1;
    }

    mov_table
}

fn gen_up_slide_mov_table() -> Vec<Vec<usize>> {
    let mut mov_table = vec![vec![]; def::BOARD_SIZE];

    let mut from_index = 0;

    while from_index < def::BOARD_SIZE {
        if !def::is_index_valid(from_index) {
            from_index += 8;
        }

        let mut mov_list_on_index = Vec::new();

        for mov_index in 0..7 {
            let mov = VERTICAL_SLIDE_MOVS[mov_index];
            let to_index = from_index as isize + mov;

            let to_index = to_index as usize;

            if !def::is_index_valid(to_index) {
                break
            }

            mov_list_on_index.push(to_index);
        }

        mov_table[from_index] = mov_list_on_index;
        from_index += 1;
    }

    mov_table
}

fn gen_down_slide_mov_table() -> Vec<Vec<usize>> {
    let mut mov_table = vec![vec![]; def::BOARD_SIZE];

    let mut from_index = 0;

    while from_index < def::BOARD_SIZE {
        if !def::is_index_valid(from_index) {
            from_index += 8;
        }

        let mut mov_list_on_index = Vec::new();

        for mov_index in 0..7 {
            let mov = VERTICAL_SLIDE_MOVS[mov_index];
            let to_index = from_index as isize - mov;

            if to_index < 0 {
                break
            }

            let to_index = to_index as usize;

            if !def::is_index_valid(to_index) {
                break
            }

            mov_list_on_index.push(to_index);
        }

        mov_table[from_index] = mov_list_on_index;
        from_index += 1;
    }

    mov_table
}

fn gen_right_slide_mov_table() -> Vec<Vec<usize>> {
    let mut mov_table = vec![vec![]; def::BOARD_SIZE];

    let mut from_index = 0;

    while from_index < def::BOARD_SIZE {
        if !def::is_index_valid(from_index) {
            from_index += 8;
        }

        let mut mov_list_on_index = Vec::new();

        for mov_index in 0..7 {
            let mov = HORIZONTAL_SLIDE_MOVS[mov_index];
            let to_index = from_index as isize + mov;

            let to_index = to_index as usize;

            if !def::is_index_valid(to_index) {
                break
            }

            mov_list_on_index.push(to_index);
        }

        mov_table[from_index] = mov_list_on_index;
        from_index += 1;
    }

    mov_table
}

fn gen_left_slide_mov_table() -> Vec<Vec<usize>> {
    let mut mov_table = vec![vec![]; def::BOARD_SIZE];

    let mut from_index = 0;

    while from_index < def::BOARD_SIZE {
        if !def::is_index_valid(from_index) {
            from_index += 8;
        }

        let mut mov_list_on_index = Vec::new();

        for mov_index in 0..7 {
            let mov = HORIZONTAL_SLIDE_MOVS[mov_index];
            let to_index = from_index as isize - mov;

            if to_index < 0 {
                break
            }

            let to_index = to_index as usize;

            if !def::is_index_valid(to_index) {
                break
            }

            mov_list_on_index.push(to_index);
        }

        mov_table[from_index] = mov_list_on_index;
        from_index += 1;
    }

    mov_table
}

fn gen_up_left_slide_mov_table() -> Vec<Vec<usize>> {
    let mut mov_table = vec![vec![]; def::BOARD_SIZE];

    let mut from_index = 0;

    while from_index < def::BOARD_SIZE {
        if !def::is_index_valid(from_index) {
            from_index += 8;
        }

        let mut mov_list_on_index = Vec::new();

        for mov_index in 0..7 {
            let mov = DESC_DIAGNOL_SLIDE_MOVS[mov_index];
            let to_index = from_index as isize + mov;

            let to_index = to_index as usize;

            if !def::is_index_valid(to_index) {
                break
            }

            mov_list_on_index.push(to_index);
        }

        mov_table[from_index] = mov_list_on_index;
        from_index += 1;
    }

    mov_table
}

fn gen_down_right_slide_mov_table() -> Vec<Vec<usize>> {
    let mut mov_table = vec![vec![]; def::BOARD_SIZE];

    let mut from_index = 0;

    while from_index < def::BOARD_SIZE {
        if !def::is_index_valid(from_index) {
            from_index += 8;
        }

        let mut mov_list_on_index = Vec::new();

        for mov_index in 0..7 {
            let mov = DESC_DIAGNOL_SLIDE_MOVS[mov_index];
            let to_index = from_index as isize - mov;

            if to_index < 0 {
                break
            }

            let to_index = to_index as usize;

            if !def::is_index_valid(to_index) {
                break
            }

            mov_list_on_index.push(to_index);
        }

        mov_table[from_index] = mov_list_on_index;
        from_index += 1;
    }

    mov_table
}

fn gen_up_right_slide_mov_table() -> Vec<Vec<usize>> {
    let mut mov_table = vec![vec![]; def::BOARD_SIZE];

    let mut from_index = 0;

    while from_index < def::BOARD_SIZE {
        if !def::is_index_valid(from_index) {
            from_index += 8;
        }

        let mut mov_list_on_index = Vec::new();

        for mov_index in 0..7 {
            let mov = ASC_DIAGNOL_SLIDE_MOVS[mov_index];
            let to_index = from_index as isize + mov;

            let to_index = to_index as usize;

            if !def::is_index_valid(to_index) {
                break
            }

            mov_list_on_index.push(to_index);
        }

        mov_table[from_index] = mov_list_on_index;
        from_index += 1;
    }

    mov_table
}

fn gen_down_left_slide_mov_table() -> Vec<Vec<usize>> {
    let mut mov_table = vec![vec![]; def::BOARD_SIZE];

    let mut from_index = 0;

    while from_index < def::BOARD_SIZE {
        if !def::is_index_valid(from_index) {
            from_index += 8;
        }

        let mut mov_list_on_index = Vec::new();

        for mov_index in 0..7 {
            let mov = ASC_DIAGNOL_SLIDE_MOVS[mov_index];
            let to_index = from_index as isize - mov;

            if to_index < 0 {
                break
            }

            let to_index = to_index as usize;

            if !def::is_index_valid(to_index) {
                break
            }

            mov_list_on_index.push(to_index);
        }

        mov_table[from_index] = mov_list_on_index;
        from_index += 1;
    }

    mov_table
}

pub struct MoveTable {
    k_mov_table: Vec<Vec<usize>>,
    n_mov_table: Vec<Vec<usize>>,

    up_mov_table: Vec<Vec<usize>>,
    down_mov_table: Vec<Vec<usize>>,
    left_mov_table: Vec<Vec<usize>>,
    right_mov_table: Vec<Vec<usize>>,

    up_left_mov_table: Vec<Vec<usize>>,
    up_right_mov_table: Vec<Vec<usize>>,
    down_right_mov_table: Vec<Vec<usize>>,
    down_left_mov_table: Vec<Vec<usize>>,
}

impl MoveTable {
    pub fn new() -> Self {
        MoveTable {
            k_mov_table: gen_k_mov_table(),
            n_mov_table: gen_n_mov_table(),

            up_mov_table: gen_up_slide_mov_table(),
            down_mov_table: gen_down_slide_mov_table(),
            left_mov_table: gen_left_slide_mov_table(),
            right_mov_table: gen_right_slide_mov_table(),

            up_left_mov_table: gen_up_left_slide_mov_table(),
            up_right_mov_table: gen_up_right_slide_mov_table(),
            down_right_mov_table: gen_down_right_slide_mov_table(),
            down_left_mov_table: gen_down_left_slide_mov_table(),
        }
    }

    pub fn gen_castle_mov_list(&self, state: &State) -> Vec<u32> {
        let cas_rights = state.cas_rights;
        let squares = state.squares;
        let all_mask = state.bitboard.w_all | state.bitboard.b_all;

        let mut mov_list = Vec::new();

        if state.player == def::PLAYER_W {
            if cas_rights & 0b1000 != 0 {
                if squares[def::CAS_SQUARE_WK - 2] == def::WK
                && squares[def::CAS_SQUARE_WK + 1] == def::WR
                && all_mask & CAS_WK_MASK == 0
                && !self.is_under_attack(state, def::CAS_SQUARE_WK, def::PLAYER_W)
                && !self.is_under_attack(state, def::CAS_SQUARE_WK - 1, def::PLAYER_W)
                && !self.is_under_attack(state, def::CAS_SQUARE_WK - 2, def::PLAYER_W) {
                    mov_list.push(util::encode_u32_mov(def::CAS_SQUARE_WK - 2, def::CAS_SQUARE_WK, def::MOV_CAS, 0));
                }
            }

            if cas_rights & 0b0100 != 0 {
                if squares[def::CAS_SQUARE_WQ + 2] == def::WK
                && squares[def::CAS_SQUARE_WQ - 2] == def::WR
                && all_mask & CAS_WQ_MASK == 0
                && !self.is_under_attack(state, def::CAS_SQUARE_WQ, def::PLAYER_W)
                && !self.is_under_attack(state, def::CAS_SQUARE_WQ + 1, def::PLAYER_W)
                && !self.is_under_attack(state, def::CAS_SQUARE_WQ + 2, def::PLAYER_W) {
                    mov_list.push(util::encode_u32_mov(def::CAS_SQUARE_WQ + 2, def::CAS_SQUARE_WQ, def::MOV_CAS, 0));
                }
            }
        } else {
            if cas_rights & 0b0010 != 0 {
                if squares[def::CAS_SQUARE_BK - 2] == def::BK
                && squares[def::CAS_SQUARE_BK + 1] == def::BR
                && all_mask & CAS_BK_MASK == 0
                && !self.is_under_attack(state, def::CAS_SQUARE_BK, def::PLAYER_B)
                && !self.is_under_attack(state, def::CAS_SQUARE_BK - 1, def::PLAYER_B)
                && !self.is_under_attack(state, def::CAS_SQUARE_BK - 2, def::PLAYER_B) {
                    mov_list.push(util::encode_u32_mov(def::CAS_SQUARE_BK - 2, def::CAS_SQUARE_BK, def::MOV_CAS, 0));
                }
            }

            if cas_rights & 0b0001 != 0 {
                if squares[def::CAS_SQUARE_BQ + 2] == def::BK
                && squares[def::CAS_SQUARE_BQ - 2] == def::BR
                && all_mask & CAS_BQ_MASK == 0
                && !self.is_under_attack(state, def::CAS_SQUARE_BQ, def::PLAYER_B)
                && !self.is_under_attack(state, def::CAS_SQUARE_BQ + 1, def::PLAYER_B)
                && !self.is_under_attack(state, def::CAS_SQUARE_BQ + 2, def::PLAYER_B) {
                    mov_list.push(util::encode_u32_mov(def::CAS_SQUARE_BQ + 2, def::CAS_SQUARE_BQ, def::MOV_CAS, 0));
                }
            }
        }

        mov_list
    }

    pub fn gen_reg_mov_list(&self, state: &State) -> (Vec<u32>, Vec<u32>) {
        let squares = state.squares;
        let player = state.player;

        let mut mov_list = Vec::new();
        let mut cap_list = Vec::new();

        let mut add_mov = |from: usize, to: usize, tp: u8, promo: u8| {
            mov_list.push(util::encode_u32_mov(from, to, tp, promo));
        };

        let mut add_cap = |from: usize, to: usize, tp: u8, promo: u8| {
            cap_list.push(util::encode_u32_mov(from, to, tp, promo));
        };

        let mut from_index = 0;

        while from_index < def::BOARD_SIZE {
            if !def::is_index_valid(from_index) {
                from_index += 8;
            }

            let moving_piece = squares[from_index];

            if moving_piece == 0 || !def::on_same_side(player, moving_piece) {
                from_index += 1;
                continue
            }

            if def::is_p(moving_piece) {
                if player == def::PLAYER_W {
                    let to_index = from_index + 16;

                    if def::is_index_valid(to_index) && squares[to_index] == 0 {
                        if to_index > 111 {
                            add_mov(from_index, to_index, def::MOV_PROMO, def::WQ);
                            add_mov(from_index, to_index, def::MOV_PROMO, def::WR);
                            add_mov(from_index, to_index, def::MOV_PROMO, def::WB);
                            add_mov(from_index, to_index, def::MOV_PROMO, def::WN);
                        } else {
                            add_mov(from_index, to_index, def::MOV_REG, 0);

                            if from_index < 24 {
                                let to_index = from_index + 32;
                                if def::is_index_valid(to_index) && squares[to_index] == 0 {
                                    add_mov(from_index, to_index, def::MOV_CR_ENP, 0);
                                }
                            }
                        }
                    }

                    let take_index = from_index + 15;
                    if def::is_index_valid(take_index) {
                        let take = squares[take_index];
                        if take != 0 && !def::on_same_side(player, take) {
                            if take_index > 111 {
                                add_cap(from_index, take_index, def::MOV_PROMO, def::WQ);
                                add_cap(from_index, take_index, def::MOV_PROMO, def::WR);
                                add_cap(from_index, take_index, def::MOV_PROMO, def::WB);
                                add_cap(from_index, take_index, def::MOV_PROMO, def::WN);
                            } else {
                                add_cap(from_index, take_index, def::MOV_REG, 0);
                            }
                        }

                        if take_index == state.enp_square {
                            add_cap(from_index, take_index, def::MOV_ENP, 0);
                        }
                    }

                    let take_index = from_index + 17;
                    if def::is_index_valid(take_index) {
                        let take = squares[take_index];
                        if take != 0 && !def::on_same_side(player, take) {
                            if take_index > 111 {
                                add_cap(from_index, take_index, def::MOV_PROMO, def::WQ);
                                add_cap(from_index, take_index, def::MOV_PROMO, def::WR);
                                add_cap(from_index, take_index, def::MOV_PROMO, def::WB);
                                add_cap(from_index, take_index, def::MOV_PROMO, def::WN);
                            } else {
                                add_cap(from_index, take_index, def::MOV_REG, 0);
                            }
                        }
                        if take_index == state.enp_square {
                            add_cap(from_index, take_index, def::MOV_ENP, 0);
                        }
                    }
                } else {
                    let to_index = from_index as isize - 16;
                    if to_index >= 0 {
                        let to_index = to_index as usize;

                        if def::is_index_valid(to_index) && squares[to_index] == 0 {
                            if to_index < 8 {
                                add_mov(from_index, to_index, def::MOV_PROMO, def::BQ);
                                add_mov(from_index, to_index, def::MOV_PROMO, def::BR);
                                add_mov(from_index, to_index, def::MOV_PROMO, def::BB);
                                add_mov(from_index, to_index, def::MOV_PROMO, def::BN);
                            } else {
                                add_mov(from_index, to_index, def::MOV_REG, 0);

                                if from_index > 95 {
                                    let to_index = from_index - 32;
                                    if def::is_index_valid(to_index) && squares[to_index] == 0 {
                                        add_mov(from_index, to_index, def::MOV_CR_ENP, 0);
                                    }
                                }
                            }
                        }

                        if from_index >= 15 {
                            let take_index = from_index - 15;
                            if def::is_index_valid(take_index) {
                                let take = squares[take_index];
                                if take != 0 && !def::on_same_side(player, take) {
                                    if take_index < 8 {
                                        add_cap(from_index, take_index, def::MOV_PROMO, def::BQ);
                                        add_cap(from_index, take_index, def::MOV_PROMO, def::BR);
                                        add_cap(from_index, take_index, def::MOV_PROMO, def::BB);
                                        add_cap(from_index, take_index, def::MOV_PROMO, def::BN);
                                    } else {
                                        add_cap(from_index, take_index, def::MOV_REG, 0);
                                    }
                                }

                                if take_index == state.enp_square && take_index != 0 {
                                    add_cap(from_index, take_index, def::MOV_ENP, 0);
                                }
                            }
                        }

                        if from_index >= 17 {
                            let take_index = from_index - 17;
                            if def::is_index_valid(take_index) {
                                let take = squares[take_index];
                                if take != 0 && !def::on_same_side(player, take) {
                                    if take_index < 8 {
                                        add_cap(from_index, take_index, def::MOV_PROMO, def::BQ);
                                        add_cap(from_index, take_index, def::MOV_PROMO, def::BR);
                                        add_cap(from_index, take_index, def::MOV_PROMO, def::BB);
                                        add_cap(from_index, take_index, def::MOV_PROMO, def::BN);
                                    } else {
                                        add_cap(from_index, take_index, def::MOV_REG, 0);
                                    }
                                }

                                if take_index == state.enp_square && take_index != 0 {
                                    add_cap(from_index, take_index, def::MOV_ENP, 0);
                                }
                            }
                        }
                    }
                }
            } else if def::is_n(moving_piece) {
                let mov_index_list = &self.n_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        add_mov(from_index, to_index, def::MOV_REG, 0);
                    } else if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }
                }
            } else if def::is_b(moving_piece) {
                let mov_index_list = &self.up_left_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        add_mov(from_index, to_index, def::MOV_REG, 0);
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.up_right_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        add_mov(from_index, to_index, def::MOV_REG, 0);
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.down_right_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        add_mov(from_index, to_index, def::MOV_REG, 0);
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.down_left_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        add_mov(from_index, to_index, def::MOV_REG, 0);
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }
            } else if def::is_r(moving_piece) {
                let mov_index_list = &self.up_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        add_mov(from_index, to_index, def::MOV_REG, 0);
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.right_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        add_mov(from_index, to_index, def::MOV_REG, 0);
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.down_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        add_mov(from_index, to_index, def::MOV_REG, 0);
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.left_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        add_mov(from_index, to_index, def::MOV_REG, 0);
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }
            } else if def::is_q(moving_piece) {
                let mov_index_list = &self.up_left_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        add_mov(from_index, to_index, def::MOV_REG, 0);
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.up_right_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        add_mov(from_index, to_index, def::MOV_REG, 0);
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.down_right_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        add_mov(from_index, to_index, def::MOV_REG, 0);
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.down_left_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        add_mov(from_index, to_index, def::MOV_REG, 0);
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.up_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        add_mov(from_index, to_index, def::MOV_REG, 0);
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.right_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        add_mov(from_index, to_index, def::MOV_REG, 0);
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.down_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        add_mov(from_index, to_index, def::MOV_REG, 0);
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.left_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        add_mov(from_index, to_index, def::MOV_REG, 0);
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }
            } else if def::is_k(moving_piece) {
                let mov_index_list = &self.k_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        add_mov(from_index, to_index, def::MOV_REG, 0);
                    } else if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }
                }
            }

            from_index += 1;
        }

        (cap_list, mov_list)
    }

    pub fn gen_capture_list(&self, state: &State) -> Vec<u32> {
        let squares = state.squares;
        let player = state.player;

        let mut cap_list = Vec::new();

        let mut add_cap = |from: usize, to: usize, tp: u8, promo: u8| {
            cap_list.push(util::encode_u32_mov(from, to, tp, promo));
        };

        let mut from_index = 0;

        while from_index < def::BOARD_SIZE {
            if !def::is_index_valid(from_index) {
                from_index += 8;
            }

            let moving_piece = squares[from_index];

            if moving_piece == 0 || !def::on_same_side(player, moving_piece) {
                from_index += 1;
                continue
            }

            if def::is_p(moving_piece) {
                if player == def::PLAYER_W {
                    let take_index = from_index + 15;
                    if def::is_index_valid(take_index) {
                        let take = squares[take_index];
                        if take != 0 && !def::on_same_side(player, take) {
                            if take_index > 111 {
                                add_cap(from_index, take_index, def::MOV_PROMO, def::WQ);
                                add_cap(from_index, take_index, def::MOV_PROMO, def::WR);
                                add_cap(from_index, take_index, def::MOV_PROMO, def::WB);
                                add_cap(from_index, take_index, def::MOV_PROMO, def::WN);
                            } else {
                                add_cap(from_index, take_index, def::MOV_REG, 0);
                            }
                        }

                        if take_index == state.enp_square {
                            add_cap(from_index, take_index, def::MOV_ENP, 0);
                        }
                    }

                    let take_index = from_index + 17;
                    if def::is_index_valid(take_index) {
                        let take = squares[take_index];
                        if take != 0 && !def::on_same_side(player, take) {
                            if take_index > 111 {
                                add_cap(from_index, take_index, def::MOV_PROMO, def::WQ);
                                add_cap(from_index, take_index, def::MOV_PROMO, def::WR);
                                add_cap(from_index, take_index, def::MOV_PROMO, def::WB);
                                add_cap(from_index, take_index, def::MOV_PROMO, def::WN);
                            } else {
                                add_cap(from_index, take_index, def::MOV_REG, 0);
                            }
                        }
                        if take_index == state.enp_square {
                            add_cap(from_index, take_index, def::MOV_ENP, 0);
                        }
                    }
                } else {
                    let to_index = from_index as isize - 16;
                    if to_index >= 0 {
                        if from_index >= 15 {
                            let take_index = from_index - 15;
                            if def::is_index_valid(take_index) {
                                let take = squares[take_index];
                                if take != 0 && !def::on_same_side(player, take) {
                                    if take_index < 8 {
                                        add_cap(from_index, take_index, def::MOV_PROMO, def::BQ);
                                        add_cap(from_index, take_index, def::MOV_PROMO, def::BR);
                                        add_cap(from_index, take_index, def::MOV_PROMO, def::BB);
                                        add_cap(from_index, take_index, def::MOV_PROMO, def::BN);
                                    } else {
                                        add_cap(from_index, take_index, def::MOV_REG, 0);
                                    }
                                }

                                if take_index == state.enp_square && take_index != 0 {
                                    add_cap(from_index, take_index, def::MOV_ENP, 0);
                                }
                            }
                        }

                        if from_index >= 17 {
                            let take_index = from_index - 17;
                            if def::is_index_valid(take_index) {
                                let take = squares[take_index];
                                if take != 0 && !def::on_same_side(player, take) {
                                    if take_index < 8 {
                                        add_cap(from_index, take_index, def::MOV_PROMO, def::BQ);
                                        add_cap(from_index, take_index, def::MOV_PROMO, def::BR);
                                        add_cap(from_index, take_index, def::MOV_PROMO, def::BB);
                                        add_cap(from_index, take_index, def::MOV_PROMO, def::BN);
                                    } else {
                                        add_cap(from_index, take_index, def::MOV_REG, 0);
                                    }
                                }

                                if take_index == state.enp_square && take_index != 0 {
                                    add_cap(from_index, take_index, def::MOV_ENP, 0);
                                }
                            }
                        }
                    }
                }
            } else if def::is_n(moving_piece) {
                let mov_index_list = &self.n_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }
                }
            } else if def::is_b(moving_piece) {
                let mov_index_list = &self.up_left_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.up_right_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.down_right_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.down_left_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }
            } else if def::is_r(moving_piece) {
                let mov_index_list = &self.up_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.right_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.down_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.left_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }
            } else if def::is_q(moving_piece) {
                let mov_index_list = &self.up_left_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.up_right_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.down_right_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.down_left_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.up_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.right_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.down_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }

                let mov_index_list = &self.left_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }

                    break
                }
            } else if def::is_k(moving_piece) {
                let mov_index_list = &self.k_mov_table[from_index];
                for to_index in mov_index_list {
                    let to_index = *to_index;
                    let taken_piece = squares[to_index];

                    if taken_piece == 0 {
                        continue
                    }

                    if !def::on_same_side(player, taken_piece) {
                        add_cap(from_index, to_index, def::MOV_REG, 0);
                    }
                }
            }

            from_index += 1;
        }

        cap_list
    }

    pub fn is_in_check(&self, state: &State) -> bool {
        let k_index = if state.player == def::PLAYER_W {
            state.wk_index
        } else {
            state.bk_index
        };

        self.is_under_attack(state, k_index, state.player)
    }

    pub fn is_under_attack(&self, state: &State, index: usize, player: u8) -> bool {
        let squares = state.squares;

        let opponent_n_count = if player == def::PLAYER_W {
            state.b_piece_list.knight
        } else {
            state.w_piece_list.knight
        };

        if opponent_n_count > 0 {
            let mov_index_list = &self.n_mov_table[index];
            for to_index in mov_index_list {
                let taken_piece = squares[*to_index];

                if taken_piece != 0 {
                    if !def::on_same_side(player, taken_piece) && def::is_n(taken_piece) {
                        return true
                    }
                }
            }
        }

        let opponent_bq_count = if player == def::PLAYER_W {
            state.b_piece_list.queen + state.b_piece_list.bishop
        } else {
            state.w_piece_list.queen + state.w_piece_list.bishop
        };

        let opponent_rq_count = if player == def::PLAYER_W {
            state.b_piece_list.queen + state.b_piece_list.rook
        } else {
            state.w_piece_list.queen + state.w_piece_list.rook
        };

        if opponent_rq_count > 0 {
            let mov_index_list = &self.up_mov_table[index];
            for to_index in mov_index_list {
                let taken_piece = squares[*to_index];

                if taken_piece != 0 {
                    if def::on_same_side(player, taken_piece) {
                        break
                    }

                    if def::is_r(taken_piece) || def::is_q(taken_piece) {
                        return true
                    }

                    break
                }
            }

            let mov_index_list = &self.down_mov_table[index];
            for to_index in mov_index_list {
                let taken_piece = squares[*to_index];

                if taken_piece != 0 {
                    if def::on_same_side(player, taken_piece) {
                        break
                    }

                    if def::is_r(taken_piece) || def::is_q(taken_piece) {
                        return true
                    }

                    break
                }
            }

            let mov_index_list = &self.left_mov_table[index];
            for to_index in mov_index_list {
                let taken_piece = squares[*to_index];

                if taken_piece != 0 {
                    if def::on_same_side(player, taken_piece) {
                        break
                    }

                    if def::is_r(taken_piece) || def::is_q(taken_piece) {
                        return true
                    }

                    break
                }
            }

            let mov_index_list = &self.right_mov_table[index];
            for to_index in mov_index_list {
                let taken_piece = squares[*to_index];

                if taken_piece != 0 {
                    if def::on_same_side(player, taken_piece) {
                        break
                    }

                    if def::is_r(taken_piece) || def::is_q(taken_piece) {
                        return true
                    }

                    break
                }
            }
        }

        if opponent_bq_count > 0 {
            let mov_index_list = &self.up_left_mov_table[index];
            for to_index in mov_index_list {
                let taken_piece = squares[*to_index];

                if taken_piece != 0 {
                    if def::on_same_side(player, taken_piece) {
                        break
                    }

                    if def::is_b(taken_piece) || def::is_q(taken_piece) {
                        return true
                    }

                    break
                }
            }

            let mov_index_list = &self.up_right_mov_table[index];
            for to_index in mov_index_list {
                let taken_piece = squares[*to_index];

                if taken_piece != 0 {
                    if def::on_same_side(player, taken_piece) {
                        break
                    }

                    if def::is_b(taken_piece) || def::is_q(taken_piece) {
                        return true
                    }

                    break
                }
            }

            let mov_index_list = &self.down_right_mov_table[index];
            for to_index in mov_index_list {
                let taken_piece = squares[*to_index];

                if taken_piece != 0 {
                    if def::on_same_side(player, taken_piece) {
                        break
                    }

                    if def::is_b(taken_piece) || def::is_q(taken_piece) {
                        return true
                    }

                    break
                }
            }

            let mov_index_list = &self.down_left_mov_table[index];
            for to_index in mov_index_list {
                let taken_piece = squares[*to_index];

                if taken_piece != 0 {
                    if def::on_same_side(player, taken_piece) {
                        break
                    }

                    if def::is_b(taken_piece) || def::is_q(taken_piece) {
                        return true
                    }

                    break
                }
            }
        }

        if player == def::PLAYER_W {
            if state.b_piece_list.pawn > 0 {
                if index < 105 {
                    let potential_pawn_attacker = squares[index + 15];
    
                    if !def::on_same_side(player, potential_pawn_attacker) && def::is_p(potential_pawn_attacker) {
                        return true
                    }
                }
    
                if index < 103 {
                    let potential_pawn_attacker = squares[index + 17];
    
                    if !def::on_same_side(player, potential_pawn_attacker) && def::is_p(potential_pawn_attacker) {
                        return true
                    }
                }
            }
        } else {
            if state.w_piece_list.pawn > 0 {
                if index as isize >= 15 {
                    let potential_pawn_attacker = squares[index - 15];
    
                    if !def::on_same_side(player, potential_pawn_attacker) && def::is_p(potential_pawn_attacker) {
                        return true
                    }
                }
    
    
                if index as isize >= 17 {
                    let potential_pawn_attacker = squares[index - 17];
    
                    if !def::on_same_side(player, potential_pawn_attacker) && def::is_p(potential_pawn_attacker) {
                        return true
                    }
                }
            }
        }

        let mov_index_list = &self.k_mov_table[index];
        for to_index in mov_index_list {
            let taken_piece = squares[*to_index];

            if taken_piece != 0 {
                if !def::on_same_side(player, taken_piece) && def::is_k(taken_piece) {
                    return true
                }
            }
        }

        false
    }

    pub fn find_attacker_list(&self, state: &State, index: usize) -> (Vec<u8>, Vec<u8>) {
        let mut attacker_list = Vec::new();

        let squares = state.squares;

        let mov_index_list = &self.n_mov_table[index];
        for to_index in mov_index_list {
            let attacker_piece = squares[*to_index];

            if attacker_piece == 0 {
                continue
            }

            if def::is_n(attacker_piece) {
                attacker_list.push(attacker_piece);
            }
        }

        let mov_index_list = &self.up_left_mov_table[index];
        let mut direction_mov_count = 0;
        for to_index in mov_index_list {
            direction_mov_count += 1;

            let attacker_piece = squares[*to_index];

            if attacker_piece == 0 {
                continue
            }

            if direction_mov_count == 1 && def::is_k(attacker_piece) {
                attacker_list.push(attacker_piece);
                continue
            }

            if def::is_b(attacker_piece) || def::is_q(attacker_piece) {
                attacker_list.push(attacker_piece);
            } else {
                break
            }
        }

        let mov_index_list = &self.up_right_mov_table[index];
        direction_mov_count = 0;
        for to_index in mov_index_list {
            direction_mov_count += 1;

            let attacker_piece = squares[*to_index];

            if attacker_piece == 0 {
                continue
            }

            if direction_mov_count == 1 && def::is_k(attacker_piece) {
                attacker_list.push(attacker_piece);
                continue
            }

            if def::is_b(attacker_piece) || def::is_q(attacker_piece) {
                attacker_list.push(attacker_piece);
            } else {
                break
            }
        }

        let mov_index_list = &self.down_right_mov_table[index];
        direction_mov_count = 0;
        for to_index in mov_index_list {
            direction_mov_count += 1;

            let attacker_piece = squares[*to_index];

            if attacker_piece == 0 {
                continue
            }

            if direction_mov_count == 1 && def::is_k(attacker_piece) {
                attacker_list.push(attacker_piece);
                continue
            }

            if def::is_b(attacker_piece) || def::is_q(attacker_piece) {
                attacker_list.push(attacker_piece);
            } else {
                break
            }
        }

        let mov_index_list = &self.down_left_mov_table[index];
        direction_mov_count = 0;
        for to_index in mov_index_list {
            direction_mov_count += 1;

            let attacker_piece = squares[*to_index];

            if attacker_piece == 0 {
                continue
            }

            if direction_mov_count == 1 && def::is_k(attacker_piece) {
                attacker_list.push(attacker_piece);
                continue
            }

            if def::is_b(attacker_piece) || def::is_q(attacker_piece) {
                attacker_list.push(attacker_piece);
            } else {
                break
            }
        }

        let mov_index_list = &self.up_mov_table[index];
        direction_mov_count = 0;
        for to_index in mov_index_list {
            direction_mov_count += 1;

            let attacker_piece = squares[*to_index];

            if attacker_piece == 0 {
                continue
            }

            if direction_mov_count == 1 && def::is_k(attacker_piece) {
                attacker_list.push(attacker_piece);
                continue
            }

            if def::is_r(attacker_piece) || def::is_q(attacker_piece) {
                attacker_list.push(attacker_piece);
            } else {
                break
            }
        }

        let mov_index_list = &self.right_mov_table[index];
        direction_mov_count = 0;
        for to_index in mov_index_list {
            direction_mov_count += 1;

            let attacker_piece = squares[*to_index];

            if attacker_piece == 0 {
                continue
            }

            if direction_mov_count == 1 && def::is_k(attacker_piece) {
                attacker_list.push(attacker_piece);
                continue
            }

            if def::is_r(attacker_piece) || def::is_q(attacker_piece) {
                attacker_list.push(attacker_piece);
            } else {
                break
            }
        }

        let mov_index_list = &self.down_mov_table[index];
        direction_mov_count = 0;
        for to_index in mov_index_list {
            direction_mov_count += 1;

            let attacker_piece = squares[*to_index];

            if attacker_piece == 0 {
                continue
            }

            if direction_mov_count == 1 && def::is_k(attacker_piece) {
                attacker_list.push(attacker_piece);
                continue
            }

            if def::is_r(attacker_piece) || def::is_q(attacker_piece) {
                attacker_list.push(attacker_piece);
            } else {
                break
            }
        }

        let mov_index_list = &self.left_mov_table[index];
        direction_mov_count = 0;
        for to_index in mov_index_list {
            direction_mov_count += 1;

            let attacker_piece = squares[*to_index];

            if attacker_piece == 0 {
                continue
            }

            if direction_mov_count == 1 && def::is_k(attacker_piece) {
                attacker_list.push(attacker_piece);
                continue
            }

            if def::is_r(attacker_piece) || def::is_q(attacker_piece) {
                attacker_list.push(attacker_piece);
            } else {
                break
            }
        }

        if index < 105 && squares[index+15] == def::BP {
            attacker_list.push(def::BP);

            let mov_index_list = &self.up_left_mov_table[index];
            for to_index in mov_index_list {
                let attacker_piece = squares[*to_index];

                if attacker_piece == 0 {
                    continue
                }

                if def::is_b(attacker_piece) || def::is_q(attacker_piece) {
                    attacker_list.push(attacker_piece);
                } else {
                    break
                }
            }
        }

        if index < 103 && squares[index+17] == def::BP {
            attacker_list.push(def::BP);

            let mov_index_list = &self.up_right_mov_table[index];
            for to_index in mov_index_list {
                let attacker_piece = squares[*to_index];

                if attacker_piece == 0 {
                    continue
                }

                if def::is_b(attacker_piece) || def::is_q(attacker_piece) {
                    attacker_list.push(attacker_piece);
                } else {
                    break
                }
            }
        }

        if index >= 15 && squares[index-15] == def::WP {
            attacker_list.push(def::WP);

            let mov_index_list = &self.down_right_mov_table[index];
            for to_index in mov_index_list {
                let attacker_piece = squares[*to_index];

                if attacker_piece == 0 {
                    continue
                }

                if def::is_b(attacker_piece) || def::is_q(attacker_piece) {
                    attacker_list.push(attacker_piece);
                } else {
                    break
                }
            }
        }

        if index >= 17 && squares[index-17] == def::WP {
            attacker_list.push(def::WP);

            let mov_index_list = &self.down_left_mov_table[index];
            for to_index in mov_index_list {
                let attacker_piece = squares[*to_index];

                if attacker_piece == 0 {
                    continue
                }

                if def::is_b(attacker_piece) || def::is_q(attacker_piece) {
                    attacker_list.push(attacker_piece);
                } else {
                    break
                }
            }
        }

        attacker_list.sort();

        let mut w_attacker_list = Vec::new();
        let mut b_attacker_list = Vec::new();

        for a in attacker_list {
            if a & 1 == 1 {
                b_attacker_list.push(a);
            } else {
                w_attacker_list.push(a);
            }
        }

        (w_attacker_list, b_attacker_list)
    }

    pub fn count_rook_mobility(&self, state: &State, index: usize, player: u8) -> i32 {
        let squares = state.squares;
        let mut mob_score = 0;

        let mov_index_list = &self.up_mov_table[index];
        for to_index in mov_index_list {
            let to_index = *to_index;
            let taken_piece = squares[to_index];

            if taken_piece == 0 {
                mob_score += 1;
                continue
            }

            if !def::on_same_side(player, taken_piece) {
                mob_score += 1;
            }

            break
        }

        let mov_index_list = &self.right_mov_table[index];
        for to_index in mov_index_list {
            let to_index = *to_index;
            let taken_piece = squares[to_index];

            if taken_piece == 0 {
                mob_score += 1;
                continue
            }

            if !def::on_same_side(player, taken_piece) {
                mob_score += 1;
            }

            break
        }

        let mov_index_list = &self.down_mov_table[index];
        for to_index in mov_index_list {
            let to_index = *to_index;
            let taken_piece = squares[to_index];

            if taken_piece == 0 {
                mob_score += 1;
                continue
            }

            if !def::on_same_side(player, taken_piece) {
                mob_score += 1;
            }

            break
        }

        let mov_index_list = &self.left_mov_table[index];
        for to_index in mov_index_list {
            let to_index = *to_index;
            let taken_piece = squares[to_index];

            if taken_piece == 0 {
                mob_score += 1;
                continue
            }

            if !def::on_same_side(player, taken_piece) {
                mob_score += 1;
            }

            break
        }

        mob_score
    }

    pub fn count_bishop_mobility(&self, state: &State, index: usize, player: u8) -> i32 {
        let squares = state.squares;
        let mut mob_score = 0;

        let mov_index_list = &self.up_left_mov_table[index];
        for to_index in mov_index_list {
            let to_index = *to_index;
            let taken_piece = squares[to_index];

            if taken_piece == 0 {
                mob_score += 1;
                continue
            }

            if !def::on_same_side(player, taken_piece) {
                mob_score += 1;
            }

            break
        }

        let mov_index_list = &self.up_right_mov_table[index];
        for to_index in mov_index_list {
            let to_index = *to_index;
            let taken_piece = squares[to_index];

            if taken_piece == 0 {
                mob_score += 1;
                continue
            }

            if !def::on_same_side(player, taken_piece) {
                mob_score += 1;
            }

            break
        }

        let mov_index_list = &self.down_right_mov_table[index];
        for to_index in mov_index_list {
            let to_index = *to_index;
            let taken_piece = squares[to_index];

            if taken_piece == 0 {
                mob_score += 1;
                continue
            }

            if !def::on_same_side(player, taken_piece) {
                mob_score += 1;
            }

            break
        }

        let mov_index_list = &self.down_left_mov_table[index];
        for to_index in mov_index_list {
            let to_index = *to_index;
            let taken_piece = squares[to_index];

            if taken_piece == 0 {
                mob_score += 1;
                continue
            }

            if !def::on_same_side(player, taken_piece) {
                mob_score += 1;
            }

            break
        }

        mob_score
    }

    pub fn count_knight_mobility(&self, state: &State, index: usize, player: u8) -> i32 {
        let squares = state.squares;
        let mut mob_score = 0;

        let mov_index_list = &self.n_mov_table[index];
        for to_index in mov_index_list {
            let to_index = *to_index;
            let taken_piece = squares[to_index];

            if !def::on_same_side(player, taken_piece) {
                mob_score += 1;
            }
        }

        mob_score
    }

    pub fn count_king_mobility(&self, index : usize) -> i32 {
        self.k_mov_table[index].len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bitboard::BitMask,
        state::State,
        prng::XorshiftPrng,
        util,
    };

    fn gen_reg_movs_test_helper(fen: &str, expected_cap_list: Vec<&str>, expected_non_cap_list: Vec<&str>, debug: bool) {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new(fen, &zob_keys, &bitmask);

        let (cap_list, non_cap_list) = MoveTable::new().gen_reg_mov_list(&state);

        if debug {
            println!("Captures:");
            for c in &cap_list {
                println!("{}", util::format_mov(*c));
            }

            println!("Moves:");
            for nc in &non_cap_list {
                println!("{}", util::format_mov(*nc));
            }
        }

        for c in &cap_list {
            let mov_str = util::format_mov(*c);
            if !expected_cap_list.contains(&&*mov_str) {
                assert!(false, "{} not matched", mov_str);
            }
        }

        for nc in &non_cap_list {
            let mov_str = util::format_mov(*nc);
            if !expected_non_cap_list.contains(&&*mov_str) {
                assert!(false, "{} not matched", mov_str);
            }
        }

        assert_eq!(cap_list.len(), expected_cap_list.len(), "capture count do not match");
        assert_eq!(non_cap_list.len(), expected_non_cap_list.len(), "non-capture count do not match");
    }

    fn gen_cas_movs_test_helper(fen: &str, expected_cas_mov_list: Vec<&str>, debug: bool) {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new(fen, &zob_keys, &bitmask);

        let cas_list = MoveTable::new().gen_castle_mov_list(&state);

        if debug {
            println!("Castles:");
            for c in &cas_list {
                println!("{}", util::format_mov(*c));
            }
        }

        for c in &cas_list {
            let mov_str = util::format_mov(*c);
            if !expected_cas_mov_list.contains(&&*mov_str) {
                assert!(false, "{} not matched", mov_str);
            }
        }

        assert_eq!(cas_list.len(), expected_cas_mov_list.len(), "castle count do not match");
    }

    #[test]
    fn test_gen_movs_1() {
        gen_reg_movs_test_helper(
            "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1",
            vec!["e4d5"],
            vec![
                "e4e5", "a2a3", "a2a4", "b2b3", "b2b4", "c2c3", "c2c4", "d2d3", "d2d4", "f2f3", "f2f4", "g2g3", "g2g4", "h2h3", "h2h4",
                "b1a3", "b1c3", "g1e2", "g1f3", "g1h3", "f1e2", "f1d3", "f1c4", "f1b5", "f1a6", "d1e2", "d1f3", "d1g4", "d1h5", "e1e2",
                ],
            false,
        );
    }

    #[test]
    fn test_gen_movs_2() {
        gen_reg_movs_test_helper(
            "2k2r2/pp2br2/1np1p2q/2NpP2p/3P2p1/2PN4/PP2Q1PP/3R1R1K b - - 8 27",
            vec!["e7c5", "f7f1"],
            vec![
                "a7a6", "a7a5", "b6a4", "b6a8", "b6c4", "b6d7", "c8b8", "c8c7", "c8d7", "c8d8", "e7d8", "e7d6", "e7f6", "e7g5", "e7h4",
                "f8d8", "f8e8", "f8g8", "f8h8", "f7f6", "f7f5", "f7f4", "f7f3", "f7f2", "f7g7", "f7h7",
                "h6g6", "h6f6", "h6g7", "h6h7", "h6h8", "h6g5", "h6f4", "h6e3", "h6d2", "h6c1",
                "g4g3", "h5h4",
                ],
            false,
        );
    }

    #[test]
    fn test_gen_movs_3() {
        gen_reg_movs_test_helper(
            "r5rk/2p1Nppp/3p3P/pp2p1P1/4P3/2qnPQK1/8/R6R w - - 1 0",
            vec!["a1a5", "e7g8", "h6g7", "f3f7"],
            vec![
                "g5g6",
                "a1a2", "a1a3", "a1a4", "a1b1", "a1c1", "a1d1", "a1e1", "a1f1", "a1g1",
                "h1g1", "h1f1", "h1e1", "h1d1", "h1c1", "h1b1", "h1h2", "h1h3", "h1h4", "h1h5",
                "e7c8", "e7c6", "e7d5", "e7f5", "e7g6",
                "f3e2", "f3d1", "f3g4", "f3h5", "f3g2", "f3f2", "f3f1", "f3f4", "f3f5", "f3f6",
                "g3g2", "g3f2", "g3g4", "g3h2", "g3h3", "g3h4", "g3f4"
                ],
            false,
        );
    }

    #[test]
    fn test_gen_movs_4() {
        gen_reg_movs_test_helper(
            "r1bqk1nr/pPpp1ppp/2n5/2b1p3/2B1P3/2N2N2/P1PP1PPP/R1BQK2R w KQkq - 0 1",
            vec!["b7a8q", "b7c8q", "b7a8r", "b7c8r", "b7a8b", "b7c8b", "b7a8n", "b7c8n", "f3e5", "c4f7"],
            vec![
                "a2a3", "a2a4", "d2d3", "d2d4", "g2g3", "g2g4", "h2h3", "h2h4",
                "b7b8q", "b7b8r", "b7b8b", "b7b8n",
                "a1b1",
                "c1b2", "c1a3",
                "c3b1", "c3a4", "c3b5", "c3d5", "c3e2",
                "c4b3", "c4b5", "c4a6", "c4d3", "c4e2", "c4f1", "c4d5", "c4e6",
                "f3d4", "f3g5", "f3h4", "f3g1",
                "d1e2",
                "e1f1", "e1e2",
                "h1g1", "h1f1",
                ],
            false,
        );
    }

    #[test]
    fn test_gen_movs_5() {
        gen_reg_movs_test_helper(
            "2k2r2/pp2br2/1np1p3/2NpP2p/3P2p1/2PN4/PP2Q1PP/2qR1R1K b - - 8 27",
            vec!["e7c5", "f7f1", "c1d1", "c1b2", "c1c3"],
            vec![
                "a7a6", "a7a5", "b6a4", "b6a8", "b6c4", "b6d7", "c8b8", "c8c7", "c8d7", "c8d8", "e7d8", "e7d6", "e7f6", "e7g5", "e7h4",
                "f8d8", "f8e8", "f8g8", "f8h8", "f7f6", "f7f5", "f7f4", "f7f3", "f7f2", "f7g7", "f7h7",
                "c1a1", "c1b1", "c1c2", "c1d2", "c1e3", "c1f4", "c1g5", "c1h6",
                "g4g3", "h5h4",
                ],
            false,
        );
    }

    #[test]
    fn test_gen_movs_6() {
        gen_cas_movs_test_helper(
            "r1bqk1nr/pPpp1ppp/2n5/2b1p3/2B1P3/2N2N2/P1PP1PPP/R1BQK2R w KQkq - 0 1",
            vec!["e1g1"],
            false,
        );
    }

    #[test]
    fn test_gen_movs_7() {
        gen_cas_movs_test_helper(
            "r3k2r/pbppnppp/1bn2q2/4p3/2B5/2N1PN2/PPPP1PPP/R1BQK2R b KQkq - 0 1",
            vec!["e8g8", "e8c8"],
            false,
        );
    }

    #[test]
    fn test_gen_movs_8() {
        gen_cas_movs_test_helper(
            "1r2k2r/pbppnppp/1bn2q2/4p3/2B5/2N1PN2/PPPP1PPP/R1BQK2R b KQk - 0 1",
            vec!["e8g8"],
            false,
        );
    }

    #[test]
    fn test_gen_movs_9() {
        gen_reg_movs_test_helper(
            "4Q1k1/pp3ppp/3B4/q2p4/5P1P/P3PbPK/1P1r4/2R5 w - - 3 5",
            vec!["e8g8", "e8f7"],
            vec![
                "e8f8", "e8d8", "e8c8", "e8b8", "e8a8", "e8e7", "e8e6", "e8e5", "e8e4", "e8d7", "e8c6", "e8b5", "e8a4",
                "d6c7", "d6b8", "d6c5", "d6b4", "d6e5", "d6e7", "d6f8",
                "a3a4", "b2b3", "b2b4",
                "e3e4", "f4f5", "g3g4", "h4h5",
                "c1b1", "c1a1", "c1d1", "c1e1", "c1f1", "c1g1", "c1h1", "c1c2", "c1c3", "c1c4", "c1c5", "c1c6", "c1c7", "c1c8",
                "h3g2", "h3h2", "h3g4",
                ],
            false,
        );
    }

    #[test]
    fn test_attack_check() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("3rr3/2pq2pk/p2p1pnp/8/2QBPP2/1P6/P5PP/4RRK1 b - - 0 1", &zob_keys, &bitmask);
        let mov_table = MoveTable::new();

        assert!(mov_table.is_under_attack(&state, util::map_sqr_notation_to_index("f6"), def::PLAYER_B));
        assert!(mov_table.is_under_attack(&state, util::map_sqr_notation_to_index("c7"), def::PLAYER_B));
        assert!(mov_table.is_under_attack(&state, util::map_sqr_notation_to_index("a6"), def::PLAYER_B));
        assert!(mov_table.is_under_attack(&state, util::map_sqr_notation_to_index("e4"), def::PLAYER_W));
        assert!(!mov_table.is_under_attack(&state, util::map_sqr_notation_to_index("d4"), def::PLAYER_W));
    }

    #[test]
    fn test_king_check_1() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("3rr1k1/2pq2p1/p2p1pnp/8/2BBPP2/1PQ5/P5PP/4RRK1 b - - 0 1", &zob_keys, &bitmask);
        let mov_table = MoveTable::new();

        assert!(mov_table.is_in_check(&state));
    }

    #[test]
    fn test_king_check_2() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("3rr1k1/2pq2p1/p2pNpnp/8/2QBPP2/1P1B4/P5PP/4RRK1 b - - 0 1", &zob_keys, &bitmask);
        let mov_table = MoveTable::new();

        assert!(!mov_table.is_in_check(&state));
    }

    #[test]
    fn test_king_check_3() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("r2qnkn1/p2b2br/1p1p1pp1/2pPpp2/1PP1P2K/PRNBB3/3QNPPP/5R2 w - - 0 1", &zob_keys, &bitmask);
        let mov_table = MoveTable::new();

        assert!(mov_table.is_in_check(&state));
    }

    #[test]
    fn test_king_check_4() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("r2q1kn1/p2b1rb1/1p1p1pp1/2pPpn2/1PP1P3/PRNBB1K1/3QNPPP/5R2 w - - 0 1", &zob_keys, &bitmask);
        let mov_table = MoveTable::new();

        assert!(mov_table.is_in_check(&state));
    }

    #[test]
    fn test_king_check_5() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("r2q1k2/p2bPrbR/1p1p1ppn/2pPpn2/1PP1P3/P1NBB3/3QNPPP/5RK1 b - - 0 1", &zob_keys, &bitmask);
        let mov_table = MoveTable::new();

        assert!(mov_table.is_in_check(&state));
    }

    #[test]
    fn test_count_rook_mobility() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("r2q1k2/p2bPrb1/1p1p1ppn/2pP1n2/1PP1R3/P1NB4/3QNPPP/5RK1 b - - 0 1", &zob_keys, &bitmask);
        let mov_table = MoveTable::new();

        assert_eq!(7, mov_table.count_rook_mobility(&state, util::map_sqr_notation_to_index("e4"), def::PLAYER_W));
        assert_eq!(1, mov_table.count_rook_mobility(&state, util::map_sqr_notation_to_index("f7"), def::PLAYER_B));
    }

    #[test]
    fn test_count_bishop_mobility() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("r2q1k2/p2bPrb1/1p1p1ppn/2pP1n2/1PP1R3/P1NB4/3QNPPP/5RK1 b - - 0 1", &zob_keys, &bitmask);
        let mov_table = MoveTable::new();

        assert_eq!(2, mov_table.count_bishop_mobility(&state, util::map_sqr_notation_to_index("d3"), def::PLAYER_W));
        assert_eq!(6, mov_table.count_bishop_mobility(&state, util::map_sqr_notation_to_index("d7"), def::PLAYER_B));
    }

    #[test]
    fn test_count_knight_mobility() {
        let zob_keys = XorshiftPrng::new().create_prn_table(def::BOARD_SIZE, def::PIECE_CODE_RANGE);
        let bitmask = BitMask::new();
        let state = State::new("r2q1k2/p2bPrb1/1p1p1ppn/1NpP1n2/1PP1R3/P2B4/3QNPPP/5RK1 b - - 0 1", &zob_keys, &bitmask);
        let mov_table = MoveTable::new();

        assert_eq!(5, mov_table.count_knight_mobility(&state, util::map_sqr_notation_to_index("b5"), def::PLAYER_W));
        assert_eq!(2, mov_table.count_knight_mobility(&state, util::map_sqr_notation_to_index("h6"), def::PLAYER_B));
    }

    #[test]
    fn test_count_king_mobility() {
        let mov_table = MoveTable::new();

        assert_eq!(5, mov_table.count_king_mobility(util::map_sqr_notation_to_index("g1")));
        assert_eq!(3, mov_table.count_king_mobility(util::map_sqr_notation_to_index("h8")));
        assert_eq!(5, mov_table.count_king_mobility(util::map_sqr_notation_to_index("f8")));
    }
}
