use crate::pos::HexPos;
use std::collections::HashSet;

pub trait HexPosContainer {
    type Iterator<'c>: Iterator<Item = HexPos> + 'c
    where
        Self: 'c;

    fn contains_hex(&self, pos: HexPos) -> bool;
    fn iter_hexes(&self) -> Self::Iterator<'_>;
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl HexPosContainer for [HexPos] {
    type Iterator<'c> = std::iter::Copied<std::slice::Iter<'c, HexPos>>;

    fn contains_hex(&self, pos: HexPos) -> bool {
        self.contains(&pos)
    }

    fn iter_hexes(&self) -> Self::Iterator<'_> {
        self.iter().copied()
    }

    fn len(&self) -> usize {
        (*self).len()
    }
}

impl HexPosContainer for Vec<HexPos> {
    type Iterator<'c> = std::vec::IntoIter<HexPos>;

    fn contains_hex(&self, pos: HexPos) -> bool {
        self.contains(&pos)
    }

    fn iter_hexes(&self) -> Self::Iterator<'_> {
        self.clone().into_iter()
    }

    fn len(&self) -> usize {
        (*self).len()
    }
}

impl HexPosContainer for HashSet<HexPos> {
    type Iterator<'c> = std::collections::hash_set::IntoIter<HexPos>;

    fn contains_hex(&self, pos: HexPos) -> bool {
        self.contains(&pos)
    }

    fn iter_hexes(&self) -> Self::Iterator<'_> {
        self.clone().into_iter()
    }

    fn len(&self) -> usize {
        (*self).len()
    }
}
