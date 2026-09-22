use crate::edge::HexEdge;
use std::f32::consts::{FRAC_PI_3, PI};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HexCorner {
    Right,
    TopRight,
    TopLeft,
    Left,
    BottomLeft,
    BottomRight,
}

impl HexCorner {
    pub const ALL: [HexCorner; 6] = [
        HexCorner::Right,
        HexCorner::TopRight,
        HexCorner::TopLeft,
        HexCorner::Left,
        HexCorner::BottomLeft,
        HexCorner::BottomRight,
    ];

    pub fn touches_edge(self, edge: HexEdge) -> bool {
        edge.touches_corner(self)
    }

    pub fn rotate(self, steps: i32) -> HexCorner {
        Self::ALL[(self as i32 + steps).rem_euclid(6) as usize]
    }

    pub fn opposite(self) -> Self {
        self.rotate(3)
    }

    pub fn steps_to(self, other: HexCorner) -> i32 {
        let diff = (other as i32 - self as i32).rem_euclid(6);
        if diff > 3 { diff - 6 } else { diff }
    }

    pub fn to_angle(self) -> f32 {
        match self {
            HexCorner::Right => 0.0,
            HexCorner::TopRight => FRAC_PI_3,
            HexCorner::TopLeft => 2.0 * FRAC_PI_3,
            HexCorner::Left => PI,
            HexCorner::BottomLeft => -2.0 * FRAC_PI_3,
            HexCorner::BottomRight => -FRAC_PI_3,
        }
    }

    pub fn offset_from_center(self, radius: f32) -> (f32, f32) {
        let angle = self.to_angle();
        (radius * angle.cos(), radius * angle.sin())
    }

    pub fn from_angle(angle: f32) -> HexCorner {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

pub enum NormHexCorner {
    Right,
    TopRight,
}

impl NormHexCorner {
    pub const ALL: [NormHexCorner; 2] = [NormHexCorner::Right, NormHexCorner::TopRight];
}

impl From<NormHexCorner> for HexCorner {
    fn from(corner: NormHexCorner) -> Self {
        match corner {
            NormHexCorner::Right => HexCorner::Right,
            NormHexCorner::TopRight => HexCorner::TopRight,
        }
    }
}

impl TryFrom<HexCorner> for NormHexCorner {
    type Error = ();
    fn try_from(corner: HexCorner) -> Result<Self, Self::Error> {
        match corner {
            HexCorner::Right => Ok(NormHexCorner::Right),
            HexCorner::TopRight => Ok(NormHexCorner::TopRight),
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
