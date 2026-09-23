use crate::{HexCoord, Sixths, pos::HexPos};
use std::{
    fmt::Display,
    ops::{Add, Mul, Neg, Sub},
};

/// A difference of two [`HexPos`] values, represented by two rectangular
/// coordinates (du, dv), where du + dv is always even.  The du coordinate
/// represents the difference in the u coordinate of the two positions, and the
/// dv coordinate represents the difference in the v coordinate of the two
/// positions.
///
/// A delta is essentially a vector; it can be added, subtracted, scaled, and
/// rotated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexDelta(HexCoord, HexCoord);

impl HexDelta {
    /// Creates a new `HexDelta` with the given du and dv values.  The sum of du
    /// and dv must be even, otherwise this function will panic.
    pub const fn new(du: HexCoord, dv: HexCoord) -> Self {
        assert!((du + dv) % 2 == 0, "du + dv must be even");
        Self(du, dv)
    }

    /// Returns the du coordinate of the delta.
    pub const fn du(&self) -> HexCoord {
        self.0
    }

    /// Returns the dv coordinate of the delta.
    pub const fn dv(&self) -> HexCoord {
        self.1
    }

    /// Returns the du and dv coordinates of the delta as a tuple.
    pub const fn du_dv(self) -> (HexCoord, HexCoord) {
        (self.0, self.1)
    }

    /// Rotates the delta by 60 degrees counterclockwise `steps` times.
    pub const fn rotated(self, steps: Sixths) -> Self {
        let (du, dv) = self.du_dv();
        match steps.rem_euclid(6) {
            0 => self,
            1 => Self((du - dv) / 2, (3 * du + dv) / 2),
            2 => Self(-(du + dv) / 2, (3 * du - dv) / 2),
            3 => Self(-du, -dv),
            4 => Self((dv - du) / 2, (-3 * du - dv) / 2),
            5 => Self((du + dv) / 2, (-3 * du + dv) / 2),
            _ => unreachable!(),
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;

    impl Arbitrary for HexDelta {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let du = i32::arbitrary(g) % 1024;
            let dv = i32::arbitrary(g) % 1024;
            let du = du - (du + dv) % 2;
            Self(du, dv)
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let (du, dv) = self.du_dv();
            Box::new(
                du.shrink()
                    .zip(dv.shrink())
                    .filter(|(du, dv)| (du + dv) % 2 == 0)
                    .map(|(du, dv)| Self(du, dv)),
            )
        }
    }

    #[test]
    fn rotate() {
        let delta = HexDelta::new(1, 1);
        assert_eq!(delta.rotated(0), delta);
        assert_eq!(delta.rotated(1), HexDelta::new(0, 2));
        assert_eq!(delta.rotated(2), HexDelta::new(-1, 1));
        assert_eq!(delta.rotated(3), -delta);
        assert_eq!(delta.rotated(4), HexDelta::new(0, -2));
        assert_eq!(delta.rotated(5), HexDelta::new(1, -1));

        let delta = HexDelta::new(2, 2);
        assert_eq!(delta.rotated(0), delta);
        assert_eq!(delta.rotated(1), HexDelta::new(0, 4));
        assert_eq!(delta.rotated(2), HexDelta::new(-2, 2));
        assert_eq!(delta.rotated(3), -delta);
        assert_eq!(delta.rotated(4), HexDelta::new(0, -4));
        assert_eq!(delta.rotated(5), HexDelta::new(2, -2));
    }

    #[quickcheck]
    fn rotate_steps(delta: HexDelta, steps: u8) {
        let by_steps = (0..steps).fold(delta, |d, _| d.rotated(1));
        assert_eq!(by_steps, delta.rotated(steps as HexCoord));

        let by_steps = (0..steps).fold(delta, |d, _| d.rotated(-1));
        assert_eq!(by_steps, delta.rotated(-(steps as HexCoord)));
    }

    #[quickcheck]
    fn rotate_and_scale_commute(delta: HexDelta, steps: u8, scale: u8) {
        let rotated_then_scaled = delta.rotated(steps as HexCoord) * scale as HexCoord;
        let scaled_then_rotated = (delta * scale as HexCoord).rotated(steps as HexCoord);
        assert_eq!(rotated_then_scaled, scaled_then_rotated);
    }
}
