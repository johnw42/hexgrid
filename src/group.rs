use crate::{HexCoord, container::HexPosContainer, delta::HexDelta, pos::HexPos};
use std::{collections::HashSet, ops::Add};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HexGroup(HashSet<HexPos>);

impl HexGroup {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashSet::with_capacity(capacity))
    }

    pub fn region(u_min: HexCoord, v_min: HexCoord, u_max: HexCoord, v_max: HexCoord) -> Self {
        let width = u_max - u_min + 1;
        let height = v_max - v_min + 1;
        // TODO: Make capacity more accurate by calculating the number of hexes in the region.
        let mut group = Self::with_capacity((width * height) as usize);
        for u in u_min..=u_max {
            for v in v_min..=v_max {
                if (u + v) % 2 == 0 {
                    group.0.insert(HexPos::new(u, v));
                }
            }
        }
        group
    }

    pub fn contains(&self, pos: HexPos) -> bool {
        self.0.contains(&pos)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn insert(&mut self, pos: HexPos) -> bool {
        self.0.insert(pos)
    }

    pub fn remove(&mut self, pos: HexPos) -> bool {
        self.0.remove(&pos)
    }

    pub fn rotate_around(&mut self, center: HexPos, steps: HexCoord) {
        *self = self.clone().rotated_around(center, steps);
    }

    pub fn rotated_around(self, center: HexPos, steps: HexCoord) -> Self {
        Self(
            self.0
                .into_iter()
                .map(|pos| (pos - center).rotated(steps) + center)
                .collect(),
        )
    }
}

impl HexPosContainer for HexGroup {
    type Iterator<'c> = <HashSet<HexPos> as HexPosContainer>::Iterator<'c>;

    fn contains_hex(&self, pos: HexPos) -> bool {
        self.0.contains_hex(pos)
    }

    fn iter_hexes(&self) -> Self::Iterator<'_> {
        self.0.iter_hexes()
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl Extend<HexPos> for HexGroup {
    fn extend<T: IntoIterator<Item = HexPos>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl IntoIterator for HexGroup {
    type Item = HexPos;
    type IntoIter = std::collections::hash_set::IntoIter<HexPos>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a HexGroup {
    type Item = HexPos;
    type IntoIter = std::iter::Copied<<&'a HashSet<HexPos> as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().copied()
    }
}

impl Add<HexDelta> for HexGroup {
    type Output = HexGroup;

    fn add(self, delta: HexDelta) -> Self::Output {
        HexGroup(self.0.into_iter().map(|pos| pos + delta).collect())
    }
}
