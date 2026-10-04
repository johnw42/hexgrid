use crate::{
    HexCoord, Sixths,
    delta::HexDelta,
    edge::{HexEdge, NormHexEdge},
    pos::HexPos,
};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, Sub},
};

/// A struct representing a position of an edge of a hexagon in a hexagonal
/// grid.
///
/// Each edge has two possible representations, which are equivalent. The
/// `norm` method returns a normalized representation of the edge position,
/// which is unique for each edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexEdgePos<E = HexEdge>
where
    E: Into<HexEdge>,
{
    pub pos: HexPos,
    pub edge: E,
}

impl<E> HexEdgePos<E>
where
    E: Into<HexEdge>,
{
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
    pub fn edge(self) -> E {
        self.edge
    }

    /// Returns the `HexPos` and `HexEdge` of the edge position as a tuple.
    pub fn pos_edge(self) -> (HexPos, E) {
        (self.pos, self.edge)
    }

    /// Returns the u and v coordinates of the hexagon position and the
    /// `HexEdge` of the edge position as a tuple.
    pub fn u_v_edge(self) -> (HexCoord, HexCoord, E) {
        (self.pos.u(), self.pos.v(), self.edge)
    }
}

impl HexEdgePos {
    /// Returns a normalized representation of the edge position, which is
    /// unique for each edge.
    pub fn norm(self) -> HexEdgePos<NormHexEdge> {
        if let Ok(norm_edge) = NormHexEdge::try_from(self.edge) {
            (self.pos, norm_edge).into()
        } else {
            (
                self.pos.neighbor(self.edge),
                self.edge.opposite().try_into().unwrap(),
            )
                .into()
        }
    }

    /// Returns the two equivalent representations of the edge position, for which
    /// `norm` returns the same value.
    pub fn variants(self) -> [Self; 2] {
        let (pos, edge) = self.norm().pos_edge();
        [
            (pos, edge.into()).into(),
            (pos.neighbor(edge), edge.opposite()).into(),
        ]
    }

    /// Returns the result of rotating the edge position around the given
    /// center by the given number of 60 degree steps.
    pub fn rotate_around(self, center: HexPos, steps: Sixths) -> Self {
        (
            self.pos.rotate_around(center, steps),
            self.edge.rotate(steps),
        )
            .into()
    }
}

impl HexEdgePos<NormHexEdge> {
    pub fn norm(self) -> HexEdgePos<NormHexEdge> {
        self
    }
}

impl<E> From<(HexCoord, HexCoord, E)> for HexEdgePos<E>
where
    E: Into<HexEdge>,
{
    fn from((u, v, edge): (HexCoord, HexCoord, E)) -> Self {
        Self {
            pos: HexPos::new(u, v),
            edge,
        }
    }
}

impl<E> From<(HexPos, E)> for HexEdgePos<E>
where
    E: Into<HexEdge>,
{
    fn from((pos, edge): (HexPos, E)) -> Self {
        Self { pos, edge }
    }
}

impl<E> Display for HexEdgePos<E>
where
    E: Into<HexEdge> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {:?})", self.pos.u(), self.pos.v(), self.edge)
    }
}

impl<E> Add<HexDelta> for HexEdgePos<E>
where
    E: Into<HexEdge>,
{
    type Output = Self;

    fn add(self, delta: HexDelta) -> Self::Output {
        Self {
            pos: self.pos + delta,
            edge: self.edge,
        }
    }
}

impl<E> Sub<HexDelta> for HexEdgePos<E>
where
    E: Into<HexEdge>,
{
    type Output = Self;

    fn sub(self, delta: HexDelta) -> Self::Output {
        Self {
            pos: self.pos - delta,
            edge: self.edge,
        }
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
