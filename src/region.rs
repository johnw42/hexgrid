use crate::{
    Cartesian, HEX_HEIGHT, HEX_WIDTH, HexCoord, HexCornerPos, HexEdgePos, HexPos, HexPosContainer,
    NormHexCorner, NormHexEdge,
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

    /// Return true iff the region contains the specified edge
    /// position.  This is true if the edge is part of a hex in the region, or if
    /// the edge is on the boundary of the region.
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
            NormHexEdge::TopRight => u == min_u - 1 && (min_v - 1..max_v).contains(&v),
            NormHexEdge::Top => (min_u..=max_u).contains(&u) && (min_v - 1..=min_v).contains(&v),
            NormHexEdge::TopLeft => u == max_u + 1 && (min_v - 1..max_v).contains(&v),
        }
    }

    /// Return true iff the region contains the specified corner
    /// position.  This is true if the corner is part of a hex in the region, or if
    /// the corner is on the boundary of the region.
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
            || (min_v - 2..=min_v).contains(&v)
                && (min_u..=max_u).contains(&u)
                && (max_v - min_v + 1) > 0
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
    pub fn new_cartesian(min: Cartesian, max: Cartesian) -> Self {
        let (min_x, min_y) = min;
        let (max_x, max_y) = max;
        let (min_u, min_v) =
            HexPos::from_center((min_x - HEX_WIDTH / 2.0, min_y - HEX_WIDTH / 2.0)).u_v();
        let (max_u, max_v) =
            HexPos::from_center((max_x + HEX_HEIGHT / 2.0, max_y + HEX_HEIGHT / 2.0)).u_v();
        Self::new(HexRegion {
            min_u,
            min_v,
            max_u,
            max_v,
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
