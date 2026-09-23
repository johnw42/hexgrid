use crate::Sixths;
use crate::delta::HexDelta;
use crate::{HexCoord, corner::HexCorner, edge::HexEdge, pos::HexPos};
use std::fmt::Debug;
use std::hash::Hash;

/// A trait for types that can be used as identifiers for hexagons in a
/// hexagonal grid.  This trait is implemented for `HexPos`, and can be
/// implemented for other types that represent hexagons in a hexagonal grid,
/// such as a struct that contains additional data about the hexagon.
pub trait HexId: Clone + Debug + PartialEq + Eq + Hash {
    /// Returns the position of the hexagon in the grid.
    fn pos(&self) -> HexPos;

    /// Returns an iterator over the relevant corners of the hexagon.
    fn corners(&self) -> impl Iterator<Item = HexCorner> + '_;

    /// Returns an iterator over the relevant edges of the hexagon.
    fn edges(&self) -> impl Iterator<Item = HexEdge> + '_;

    /// Returns the result of rotating the hexagons around the given center by
    /// the given number of 60 degree steps.
    fn rotate_around(self, center: HexPos, steps: Sixths) -> Self;

    /// Returns the result of shifting the hexagons by the given delta by adding
    /// the delta to each position.
    fn shift(self, delta: HexDelta) -> Self;

    /// Shorthand for `self.pos().u()`.  See [`HexPos::u()`] for more information.
    fn u(&self) -> HexCoord {
        self.pos().u()
    }

    /// Shorthand for `self.pos().v()`.  See [`HexPos::v()`] for more information.
    fn v(&self) -> HexCoord {
        self.pos().v()
    }

    /// Shorthand for `self.pos().u_v()`.  See [`HexPos::u_v()`] for more information.
    fn u_v(&self) -> (HexCoord, HexCoord) {
        self.pos().u_v()
    }
}
