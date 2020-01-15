use crate::{
    def,
    state::State,
};

pub static TERM_VAL: i32 = 10000;
pub static ADVANCE_VAL: i32 = 200;
pub static EQUAL_EXCHANGE_VAL: i32 = 10;
pub static LOSING_EXCHANGE_VAL: i32 = -50;
pub static K_VAL: i32 = 20000;

static Q_VAL: i32 = 950;
static R_VAL: i32 = 500;
static B_VAL: i32 = 330;
static N_VAL: i32 = 320;
static P_VAL: i32 = 100;

static DOUBLE_BISHOP_VAL: i32 = 20;
static ENDGAME_PAWN_EXTRA_VAL: i32 = 50;
static DUP_PAWN_PEN: i32 = 30;
static KING_SAFETY: i32 = 50;
static DRAW_PEN: i32 = 200;

static WK_SQR_VAL: [i32; def::BOARD_SIZE] = [
     20, 30, 10,  0,  0, 10, 30, 20, 0,  0,  0,  0,  0,  0,  0,  0,
     20, 20,  0,  0,  0,  0, 20, 20, 0,  0,  0,  0,  0,  0,  0,  0,
    -10,-20,-20,-20,-20,-20,-20,-10, 0,  0,  0,  0,  0,  0,  0,  0,
    -20,-30,-30,-40,-40,-30,-30,-20, 0,  0,  0,  0,  0,  0,  0,  0,
    -30,-40,-40,-50,-50,-40,-40,-30, 0,  0,  0,  0,  0,  0,  0,  0,
    -30,-40,-40,-50,-50,-40,-40,-30, 0,  0,  0,  0,  0,  0,  0,  0,
    -30,-40,-40,-50,-50,-40,-40,-30, 0,  0,  0,  0,  0,  0,  0,  0,
    -30,-40,-40,-50,-50,-40,-40,-30,
];

static WQ_SQR_VAL: [i32; def::BOARD_SIZE] = [
    -20,-10,-10, -5, -5,-10,-10,-20, 0,  0,  0,  0,  0,  0,  0,  0,
    -10,  0,  0,  0,  0,  0,  0,-10, 0,  0,  0,  0,  0,  0,  0,  0,
    -10,  0,  5,  5,  5,  5,  0,-10, 0,  0,  0,  0,  0,  0,  0,  0,
     -5,  0,  5,  5,  5,  5,  0, -5, 0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  5,  5,  5,  5,  0, -5, 0,  0,  0,  0,  0,  0,  0,  0,
    -10,  5,  5,  5,  5,  5,  0,-10, 0,  0,  0,  0,  0,  0,  0,  0,
    -10,  0,  5,  0,  0,  0,  0,-10, 0,  0,  0,  0,  0,  0,  0,  0,
    -20,-10,-10, -5, -5,-10,-10,-20
];

static WR_SQR_VAL: [i32; def::BOARD_SIZE] = [
     0,  0,  0,  5,  5,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    -5,  0,  0,  0,  0,  0,  0, -5,  0,  0,  0,  0,  0,  0,  0,  0,
    -5,  0,  0,  0,  0,  0,  0, -5,  0,  0,  0,  0,  0,  0,  0,  0,
    -5,  0,  0,  0,  0,  0,  0, -5,  0,  0,  0,  0,  0,  0,  0,  0,
    -5,  0,  0,  0,  0,  0,  0, -5,  0,  0,  0,  0,  0,  0,  0,  0,
    -5,  0,  0,  0,  0,  0,  0, -5,  0,  0,  0,  0,  0,  0,  0,  0,
     5, 10, 10, 10, 10, 10, 10,  5,  0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0,
];

