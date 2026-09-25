use crate::{
    Cartesian, HEX_HEIGHT, HEX_HORIZONTAL_SPACING, HEX_VERTICAL_SPACING, HEX_WIDTH, HexCoord,
    HexCorner, HexCornerPos, HexEdge, HexEdgePos, HexPos, HexPosContainer, NormHexCorner,
    NormHexEdge,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexRegion {
    min_u: HexCoord,
    min_v: HexCoord,
    max_u: HexCoord,
    max_v: HexCoord,
}

impl HexRegion {
    pub fn new(min_u: HexCoord, min_v: HexCoord, max_u: HexCoord, max_v: HexCoord) -> Self {
        Self {
            min_u,
            min_v,
            max_u,
            max_v,
        }
    }

    /// The width of the region in terms of the number of hexes in the u direction.
    pub const fn width(&self) -> HexCoord {
        self.max_u - self.min_u + 1
    }

    /// The height of the region in terms of the number of hexes in the v direction.
    pub const fn height(&self) -> HexCoord {
        self.max_v - self.min_v + 1
    }

    /// Return true iff the grid of this size contains the specified edge
    /// position.  This is true if the edge is part of a hex in the grid, or if
    /// the edge is on the boundary of the grid.
    pub fn contains_edge(&self, edge_pos: HexEdgePos) -> bool {
        if self.contains_hex(edge_pos.pos()) {
            return true;
        }

        let (pos, edge) = edge_pos.norm().pos_edge();
        let (u, v) = pos.u_v();
        let Self {
            min_u,
            min_v,
            max_u,
            max_v,
        } = *self;

        match edge {
            NormHexEdge::TopRight if u == min_u - 1 => (min_v - 1..max_v).contains(&v),
            NormHexEdge::TopRight if v == min_v - 1 => {
                u % 2 != 0 && (min_u - 1..max_u).contains(&u)
            }
            NormHexEdge::Top if (min_v - 2..min_v).contains(&v) => (min_u..=max_u).contains(&u),
            NormHexEdge::TopLeft if u == max_u + 1 => (min_v - 1..max_v).contains(&v),
            NormHexEdge::TopLeft if v == min_v - 1 => u % 2 != 0 && (min_u..=max_u).contains(&u),
            _ => self.contains_hex(pos),
        }
    }

    /// Return true iff the grid of this size contains the specified corner
    /// position.  This is true if the corner is part of a hex in the grid, or if
    /// the corner is on the boundary of the grid.
    pub fn contains_corner(&self, corner_pos: HexCornerPos) -> bool {
        if self.contains_hex(corner_pos.pos()) {
            return true;
        }

        let (pos, corner) = corner_pos.norm().pos_corner();
        let (u, v) = pos.u_v();
        let Self {
            min_u,
            min_v,
            max_u,
            max_v,
        } = *self;

        let corner_u_matches = match corner {
            NormHexCorner::TopRight => u == min_u - 1,
            NormHexCorner::TopLeft => u == max_u + 1,
        };
        corner_u_matches && (min_v - 1..max_v).contains(&v)
            || (-2..=0).contains(&v) && (min_u..=max_u).contains(&u) && max_v - min_v + 1 > 0
            || self.contains_hex(pos)
    }

    pub fn iter_edges(&self) -> HexEdgeIterator {
        HexEdgeIterator::new(*self)
    }

    pub fn iter_corners(&self) -> HexCornerIterator {
        HexCornerIterator::new(*self)
    }
}

impl HexPosContainer for HexRegion {
    type Iterator<'c> = HexRegionIterator;

    fn contains_hex(&self, pos: HexPos) -> bool {
        pos.u() >= self.min_u
            && pos.u() <= self.max_u
            && pos.v() >= self.min_v
            && pos.v() <= self.max_v
    }

    fn iter_hexes(&self) -> Self::Iterator<'_> {
        HexRegionIterator::new(*self)
    }

    fn len(&self) -> usize {
        (self.max_u - self.min_u + 1) as usize * (self.max_v - self.min_v + 1) as usize / 2
            + ((self.max_u - self.min_u + 1) as usize * (self.max_v - self.min_v + 1) as usize % 2)
    }
}

/// An iterator over all hexagonal grid positions in a rectangular area, in
/// row-major order, starting with the bottom-left corner.  The area is defined
/// by the minimum and maximum u and v coordinates, inclusive.  The iterator
/// will only return positions where u + v is even, as required by the hexagonal
/// grid coordinate system.
pub struct HexRegionIterator {
    u: HexCoord,
    v: HexCoord,
    min_u: HexCoord,
    max_u: HexCoord,
    max_v: HexCoord,
}

