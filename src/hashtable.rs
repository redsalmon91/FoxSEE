type Key = u64;
type BitMask = u64;
type Player = u8;
type Flag = u8;
type Depth = u8;
type CasRights = u8;
type EnpSqr = u32;

type RegTableEntry = (Key, BitMask, Player, Depth, CasRights, EnpSqr, Flag, i32, u32);

pub const HASH_TYPE_ALPHA: u8 = 1;
pub const HASH_TYPE_BETA: u8 = 2;

#[derive(PartialEq, Debug)]
pub enum RegLookupResult {
    Match(u8, i32, u32),
    MovOnly(u32),
    NoMatch,
}

pub struct RegHashTable {
    mod_base: u64,
    table: Vec<RegTableEntry>,
}

impl RegHashTable {
    pub fn new(size: usize) -> Self {
        RegHashTable {
            mod_base: (size - 1) as u64,
            table: vec![(0, 0, 0, 0, 0, 0, 0, 0, 0); size],
        }
    }

    pub fn get(&self, key: u64, bit_mask: u64, player: u8, depth: u8, cas_rights: u8, enp_sqr: usize) -> RegLookupResult {
        let (k, bm, p, d, c, e, f, s, m) = self.table[(key & self.mod_base) as usize];

        if k == key && p == player && c == cas_rights && e == enp_sqr as u32 && bm == bit_mask {
            if d == depth {
                RegLookupResult::Match(f, s, m)
            } else {
                RegLookupResult::MovOnly(m)
            }
        } else {
            RegLookupResult::NoMatch
        }
    }

    pub fn set(&mut self, key: u64, bit_mask: u64, player: u8, depth: u8, cas_rights: u8, enp_sqr: usize, flag: u8, score: i32, mov: u32) {
        self.table[(key & self.mod_base) as usize] = (key, bit_mask, player, depth, cas_rights, enp_sqr as u32, flag, score, mov);
    }

    pub fn clear(&mut self) {
        self.table = vec![(0, 0, 0, 0, 0, 0, 0, 0, 0); self.mod_base as usize + 1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set_reg_entries() {
        let mut table = RegHashTable::new(32786);
        table.set(10012, 101, 2, 10, 0, 0, 1, -100, 123);

        assert_eq!(RegLookupResult::Match(1, -100, 123), table.get(10012, 101, 2, 10, 0, 0));
        assert_eq!(RegLookupResult::NoMatch, table.get(10012, 101, 1, 10, 0, 0));
        assert_eq!(RegLookupResult::MovOnly(123), table.get(10012, 101, 2, 5, 0, 0));
        assert_eq!(RegLookupResult::NoMatch, table.get(10012, 101, 2, 10, 2, 0));
        assert_eq!(RegLookupResult::NoMatch, table.get(10012, 101, 2, 10, 0, 99));
    }
}
