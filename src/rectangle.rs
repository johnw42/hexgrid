use crate::{
    Cartesian, HEX_HORIZONTAL_SPACING, HEX_VERTICAL_SPACING, HEX_WIDTH, HexCoord, HexCorner,
    HexCornerPos, HexEdge, HexEdgePos, HexPos, HexPosContainer, NormHexCorner, NormHexEdge,
};

/// A rectangular region of a hexagonal grid, defined by minimum and maximum u and v coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexRectangle {
    min_u: HexCoord,
    min_v: HexCoord,
    max_u: HexCoord,
    max_v: HexCoord,
}

impl HexRectangle {
    /// Create a new rectangle with the specified minimum and maximum u and v coordinates.
    ///
    /// The region will be empty if the minimum coordinates are greater than the maximum coordinates.
    pub fn new(min_u: HexCoord, min_v: HexCoord, max_u: HexCoord, max_v: HexCoord) -> Self {
        Self {
            min_u,
            min_v,
            max_u,
            max_v,
        }
    }

    /// Creates a new rectangle that contains all hexagonal grid
    /// positions in the rectangular area defined by the given minimum and
    /// maximum Cartesian coordinates, inclusive.  The rectangle will include all
    /// hexagonal grid positions that intersect the rectangle defined by the
    /// given Cartesian coordinates.
    pub fn cartesian(min: Cartesian, max: Cartesian) -> Self {
        let (min_x, min_y) = min;
        let (max_x, max_y) = max;
        Self::new(
            ((min_x + HEX_WIDTH / 2.0) / HEX_HORIZONTAL_SPACING).floor() as HexCoord,
            (min_y / HEX_VERTICAL_SPACING).floor() as HexCoord,
            ((max_x - HEX_WIDTH / 2.0) / HEX_HORIZONTAL_SPACING).ceil() as HexCoord,
            (max_y / HEX_VERTICAL_SPACING).ceil() as HexCoord,
        )
    }

    /// The width of the region in terms of the number of hexes in the u direction.
    pub fn width(&self) -> HexCoord {
        0.max(self.max_u - self.min_u + 1)
    }

    /// The height of the region in terms of the number of hexes in the v direction.
    pub fn height(&self) -> HexCoord {
        0.max(self.max_v - self.min_v + 1)
    }

    /// Tests whether the region is empty (i.e. it contains no hexes).
    pub fn is_empty(&self) -> bool {
        self.min_u > self.max_u
            || self.min_v > self.max_v
            || self.min_v == self.max_v
                && self.min_u == self.max_u
                && (self.min_u + self.min_v) % 2 != 0
    }

    /// Return true iff a grid of this size contains the specified edge
    /// position.  This is true if the edge is part of a hex in the grid, or if
    /// the edge is on the boundary of the grid.
    pub fn contains_edge(&self, edge_pos: HexEdgePos) -> bool {
        if self.contains_hex(edge_pos.pos()) {
            return true;
        }
        if self.is_empty() {
            return false;
        }

        let (pos, edge) = edge_pos.norm().pos_edge();
        let (u, v) = pos.u_v();
        let Self {
            min_u,
            min_v,
            max_u,
            max_v,
        } = *self;

        // Handle the special case where the rectangle has only one row of
        // hexes.  This case is strange because none of the hexes are neighbors
        // of each other.
        if min_v == max_v {
            return match edge {
                NormHexEdge::TopRight if v == min_v - 1 => (min_u - 1..=max_u - 1).contains(&u),
                NormHexEdge::Top if v == min_v - 2 => (min_u..=max_u).contains(&u),
                NormHexEdge::TopLeft if v == min_v - 1 => (min_u + 1..=max_u + 1).contains(&u),
                _ => self.contains_hex(pos),
            };
        }

        match edge {
            NormHexEdge::TopRight if u == min_u - 1 => (min_v - 1..max_v).contains(&v),
            NormHexEdge::TopRight if v == min_v - 1 => {
                u % 2 != 0 && (min_u - 1..max_u).contains(&u)
            }
            NormHexEdge::Top if (min_v - 2..min_v).contains(&v) => (min_u..=max_u).contains(&u),
            NormHexEdge::TopLeft if u == max_u + 1 => (min_v - 1..max_v).contains(&v),
            NormHexEdge::TopLeft if v == min_v - 1 => {
                u % 2 != 0 && (min_u + 1..=max_u).contains(&u)
            }
            _ => self.contains_hex(pos),
        }
    }

