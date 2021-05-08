/*
 * Copyright (C) 2020-2021 Zixiao Han
 */

#[derive(Clone, Copy)]
struct TableEntry {
    key: u64,
    safe_check: u64,
    flag: u8,
    age: u16,
    depth: u8,
    score: i32,
    eval: i32,
    mov: u32,
}

impl TableEntry {
    fn empty() -> Self {
        TableEntry {
            key: 0,
            safe_check: 0,
            flag: 0,
            age: 0,
            depth: 0,
            score: 0,
            eval: 0,
            mov: 0,
        }
    }
}

pub const HASH_TYPE_EXACT: u8 = 1;
pub const HASH_TYPE_ALPHA: u8 = 2;
pub const HASH_TYPE_BETA: u8 = 3;

pub struct LookupResult {
    pub flag: u8,
    pub depth: u8,
    pub score: i32,
    pub eval: i32,
    pub mov: u32,
}

pub struct DepthPreferredHashTable {
    mod_base: u64,
    table: Vec<TableEntry>,
    utilization_count: u64,
}

impl DepthPreferredHashTable {
    pub fn new(size: usize) -> Self {
        DepthPreferredHashTable {
            mod_base: (size - 1) as u64,
            table: vec![TableEntry::empty(); size],
            utilization_count: 0,
        }
    }

    pub fn get(&self, key: u64, safe_check: u64) -> Option<LookupResult> {
        let entry = &self.table[(key & self.mod_base) as usize];

        if entry.key == key && entry.safe_check == safe_check {
            Some(LookupResult {
                flag: entry.flag,
                depth: entry.depth,
                score: entry.score,
                eval: entry.eval,
                mov: entry.mov,
            })
        } else {
            None
        }
    }

    pub fn set(&mut self, key: u64, safe_check: u64, depth: u8, age: u16, flag: u8, score: i32, eval: i32, mov: u32) {        
        let entry = &mut self.table[(key & self.mod_base) as usize];

        if entry.flag == 0 {
            self.utilization_count += 1;
        }

        if (depth as u16 + age) >= (entry.depth as u16 + entry.age) {
            self.table[(key & self.mod_base) as usize] = TableEntry {
                key,
                safe_check,
                flag,
                age,
                depth,
                score,
                eval,
                mov,
            };
        }
    }

    pub fn get_utilization_permill(&self) -> u64 {
        self.utilization_count * 1000 / (self.mod_base + 1)
    }

    pub fn clear(&mut self) {
        self.table = vec![TableEntry::empty(); self.mod_base as usize + 1];
        self.utilization_count = 0;
    }
}
