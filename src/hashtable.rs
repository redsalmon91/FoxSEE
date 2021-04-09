/*
 * Copyright (C) 2020-2021 Zixiao Han
 */

#[derive(Clone, Copy)]
struct TableEntry {
    key: u64,
    exact: (i32, u8),
    lower_bound: (i32, u8),
    upper_bound: (i32, u8),
    age: u16,
    depth: u8,
    mov: u32,
}

impl TableEntry {
    fn empty() -> Self {
        TableEntry {
            key: 0,
            exact: (0,0),
            lower_bound: (0,0),
            upper_bound: (0,0),
            age: 0,
            depth: 0,
            mov: 0,
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

    pub fn get(&self, key: u64, depth: u8) -> Option<LookupResult> {
        let entry = &self.table[(key & self.mod_base) as usize];

        if entry.key == key {
            let mut result = LookupResult {
                exact: (false, 0),
                lower_bound: (false, 0),
                upper_bound: (false, 0),
                mov: entry.mov,
            };

            let (score, score_depth) = entry.exact;

            if score_depth >= depth {
                result.exact = (true, score);
            }

            let (score, score_depth) = entry.lower_bound;

            if score_depth >= depth {
                result.lower_bound = (true, score);
            }

            let (score, score_depth) = entry.upper_bound;

            if score_depth >= depth {
                result.upper_bound = (true, score);
            }

            Some(result)
        } else {
            None
        }
    }

    pub fn set(&mut self, key: u64, depth: u8, age: u16, flag: u8, score: i32, mov: u32) {
        let mut entry = &mut self.table[(key & self.mod_base) as usize];

        if entry.key != key {
            if (depth as u16 + age) >= (entry.depth as u16 + entry.age) {
                let mut new_entry = TableEntry {
                    key,
                    exact: (0, 0),
                    lower_bound: (0, 0),
                    upper_bound: (0, 0),
                    depth,
                    age,
                    mov,
                };

                match flag {
                    HASH_TYPE_EXACT => {
                        new_entry.exact = (score, depth);
                    },
                    HASH_TYPE_BETA => {
                        new_entry.lower_bound = (score, depth);
                    },
                    HASH_TYPE_ALPHA => {
                        new_entry.upper_bound = (score, depth);
                    },
                    _ => {},
                }

                self.table[(key & self.mod_base) as usize] = new_entry;
            }
        } else {
            match flag {
                HASH_TYPE_EXACT => {
                    let (_saved_score, saved_depth) = entry.exact;

                    if depth > saved_depth {
                        entry.exact = (score, depth);
                    }
                },
                HASH_TYPE_BETA => {
                    let (saved_score, saved_depth) = entry.lower_bound;

                    if depth > saved_depth || (depth == saved_depth && score > saved_score) {
                        entry.lower_bound = (score, depth);
                    }
                },
                HASH_TYPE_ALPHA => {
                    let (saved_score, saved_depth) = entry.upper_bound;

                    if depth > saved_depth || (depth == saved_depth && score < saved_score) {
                        entry.upper_bound = (score, depth);
                    }
                },
                _ => {},
            }

            if depth > entry.depth {
                entry.depth = depth;
            }

            entry.mov = mov;
        }
    }

    pub fn clear(&mut self) {
        self.table = vec![TableEntry::empty(); self.mod_base as usize + 1];
    }
}