    /// Return true iff a grid of this size contains the specified corner
    /// position.  This is true if the corner is part of a hex in the grid, or if
    /// the corner is on the boundary of the grid.
    pub fn contains_corner(&self, corner_pos: HexCornerPos) -> bool {
        if self.contains_hex(corner_pos.pos()) {
            return true;
        }
        if self.is_empty() {
            return false;
        }

        let (pos, corner) = corner_pos.norm().pos_corner();
        let (u, v) = pos.u_v();
        let Self {
            min_u,
            min_v,
            max_u,
            max_v,
        } = *self;

        // Handle the special case where the rectangle has only one row of
        // hexes.  This case is strange because none of the hexes are neighbors
        // of each other.
        if min_v == max_v {
            return match corner {
                NormHexCorner::TopRight if v == min_v - 1 => (min_u - 1..=max_u - 1).contains(&u),
                NormHexCorner::TopLeft if v == min_v - 1 => (min_u + 1..=max_u + 1).contains(&u),
                _ if v == min_v - 2 => (min_u..=max_u).contains(&u),
                _ => self.contains_hex(pos),
            };
        }

        let corner_u_matches = match corner {
            NormHexCorner::TopRight => u == min_u - 1,
            NormHexCorner::TopLeft => u == max_u + 1,
        };
        corner_u_matches && (min_v - 1..max_v).contains(&v)
            || (min_v - 2..=min_v).contains(&v)
                && (min_u..=max_u).contains(&u)
                && max_v - min_v + 1 > 0
            || self.contains_hex(pos)
    }

    /// Gets an interator over the edges of all hexagons in the rectangle.
    pub fn iter_edges(&self) -> HexRectangleEdgeIterator {
        HexRectangleEdgeIterator::new(*self)
    }

    /// Gets an interator over the corners of all hexagons in the rectangle.
    pub fn iter_corners(&self) -> HexRectangleCornerIterator {
        HexRectangleCornerIterator::new(*self)
    }

    #[cfg(test)]
    fn inflate(self) -> Self {
        Self {
            min_u: self.min_u - 1,
            min_v: self.min_v - 1,
            max_u: self.max_u + 1,
            max_v: self.max_v + 1,
        }
    }
}

impl Default for HexRectangle {
    /// Creates a new empty rectangle.
    fn default() -> Self {
        Self {
            min_u: 0,
            min_v: 0,
            max_u: -1,
            max_v: -1,
        }
    }
}

impl HexPosContainer for HexRectangle {
    type Iterator<'c> = HexRectangleIterator;

    fn contains_hex(&self, pos: HexPos) -> bool {
        pos.u() >= self.min_u
            && pos.u() <= self.max_u
            && pos.v() >= self.min_v
            && pos.v() <= self.max_v
            && !self.is_empty()
    }

    fn iter_hexes(&self) -> Self::Iterator<'_> {
        HexRectangleIterator::new(*self)
    }

    fn len(&self) -> usize {
        (self.max_u - self.min_u + 1) as usize * (self.max_v - self.min_v + 1) as usize / 2
            + ((self.max_u - self.min_u + 1) as usize * (self.max_v - self.min_v + 1) as usize % 2)
    }
}

impl IntoIterator for HexRectangle {
    type Item = HexPos;
    type IntoIter = HexRectangleIterator;

    fn into_iter(self) -> Self::IntoIter {
        HexRectangleIterator::new(self)
    }
}

