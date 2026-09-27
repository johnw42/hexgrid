use crate::{CubicPos, HexCoord, HexPos};
use std::fmt::Display;

/// A location of a hexagon in offset coordinates, represented as a pair (u, v)
/// of `HexCoord` values.  The offset coordinates are based on a rectangular
/// grid, where the u coordinate is the horizontal position, and the v
/// coordinate is the vertical position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OffsetPos(HexCoord, HexCoord);

impl OffsetPos {
    /// Creates a new `OffsetPos` with the given u and v values.
    pub const fn new(u: HexCoord, v: HexCoord) -> Self {
        Self(u, v)
    }

    /// Returns the u coordinate of the position.
    pub const fn u(self) -> HexCoord {
        self.0
    }

    /// Returns the v coordinate of the position.
    pub const fn v(self) -> HexCoord {
        self.1
    }

    /// Returns the u and v coordinates of the position as a tuple.
    pub const fn u_v(self) -> (HexCoord, HexCoord) {
        (self.u(), self.v())
    }

    /// Converts this position to a `HexPos`.
    pub fn to_pos(self) -> HexPos {
        self.into()
    }

    /// Converts this position to an `CubicPos`.
    pub fn to_cubic(self) -> CubicPos {
        self.into()
    }
}

impl Display for OffsetPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.u(), self.v())
    }
}

impl From<HexPos> for OffsetPos {
    fn from(pos: HexPos) -> Self {
        let (u, v) = pos.u_v();
        Self::new(u, if v < 0 { (v - 1) / 2 } else { v / 2 })
    }
}

impl From<OffsetPos> for HexPos {
    fn from(pos: OffsetPos) -> Self {
        let (u, v) = pos.u_v();
        Self::new(u, v * 2 + u.rem_euclid(2))
    }
}

impl From<CubicPos> for OffsetPos {
    fn from(pos: CubicPos) -> Self {
        HexPos::from(pos).into()
    }
}

impl From<OffsetPos> for CubicPos {
    fn from(pos: OffsetPos) -> Self {
        HexPos::from(pos).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn offset_pos_conversion(pos: HexPos) {
        assert_eq!(HexPos::from(OffsetPos::from(pos)), pos);
    }
}
