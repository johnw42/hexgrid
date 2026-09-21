use crate::pos::HexPos;
use std::collections::HashSet;

pub trait HexPosContainer {
    type Iterator: Iterator<Item = HexPos>;

    fn contains_hex(&self, pos: HexPos) -> bool;
    fn iter_hexes(&self) -> Self::Iterator;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a> HexPosContainer for &'a [HexPos] {
    type Iterator = std::iter::Copied<std::slice::Iter<'a, HexPos>>;

    fn contains_hex(&self, pos: HexPos) -> bool {
        self.contains(&pos)
    }

    fn iter_hexes(&self) -> Self::Iterator {
        self.iter().copied()
    }

    fn len(&self) -> usize {
        (*self).len()
    }
}

impl HexPosContainer for Vec<HexPos> {
    type Iterator = std::vec::IntoIter<HexPos>;

    fn contains_hex(&self, pos: HexPos) -> bool {
        self.contains(&pos)
    }

    fn iter_hexes(&self) -> Self::Iterator {
        self.clone().into_iter()
    }

    fn len(&self) -> usize {
        (*self).len()
    }
}

impl HexPosContainer for HashSet<HexPos> {
    type Iterator = std::collections::hash_set::IntoIter<HexPos>;

    fn contains_hex(&self, pos: HexPos) -> bool {
        self.contains(&pos)
    }

    fn iter_hexes(&self) -> Self::Iterator {
        self.clone().into_iter()
    }

    fn len(&self) -> usize {
        (*self).len()
    }
}
