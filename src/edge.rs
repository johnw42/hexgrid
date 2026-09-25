use crate::{Sixths, corner::HexCorner};

/// Identifier for an edge of a hexagon, represented as an enum with six
/// variants corresponding to the six edges of a hexagon and the integers 0 to
/// 5, numbered in counter-clockwise order starting from the top-right edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HexEdge {
    TopRight,
    Top,
    TopLeft,
    BottomLeft,
    Bottom,
    BottomRight,
}

impl HexEdge {
    /// All edges in counter-clockwise order.
    ///
    /// ```
    /// use hexgrid::HexEdge;
    ///
    /// for edge in HexEdge::ALL {
    ///     assert_eq!(edge, HexEdge::ALL[edge as usize]);
    /// }
    /// ```
    pub const ALL: [HexEdge; 6] = [
        HexEdge::TopRight,
        HexEdge::Top,
        HexEdge::TopLeft,
        HexEdge::BottomLeft,
        HexEdge::Bottom,
        HexEdge::BottomRight,
    ];

    /// Gets the corners that this edge connects, in counter-clockwise order.
    pub fn ends(self) -> [HexCorner; 2] {
        match self {
            HexEdge::TopRight => [HexCorner::Right, HexCorner::TopRight],
            HexEdge::Top => [HexCorner::TopRight, HexCorner::TopLeft],
            HexEdge::TopLeft => [HexCorner::TopLeft, HexCorner::Left],
            HexEdge::BottomLeft => [HexCorner::Left, HexCorner::BottomLeft],
            HexEdge::Bottom => [HexCorner::BottomLeft, HexCorner::BottomRight],
            HexEdge::BottomRight => [HexCorner::BottomRight, HexCorner::Right],
        }
    }

    /// Returns true iff the given corner touches this edge.
    pub fn touches_corner(self, corner: HexCorner) -> bool {
        self.ends().contains(&corner)
    }

    /// Returns the result of rotating this edge by the given number of steps,
    /// where each step is a 60 degree rotation counter-clockwise.  The number
    /// of steps can be negative, in which case the rotation is clockwise.
    pub fn rotate(self, steps: Sixths) -> Self {
        Self::ALL[(self as Sixths + steps).rem_euclid(6) as usize]
    }

    /// Returns the edge that is opposite this edge, which is the result of
    /// rotating this edge by 180 degrees.
    pub fn opposite(self) -> HexEdge {
        self.rotate(3)
    }

    /// Returns a number in the range -2..=3 indicating how many steps you would
    /// need to rotate this edge to get to the other edge.
    pub fn steps_to(self, other: HexEdge) -> i32 {
        let diff = (other as i32 - self as i32).rem_euclid(6);
        if diff > 3 { diff - 6 } else { diff }
    }
}

/// A set of possible values of `HexEdge` that are used by the `norm` method
/// to provide a unique representation of an edge position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NormHexEdge {
    TopRight,
    Top,
    TopLeft,
}

impl NormHexEdge {
    /// All normalized edges in counter-clockwise order.
    ///
    /// ```
    /// use hexgrid::NormHexEdge;
    ///
    /// for edge in NormHexEdge::ALL {
    ///     assert_eq!(edge, NormHexEdge::ALL[edge as usize]);
    /// }
    /// ```
    pub const ALL: [NormHexEdge; 3] = [
        NormHexEdge::TopRight,
        NormHexEdge::Top,
        NormHexEdge::TopLeft,
    ];

    /// Returns the result of rotating this normalized edge by the given number
    /// of 180 degrees.
    pub fn opposite(self) -> HexEdge {
        HexEdge::from(self).opposite()
    }
}

impl From<NormHexEdge> for HexEdge {
    fn from(edge: NormHexEdge) -> Self {
        match edge {
            NormHexEdge::TopRight => HexEdge::TopRight,
            NormHexEdge::Top => HexEdge::Top,
            NormHexEdge::TopLeft => HexEdge::TopLeft,
        }
    }
}

impl TryFrom<HexEdge> for NormHexEdge {
    type Error = ();
    fn try_from(edge: HexEdge) -> Result<Self, Self::Error> {
        match edge {
            HexEdge::TopRight => Ok(NormHexEdge::TopRight),
            HexEdge::Top => Ok(NormHexEdge::Top),
            HexEdge::TopLeft => Ok(NormHexEdge::TopLeft),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all() {
        for steps in 0..6 {
            assert_eq!(
                HexEdge::ALL[steps as usize],
                HexEdge::TopRight.rotate(steps),
                "steps: {}",
                steps
            );
        }
    }

    #[test]
    fn rotation() {
        for edge in HexEdge::ALL {
            for steps in -10..=10 {
                let rotated = edge.rotate(steps);
                assert!(
                    HexEdge::ALL.contains(&rotated),
                    "edge: {:?}, steps: {}, rotated: {:?}",
                    edge,
                    steps,
                    rotated
                );
                let steps_back = rotated.steps_to(edge);
                assert_eq!(
                    rotated.rotate(steps_back),
                    edge,
                    "edge: {:?}, steps: {}, rotated: {:?}, steps_back: {}",
                    edge,
                    steps,
                    rotated,
                    steps_back
                );
            }
        }
    }

    #[test]
    fn ends() {
        for edge in HexEdge::ALL {
            let ends = edge.ends();
            assert_eq!(ends[1], ends[0].rotate(1));
            assert_eq!(ends[0], ends[1].rotate(-1));
            assert_eq!(edge, ends[0].adjacent_edges()[1]);
            assert_eq!(edge, ends[1].adjacent_edges()[0]);
        }
    }
}
