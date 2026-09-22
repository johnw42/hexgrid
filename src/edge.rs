use crate::{
    HexCoord,
    container::HexPosContainer,
    corner::HexCorner,
    grid_size::HexGridSize,
    pos::{HexPos, HexPosIterator},
};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    pub const ALL: [HexEdge; 6] = [
        HexEdge::TopRight,
        HexEdge::Top,
        HexEdge::TopLeft,
        HexEdge::BottomLeft,
        HexEdge::Bottom,
        HexEdge::BottomRight,
    ];

    /// Gets the corners that this edge connects in counter-clockwise order.
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

    pub fn touches_corner(self, corner: HexCorner) -> bool {
        self.ends().contains(&corner)
    }

    pub fn rotate(self, steps: i32) -> Self {
        Self::ALL[(self as i32 + steps).rem_euclid(6) as usize]
    }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NormHexEdge {
    TopRight,
    Top,
    TopLeft,
}

impl NormHexEdge {
    pub const ALL: [NormHexEdge; 3] = [
        NormHexEdge::TopRight,
        NormHexEdge::Top,
        NormHexEdge::TopLeft,
    ];

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexEdgePos {
    pub pos: HexPos,
    pub edge: HexEdge,
}

impl HexEdgePos {
    pub fn pos(self) -> HexPos {
        self.pos
    }

    pub fn u(self) -> HexCoord {
        self.pos.u()
    }

    pub fn v(self) -> HexCoord {
        self.pos.v()
    }

    pub fn edge(self) -> HexEdge {
        self.edge
    }

    pub fn pos_edge(self) -> (HexPos, HexEdge) {
        (self.pos, self.edge)
    }

    pub fn u_v_edge(self) -> (HexCoord, HexCoord, HexEdge) {
        (self.pos.u(), self.pos.v(), self.edge)
    }

    pub fn norm(self) -> (HexPos, NormHexEdge) {
        if let Ok(primary_edge) = self.edge.try_into() {
            (self.pos, primary_edge)
        } else {
            (
                self.pos.neighbor(self.edge),
                self.edge.opposite().try_into().unwrap(),
            )
        }
    }

    pub fn variants(self) -> [Self; 2] {
        let (pos, edge) = self.norm();
        [
            (pos, edge).into(),
            (pos.neighbor(edge), edge.opposite()).into(),
        ]
    }
}

impl<E> From<(HexCoord, HexCoord, E)> for HexEdgePos
where
    E: Into<HexEdge>,
{
    fn from((u, v, edge): (HexCoord, HexCoord, E)) -> Self {
        Self {
            pos: HexPos::new(u, v),
            edge: edge.into(),
        }
    }
}

impl<E> From<(HexPos, E)> for HexEdgePos
where
    E: Into<HexEdge>,
{
    fn from((pos, edge): (HexPos, E)) -> Self {
        Self {
            pos,
            edge: edge.into(),
        }
    }
}

impl Display for HexEdgePos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {:?})", self.pos.u(), self.pos.v(), self.edge)
    }
}

pub struct HexEdgeIterator {
    width: HexCoord,
    edge: HexEdge,
    pos: Option<HexPos>,
    pos_iter: HexPosIterator,
}

impl HexEdgeIterator {
    pub fn new(size: HexGridSize) -> Self {
        let mut pos_iter = size.iter_hexes();
        let pos = pos_iter.next();
        Self {
            width: size.width(),
            edge: HexEdge::TopRight,
            pos,
            pos_iter,
        }
    }
}

impl Iterator for HexEdgeIterator {
    type Item = HexEdgePos;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let pos = self.pos?;
            let edge = self.edge;
            self.edge = self.edge.rotate(1);
            if self.edge == HexEdge::TopRight {
                self.pos = self.pos_iter.next();
            }
            let is_valid_edge = match edge {
                HexEdge::TopRight | HexEdge::Top | HexEdge::TopLeft => true,
                HexEdge::BottomLeft => pos.u() == 0 || pos.v() == 0,
                HexEdge::Bottom => pos.v() <= 1,
                HexEdge::BottomRight => pos.v() == 0 || pos.u() == self.width - 1,
            };
            if is_valid_edge {
                return Some(HexEdgePos::from((pos, edge)));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;
    use std::collections::HashSet;

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

    #[test]
    fn variants() {
        for edge in HexEdge::ALL {
            let edge_pos = HexEdgePos::from((0, 0, edge));
            for variant in edge_pos.variants() {
                assert_eq!(variant.norm(), edge_pos.norm());
            }
        }
    }

    #[quickcheck]
    fn iterator(size: HexGridSize) {
        let mut seen_edges = HashSet::new();
        for pos in size.iter_hexes() {
            for edge in HexEdge::ALL {
                seen_edges.insert(HexEdgePos::from((pos, edge)).norm());
            }
        }
        let iter_edges = HexEdgeIterator::new(size)
            .map(|e| e.norm())
            .collect::<HashSet<_>>();
        assert_eq!(seen_edges, iter_edges);
    }
}
