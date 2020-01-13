use crate::def;

const N_MOVS: [isize; 8] = [14, 18, 31, 33, -14, -18, -31, -33];
const K_MOVS: [isize; 8] = [1, 15, 16, 17, -1, -15, -16, -17];

const VERTICAL_SLIDE_MOVS: [isize; 7] = [16, 32, 48, 64, 80, 96, 112];
const HORIZONTAL_SLIDE_MOVS: [isize; 7] = [1, 2, 3, 4, 5, 6, 7];

const DESC_DIAGNOL_SLIDE_MOVS: [isize; 7] = [15, 30, 45, 60, 75, 90, 105];
const ASC_DIAGNOL_SLIDE_MOVS: [isize; 7] = [17, 34, 51, 68, 85, 102, 119];

pub fn gen_n_mov_table() -> Vec<Vec<usize>> {
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

pub fn gen_k_mov_table() -> Vec<Vec<usize>> {
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

pub fn gen_up_slide_mov_table() -> Vec<Vec<usize>> {
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

pub fn gen_down_slide_mov_table() -> Vec<Vec<usize>> {
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

pub fn gen_right_slide_mov_table() -> Vec<Vec<usize>> {
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

pub fn gen_left_slide_mov_table() -> Vec<Vec<usize>> {
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

pub fn gen_up_left_slide_mov_table() -> Vec<Vec<usize>> {
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

pub fn gen_down_right_slide_mov_table() -> Vec<Vec<usize>> {
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

pub fn gen_up_right_slide_mov_table() -> Vec<Vec<usize>> {
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

pub fn gen_down_left_slide_mov_table() -> Vec<Vec<usize>> {
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
