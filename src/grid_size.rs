use crate::{
    HexCoord, HexRegion,
    container::HexPosContainer,
    corner_pos::HexCornerPos,
    edge_pos::HexEdgePos,
    pos::HexPos,
    region::{HexCornerIterator, HexEdgeIterator, HexRegionIterator},
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
        HexRegion::from(*self).contains_edge(edge_pos)
    }

    /// Return true iff the grid of this size contains the specified corner
    /// position.  This is true if the corner is part of a hex in the grid, or if
    /// the corner is on the boundary of the grid.
    pub fn contains_corner(&self, corner_pos: HexCornerPos) -> bool {
        HexRegion::from(*self).contains_corner(corner_pos)
    }

    pub fn iter_edges(&self) -> HexEdgeIterator {
        HexRegion::from(*self).iter_edges()
    }

    pub fn iter_corners(&self) -> HexCornerIterator {
        HexRegion::from(*self).iter_corners()
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

impl From<HexGridSize> for HexRegion {
    fn from(size: HexGridSize) -> Self {
        Self::new(0, 0, size.width - 1, size.height - 1)
    }
}

impl HexPosContainer for HexGridSize {
    type Iterator<'c> = HexRegionIterator;

    fn contains_hex(&self, pos: HexPos) -> bool {
        HexRegion::from(*self).contains_hex(pos)
    }

    fn iter_hexes(&self) -> Self::Iterator<'_> {
        HexRegionIterator::new(HexRegion::from(*self))
    }

    fn len(&self) -> usize {
        HexRegion::from(*self).len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{HexCorner, HexEdge, HexPerimeterIterator};
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

    #[test]
    fn contains_edge1() {
        let size = HexGridSize::new(1, 1).unwrap();
        for edge in HexEdge::ALL {
            let pos = HexPos::new(0, 0);
            let edge_pos = HexEdgePos::from((pos, edge));
            assert!(
                size.contains_edge(edge_pos),
                "size: {}, edge_pos: {:?}, norm: {:?}",
                size,
                edge_pos,
                edge_pos.norm(),
            );
            let neighbor = pos.neighbor(edge);
            let neighbor_edge = edge.opposite();
            let edge_pos = HexEdgePos::from((neighbor, neighbor_edge));
            assert!(
                size.contains_edge(edge_pos),
                "size: {}, edge_pos: {:?}, norm: {:?}",
                size,
                edge_pos,
                edge_pos.norm()
            );
        }
    }

    #[quickcheck]
    fn contains_edge(size: HexGridSize) {
        for edge_pos in HexPerimeterIterator::new(&size) {
            assert!(size.contains_edge(edge_pos));
            let neighbor_edge = HexEdgePos::from((
                edge_pos.pos().neighbor(edge_pos.edge()),
                edge_pos.edge().opposite(),
            ));
            assert!(
                size.contains_edge(neighbor_edge),
                "size: {}, edge_pos: {:?}, neighbor_edge: {:?}, norm: {:?}",
                size,
                edge_pos,
                neighbor_edge,
                neighbor_edge.norm()
            );
        }
    }

    #[test]
    fn contains_corner1() {
        let size = HexGridSize::new(1, 1).unwrap();
        for corner in HexCorner::ALL {
            let pos = HexPos::new(0, 0);
            assert!(size.contains_corner(HexCornerPos::from((pos, corner))));
            for (neighbor_pos, neighbor_corner) in pos.neighbors_at_corner(corner) {
                assert!(size.contains_corner(HexCornerPos::from((neighbor_pos, neighbor_corner))));
            }
        }
    }

    #[quickcheck]
    fn contains_corner(size: HexGridSize) {
        for edge_pos in HexPerimeterIterator::new(&size) {
            let neighbor_edge = HexEdgePos::from((
                edge_pos.pos().neighbor(edge_pos.edge()),
                edge_pos.edge().opposite(),
            ));
            for corner in edge_pos.edge().ends().into_iter().flat_map(|c| {
                edge_pos
                    .pos()
                    .neighbors_at_corner(c)
                    .into_iter()
                    .find_map(|(p, c)| {
                        if p == neighbor_edge.pos() {
                            Some(c)
                        } else {
                            None
                        }
                    })
            }) {
                let corner_pos = HexCornerPos::from((neighbor_edge.pos(), corner));
                assert!(
                    size.contains_corner(corner_pos),
                    "size: {}, edge_pos: {:?}, corner_pos: {:?}, norm: {:?}",
                    size,
                    edge_pos,
                    corner_pos,
                    corner_pos.norm()
                );
            }
        }
    }

    #[quickcheck]
    fn edge_iterator(size: HexGridSize) {
        let mut seen_edges = HashSet::new();
        for pos in size.iter_hexes() {
            for edge in HexEdge::ALL {
                seen_edges.insert(HexEdgePos::from((pos, edge)).norm());
            }
        }
        let iter_edges = size.iter_edges().map(|e| e.norm()).collect::<Vec<_>>();
        assert_eq!(seen_edges.len(), iter_edges.len());
        assert_eq!(seen_edges, iter_edges.into_iter().collect::<HashSet<_>>());
    }

    #[quickcheck]
    fn corner_iterator(size: HexGridSize) {
        let mut seen_corners = HashSet::new();
        for pos in size.iter_hexes() {
            for corner in HexCorner::ALL {
                seen_corners.insert(HexCornerPos::from((pos, corner)).norm());
            }
        }
        let iter_corners = size.iter_corners().collect::<Vec<_>>();
        assert_eq!(
            seen_corners,
            iter_corners
                .iter()
                .map(|c| c.norm())
                .collect::<HashSet<_>>()
        );
        assert_eq!(
            seen_corners.len(),
            iter_corners.len(),
            "iter_corners: {:?}",
            iter_corners
        );
    }
}
