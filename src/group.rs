use crate::{HexCoord, container::HexPosContainer, delta::HexDelta, id::HexId, pos::HexPos};
use std::collections::HashSet;

/// A collection of hexagons, edges, or corners that can be manipulated as a
/// group.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexGroup<H = HexPos>(HashSet<H>)
where
    H: HexId;

impl<H> Default for HexGroup<H>
where
    H: HexId,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<H> HexGroup<H>
where
    H: HexId,
{
    /// Creates a new empty `HexGroup`.
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    /// Creates a new `HexGroup` with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashSet::with_capacity(capacity))
    }

    /// Returns true if the group contains the given hexagon, edge, or corner.
    pub fn contains(&self, pos: H) -> bool {
        self.0.contains(&pos)
    }

    /// Returns the number of elements in the group.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the group contains no elements.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Inserts the given hexagon, edge, or corner into the group.  Returns true if
    /// the element was not already present in the group, and false if it was
    /// already present.
    pub fn insert(&mut self, pos: H) -> bool {
        self.0.insert(pos)
    }

    /// Removed an item from the group.  Returns true if the item was present in
    /// the group, and false if it was not.
    pub fn remove(&mut self, pos: &H) -> bool {
        self.0.remove(pos)
    }

    /// Returns a new group with all items rotated around the given center by
    /// the given number of 60 degree steps.
    pub fn rotate_around(self, center: HexPos, steps: HexCoord) -> Self {
        Self(
            self.0
                .into_iter()
                .map(|pos| pos.rotate_around(center, steps))
                .collect(),
        )
    }

    /// Returns a new group with all items shifted by the given delta.
    pub fn shift(self, delta: HexDelta) -> Self {
        Self(self.0.into_iter().map(|pos| pos.shift(delta)).collect())
    }
}

impl HexGroup {
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

impl<H> Extend<H> for HexGroup<H>
where
    H: HexId,
{
    fn extend<T: IntoIterator<Item = H>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl<H> IntoIterator for HexGroup<H>
where
    H: HexId,
{
    type Item = H;
    type IntoIter = std::collections::hash_set::IntoIter<H>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, H> IntoIterator for &'a HexGroup<H>
where
    H: HexId,
{
    type Item = &'a H;
    type IntoIter = std::collections::hash_set::Iter<'a, H>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
