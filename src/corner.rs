use crate::{
    HexCoord,
    container::HexPosContainer as _,
    edge::HexEdge,
    grid_size::HexGridSize,
    pos::{HexPos, HexPosIterator},
};
use std::{
    f32::consts::{FRAC_PI_3, PI},
    fmt::Display,
};

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

pub struct HexCornerIterator {
    width: HexCoord,
    corner: HexCorner,
    pos: Option<HexPos>,
    pos_iter: HexPosIterator,
}

impl HexCornerIterator {
    pub fn new(size: HexGridSize) -> Self {
        let mut pos_iter = size.iter_hexes();
        let pos = pos_iter.next();
        Self {
            width: size.width(),
            corner: HexCorner::TopRight,
            pos,
            pos_iter,
        }
    }
}

impl Iterator for HexCornerIterator {
    type Item = HexCornerPos;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let pos = self.pos?;
            let corner = self.corner;
            self.corner = self.corner.rotate(1);
            if self.corner == HexCorner::TopRight {
                self.pos = self.pos_iter.next();
            }
            let is_valid_corner = match corner {
                HexCorner::Right | HexCorner::TopRight | HexCorner::TopLeft => true,
                HexCorner::Left => pos.u() == 0,
                HexCorner::BottomLeft => pos.v() == 0 && pos.v() == 0 || pos.v() == 1,
                HexCorner::BottomRight => {
                    pos.v() < 2
                        || (self.width % 2 == 0 && pos.v() == 1 && pos.u() == self.width - 1)
                }
            };
            if is_valid_corner {
                return Some(HexCornerPos::from((pos, corner)));
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexCornerPos {
    pub pos: HexPos,
    pub corner: HexCorner,
}

impl HexCornerPos {
    pub fn u(self) -> HexCoord {
        self.pos.u()
    }

    pub fn v(self) -> HexCoord {
        self.pos.v()
    }

    pub fn pos(self) -> HexPos {
        self.pos
    }

    pub fn corner(self) -> HexCorner {
        self.corner
    }

    pub fn pos_corner(self) -> (HexPos, HexCorner) {
        (self.pos, self.corner)
    }

    pub fn u_v_corner(self) -> (HexCoord, HexCoord, HexCorner) {
        (self.pos.u(), self.pos.v(), self.corner)
    }

    pub fn norm(self) -> (HexPos, NormHexCorner) {
        let pos = self.pos;
        match self.corner {
            HexCorner::Right => (pos, NormHexCorner::Right),
            HexCorner::TopRight => (pos, NormHexCorner::TopRight),
            HexCorner::TopLeft => (pos.shift(-1, 1), NormHexCorner::Right),
            HexCorner::Left => (pos.shift(-1, -1), NormHexCorner::TopRight),
            HexCorner::BottomLeft => (pos.shift(-1, -1), NormHexCorner::Right),
            HexCorner::BottomRight => (pos.shift(0, -2), NormHexCorner::TopRight),
        }
    }

    pub fn variants(self) -> [Self; 3] {
        let (pos, corner) = self.norm();
        match corner {
            NormHexCorner::Right => [
                (pos, HexCorner::Right).into(),
                (pos.shift(1, 1), HexCorner::BottomLeft).into(),
                (pos.shift(1, -1), HexCorner::TopLeft).into(),
            ],
            NormHexCorner::TopRight => [
                (pos, HexCorner::TopRight).into(),
                (pos.shift(0, 2), HexCorner::BottomRight).into(),
                (pos.shift(1, 1), HexCorner::Left).into(),
            ],
        }
    }
}

impl<C> From<(HexCoord, HexCoord, C)> for HexCornerPos
where
    C: Into<HexCorner>,
{
    fn from((u, v, corner): (HexCoord, HexCoord, C)) -> Self {
        Self {
            pos: HexPos::new(u, v),
            corner: corner.into(),
        }
    }
}

impl<C> From<(HexPos, C)> for HexCornerPos
where
    C: Into<HexCorner>,
{
    fn from((pos, corner): (HexPos, C)) -> Self {
        Self {
            pos,
            corner: corner.into(),
        }
    }
}

impl Display for HexCornerPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {:?})", self.u(), self.v(), self.corner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;
    use std::collections::HashSet;

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

    #[test]
    fn variants() {
        for corner in HexCorner::ALL {
            let corner_pos = HexCornerPos::from((0, 0, corner));
            for variant in corner_pos.variants() {
                assert_eq!(variant.norm(), corner_pos.norm());
            }
        }
    }

    #[quickcheck]
    fn iterator(size: HexGridSize) {
        let mut seen_corners = HashSet::new();
        for pos in size.iter_hexes() {
            for corner in HexCorner::ALL {
                seen_corners.insert(HexCornerPos::from((pos, corner)).norm());
            }
        }
        let iter_corners = HexCornerIterator::new(size)
            .map(|c| c.norm())
            .collect::<HashSet<_>>();
        assert_eq!(seen_corners, iter_corners);
    }
}