static WB_SQR_VAL: [i32; def::BOARD_SIZE] = [
    -20,-10,-10,-10,-10,-10,-10,-20, 0,  0,  0,  0,  0,  0,  0,  0,
    -10,  5,  0,  0,  0,  0,  5,-10, 0,  0,  0,  0,  0,  0,  0,  0,
    -10, 10, 10, 10, 10, 10, 10,-10, 0,  0,  0,  0,  0,  0,  0,  0,
    -10,  0, 10, 10, 10, 10,  0,-10, 0,  0,  0,  0,  0,  0,  0,  0,
    -10,  5,  5, 10, 10,  5,  5,-10, 0,  0,  0,  0,  0,  0,  0,  0,
    -10,  0,  5, 10, 10,  5,  0,-10, 0,  0,  0,  0,  0,  0,  0,  0,
    -10,  0,  0,  0,  0,  0,  0,-10, 0,  0,  0,  0,  0,  0,  0,  0,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

static WN_SQR_VAL: [i32; def::BOARD_SIZE] = [
    -50,-40,-30,-30,-30,-30,-40,-50, 0,  0,  0,  0,  0,  0,  0,  0,
    -40,-20,  0,  0,  0,  0,-20,-40, 0,  0,  0,  0,  0,  0,  0,  0,
    -30,  5, 10, 15, 15, 10,  5,-30, 0,  0,  0,  0,  0,  0,  0,  0,
    -30,  0, 15, 20, 20, 15,  0,-30, 0,  0,  0,  0,  0,  0,  0,  0,
    -30,  5, 10, 15, 15, 10,  5,-30, 0,  0,  0,  0,  0,  0,  0,  0,
    -30,  0, 10, 15, 15, 10,  0,-30, 0,  0,  0,  0,  0,  0,  0,  0,
    -40,-20,  0,  5,  5,  0,-20,-40, 0,  0,  0,  0,  0,  0,  0,  0,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

static WP_SQR_VAL: [i32; def::BOARD_SIZE] = [
     0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     5, 10, 10,-20,-20, 10, 10,  5,  0,  0,  0,  0,  0,  0,  0,  0,
     5, -5,-10,  0,  0,-10, -5,  5,  0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0, 20, 20,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     5,  5, 10, 25, 25, 10,  5,  5,  0,  0,  0,  0,  0,  0,  0,  0,
    10, 10, 20, 30, 30, 20, 10, 10,  0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,  0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0
];

static BK_SQR_VAL: [i32; def::BOARD_SIZE] = [
    -30,-40,-40,-50,-50,-40,-40,-30,  0,  0,  0,  0,  0,  0,  0,  0,
    -30,-40,-40,-50,-50,-40,-40,-30,  0,  0,  0,  0,  0,  0,  0,  0,
    -30,-40,-40,-50,-50,-40,-40,-30,  0,  0,  0,  0,  0,  0,  0,  0,
    -30,-40,-40,-50,-50,-40,-40,-30,  0,  0,  0,  0,  0,  0,  0,  0,
    -20,-30,-30,-40,-40,-30,-30,-20,  0,  0,  0,  0,  0,  0,  0,  0,
    -10,-20,-20,-20,-20,-20,-20,-10,  0,  0,  0,  0,  0,  0,  0,  0,
     20, 20,  0,  0,  0,  0, 20, 20,  0,  0,  0,  0,  0,  0,  0,  0,
     20, 30, 10,  0,  0, 10, 30, 20
];

static BQ_SQR_VAL: [i32; def::BOARD_SIZE] = [
    -20,-10,-10, -5, -5,-10,-10,-20,  0,  0,  0,  0,  0,  0,  0,  0,
    -10,  0,  0,  0,  0,  0,  0,-10,  0,  0,  0,  0,  0,  0,  0,  0,
    -10,  0,  5,  5,  5,  5,  0,-10,  0,  0,  0,  0,  0,  0,  0,  0,
     -5,  0,  5,  5,  5,  5,  0, -5,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  5,  5,  5,  5,  0, -5,  0,  0,  0,  0,  0,  0,  0,  0,
    -10,  5,  5,  5,  5,  5,  0,-10,  0,  0,  0,  0,  0,  0,  0,  0,
    -10,  0,  5,  0,  0,  0,  0,-10,  0,  0,  0,  0,  0,  0,  0,  0,
    -20,-10,-10, -5, -5,-10,-10,-20
];

static BR_SQR_VAL: [i32; def::BOARD_SIZE] = [
     0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     5, 10, 10, 10, 10, 10, 10,  5,  0,  0,  0,  0,  0,  0,  0,  0,
    -5,  0,  0,  0,  0,  0,  0, -5,  0,  0,  0,  0,  0,  0,  0,  0,
    -5,  0,  0,  0,  0,  0,  0, -5,  0,  0,  0,  0,  0,  0,  0,  0,
    -5,  0,  0,  0,  0,  0,  0, -5,  0,  0,  0,  0,  0,  0,  0,  0,
    -5,  0,  0,  0,  0,  0,  0, -5,  0,  0,  0,  0,  0,  0,  0,  0,
    -5,  0,  0,  0,  0,  0,  0, -5,  0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  5,  5,  0,  0,  0
];

static BB_SQR_VAL: [i32; def::BOARD_SIZE] = [
    -20,-10,-10,-10,-10,-10,-10,-20,  0,  0,  0,  0,  0,  0,  0,  0,
    -10,  0,  0,  0,  0,  0,  0,-10,  0,  0,  0,  0,  0,  0,  0,  0,
    -10,  0,  5, 10, 10,  5,  0,-10,  0,  0,  0,  0,  0,  0,  0,  0,
    -10,  5,  5, 10, 10,  5,  5,-10,  0,  0,  0,  0,  0,  0,  0,  0,
    -10,  0, 10, 10, 10, 10,  0,-10,  0,  0,  0,  0,  0,  0,  0,  0,
    -10, 10, 10, 10, 10, 10, 10,-10,  0,  0,  0,  0,  0,  0,  0,  0,
    -10,  5,  0,  0,  0,  0,  5,-10,  0,  0,  0,  0,  0,  0,  0,  0,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

static BN_SQR_VAL: [i32; def::BOARD_SIZE] = [
    -50,-40,-30,-30,-30,-30,-40,-50,  0,  0,  0,  0,  0,  0,  0,  0,
    -40,-20,  0,  0,  0,  0,-20,-40,  0,  0,  0,  0,  0,  0,  0,  0,
    -30,  0, 10, 15, 15, 10,  0,-30,  0,  0,  0,  0,  0,  0,  0,  0,
    -30,  5, 15, 20, 20, 15,  5,-30,  0,  0,  0,  0,  0,  0,  0,  0,
    -30,  0, 15, 20, 20, 15,  0,-30,  0,  0,  0,  0,  0,  0,  0,  0,
    -30,  5, 10, 15, 15, 10,  5,-30,  0,  0,  0,  0,  0,  0,  0,  0,
    -40,-20,  0,  5,  5,  0,-20,-40,  0,  0,  0,  0,  0,  0,  0,  0,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

static BP_SQR_VAL: [i32; def::BOARD_SIZE] = [
     0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,  0,  0,  0,  0,  0,  0,  0,  0,
    10, 10, 20, 30, 30, 20, 10, 10,  0,  0,  0,  0,  0,  0,  0,  0,
     5,  5, 10, 25, 25, 10,  5,  5,  0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0, 20, 20,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     5, -5,-10,  0,  0,-10, -5,  5,  0,  0,  0,  0,  0,  0,  0,  0,
     5, 10, 10,-20,-20, 10, 10,  5,  0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0
];

static END_WP_SQR_VAL: [i32; def::BOARD_SIZE] = [
     0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     5, 10, 10, 10, 10, 10, 10,  5,  0,  0,  0,  0,  0,  0,  0,  0,
    10, 20, 20, 20, 20, 20, 20, 10,  0,  0,  0,  0,  0,  0,  0,  0,
    30, 50, 50, 50, 50, 50, 50, 30,  0,  0,  0,  0,  0,  0,  0,  0,
    50, 90, 90, 90, 90, 90, 90, 50,  0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0
];

static END_BP_SQR_VAL: [i32; def::BOARD_SIZE] = [
     0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    50, 90, 90, 90, 90, 90, 90, 50,  0,  0,  0,  0,  0,  0,  0,  0,
    30, 50, 50, 50, 50, 50, 50, 30,  0,  0,  0,  0,  0,  0,  0,  0,
    10, 20, 20, 20, 20, 20, 20, 10,  0,  0,  0,  0,  0,  0,  0,  0,
     5, 10, 10, 10, 10, 10, 10,  5,  0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0
];

static END_WK_SQR_VAL: [i32; def::BOARD_SIZE] = [
    -10,-10,-10,-10,-10,-10,-10,-10,  0,  0,  0,  0,  0,  0,  0,  0,
    -10, -5, -5, -5, -5, -5, -5,-10,  0,  0,  0,  0,  0,  0,  0,  0,
    -10, -5,  0,  5,  5,  0, -5,-10,  0,  0,  0,  0,  0,  0,  0,  0,
    -10, -5,  5, 10, 10,  5, -5,-10,  0,  0,  0,  0,  0,  0,  0,  0,
    -10, -5, 10, 10, 10, 10, -5,-10,  0,  0,  0,  0,  0,  0,  0,  0,
    -10,  5,  5, 10, 10,  5,  5,-10,  0,  0,  0,  0,  0,  0,  0,  0,
    -10,  0,  5,  5,  5,  5,  0,-10,  0,  0,  0,  0,  0,  0,  0,  0,
     -5,  0,  0,  0,  0,  0,  0, -5
];

static END_BK_SQR_VAL: [i32; def::BOARD_SIZE] = [
     -5,  0,  0,  0,  0,  0,  0, -5,  0,  0,  0,  0,  0,  0,  0,  0,
    -10,  0,  5,  5,  5,  5,  0,-10,  0,  0,  0,  0,  0,  0,  0,  0,
    -10,  5,  5, 10, 10,  5,  5,-10,  0,  0,  0,  0,  0,  0,  0,  0,
    -10, -5, 10, 10, 10, 10, -5,-10,  0,  0,  0,  0,  0,  0,  0,  0,
    -10, -5,  5, 10, 10,  5, -5,-10,  0,  0,  0,  0,  0,  0,  0,  0,
    -10, -5,  0,  5,  5,  0, -5,-10,  0,  0,  0,  0,  0,  0,  0,  0,
    -10, -5, -5, -5, -5, -5, -5,-10,  0,  0,  0,  0,  0,  0,  0,  0,
    -10,-10,-10,-10,-10,-10,-10,-10
];

pub fn val_of(piece: u8) -> i32 {
    match piece {
        def::WK => K_VAL,
        def::WQ => Q_VAL,
        def::WR => R_VAL,
        def::WB => B_VAL,
        def::WN => N_VAL,
        def::WP => P_VAL,

        def::BK => K_VAL,
        def::BQ => Q_VAL,
        def::BR => R_VAL,
        def::BB => B_VAL,
        def::BN => N_VAL,
        def::BP => P_VAL,

        _ => 0,
    }
}

pub fn eval_state(state: &State) -> i32 {
    let squares = state.squares;
    let mut index = 0;
    let mut base_score = 0;
    let mut midgame_score = 0;
    let mut endgame_score = 0;

    let mut wp_count = 0;
    let mut bp_count = 0;
    let mut wq_count = 0;
    let mut bq_count = 0;
    let mut wb_count = 0;
    let mut bb_count = 0;
    let mut w_piece_count = 0;
    let mut b_piece_count = 0;
    let mut wk_safe = true;
    let mut bk_safe = true;

    while index < def::BOARD_SIZE {
        if !def::is_index_valid(index) {
            index += 8;
        }

        let moving_piece = squares[index];

        if moving_piece == 0 {
            index += 1;
            continue
        }

        match moving_piece {
            def::WP => {
                base_score += P_VAL;
                midgame_score += WP_SQR_VAL[index];

                if squares[index + 16] == 0 {
                    endgame_score += END_WP_SQR_VAL[index];
                } else if squares[index + 16] == def::WP {
                    midgame_score -= DUP_PAWN_PEN;
                    endgame_score -= DUP_PAWN_PEN;
                }

                wp_count += 1;
            },
            def::BP => {
                base_score -= P_VAL;
                midgame_score -= BP_SQR_VAL[index];

                if squares[index - 16] == 0 {
                    endgame_score -= END_BP_SQR_VAL[index];
                }else if squares[index - 16] == def::BP {
                    midgame_score += DUP_PAWN_PEN;
                    endgame_score += DUP_PAWN_PEN;
                }

                bp_count += 1;
            },

            def::WN => {
                base_score += N_VAL;
                midgame_score += WN_SQR_VAL[index];
                w_piece_count += 1;
            },
            def::BN => {
                base_score -= N_VAL;
                midgame_score -= BN_SQR_VAL[index];
                b_piece_count += 1;
            },

            def::WB => {
                base_score += B_VAL;
                midgame_score += WB_SQR_VAL[index];
                wb_count += 1;
                w_piece_count += 1;
            },
            def::BB => {
                base_score -= B_VAL;
                midgame_score -= BB_SQR_VAL[index];
                bb_count += 1;
                b_piece_count += 1;
            },

            def::WR => {
                base_score += R_VAL;
                midgame_score += WR_SQR_VAL[index];
                w_piece_count += 1;
            },
            def::BR => {
                base_score -= R_VAL;
                midgame_score -= BR_SQR_VAL[index];
                b_piece_count += 1;
            },

            def::WQ => {
                base_score += Q_VAL;
                midgame_score += WQ_SQR_VAL[index];
                wq_count += 1;
                w_piece_count += 1;
            },
            def::BQ => {
                base_score -= Q_VAL;
                midgame_score -= BQ_SQR_VAL[index];
                bq_count += 1;
                b_piece_count += 1;
            },

            def::WK => {
                base_score += K_VAL;
                midgame_score += WK_SQR_VAL[index];
                endgame_score += END_WK_SQR_VAL[index];

                if index > 23 || !def::is_p(squares[index + 16]) {
                    wk_safe = false;
                }
            },
            def::BK => {
                base_score -= K_VAL;
                midgame_score -= BK_SQR_VAL[index];
                endgame_score -= END_BK_SQR_VAL[index];

                if index < 96 || !def::is_p(squares[index - 16]) {
                    bk_safe = false;
                }
            },
            _ => {},
        }

        index += 1;
    }

    if (wq_count == 0 || bq_count == 0 || w_piece_count == 1 || b_piece_count == 1) && (w_piece_count < 4 || b_piece_count < 4) {
        if wp_count < 5 || bp_count < 5 {
            base_score += wp_count * ENDGAME_PAWN_EXTRA_VAL - bp_count * ENDGAME_PAWN_EXTRA_VAL;
        }

        if wb_count > 1 {
            endgame_score += DOUBLE_BISHOP_VAL;
        }

        if bb_count > 1 {
            endgame_score -= DOUBLE_BISHOP_VAL;
        }

        if base_score > ADVANCE_VAL && wp_count == 0 {
            base_score -= DRAW_PEN;
        }

        if base_score < -ADVANCE_VAL && bp_count == 0 {
            base_score += DRAW_PEN;
        }

        return base_score + endgame_score
    }

    if wq_count > 0 && !bk_safe {
        midgame_score += KING_SAFETY;
    }

    if bq_count > 0 && !wk_safe {
        midgame_score -= KING_SAFETY;
    }

    base_score + midgame_score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        state::State,
    };

    #[test]
    fn test_eval() {
        let state = State::new("4k2r/pbppnppp/1bn2q2/4p3/2B5/2N1P3/PPPP1PPP/R1BQK2R b KQk - 0 1");
        assert_eq!(240, eval_state(&state));

        let state = State::new("4k2r/pbppnppp/1bn5/4p3/2B5/2N1P3/PPPP1PPP/R1BQK2R b KQk - 0 1");
        assert_eq!(1245, eval_state(&state));
    }
}
