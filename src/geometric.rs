use crate::Sixths;
use crate::delta::HexDelta;
use crate::pos::HexPos;

/// A trait for types that can be used as identifiers for hexagons in a
/// hexagonal grid.  This trait is implemented for `HexPos`, and can be
/// implemented for other types that represent hexagons in a hexagonal grid,
/// such as a struct that contains additional data about the hexagon.
pub trait HexGeometric {
    /// Returns the result of rotating the hexagons around the given center by
    /// the given number of 60 degree steps.
    fn rotate_around(self, center: HexPos, steps: Sixths) -> Self;

    /// Returns the result of shifting the hexagons by the given delta by adding
    /// the delta to each position.
    fn translate(self, delta: HexDelta) -> Self;
}
