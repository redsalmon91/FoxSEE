/*
 * Copyright (C) 2020 Zixiao Han
 */

type Key = u64;
type Player = u8;
type Flag = u8;
type Depth = u8;
type CasRights = u8;
type EnpSqr = u8;

type TableEntry = (Key, Player, Depth, CasRights, EnpSqr, Flag, i32, u32);

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
            table: vec![(0, 0, 0, 0, 0, 0, 0, 0); size],
        }
    }

    pub fn get(&self, key: u64, player: u8, depth: u8, cas_rights: u8, enp_sqr: usize) -> LookupResult {
        let (k, p, d, c, e, f, s, m) = self.table[(key & self.mod_base) as usize];

        if k == key && p == player && c == cas_rights && e == enp_sqr as u8 {
            if d >= depth {
                LookupResult::Match(f, s, m)
            } else {
                LookupResult::MovOnly(m)
            }
        } else {
            LookupResult::NoMatch
        }
    }

    pub fn set(&mut self, key: u64, player: u8, depth: u8, cas_rights: u8, enp_sqr: usize, flag: u8, score: i32, mov: u32) -> bool {
        let (k, _p, d, _c, _e, _f, _s, _m) = self.table[(key & self.mod_base) as usize];

        if k != key || depth >= d {
            self.table[(key & self.mod_base) as usize] = (key, player, depth, cas_rights, enp_sqr as u8, flag, score, mov);
            return true
        }

        false
    }

    pub fn clear(&mut self) {
        self.table = vec![(0, 0, 0, 0, 0, 0, 0, 0); self.mod_base as usize + 1];
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
            table: vec![(0, 0, 0, 0, 0, 0, 0, 0); size],
        }
    }

    pub fn get(&self, key: u64, player: u8, depth: u8, cas_rights: u8, enp_sqr: usize) -> LookupResult {
        let (k, p, d, c, e, f, s, m) = self.table[(key & self.mod_base) as usize];

        if k == key && p == player && c == cas_rights && e == enp_sqr as u8 {
            if d >= depth {
                LookupResult::Match(f, s, m)
            } else {
                LookupResult::MovOnly(m)
            }
        } else {
            LookupResult::NoMatch
        }
    }

    pub fn set(&mut self, key: u64, player: u8, depth: u8, cas_rights: u8, enp_sqr: usize, flag: u8, score: i32, mov: u32) {
        self.table[(key & self.mod_base) as usize] = (key, player, depth, cas_rights, enp_sqr as u8, flag, score, mov);
    }

    pub fn clear(&mut self) {
        self.table = vec![(0, 0, 0, 0, 0, 0, 0, 0); self.mod_base as usize + 1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set_dp_entries() {
        let mut table = DepthPreferredHashTable::new(32786);

        table.set(10012, 2, 10, 0, 0, 1, -100, 123);
        assert_eq!(LookupResult::Match(1, -100, 123), table.get(10012, 2, 10, 0, 0));
        assert_eq!(LookupResult::NoMatch, table.get(10012, 1, 10, 0, 0));
        assert_eq!(LookupResult::Match(1, -100, 123), table.get(10012, 2, 5, 0, 0));
        assert_eq!(LookupResult::NoMatch, table.get(10012, 2, 10, 2, 0));
        assert_eq!(LookupResult::NoMatch, table.get(10012, 2, 10, 0, 99));

        table.set(10012, 2, 5, 0, 0, 1, 300, 999);
        assert_eq!(LookupResult::Match(1, -100, 123), table.get(10012, 2, 10, 0, 0));
        assert_eq!(LookupResult::NoMatch, table.get(10012, 1, 10, 0, 0));
        assert_eq!(LookupResult::Match(1, -100, 123), table.get(10012, 2, 5, 0, 0));
        assert_eq!(LookupResult::NoMatch, table.get(10012, 2, 10, 2, 0));
        assert_eq!(LookupResult::NoMatch, table.get(10012, 2, 10, 0, 99));

        table.set(10012, 2, 11, 0, 0, 1, 101, 223);
        assert_eq!(LookupResult::Match(1, 101, 223), table.get(10012, 2, 11, 0, 0));
        assert_eq!(LookupResult::NoMatch, table.get(10012, 1, 11, 0, 0));
        assert_eq!(LookupResult::Match(1, 101, 223), table.get(10012, 2, 10, 0, 0));
        assert_eq!(LookupResult::NoMatch, table.get(10012, 2, 11, 2, 0));
        assert_eq!(LookupResult::NoMatch, table.get(10012, 2, 11, 0, 99));
    }

    #[test]
    fn test_get_set_ar_entries() {
        let mut table = AlwaysReplaceHashTable::new(32786);

        table.set(10012, 2, 10, 0, 0, 1, -100, 123);
        assert_eq!(LookupResult::Match(1, -100, 123), table.get(10012, 2, 10, 0, 0));
        assert_eq!(LookupResult::NoMatch, table.get(10012, 1, 10, 0, 0));
        assert_eq!(LookupResult::Match(1, -100, 123), table.get(10012, 2, 5, 0, 0));
        assert_eq!(LookupResult::NoMatch, table.get(10012, 2, 10, 2, 0));
        assert_eq!(LookupResult::NoMatch, table.get(10012, 2, 10, 0, 99));

        table.set(10012, 2, 5, 0, 0, 1, 200, 666);
        assert_eq!(LookupResult::Match(1, 200, 666), table.get(10012, 2, 5, 0, 0));
        assert_eq!(LookupResult::NoMatch, table.get(10012, 1, 10, 0, 0));
        assert_eq!(LookupResult::MovOnly(666), table.get(10012, 2, 10, 0, 0));
        assert_eq!(LookupResult::NoMatch, table.get(10012, 2, 10, 2, 0));
        assert_eq!(LookupResult::NoMatch, table.get(10012, 2, 10, 0, 99));
    }
}
