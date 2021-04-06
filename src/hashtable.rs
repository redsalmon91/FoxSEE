/*
 * Copyright (C) 2020 Zixiao Han
 */

#[derive(Clone)]
struct TableEntry {
    key: u64,
    board_key: u64,
    depth: u8,
    age: u16,
    flag: u8,
    score: i32,
    mov: u32,
}

impl TableEntry {
    fn empty() -> Self {
        TableEntry {
            key: 0,
            board_key: 0,
            depth: 0,
            age: 0,
            flag: 0,
            score: 0,
            mov: 0,
        }
    }
}

pub const HASH_TYPE_EXACT: u8 = 0;
pub const HASH_TYPE_ALPHA: u8 = 1;
pub const HASH_TYPE_BETA: u8 = 2;

#[derive(PartialEq, Debug)]
pub enum LookupResult {
    Match(u8, i32, u32),
    MovOnly(u32),
    NoMatch,
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

    pub fn get(&self, key: u64, board_key: u64, depth: u8) -> LookupResult {
        let entry = &self.table[(key & self.mod_base) as usize];

        if entry.key == key && entry.board_key == board_key {
            if entry.depth >= depth {
                LookupResult::Match(entry.flag, entry.score, entry.mov)
            } else {
                LookupResult::MovOnly(entry.mov)
            }
        } else {
            LookupResult::NoMatch
        }
    }

    pub fn set(&mut self, key: u64, board_key: u64, depth: u8, age: u16, flag: u8, score: i32, mov: u32) -> bool {
        let entry = &self.table[(key & self.mod_base) as usize];

        if (depth as u16 + age) >= (entry.depth as u16 + entry.age) {
            self.table[(key & self.mod_base) as usize] = TableEntry {
                key,
                board_key,
                depth,
                age,
                flag,
                score, 
                mov,
            };

            return true
        }

        false
    }

    pub fn clear(&mut self) {
        self.table = vec![TableEntry::empty(); self.mod_base as usize + 1];
    }
}

