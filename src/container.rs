use crate::pos::HexPos;
use std::collections::HashSet;

/// A trait for containers that can hold HexPos values.
///
/// Has implementations for many common types.
pub trait HexPosContainer {
    /// The type returned by the `iter_hexes` method.
    type Iterator<'c>: Iterator<Item = HexPos> + 'c
    where
        Self: 'c;

    /// Checks if the container contains the given HexPos.
    fn contains_hex(&self, pos: HexPos) -> bool;

    /// Returns an iterator over the HexPos values in the container.
    fn iter_hexes(&self) -> Self::Iterator<'_>;

    /// Returns the number of HexPos values in the container.
    fn len(&self) -> usize;

    /// Checks if the container is empty.
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
