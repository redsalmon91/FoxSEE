/*
 * Copyright (C) 2020 Zixiao Han
 */

#[derive(Clone)]
struct TableEntry {
    key: u64,
    player: u8,
    depth: u8,
    cas_rights: u8,
    enp_sqr: u8,
    flag: u8,
    score: i32,
    mov: u32,
}

impl TableEntry {
    fn empty() -> Self {
        TableEntry {
            key: 0,
            player: 0,
            depth: 0,
            cas_rights: 0,
            enp_sqr: 0,
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

    pub fn get(&self, key: u64, player: u8, depth: u8, cas_rights: u8, enp_sqr: usize) -> LookupResult {
        let entry = &self.table[(key & self.mod_base) as usize];

        if entry.key == key && entry.player == player && entry.cas_rights == cas_rights && entry.enp_sqr == enp_sqr as u8 {
            if entry.depth >= depth {
                LookupResult::Match(entry.flag, entry.score, entry.mov)
            } else {
                LookupResult::MovOnly(entry.mov)
            }
        } else {
            LookupResult::NoMatch
        }
    }

    pub fn set(&mut self, key: u64, player: u8, depth: u8, cas_rights: u8, enp_sqr: usize, flag: u8, score: i32, mov: u32) -> bool {
        let entry = &self.table[(key & self.mod_base) as usize];

        if depth >= entry.depth || key != entry.key {
            self.table[(key & self.mod_base) as usize] = TableEntry {
                key,
                player,
                depth,
                cas_rights,
                enp_sqr: enp_sqr as u8,
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

pub struct AlwaysReplaceHashTable {
    mod_base: u64,
    table: Vec<TableEntry>,
}

impl AlwaysReplaceHashTable {
    pub fn new(size: usize) -> Self {
        AlwaysReplaceHashTable {
            mod_base: (size - 1) as u64,
            table: vec![TableEntry::empty(); size],
        }
    }

    pub fn get(&self, key: u64, player: u8, depth: u8, cas_rights: u8, enp_sqr: usize) -> LookupResult {
        let entry = &self.table[(key & self.mod_base) as usize];

        if entry.key == key && entry.player == player && entry.cas_rights == cas_rights && entry.enp_sqr == enp_sqr as u8 {
            if entry.depth >= depth {
                LookupResult::Match(entry.flag, entry.score, entry.mov)
            } else {
                LookupResult::MovOnly(entry.mov)
            }
        } else {
            LookupResult::NoMatch
        }
    }

    pub fn set(&mut self, key: u64, player: u8, depth: u8, cas_rights: u8, enp_sqr: usize, flag: u8, score: i32, mov: u32) {
        self.table[(key & self.mod_base) as usize] = TableEntry {
            key,
            player,
            depth,
            cas_rights,
            enp_sqr: enp_sqr as u8,
            flag,
            score, 
            mov,
        };
    }

    pub fn clear(&mut self) {
        self.table = vec![TableEntry::empty(); self.mod_base as usize + 1];
    }
}
