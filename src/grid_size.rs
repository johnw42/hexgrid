use crate::{
    HexCoord,
    container::HexPosContainer,
    corner::{HexCorner, NormHexCorner},
    corner_pos::HexCornerPos,
    edge::{HexEdge, NormHexEdge},
    edge_pos::HexEdgePos,
    pos::{HexPos, HexPosIterator},
};
use std::fmt::Display;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexGridSize {
    width: HexCoord,
    height: HexCoord,
}

/// A possible error that can occur when creating a [`HexGridSize`].
#[derive(Debug, thiserror::Error)]
pub enum HexGridSizeError {
    /// The width of the grid is negative.
    #[error("Negative width: {0}")]
    NegativeWidth(HexCoord),
    /// The height of the grid is negative.
    #[error("Negative height: {0}")]
    NegativeHeight(HexCoord),
    /// The width of the grid is zero but the height is non-zero.
    #[error("Width is zero but height is non-zero: {0}")]
    WidthZeroHeightNonZero(HexCoord),
    /// The height of the grid is zero but the width is non-zero.
    #[error("Height is zero but width is non-zero: {0}")]
    HeightZeroWidthNonZero(HexCoord),
    /// The width of the grid is greater than one but the height is less than two.  This is an error
    /// because the hexes in the grid would not be contiguous.
    #[error("Width greater than one but height less than two: width={0}, height={1}")]
    WidthGreaterThanOneHeightLessThanTwo(HexCoord, HexCoord),
    /// The width of the grid is one but the height is even.  This is an error because
    /// using `height - 1` would produce a grid of the same dimensions.
    #[error("Width is one but height is even: height={0}")]
    WidthOneHeightEven(HexCoord),
}

/// The size of a [`HexGrid`] in terms of the number of hexes in the u and v
/// directions.
///
/// Some sizes are invalid, for example a width of 1 and a height of 3 is
/// invalid because the hexes would not be able to form a proper grid. See
/// [`HexGridSize::new`] for more information on valid sizes.
impl HexGridSize {
    /// Creates a new `HexGridSize` with the given width and height.  The width
    /// and height must be non-negative, and the width and height must be
    /// compatible with a hexagonal grid.
    pub const fn new(width: HexCoord, height: HexCoord) -> Result<Self, HexGridSizeError> {
        if width < 0 {
            return Err(HexGridSizeError::NegativeWidth(width));
        }
        if height < 0 {
            return Err(HexGridSizeError::NegativeHeight(height));
        }
        if height == 0 && width != 0 {
            return Err(HexGridSizeError::WidthZeroHeightNonZero(height));
        }
        if width == 0 && height != 0 {
            return Err(HexGridSizeError::HeightZeroWidthNonZero(width));
        }
        if width > 1 && height <= 1 {
            return Err(HexGridSizeError::WidthGreaterThanOneHeightLessThanTwo(
                width, height,
            ));
        }
        if width == 1 && height % 2 == 0 {
            return Err(HexGridSizeError::WidthOneHeightEven(height));
        }
        Ok(Self { width, height })
    }

    /// The width of the grid in terms of the number of hexes in the u direction.
    pub const fn width(&self) -> HexCoord {
        self.width
    }

    /// The height of the grid in terms of the number of hexes in the v direction.
    pub const fn height(&self) -> HexCoord {
        self.height
    }

    /// Unpacks the width and height of the grid into a tuple.
    pub const fn unpack(&self) -> (HexCoord, HexCoord) {
        (self.width, self.height)
    }

    /// Returns the number of hexes in a row of the grid that has an even v coordinate.
    pub const fn even_row_size(&self) -> HexCoord {
        1 + (self.width - 1) / 2
    }

    /// Returns the number of hexes in a row of the grid that has an odd v coordinate.
    pub const fn odd_row_size(&self) -> HexCoord {
        self.width / 2
    }