impl HexRegionIterator {
    /// Creates a new iterator that will iterate over all hexagonal grid
    /// positions in the rectangular area defined by the given minimum and
    /// maximum u and v coordinates, inclusive.  The iterator will only return
    /// positions where u + v is even, as required by the hexagonal grid
    /// coordinate system.  If `min_u` > `max_u` or `min_v` > `max_v`, the
    /// iterator will be empty.
    pub fn new(region: HexRegion) -> Self {
        let HexRegion {
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

    /// Creates a new iterator that will iterate over all hexagonal grid
    /// positions in the rectangular area defined by the given minimum and
    /// maximum Cartesian coordinates, inclusive.  The iterator will include all
    /// hexagonal grid positions that intersect the rectangle defined by the
    /// given Cartesian coordinates.
    pub fn cartesian(min: Cartesian, max: Cartesian) -> Self {
        let (min_x, min_y) = min;
        let (max_x, max_y) = max;
        Self::new(HexRegion {
            min_u: ((min_x + HEX_WIDTH / 2.0) / HEX_HORIZONTAL_SPACING).floor() as HexCoord,
            min_v: (min_y / HEX_VERTICAL_SPACING).floor() as HexCoord,
            max_u: ((max_x - HEX_WIDTH / 2.0) / HEX_HORIZONTAL_SPACING).ceil() as HexCoord,
            max_v: (max_y / HEX_VERTICAL_SPACING).ceil() as HexCoord,
        })
    }
}

impl Iterator for HexRegionIterator {
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

pub struct HexEdgeIterator {
    min_u: HexCoord,
    min_v: HexCoord,
    max_u: HexCoord,
    edge: HexEdge,
    pos: Option<HexPos>,
    pos_iter: HexRegionIterator,
}

impl HexEdgeIterator {
    pub fn new(region: HexRegion) -> Self {
        let mut pos_iter = region.iter_hexes();
        let pos = pos_iter.next();
        Self {
            min_u: region.min_u,
            min_v: region.min_v,
            max_u: region.max_u,
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

pub struct HexCornerIterator {
    min_u: HexCoord,
    min_v: HexCoord,
    max_u: HexCoord,
    max_v: HexCoord,
    corner: HexCorner,
    pos: Option<HexPos>,
    pos_iter: HexRegionIterator,
}

impl HexCornerIterator {
    pub fn new(region: HexRegion) -> Self {
        let mut pos_iter = region.iter_hexes();
        let pos = pos_iter.next();
        Self {
            min_u: region.min_u,
            min_v: region.min_v,
            max_u: region.max_u,
            max_v: region.max_v,
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
    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;
    use std::collections::HashSet;

    impl Arbitrary for HexRegion {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let min_u = HexCoord::arbitrary(g) % 4;
            let min_v = HexCoord::arbitrary(g) % 4;
            let max_u = min_u + HexCoord::arbitrary(g).rem_euclid(4);
            let max_v = min_v + HexCoord::arbitrary(g).rem_euclid(4);
            Self::new(min_u, min_v, max_u, max_v)
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let width = self.width();
            let height = self.height();
            let min_u = self.min_u;
            let min_v = self.min_v;
            Box::new(
                (1..=(width + height))
                    .flat_map(move |shrink_amount| {
                        if shrink_amount % 2 == 0 {
                            vec![(width - shrink_amount / 2, height - shrink_amount / 2)]
                                .into_iter()
                        } else {
                            vec![
                                (width - shrink_amount / 2, height - (shrink_amount + 1) / 2),
                                (width - (shrink_amount + 1) / 2, height - shrink_amount / 2),
                            ]
                            .into_iter()
                        }
                    })
                    .map(move |(w, h)| HexRegion::new(min_u, min_v, min_u + w - 1, min_v + h - 1)),
            )
        }
    }

    #[quickcheck]
    fn edge_iterator(region: HexRegion) {
        let mut seen_edges = HashSet::new();
        for pos in region.iter_hexes() {
            for edge in HexEdge::ALL {
                seen_edges.insert(HexEdgePos::from((pos, edge)).norm());
            }
        }
        let iter_edges = region.iter_edges().map(|e| e.norm()).collect::<Vec<_>>();
        assert_eq!(seen_edges.len(), iter_edges.len());
        assert_eq!(seen_edges, iter_edges.into_iter().collect::<HashSet<_>>());
    }

    #[quickcheck]
    fn corner_iterator(region: HexRegion) {
        let mut expected_corners = HashSet::new();
        for pos in region.iter_hexes() {
            for corner in HexCorner::ALL {
                expected_corners.insert(HexCornerPos::from((pos, corner)).norm());
            }
        }
        let actual_corners = region.iter_corners().collect::<Vec<_>>();
        assert_eq!(
            expected_corners.clone(),
            actual_corners
                .clone()
                .iter()
                .map(|c| c.norm())
                .collect::<HashSet<_>>()
        );
        assert_eq!(expected_corners.len(), actual_corners.len());
    }
}
