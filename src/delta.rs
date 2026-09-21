use crate::{HexCoord, pos::HexPos};
use std::{
    fmt::Display,
    ops::{Add, Mul, Neg, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexDelta(HexCoord, HexCoord);

impl HexDelta {
    pub const fn new(du: HexCoord, dv: HexCoord) -> Self {
        assert!((du + dv) % 2 == 0, "du + dv must be even");
        Self(du, dv)
    }

    pub const fn du(&self) -> HexCoord {
        self.0
    }

    pub const fn dv(&self) -> HexCoord {
        self.1
    }

    pub const fn unpack(self) -> (HexCoord, HexCoord) {
        (self.0, self.1)
    }
}

impl Display for HexDelta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Neg for HexDelta {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0, -self.1)
    }
}

impl Add for HexDelta {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Add<HexPos> for HexDelta {
    type Output = HexPos;

    fn add(self, pos: HexPos) -> HexPos {
        pos + self
    }
}

impl Sub for HexDelta {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl Sub for HexPos {
    type Output = HexDelta;

    fn sub(self, other: Self) -> HexDelta {
        HexDelta(self.u() - other.u(), self.v() - other.v())
    }
}

impl Mul<HexCoord> for HexDelta {
    type Output = Self;

    fn mul(self, rhs: HexCoord) -> Self {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Mul<HexDelta> for HexCoord {
    type Output = HexDelta;

    fn mul(self, rhs: HexDelta) -> HexDelta {
        rhs * self
    }
}
