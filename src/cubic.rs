use crate::{HexCoord, HexPos, OffsetPos};
use std::fmt::Display;

/// A location of a hexagon in cubic coordinates, represented as a pair (q, r)
/// of `HexCoord` values.  The third coordinate s is implied by the constraint q
/// + r + s = 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CubicPos(HexCoord, HexCoord);

impl CubicPos {
    /// The origin, (0,0,0).
    pub const ORIGIN: CubicPos = CubicPos(0, 0);

    /// Creates a new `CubicPos` with the given q and r values.  The s value is
    /// implied by the constraint q + r + s = 0.
    pub const fn new(q: HexCoord, r: HexCoord) -> Self {
        Self(q, r)
    }

    /// Creates a new `CubicPos` with the given q and r values.  The s value is
    /// implied by the constraint q + r + s = 0.
    pub const fn from_q_r((q, r): (HexCoord, HexCoord)) -> Self {
        Self(q, r)
    }

    /// Creates a new `CubicPos` with the given q and s values.  The r value is
    /// implied by the constraint q + r + s = 0.
    pub const fn from_q_s((q, s): (HexCoord, HexCoord)) -> Self {
        Self(q, -q - s)
    }

    /// Creates a new `CubicPos` with the given r and s values.  The q value is
    /// implied by the constraint q + r + s = 0.
    pub const fn from_r_s((r, s): (HexCoord, HexCoord)) -> Self {
        Self(-r - s, r)
    }

    /// Creates a new `CubicPos` with the given q, r, and s values.  The sum of
    /// q, r, and s must be zero, otherwise this function will return `None`.
    pub const fn from_q_r_s((q, r, s): (HexCoord, HexCoord, HexCoord)) -> Option<Self> {
        if q + r + s == 0 {
            Some(Self::new(q, r))
        } else {
            None
        }
    }

    /// Returns the q coordinate of the position.
    pub const fn q(self) -> HexCoord {
        self.0
    }

    /// Returns the r coordinate of the position.
    pub const fn r(self) -> HexCoord {
        self.1
    }

    /// Returns the s coordinate of the position.
    pub const fn s(self) -> HexCoord {
        -self.0 - self.1
    }

    /// Returns the q and r coordinates of the position as a tuple.
    pub const fn q_r(self) -> (HexCoord, HexCoord) {
        (self.q(), self.r())
    }

    /// Returns the q, r, and s coordinates of the position as a tuple.
    pub const fn q_r_s(self) -> (HexCoord, HexCoord, HexCoord) {
        (self.q(), self.r(), self.s())
    }

    /// Returns the minimum number of steps required to reach another cubic
    /// position from this cubic position.
    pub const fn steps_to(self, other: Self) -> HexCoord {
        let (q1, r1, s1) = self.q_r_s();
        let (q2, r2, s2) = other.q_r_s();
        let dq = (q1 - q2).abs();
        let dr = (r1 - r2).abs();
        let ds = (s1 - s2).abs();
        (dq + dr + ds) / 2
    }

    /// Converts this position to a `HexPos`.
    pub fn to_pos(self) -> HexPos {
        self.into()
    }

    /// Converts this position to an `OffsetPos`.
    pub fn to_offset(self) -> OffsetPos {
        self.into()
    }
}

impl Display for CubicPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.q(), self.r(), self.s())
    }
}

impl From<HexPos> for CubicPos {
    fn from(pos: HexPos) -> Self {
        let (u, v) = pos.u_v();
        let q = u;
        let r = (v - u) / 2;
        CubicPos(q, r)
    }
}

impl From<CubicPos> for HexPos {
    fn from(axial: CubicPos) -> Self {
        let (q, r) = axial.q_r();
        let u = q;
        let v = 2 * r + q;
        HexPos::new(u, v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn cubic_pos_conversion(pos: HexPos) {
        assert_eq!(HexPos::from(CubicPos::from(pos)), pos);
    }
}
