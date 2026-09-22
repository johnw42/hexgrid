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
pub enum PrimaryHexEdge {
    TopRight,
    Top,
    TopLeft,
}

impl PrimaryHexEdge {
    pub const ALL: [PrimaryHexEdge; 3] = [
        PrimaryHexEdge::TopRight,
        PrimaryHexEdge::Top,
        PrimaryHexEdge::TopLeft,
    ];

    pub fn opposite(self) -> HexEdge {
        HexEdge::from(self).opposite()
    }
}

impl From<PrimaryHexEdge> for HexEdge {
    fn from(edge: PrimaryHexEdge) -> Self {
        match edge {
            PrimaryHexEdge::TopRight => HexEdge::TopRight,
            PrimaryHexEdge::Top => HexEdge::Top,
            PrimaryHexEdge::TopLeft => HexEdge::TopLeft,
        }
    }
}

impl TryFrom<HexEdge> for PrimaryHexEdge {
    type Error = ();
    fn try_from(edge: HexEdge) -> Result<Self, Self::Error> {
        match edge {
            HexEdge::TopRight => Ok(PrimaryHexEdge::TopRight),
            HexEdge::Top => Ok(PrimaryHexEdge::Top),
            HexEdge::TopLeft => Ok(PrimaryHexEdge::TopLeft),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexPosWithEdge {
    pub pos: HexPos,
    pub edge: PrimaryHexEdge,
}

impl HexPosWithEdge {
    pub fn new(pos: HexPos, edge: HexEdge) -> Self {
        if let Ok(primary_edge) = edge.try_into() {
            Self {
                pos,
                edge: primary_edge,
            }
        } else {
            Self {
                pos: pos.neighbor(edge),
                edge: edge.opposite().try_into().unwrap(),
            }
        }
    }

    pub fn pos(self) -> HexPos {
        self.pos
    }

    pub fn edge(self) -> HexEdge {
        self.edge.into()
    }

    pub fn variants(self) -> [(HexPos, HexEdge); 2] {
        let Self { pos, edge } = self;
        [(pos, edge.into()), (pos.neighbor(edge), edge.opposite())]
    }
}

impl From<(HexPos, HexEdge)> for HexPosWithEdge {
    fn from((pos, edge): (HexPos, HexEdge)) -> Self {
        Self::new(pos, edge)
    }
}

impl From<HexPosWithEdge> for (HexPos, PrimaryHexEdge) {
    fn from(pos_with_edge: HexPosWithEdge) -> Self {
        let HexPosWithEdge { pos, edge } = pos_with_edge;
        (pos, edge)
    }
}

impl From<HexPosWithEdge> for (HexPos, HexEdge) {
    fn from(pos_with_edge: HexPosWithEdge) -> Self {
        let HexPosWithEdge { pos, edge } = pos_with_edge;
        (pos, edge.into())
    }
}

impl Display for HexPosWithEdge {
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
    type Item = HexPosWithEdge;
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
                return Some(HexPosWithEdge::new(pos, edge));
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

    #[quickcheck]
    fn iterator(size: HexGridSize) {
        let mut seen_edges = HashSet::new();
        for pos in size.iter_hexes() {
            for edge in HexEdge::ALL {
                seen_edges.insert(HexPosWithEdge::new(pos, edge));
            }
        }
        let iter_edges = HexEdgeIterator::new(size).collect::<HashSet<_>>();
        assert_eq!(seen_edges, iter_edges);
    }

    #[test]
    fn variants() {
        for edge in HexEdge::ALL {
            let pos = HexPos::new(0, 0);
            let pos_with_edge = HexPosWithEdge::new(pos, edge);
            for (var_pos, var_edge) in pos_with_edge.variants() {
                assert_eq!(
                    HexPosWithEdge::new(var_pos, var_edge),
                    HexPosWithEdge::new(pos, edge),
                    "pos: {:?}, edge: {:?}, pos_with_edge: {:?}, var_pos: {:?}, var_edge: {:?}",
                    pos,
                    edge,
                    pos_with_edge,
                    var_pos,
                    var_edge
                );
            }
        }
    }
}