    /// Return true iff the grid of this size contains the specified edge
    /// position.  This is true if the edge is part of a hex in the grid, or if
    /// the edge is on the boundary of the grid.
    pub fn contains_edge(&self, edge_pos: HexEdgePos) -> bool {
        if self.contains_hex(edge_pos.pos()) {
            return true;
        }

        let (pos, edge): (HexPos, NormHexEdge) = edge_pos.norm();
        let (u, v) = pos.u_v();
        let (width, height) = self.unpack();

        match edge {
            NormHexEdge::TopRight => u == -1 && (-1..height - 1).contains(&v),
            NormHexEdge::Top => (0..width).contains(&u) && (-1..=0).contains(&v),
            NormHexEdge::TopLeft => u == width && (-1..height - 1).contains(&v),
        }
    }

    /// Return true iff the grid of this size contains the specified corner
    /// position.  This is true if the corner is part of a hex in the grid, or if
    /// the corner is on the boundary of the grid.
    pub fn contains_corner(&self, corner_pos: HexCornerPos) -> bool {
        if self.contains_hex(corner_pos.pos()) {
            return true;
        }

        let (pos, corner): (HexPos, NormHexCorner) = corner_pos.norm();
        let (u, v) = pos.u_v();
        let (width, height) = self.unpack();

        let corner_u_matches = match corner {
            NormHexCorner::TopRight => u == -1,
            NormHexCorner::TopLeft => u == width,
        };
        corner_u_matches && (-1..height - 1).contains(&v)
            || (-2..=0).contains(&v) && (0..width).contains(&u) && height > 0
    }

    #[cfg(test)]
    pub fn test_sizes() -> impl Iterator<Item = Self> {
        let mut sizes = (0..=9)
            .flat_map(|width| (0..=9).map(move |height| (width, height)))
            .filter_map(|(width, height)| Self::new(width, height).ok())
            .collect::<Vec<_>>();
        sizes.sort_by_key(|size| (size.width + size.height, size.width));
        sizes.into_iter()
    }
}

impl Display for HexGridSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}✕{}", self.width, self.height)
    }
}

impl HexPosContainer for HexGridSize {
    type Iterator<'c> = HexPosIterator;

    fn contains_hex(&self, pos: HexPos) -> bool {
        pos.u() >= 0 && pos.u() < self.width && pos.v() >= 0 && pos.v() < self.height
    }

    fn iter_hexes(&self) -> Self::Iterator<'_> {
        HexPosIterator::new(0, 0, self.width - 1, self.height - 1)
    }

    fn len(&self) -> usize {
        self.width as usize * self.height as usize / 2
            + (self.width as usize * self.height as usize % 2)
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

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;
    use std::collections::HashSet;

    impl Arbitrary for HexGridSize {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            loop {
                let (width, height) = if bool::arbitrary(g) {
                    // Make sure small sizes are checked more often, since they are
                    // more likely to have edge cases.
                    (HexCoord::arbitrary(g) % 3, HexCoord::arbitrary(g) % 3)
                } else {
                    (HexCoord::arbitrary(g) % 64, HexCoord::arbitrary(g) % 64)
                };
                if let Ok(size) = HexGridSize::new(width, height) {
                    return size;
                }
            }
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let Self { width, height } = *self;
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
                    .filter_map(|(w, h)| HexGridSize::new(w, h).ok()),
            )
        }
    }

    #[test]
    fn shrink() {
        let size = HexGridSize::new(4, 4).unwrap();
        let shrunk_sizes: Vec<_> = size.shrink().collect();
        let expected_sizes: Vec<_> = [
            (4, 3),
            (3, 4),
            (3, 3),
            (3, 2),
            (2, 3),
            (2, 2),
            (2, 1),
            (1, 2),
            (1, 1),
            (1, 0),
            (0, 1),
            (0, 0),
        ]
        .into_iter()
        .filter_map(|(w, h)| HexGridSize::new(w, h).ok())
        .collect();
        assert_eq!(shrunk_sizes, expected_sizes);
    }

    #[quickcheck]
    fn len(size: HexGridSize) {
        assert_eq!(size.len(), size.iter_hexes().count());
    }

    #[quickcheck]
    fn edge_iterator(size: HexGridSize) {
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

    #[quickcheck]
    fn corner_iterator(size: HexGridSize) {
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
