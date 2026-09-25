use crate::{Cartesian, Radians, Sixths, edge::HexEdge};
use std::f32::consts::FRAC_PI_3;

/// Identifier for a corner of a hexagon, represented as an enum with six
/// variants corresponding to the six corners of a hexagon and the integers 0 to
/// 5, numbered in counter-clockwise order starting from the right corner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HexCorner {
    Right,
    TopRight,
    TopLeft,
    Left,
    BottomLeft,
    BottomRight,
}

impl HexCorner {
    /// All corners in counter-clockwise order.
    ///
    /// ```
    /// use hexgridrect::HexCorner;
    ///
    /// for corner in HexCorner::ALL {
    ///     assert_eq!(corner, HexCorner::ALL[corner as usize]);
    /// }
    /// ```
    pub const ALL: [HexCorner; 6] = [
        HexCorner::Right,
        HexCorner::TopRight,
        HexCorner::TopLeft,
        HexCorner::Left,
        HexCorner::BottomLeft,
        HexCorner::BottomRight,
    ];

    /// Returns true iff the given edge touches this corner.
    pub fn touches_edge(self, edge: HexEdge) -> bool {
        edge.touches_corner(self)
    }

    /// Returns the result of rotating this corner by the given number of steps,
    /// where each step is a 60 degree rotation counter-clockwise.  The number
    /// of steps can be negative, in which case the rotation is clockwise.
    pub fn rotate(self, steps: i32) -> HexCorner {
        Self::ALL[(self as i32 + steps).rem_euclid(6) as usize]
    }

    /// Returns the corner that is opposite this corner, which is the result of
    /// rotating this corner by 180 degrees.
    pub fn opposite(self) -> Self {
        self.rotate(3)
    }

    /// Returns a number in the range -2..=3 indicating how many steps you would
    /// need to rotate this corner to get to the other corner.
    ///
    /// ```
    /// use hexgridrect::HexCorner;
    ///
    /// for corner in HexCorner::ALL {
    ///     for other in HexCorner::ALL {
    ///         let steps = corner.steps_to(other);
    ///         assert_eq!(corner.rotate(steps), other);
    ///     }
    /// }
    /// ```
    pub fn steps_to(self, other: HexCorner) -> Sixths {
        let diff = (other as i32 - self as i32).rem_euclid(6);
        if diff > 3 { diff - 6 } else { diff }
    }

    /// Converts the number of steps to rotate this corner to the other corner into an angle in radians.
    pub fn steps_to_angle(steps: Sixths) -> Radians {
        steps as Radians * FRAC_PI_3
    }

    /// Assuming a hexagon centered at the origin, returns the angle in radians
    /// of this corner from the center of the hexagon, as measured
    /// counter-clockwise from the positive u-axis.
    pub fn to_angle(self) -> Radians {
        Self::steps_to_angle(self as Sixths)
    }

    /// Assuming a hexagon centered at the origin, returns the Cartesian
    /// coordinates of this corner from the center of the hexagon, as measured
    /// counter-clockwise from the positive u-axis, assuming the width of the
    /// hexagon is 1.0 unit.
    pub fn offset_from_center(self) -> Cartesian {
        let angle = self.to_angle();
        (angle.cos(), angle.sin())
    }

    /// Converts an angle in radians, as returned by `to_angle`, into the
    /// nearest corresponding corner of a hexagon.
    ///
    /// ```
    /// use hexgridrect::{Radians, HexCorner};
    /// use std::f32::consts::PI;
    ///
    /// let epsilon: Radians = 0.01;
    /// for tweak in [-PI/6.0 + epsilon, 0.0, PI/6.0 - epsilon] {
    ///     for corner in HexCorner::ALL {
    ///         assert_eq!(corner, HexCorner::from_angle(corner.to_angle() + tweak));
    ///     }
    /// }
    /// ```
    pub fn from_angle(angle: Radians) -> HexCorner {
        Self::Right.rotate((angle / FRAC_PI_3).round() as i32)
    }

    /// Returns the two edges that are adjacent to this corner.  The order is
    /// the previous edge (clockwise) first, then the next edge
    /// (counter-clockwise).
    pub fn adjacent_edges(self) -> [HexEdge; 2] {
        match self {
            HexCorner::Right => [HexEdge::BottomRight, HexEdge::TopRight],
            HexCorner::TopRight => [HexEdge::TopRight, HexEdge::Top],
            HexCorner::TopLeft => [HexEdge::Top, HexEdge::TopLeft],
            HexCorner::Left => [HexEdge::TopLeft, HexEdge::BottomLeft],
            HexCorner::BottomLeft => [HexEdge::BottomLeft, HexEdge::Bottom],
            HexCorner::BottomRight => [HexEdge::Bottom, HexEdge::BottomRight],
        }
    }
}

/// A set of possible values of `HexCorner` that are used by the `norm` method
/// to provide a unique representation of a corner position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NormHexCorner {
    TopRight,
    TopLeft,
}

impl NormHexCorner {
    /// All normalized corners in counter-clockwise order.
    ///
    /// ```
    /// use hexgridrect::NormHexCorner;
    ///
    /// for corner in NormHexCorner::ALL {
    ///     assert_eq!(corner, NormHexCorner::ALL[corner as usize]);
    /// }
    /// ```
    pub const ALL: [NormHexCorner; 2] = [NormHexCorner::TopRight, NormHexCorner::TopLeft];
}

impl From<NormHexCorner> for HexCorner {
    fn from(corner: NormHexCorner) -> Self {
        match corner {
            NormHexCorner::TopRight => HexCorner::TopRight,
            NormHexCorner::TopLeft => HexCorner::TopLeft,
        }
    }
}

impl TryFrom<HexCorner> for NormHexCorner {
    type Error = ();
    fn try_from(corner: HexCorner) -> Result<Self, Self::Error> {
        match corner {
            HexCorner::TopRight => Ok(NormHexCorner::TopRight),
            HexCorner::TopLeft => Ok(NormHexCorner::TopLeft),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotation() {
        for corner in HexCorner::ALL {
            for steps in -10..=10 {
                let rotated = corner.rotate(steps);
                assert!(
                    HexCorner::ALL.contains(&rotated),
                    "corner: {:?}, steps: {}, rotated: {:?}",
                    corner,
                    steps,
                    rotated
                );
                let steps_back = rotated.steps_to(corner);
                assert_eq!(
                    rotated.rotate(steps_back),
                    corner,
                    "corner: {:?}, steps: {}, rotated: {:?}, steps_back: {}",
                    corner,
                    steps,
                    rotated,
                    steps_back
                );
            }
        }
    }

    #[test]
    fn adjacent_edges() {
        for corner in HexCorner::ALL {
            let edges = corner.adjacent_edges();
            assert_eq!(edges[0].rotate(1), edges[1]);
            assert_eq!(edges[1].rotate(-1), edges[0]);
            assert_eq!(corner, edges[0].ends()[1]);
            assert_eq!(corner, edges[1].ends()[0]);
        }
    }
}
