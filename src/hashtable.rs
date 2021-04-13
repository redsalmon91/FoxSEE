/*
 * Copyright (C) 2020-2021 Zixiao Han
 */

#[derive(Clone, Copy)]
struct TableEntry {
    key: u64,
    safe_check: u16,
    age: u16,
    mov: u32,
    depth: u8,
    exact_score: i32,
    exact_depth: u8,
    lb_score: i32,
    lb_depth: u8,
    ub_score: i32,
    ub_depth: u8,
}

impl TableEntry {
    fn empty() -> Self {
        TableEntry {
            key: 0,
            safe_check: 0,
            age: 0,
            mov: 0,
            depth: 0,
            exact_score: 0,
            exact_depth: 0,
            lb_score: 0,
            lb_depth: 0,
            ub_score: 0,
            ub_depth: 0,
        }
    }
}

pub const HASH_TYPE_EXACT: u8 = 0;
pub const HASH_TYPE_ALPHA: u8 = 1;
pub const HASH_TYPE_BETA: u8 = 2;

#[derive(PartialEq, Debug)]
pub struct LookupResult {
    pub exact: (bool, i32),
    pub lower_bound: (bool, i32),
    pub upper_bound: (bool, i32),
    pub mov: u32,
}

pub struct DepthPreferredHashTable {
    mod_base: u64,
    table: Vec<TableEntry>,
}

impl DepthPreferredHashTable {
    pub fn new(size: usize) -> Self {
        DepthPreferredHashTable {
            mod_base: (size - 1) as u64,
            table: vec![TableEntry::empty(); size],
        }
    }

    pub fn get(&self, key: u64, safe_check: u16, depth: u8) -> Option<LookupResult> {
        let entry = &self.table[(key & self.mod_base) as usize];

        if entry.key == key && entry.safe_check == safe_check {
            let mut result = LookupResult {
                exact: (false, 0),
                lower_bound: (false, 0),
                upper_bound: (false, 0),
                mov: entry.mov,
            };

            if entry.exact_depth >= depth {
                result.exact = (true, entry.exact_score);
            }

            if entry.lb_depth >= depth {
                result.lower_bound = (true, entry.lb_score);
            }

            if entry.ub_depth >= depth {
                result.upper_bound = (true, entry.ub_score);
            }

            Some(result)
        } else {
            None
        }
    }

    pub fn set(&mut self, key: u64, safe_check: u16, depth: u8, age: u16, flag: u8, score: i32, mov: u32) {
        let mut entry = &mut self.table[(key & self.mod_base) as usize];

        if entry.key == key && entry.safe_check == safe_check {
            match flag {
                HASH_TYPE_EXACT => {
                    if depth > entry.exact_depth {
                        entry.exact_score = score;
                        entry.exact_depth = depth;

                        if depth >= entry.depth {
                            entry.mov = mov;
                        }
                    }
                },
                HASH_TYPE_BETA => {
                    if depth > entry.lb_depth || (depth == entry.lb_depth && score > entry.lb_score) {
                        entry.lb_score = score;
                        entry.lb_depth = depth;

                        if depth > entry.depth {
                            entry.mov = mov;
                        }
                    }
                },
                HASH_TYPE_ALPHA => {
                    if depth > entry.ub_depth || (depth == entry.ub_depth && score < entry.ub_score) {
                        entry.ub_score = score;
                        entry.ub_depth = depth;

                        if depth > entry.depth {
                            entry.mov = mov;
                        }
                    }
                },
                _ => {},
            }

            if depth > entry.depth {
                entry.depth = depth;
            }
        } else {
            if (depth as u16 + age) >= (entry.depth as u16 + entry.age) {
                let mut new_entry = TableEntry {
                    key,
                    safe_check,
                    age,
                    mov,
                    depth,
                    exact_score: 0,
                    exact_depth: 0,
                    lb_score: 0,
                    lb_depth: 0,
                    ub_score: 0,
                    ub_depth: 0,
                };

                match flag {
                    HASH_TYPE_EXACT => {
                        new_entry.exact_score = score;
                        new_entry.exact_depth = depth;
                    },
                    HASH_TYPE_BETA => {
                        new_entry.lb_score = score;
                        new_entry.lb_depth = depth;
                    },
                    HASH_TYPE_ALPHA => {
                        new_entry.ub_score = score;
                        new_entry.ub_depth = depth;
                    },
                    _ => {},
                }

                self.table[(key & self.mod_base) as usize] = new_entry;
            }
        }
    }

    pub fn clear(&mut self) {
        self.table = vec![TableEntry::empty(); self.mod_base as usize + 1];
    }
}
