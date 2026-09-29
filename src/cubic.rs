use crate::{HexCoord, HexPos, OffsetPos};
use std::{
    fmt::Display,
    ops::{Add, Mul, Neg, Sub},
};

/// A location of a hexagon in cubic coordinates, represented as a pair (q, r)
/// of `HexCoord` values.  The third coordinate s is implied by the constraint q
/// + r + s = 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CubicPos(HexCoord, HexCoord);

impl CubicPos {
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

impl Add<CubicDelta> for CubicPos {
    type Output = Self;

    fn add(self, other: CubicDelta) -> Self {
        Self(self.q() + other.dq(), self.r() + other.dr())
    }
}

impl Sub for CubicPos {
    type Output = CubicDelta;

    fn sub(self, other: Self) -> CubicDelta {
        CubicDelta(self.q() - other.q(), self.r() - other.r())
    }
}

impl Sub<CubicDelta> for CubicPos {
    type Output = Self;

    fn sub(self, other: CubicDelta) -> Self {
        Self(self.q() - other.dq(), self.r() - other.dr())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CubicDelta(HexCoord, HexCoord);

impl CubicDelta {
    /// Creates a new `CubicDelta` with the given dq and dr values.  The ds value is
    /// implied by the constraint dq + dr + ds = 0.
    pub const fn from_dq_dr(dq: HexCoord, dr: HexCoord) -> Self {
        Self(dq, dr)
    }

    /// Creates a new `CubicDelta` with the given dq and ds values.  The dr value is
    /// implied by the constraint dq + dr + ds = 0.
    pub const fn from_dq_ds(dq: HexCoord, ds: HexCoord) -> Self {
        Self(dq, -dq - ds)
    }

    /// Creates a new `CubicDelta` with the given dr and ds values.  The dq value is
    /// implied by the constraint dq + dr + ds = 0.
    pub const fn from_dr_ds(dr: HexCoord, ds: HexCoord) -> Self {
        Self(-dr - ds, dr)
    }

    /// Creates a new `CubicDelta` with the given dq, dr, and ds values.  The sum of
    /// dq, dr, and ds must be zero, otherwise this function will return `None`.
    pub const fn from_dq_dr_ds(dq: HexCoord, dr: HexCoord, ds: HexCoord) -> Option<Self> {
        if dq + dr + ds == 0 {
            Some(Self::from_dq_dr(dq, dr))
        } else {
            None
        }
    }

    /// Returns the dq coordinate of the delta.
    pub const fn dq(self) -> HexCoord {
        self.0
    }

    /// Returns the dr coordinate of the delta.
    pub const fn dr(self) -> HexCoord {
        self.1
    }

    /// Returns the ds coordinate of the delta.
    pub const fn ds(self) -> HexCoord {
        -self.0 - self.1
    }

    /// Returns the dq and dr coordinates of the delta as a tuple.
    pub const fn dq_dr(self) -> (HexCoord, HexCoord) {
        (self.dq(), self.dr())
    }

    /// Returns the dq, dr, and ds coordinates of the delta as a tuple.
    pub const fn dq_dr_ds(self) -> (HexCoord, HexCoord, HexCoord) {
        (self.dq(), self.dr(), self.ds())
    }
}

impl Add for CubicDelta {
    type Output = CubicDelta;

    fn add(self, other: Self) -> Self {
        Self(self.dq() + other.dq(), self.dr() + other.dr())
    }
}

impl Add<CubicPos> for CubicDelta {
    type Output = CubicPos;

    fn add(self, other: CubicPos) -> CubicPos {
        CubicPos(self.dq() + other.q(), self.dr() + other.r())
    }
}

impl Sub for CubicDelta {
    type Output = CubicDelta;

    fn sub(self, other: Self) -> Self {
        Self(self.dq() - other.dq(), self.dr() - other.dr())
    }
}

impl Neg for CubicDelta {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.dq(), -self.dr())
    }
}

impl Mul<HexCoord> for CubicDelta {
    type Output = Self;

    fn mul(self, rhs: HexCoord) -> Self {
        Self(self.dq() * rhs, self.dr() * rhs)
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
