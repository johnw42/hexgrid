use crate::{
    HexCoord,
    delta::HexDelta,
    edge::{HexEdge, NormHexEdge},
    id::HexId,
    pos::HexPos,
};
use std::fmt::Display;

/// A struct representing a position of an edge of a hexagon in a hexagonal
/// grid.
///
/// Each edge has two possible representations, which are equivalent. The
/// `norm` method returns a normalized representation of the edge position,
/// which is unique for each edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexEdgePos {
    pub pos: HexPos,
    pub edge: HexEdge,
}

impl HexEdgePos {
    /// Returns the `HexPos` of the hexagon position.
    pub fn pos(self) -> HexPos {
        self.pos
    }

    /// Returns the u coordinate of the hexagon position.
    pub fn u(self) -> HexCoord {
        self.pos.u()
    }

    /// Returns the v coordinate of the hexagon position.
    pub fn v(self) -> HexCoord {
        self.pos.v()
    }

    /// Returns the `HexEdge` of the edge position.
    pub fn edge(self) -> HexEdge {
        self.edge
    }

    /// Returns the `HexPos` and `HexEdge` of the edge position as a tuple.
    pub fn pos_edge(self) -> (HexPos, HexEdge) {
        (self.pos, self.edge)
    }

    /// Returns the u and v coordinates of the hexagon position and the
    /// `HexEdge` of the edge position as a tuple.
    pub fn u_v_edge(self) -> (HexCoord, HexCoord, HexEdge) {
        (self.pos.u(), self.pos.v(), self.edge)
    }

    /// Returns a normalized representation of the edge position, which is
    /// unique for each edge.
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

    /// Returns the two equivalent representations of the edge position, for which
    /// `norm` returns the same value.
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

impl HexId for HexEdgePos {
    fn pos(&self) -> HexPos {
        self.pos
    }

    fn corners(&self) -> impl Iterator<Item = crate::corner::HexCorner> + '_ {
        std::iter::empty()
    }

    fn edges(&self) -> impl Iterator<Item = HexEdge> + '_ {
        std::iter::once(self.edge)
    }

    fn rotate_around(self, center: HexPos, steps: HexCoord) -> Self {
        (self.pos.rotate_around(center, steps), self.edge).into()
    }

    fn shift(self, delta: HexDelta) -> Self {
        (self.pos.shift(delta), self.edge).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants() {
        for edge in HexEdge::ALL {
            let edge_pos = HexEdgePos::from((0, 0, edge));
            for variant in edge_pos.variants() {
                assert_eq!(variant.norm(), edge_pos.norm());
            }
        }
    }
}