impl IntoIterator for &HexRectangle {
    type Item = HexPos;
    type IntoIter = HexRectangleIterator;

    fn into_iter(self) -> Self::IntoIter {
        HexRectangleIterator::new(*self)
    }
}

/// The type of iterator returned by [`HexRectangle::iter_hexes`].
pub struct HexRectangleIterator {
    u: HexCoord,
    v: HexCoord,
    min_u: HexCoord,
    max_u: HexCoord,
    max_v: HexCoord,
}

impl HexRectangleIterator {
    fn new(region: HexRectangle) -> Self {
        let HexRectangle {
            min_u,
            min_v,
            max_u,
            max_v,
        } = region;
        Self {
            u: min_u,
            v: min_v,
            min_u,
            max_u,
            max_v,
        }
    }
}

impl Iterator for HexRectangleIterator {
    type Item = HexPos;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;
        while result.is_none() && self.v <= self.max_v && self.u <= self.max_u {
            if (self.u + self.v) % 2 != 0 {
                self.u += 1;
            }
            if self.u <= self.max_u {
                result = Some(HexPos::new(self.u, self.v));
            }
            self.u += 1;
            if self.u > self.max_u {
                self.u = self.min_u;
                self.v += 1;
            }
        }

        result
    }
}

pub struct HexRectangleEdgeIterator {
    min_u: HexCoord,
    min_v: HexCoord,
    max_u: HexCoord,
    edge: HexEdge,
    pos: Option<HexPos>,
    pos_iter: HexRectangleIterator,
}

impl HexRectangleEdgeIterator {
    fn new(rect: HexRectangle) -> Self {
        let mut pos_iter = rect.iter_hexes();
        let pos = pos_iter.next();
        Self {
            min_u: rect.min_u,
            min_v: rect.min_v,
            max_u: rect.max_u,
            edge: HexEdge::TopRight,
            pos,
            pos_iter,
        }
    }
}

impl Iterator for HexRectangleEdgeIterator {
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
                HexEdge::BottomLeft => pos.u() == self.min_u || pos.v() == self.min_v,
                HexEdge::Bottom => pos.v() <= self.min_v + 1,
                HexEdge::BottomRight => pos.v() == self.min_v || pos.u() == self.max_u,
            };
            if is_valid_edge {
                return Some(HexEdgePos::from((pos, edge)));
            }
        }
    }
}

pub struct HexRectangleCornerIterator {
    min_u: HexCoord,
    min_v: HexCoord,
    max_u: HexCoord,
    max_v: HexCoord,
    corner: HexCorner,
    pos: Option<HexPos>,
    pos_iter: HexRectangleIterator,
}

impl HexRectangleCornerIterator {
    fn new(rect: HexRectangle) -> Self {
        let mut pos_iter = rect.iter_hexes();
        let pos = pos_iter.next();
        Self {
            min_u: rect.min_u,
            min_v: rect.min_v,
            max_u: rect.max_u,
            max_v: rect.max_v,
            corner: HexCorner::TopRight,
            pos,
            pos_iter,
        }
    }
}

impl Iterator for HexRectangleCornerIterator {
    type Item = HexCornerPos;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let pos = self.pos?;
            let corner = self.corner;
            self.corner = self.corner.rotate(1);
            if self.corner == HexCorner::TopRight {
                self.pos = self.pos_iter.next();
            }
            let Self {
                min_u,
                min_v,
                max_u,
                max_v,
                ..
            } = *self;
            let is_valid_corner = match corner {
                HexCorner::TopRight | HexCorner::TopLeft => true,
                HexCorner::Right => pos.u() == max_u,
                HexCorner::Left => pos.u() == min_u,
                HexCorner::BottomLeft if min_v % 2 == 0 => {
                    pos.v() == min_v && pos.u() % 2 == 0 || pos.v() == min_v + 1 && pos.u() % 2 != 0
                }
                HexCorner::BottomLeft => {
                    pos.v() == min_v + 1 && pos.u() % 2 == 0 || pos.v() == min_v && pos.u() % 2 != 0
                }
                HexCorner::BottomRight => {
                    pos.v() < min_v + 2
                        || (max_u % 2 != 0 && pos.v() == min_v + 1 && pos.u() == max_u)
                        || (max_u % 2 == 0 && pos.v() == min_v + 1 && pos.u() == min_u)
                }
            };
            if is_valid_corner || min_v == max_v {
                return Some(HexCornerPos::from((pos, corner)));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_eq_sets, assert_unique};
    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;
    use std::collections::HashSet;

