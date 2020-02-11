type Key = u64;
type BitMask = u32;
type Player = u8;
type Flag = u8;
type Depth = u8;
type CasRights = u8;
type EnpSqr = usize;

type RegTableEntry = (Key, BitMask, Player, Depth, CasRights, EnpSqr, Flag, i32, u32);

pub const HASH_TYPE_ALPHA: u8 = 1;
pub const HASH_TYPE_BETA: u8 = 2;

static SHRINK_MASK: u64 = 0b00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111;

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

        if k == key && p == player && c == cas_rights && e == enp_sqr && bm == ((bit_mask >> 16) & SHRINK_MASK) as u32 {
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
        self.table[(key & self.mod_base) as usize] = (key, ((bit_mask >> 16) & SHRINK_MASK) as u32, player, depth, cas_rights, enp_sqr, flag, score, mov);
    }

    pub fn clear(&mut self) {
        self.table = vec![(0, 0, 0, 0, 0, 0, 0, 0, 0); self.mod_base as usize + 1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shrink_bitmask() {
        assert_eq!(0b10101010_11111111_01010101_00001111, ((0b11001101_00100001_10101010_11111111_01010101_00001111_00100011_11010101 >> 16) & SHRINK_MASK) as u32);
        assert_eq!(0b10101010_11111111_01010101_00001111, ((0b00000000_11111111_10101010_11111111_01010101_00001111_11111111_00000000 >> 16) & SHRINK_MASK) as u32);
    }

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