    impl Arbitrary for HexRectangle {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            if u8::arbitrary(g) % 16 == 0 {
                return Self::default();
            }
            let min_u = (u8::arbitrary(g) % 4) as HexCoord;
            let min_v = (u8::arbitrary(g) % 4) as HexCoord;
            let max_u = min_u + ((u8::arbitrary(g) % 4) as HexCoord);
            let max_v = min_v + ((u8::arbitrary(g) % 4) as HexCoord);
            Self::new(min_u, min_v, max_u, max_v)
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let width = self.width();
            let height = self.height();
            let min_u = self.min_u;
            let min_v = self.min_v;
            let mut items = Vec::new();
            if width > height {
                items.push(HexRectangle::new(
                    min_u,
                    min_v,
                    min_u + width - 2,
                    min_v + height - 1,
                ));
            } else if height > 0 {
                items.push(HexRectangle::new(
                    min_u,
                    min_v,
                    min_u + width - 1,
                    min_v + height - 2,
                ));
            }
            Box::new(items.into_iter())
        }
    }

    #[test]
    fn empty_rect() {
        let rect = HexRectangle::default();
        assert_eq!(rect.width(), 0);
        assert_eq!(rect.height(), 0);
        assert!(rect.iter_hexes().next().is_none());
        assert!(rect.iter_edges().next().is_none());
        assert!(rect.iter_corners().next().is_none());
    }

    #[test]
    fn rect1_nonempty() {
        let rect = HexRectangle::new(0, 0, 0, 0);
        assert!(!rect.is_empty());
        assert_eq!(rect.width(), 1);
        assert_eq!(rect.height(), 1);
        assert_eq!(rect.iter_hexes().count(), 1);
        assert_eq!(rect.iter_edges().count(), 6);
        assert_eq!(rect.iter_corners().count(), 6);

        assert!(rect.contains_hex(HexPos::ORIGIN));
        for edge in HexEdge::ALL {
            assert!(rect.contains_edge(HexEdgePos::from((HexPos::ORIGIN, edge))));
            let neighbor = HexPos::ORIGIN.neighbor(edge);
            let neighbor_edge = edge.opposite();
            assert!(!rect.contains_hex(neighbor));
            for other_edge in HexEdge::ALL {
                assert_eq!(
                    other_edge == neighbor_edge,
                    rect.contains_edge(HexEdgePos::from((neighbor, other_edge))),
                    "edge: {:?}, other_edge: {:?}, neighbor: {:?}",
                    edge,
                    other_edge,
                    neighbor
                );
            }
        }
        for corner in HexCorner::ALL {
            assert!(rect.contains_corner(HexCornerPos::from((HexPos::ORIGIN, corner))));
            for other_corner in HexCorner::ALL {
                for (neighbor, neighbor_corner) in HexPos::ORIGIN.neighbors_at_corner(corner) {
                    if other_corner == neighbor_corner {
                        assert!(
                            rect.contains_corner(HexCornerPos::from((neighbor, neighbor_corner)))
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn rect1_empty() {
        let rect = HexRectangle::new(1, 0, 1, 0);
        assert!(rect.is_empty());
        assert_eq!(rect.width(), 1);
        assert_eq!(rect.height(), 1);
        assert_eq!(rect.iter_hexes().count(), 0);
        assert_eq!(rect.iter_edges().count(), 0);
        assert_eq!(rect.iter_corners().count(), 0);
    }

    #[quickcheck]
    fn hex_iterator(size: HexRectangle) {
        let expected = (size.min_v..=size.max_v)
            .flat_map(|v| {
                (size.min_u..=size.max_u)
                    .filter(move |&u| (u + v) % 2 == 0)
                    .map(move |u| HexPos::new(u, v))
            })
            .collect::<HashSet<_>>();
        let actual = size.iter_hexes().collect::<HashSet<_>>();
        assert_eq!(expected, actual);
    }

    #[quickcheck]
    fn contains_edge(rect: HexRectangle) {
        let mut expected = HashSet::new();
        for pos in rect.iter_hexes() {
            for edge in HexEdge::ALL {
                expected.insert(HexEdgePos::from((pos, edge)).norm());
            }
        }
        let mut actual = HashSet::new();
        for pos in rect.inflate().iter_hexes() {
            for edge in HexEdge::ALL {
                let edge_pos = HexEdgePos::from((pos, edge));
                if rect.contains_edge(edge_pos) {
                    actual.insert(edge_pos.norm());
                }
            }
        }
        assert_eq_sets!(expected, actual);
    }

    #[quickcheck]
    fn contains_corner(rect: HexRectangle) {
        let mut expected = HashSet::new();
        for pos in rect.iter_hexes() {
            for corner in HexCorner::ALL {
                expected.insert(HexCornerPos::from((pos, corner)).norm());
            }
        }
        let mut actual = HashSet::new();
        for pos in rect.inflate().iter_hexes() {
            for corner in HexCorner::ALL {
                let corner_pos = HexCornerPos::from((pos, corner));
                if rect.contains_corner(corner_pos) {
                    actual.insert(corner_pos.norm());
                }
            }
        }
        assert_eq!(expected, actual);
    }

    #[quickcheck]
    fn edge_iterator1(rect: HexRectangle) {
        let mut expected = HashSet::new();
        for pos in rect.iter_hexes() {
            for edge in HexEdge::ALL {
                expected.insert(HexEdgePos::from((pos, edge)).norm());
            }
        }
        let actual = rect.iter_edges().map(|e| e.norm()).collect::<Vec<_>>();
        assert_unique!(actual.clone());
        assert_eq_sets!(expected, actual);
    }

    #[quickcheck]
    fn edge_iterator2(rect: HexRectangle) {
        let mut expected = HashSet::new();
        for pos in rect.inflate().iter_hexes() {
            for edge in HexEdge::ALL {
                let edge_pos = HexEdgePos::from((pos, edge));
                if rect.contains_edge(edge_pos) {
                    expected.insert(edge_pos.norm());
                }
            }
        }
        let actual = rect.iter_edges().map(|e| e.norm()).collect::<Vec<_>>();
        assert_unique!(actual.clone());
        assert_eq_sets!(expected, actual);
    }

    #[quickcheck]
    fn corner_iterator1(rect: HexRectangle) {
        let mut expected = HashSet::new();
        for pos in rect.iter_hexes() {
            for corner in HexCorner::ALL {
                expected.insert(HexCornerPos::from((pos, corner)).norm());
            }
        }
        let actual = rect.iter_corners().map(|c| c.norm()).collect::<Vec<_>>();
        assert_unique!(actual.clone());
        assert_eq_sets!(expected, actual);
    }

    #[quickcheck]
    fn corner_iterator2(rect: HexRectangle) {
        let mut expected = HashSet::new();
        for pos in rect.inflate().iter_hexes() {
            for corner in HexCorner::ALL {
                let corner_pos = HexCornerPos::from((pos, corner));
                if rect.contains_corner(corner_pos) {
                    expected.insert(corner_pos.norm());
                }
            }
        }
        let actual = rect.iter_corners().map(|c| c.norm()).collect::<Vec<_>>();
        assert_unique!(actual.clone());
        assert_eq_sets!(expected, actual);
    }
}
