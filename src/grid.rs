use crate::{
    HexCoord, HexRegionIterator,
    container::HexPosContainer,
    corner::{HexCorner, NormHexCorner},
    corner_pos::HexCornerPos,
    edge::{HexEdge, NormHexEdge},
    edge_pos::HexEdgePos,
    grid_size::{HexCornerIterator, HexEdgeIterator, HexGridSize},
    pos::HexPos,
};

/// A hexagonal grid of hexes, edges, and corners, with associated data for each.
pub struct HexGrid<H, E = (), C = ()> {
    size: HexGridSize,
    even_row_size: HexCoord,
    odd_row_size: HexCoord,
    hexes: Vec<Hex<H, E, C>>,
    bottom_left_edges: Vec<E>,
    bottom_edges: Vec<E>,
    bottom_right_edges: Vec<E>,
    right_edge_corners: Vec<C>,
    left_edge_corners: Vec<C>,
    bottom_edge_corners: Vec<C>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EdgeIndex {
    Hex(usize, NormHexEdge),
    BottomLeft(usize),
    Bottom(usize),
    BottomRight(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CornerIndex {
    Hex(usize, NormHexCorner),
    Right(usize),
    Left(usize),
    Bottom(usize),
}

struct Hex<H, E, C> {
    data: H,
    edges: [E; 3],
    corners: [C; 2],
}

impl<H, E, C> HexGrid<H, E, C> {
    /// Creates a new `HexGrid` with the given size, and initializes the data
    /// for each hex, edge, and corner using the provided functions.
    pub fn new(
        size: HexGridSize,
        mut hex_init: impl FnMut(HexPos) -> H,
        mut edge_init: impl FnMut(HexEdgePos) -> E,
        mut corner_init: impl FnMut(HexCornerPos) -> C,
    ) -> Self {
        let mut grid = HexGrid {
            size,
            even_row_size: size.even_row_size(),
            odd_row_size: size.odd_row_size(),
            hexes: Vec::new(),
            bottom_left_edges: Vec::new(),
            bottom_edges: Vec::new(),
            bottom_right_edges: Vec::new(),
            right_edge_corners: Vec::new(),
            left_edge_corners: Vec::new(),
            bottom_edge_corners: Vec::new(),
        };

        let (width, height) = size.unpack();

        if width > 0 && height > 0 {
            grid.hexes.reserve_exact(size.len());
            for pos in grid.iter_hexes() {
                let hex = Hex {
                    data: hex_init(pos),
                    edges: [
                        edge_init(HexEdgePos::from((pos, HexEdge::TopRight))),
                        edge_init(HexEdgePos::from((pos, HexEdge::Top))),
                        edge_init(HexEdgePos::from((pos, HexEdge::TopLeft))),
                    ],
                    corners: [
                        corner_init(HexCornerPos::from((pos, HexCorner::TopRight))),
                        corner_init(HexCornerPos::from((pos, HexCorner::TopLeft))),
                    ],
                };
                grid.hexes.push(hex);
            }

            for i in 0..((height + 1) / 2) {
                grid.bottom_left_edges.push(edge_init(HexEdgePos::from((
                    0,
                    i * 2,
                    HexEdge::BottomLeft,
                ))));
            }
            for i in 1..((width + 1) / 2) {
                grid.bottom_left_edges.push(edge_init(HexEdgePos::from((
                    i * 2,
                    0,
                    HexEdge::BottomLeft,
                ))));
            }

            for i in 0..((width + 1) / 2) {
                grid.bottom_right_edges.push(edge_init(HexEdgePos::from((
                    i * 2,
                    0,
                    HexEdge::BottomRight,
                ))));
            }
            if width % 2 == 0 {
                for i in 0..(height / 2) {
                    grid.bottom_right_edges.push(edge_init(HexEdgePos::from((
                        width - 1,
                        i * 2 + 1,
                        HexEdge::BottomRight,
                    ))));
                }
            } else {
                for i in 1..((height + 1) / 2) {
                    grid.bottom_right_edges.push(edge_init(HexEdgePos::from((
                        width - 1,
                        i * 2,
                        HexEdge::BottomRight,
                    ))));
                }
            }
            for u in 0..width {
                grid.bottom_edges
                    .push(edge_init(HexEdgePos::from((u, u % 2, HexEdge::Bottom))));
            }

            for i in 0..((height + 1) / 2) {
                grid.left_edge_corners.push(corner_init(HexCornerPos::from((
                    -1,
                    -1 + 2 * i,
                    HexCorner::TopRight,
                ))));
            }
            let num_right_edge_corners = if width % 2 == height % 2 {
                (height + 1) / 2
            } else {
                height / 2
            };
            for i in 0..num_right_edge_corners {
                grid.right_edge_corners
                    .push(corner_init(HexCornerPos::from((
                        width,
                        if width % 2 == 0 { 2 * i } else { 2 * i - 1 },
                        HexCorner::TopLeft,
                    ))));
            }
            for i in 0..width {
                let pos = HexPos::new(i, -2 + i % 2);
                grid.bottom_edge_corners
                    .push(corner_init(HexCornerPos::from((pos, HexCorner::TopLeft))));
                grid.bottom_edge_corners
                    .push(corner_init(HexCornerPos::from((pos, HexCorner::TopRight))));
            }
        }

        if size_of::<H>() + size_of::<E>() + size_of::<C>() > 0 {
            debug_assert_eq!(grid.hexes.len(), grid.hexes.capacity());
        }

        grid
    }

    /// Creates a new `HexGrid` with the given size, and initializes the data
    /// for each hex, edge, and corner using the `Default` trait.
    pub fn new_with_defaults(size: HexGridSize) -> Self
    where
        H: Default,
        E: Default,
        C: Default,
    {
        Self::new(size, |_| H::default(), |_| E::default(), |_| C::default())
    }

    /// Returns the size of the grid.
    pub fn size(&self) -> HexGridSize {
        self.size
    }

    /// Returns the width of the grid, which is roughly twice the number of hexes in each row.
    pub fn width(&self) -> HexCoord {
        self.size.width()
    }

    /// Returns the height of the grid, which is roughly twice the number of hexes in each column.
    pub fn height(&self) -> HexCoord {
        self.size.height()
    }

    /// Return true iff the grid contains a hex at the given position.
    pub fn has_hex(&self, pos: HexPos) -> bool {
        self.size.contains_hex(pos)
    }

    /// Returns true iff the grid contains an edge at the given position.
    pub fn has_edge(&self, edge_pos: HexEdgePos) -> bool {
        self.size.contains_edge(edge_pos)
    }

    /// Returns true iff the grid contains a corner at the given position.
    pub fn has_corner(&self, corner_pos: HexCornerPos) -> bool {
        self.size.contains_corner(corner_pos)
    }

    /// Returns an iterator over the positions of all edges in the grid.
    pub fn iter_edges(&self) -> HexEdgeIterator {
        HexEdgeIterator::new(self.size)
    }

    /// Returns an iterator over the positions of all corners in the grid.
    pub fn iter_corners(&self) -> HexCornerIterator {
        HexCornerIterator::new(self.size)
    }

    /// Returns a reference to the data associated with the hex at the given
    /// position.  Panics if `has_hex(pos)` is false.
    pub fn hex(&self, pos: HexPos) -> &H {
        &self.hexes[self.hex_index(pos)].data
    }

    /// Returns a mutable reference to the data associated with the hex at the
    /// given position.  Panics if `has_hex(pos)` is false.
    pub fn hex_mut(&mut self, pos: HexPos) -> &mut H {
        let index = self.hex_index(pos);
        &mut self.hexes[index].data
    }

    /// Returns a reference to the data associated with the edge at the given
    /// position.  Panics if `has_edge(pos)` is false.
    pub fn edge(&self, pos: HexEdgePos) -> &E {
        match self.edge_index(pos) {
            EdgeIndex::Hex(index, edge) => &self.hexes[index].edges[edge as usize],
            EdgeIndex::BottomLeft(index) => &self.bottom_left_edges[index],
            EdgeIndex::Bottom(index) => &self.bottom_edges[index],
            EdgeIndex::BottomRight(index) => &self.bottom_right_edges[index],
        }
    }

    /// Returns a mutable reference to the data associated with the edge at the
    /// given position.  Panics if `has_edge(pos)` is false.
    pub fn edge_mut(&mut self, pos: HexEdgePos) -> &mut E {
        match self.edge_index(pos) {
            EdgeIndex::Hex(index, edge) => &mut self.hexes[index].edges[edge as usize],
            EdgeIndex::BottomLeft(index) => &mut self.bottom_left_edges[index],
            EdgeIndex::Bottom(index) => &mut self.bottom_edges[index],
            EdgeIndex::BottomRight(index) => &mut self.bottom_right_edges[index],
        }
    }

    /// Returns a reference to the data associated with the corner at the given
    /// position.  Panics if `has_corner(pos)` is false.
    pub fn corner(&self, pos: HexCornerPos) -> &C {
        match self.corner_index(pos) {
            CornerIndex::Hex(index, corner) => &self.hexes[index].corners[corner as usize],
            CornerIndex::Right(index) => &self.right_edge_corners[index],
            CornerIndex::Left(index) => &self.left_edge_corners[index],
            CornerIndex::Bottom(index) => &self.bottom_edge_corners[index],
        }
    }

    /// Returns a mutable reference to the data associated with the corner at the
    /// given position.  Panics if `has_corner(pos)` is false.
    pub fn corner_mut(&mut self, pos: HexCornerPos) -> &mut C {
        match self.corner_index(pos) {
            CornerIndex::Hex(index, corner) => &mut self.hexes[index].corners[corner as usize],
            CornerIndex::Right(index) => &mut self.right_edge_corners[index],
            CornerIndex::Bottom(index) => &mut self.bottom_edge_corners[index],
            CornerIndex::Left(index) => &mut self.left_edge_corners[index],
        }
    }

    fn hex_index(&self, pos: HexPos) -> usize {
        assert!(
            self.has_hex(pos),
            "Hex at position {:?} does not exist",
            pos
        );
        let (u, v) = pos.u_v();
        if v % 2 == 0 {
            (u / 2 + self.even_row_size * (v / 2) + self.odd_row_size * ((v + 1) / 2)) as usize
        } else {
            (u / 2 + self.even_row_size * ((v + 1) / 2) + self.odd_row_size * (v / 2)) as usize
        }
    }

    fn edge_index(&self, edge_pos: HexEdgePos) -> EdgeIndex {
        assert!(
            self.has_edge(edge_pos),
            "Edge at position {:?} does not exist",
            edge_pos
        );

        let (pos, edge): (HexPos, NormHexEdge) = edge_pos.norm();
        let (u, v) = pos.u_v();
        let (width, height) = self.size.unpack();

        if self.has_hex(pos) {
            EdgeIndex::Hex(self.hex_index(pos), edge)
        } else {
            match edge {
                NormHexEdge::TopRight if u == -1 => EdgeIndex::BottomLeft(((v + 1) / 2) as usize),
                NormHexEdge::TopRight if v == -1 => {
                    EdgeIndex::BottomLeft(((u + 1) / 2 + (height + 1) / 2 - 1) as usize)
                }
                NormHexEdge::Top if v < 0 => EdgeIndex::Bottom(u as usize),
                NormHexEdge::TopLeft if v == -1 => EdgeIndex::BottomRight((u / 2) as usize),
                NormHexEdge::TopLeft if u == width => {
                    EdgeIndex::BottomRight((width / 2 + (v + 1) / 2) as usize)
                }
                _ => panic!("Invalid edge at position {:?}", edge_pos),
            }
        }
    }

    fn corner_index(&self, corner_pos: HexCornerPos) -> CornerIndex {
        assert!(
            self.has_corner(corner_pos),
            "Corner at position {:?} does not exist",
            corner_pos
        );

        let (pos, corner) = corner_pos.norm();
        let (u, v) = pos.u_v();
        let width = self.size.width();
        if self.has_hex(pos) {
            CornerIndex::Hex(self.hex_index(pos), corner)
        } else if u == -1 && corner == NormHexCorner::TopRight {
            CornerIndex::Left(((v + 1) / 2) as usize)
        } else if u == width && corner == NormHexCorner::TopLeft {
            if width % 2 == 0 {
                CornerIndex::Right((v / 2) as usize)
            } else {
                CornerIndex::Right(((v + 1) / 2) as usize)
            }
        } else {
            let offset: usize = match corner {
                NormHexCorner::TopRight => 1,
                NormHexCorner::TopLeft => 0,
            };
            debug_assert!(v == -2 && u % 2 == 0 || v == -1 && u % 2 != 0);
            CornerIndex::Bottom(u as usize * 2 + offset)
        }
    }
}

impl<H, E, C> Default for HexGrid<H, E, C> {
    /// Creates a new grid with size 0 by 0.
    fn default() -> Self {
        Self::new(
            HexGridSize::default(),
            |_| unreachable!(),
            |_| unreachable!(),
            |_| unreachable!(),
        )
    }
}

impl<H, E, C> HexPosContainer for HexGrid<H, E, C> {
    type Iterator<'c>
        = HexRegionIterator
    where
        Self: 'c;

    fn contains_hex(&self, pos: HexPos) -> bool {
        self.size.contains_hex(pos)
    }

    fn iter_hexes(&self) -> Self::Iterator<'_> {
        self.size.iter_hexes()
    }

    fn len(&self) -> usize {
        self.hexes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HexRegion;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn index(size: HexGridSize) {
        let grid = HexGrid::<i32>::new_with_defaults(size);
        for (i, pos) in grid.iter_hexes().enumerate() {
            assert_eq!(
                grid.hex_index(pos),
                i,
                "size: {:?}, pos: {:?}, i: {}",
                size,
                pos,
                i
            );
        }
    }

    #[test]
    fn vec_dimensions() {
        for size in HexGridSize::test_sizes() {
            eprintln!("size: {:?}", size);
            let grid = HexGrid::<i32>::new_with_defaults(size);
            let mut hex_grid_size = 0;
            let mut bottom_left_edges_size = 0;
            let mut bottom_edges_size = 0;
            let mut bottom_right_edges_size = 0;
            let mut right_edge_corners_size = 0;
            let mut left_edge_corners_size = 0;
            let mut bottom_edge_corners_size = 0;

            for pos in grid.iter_hexes() {
                {
                    let index = grid.hex_index(pos);
                    hex_grid_size = hex_grid_size.max(index + 1);
                }

                for edge in HexEdge::ALL {
                    match grid.edge_index((pos, edge).into()) {
                        EdgeIndex::BottomLeft(index) => {
                            bottom_left_edges_size = bottom_left_edges_size.max(index + 1);
                        }
                        EdgeIndex::Bottom(index) => {
                            bottom_edges_size = bottom_edges_size.max(index + 1);
                        }
                        EdgeIndex::BottomRight(index) => {
                            bottom_right_edges_size = bottom_right_edges_size.max(index + 1);
                        }
                        _ => {}
                    }
                }

                for corner in HexCorner::ALL {
                    match grid.corner_index((pos, corner).into()) {
                        CornerIndex::Right(index) => {
                            right_edge_corners_size = right_edge_corners_size.max(index + 1);
                        }
                        CornerIndex::Left(index) => {
                            left_edge_corners_size = left_edge_corners_size.max(index + 1);
                        }
                        CornerIndex::Bottom(index) => {
                            bottom_edge_corners_size = bottom_edge_corners_size.max(index + 1);
                        }
                        _ => {}
                    }
                }
            }

            assert_eq!(hex_grid_size, grid.hexes.len());
            assert_eq!(hex_grid_size, grid.size().len());

            assert_eq!(bottom_left_edges_size, grid.bottom_left_edges.len());
            assert_eq!(bottom_edges_size, grid.bottom_edges.len());
            assert_eq!(bottom_right_edges_size, grid.bottom_right_edges.len());

            assert_eq!(bottom_edge_corners_size, grid.bottom_edge_corners.len());
            assert_eq!(right_edge_corners_size, grid.right_edge_corners.len());
            assert_eq!(left_edge_corners_size, grid.left_edge_corners.len());
        }
    }

    #[quickcheck]
    fn edge_order(size: HexGridSize) {
        let grid = HexGrid::new(size, |_| (), |pos| pos, |_| ());
        for pos in grid.iter_hexes() {
            for edge in HexEdge::ALL {
                assert_eq!(
                    HexEdgePos::from((pos, edge)).norm(),
                    grid.edge((pos, edge).into()).norm(),
                    "pos: {:?}, edge: {:?}",
                    pos,
                    edge
                );
            }
        }
    }

    #[quickcheck]
    fn corner_order(size: HexGridSize) {
        let grid = HexGrid::new(size, |_| (), |_| (), |pos| pos);
        for pos in grid.iter_hexes() {
            for corner in HexCorner::ALL {
                let left = HexCornerPos::from((pos, corner)).norm();
                let right = grid.corner((pos, corner).into()).norm();
                assert_eq!(left, right);
            }
        }
    }

    #[test]
    fn indices_in_range() {
        for width in 0..=3 {
            for height in 0..=3 {
                if let Ok(size) = HexGridSize::new(width, height) {
                    let grid = HexGrid::<()>::new_with_defaults(size);
                    for pos in HexRegion::new(-3, -3, width + 2, height + 2).iter_hexes() {
                        eprintln!("size: {}, pos: {}", size, pos);
                        if grid.has_hex(pos) {
                            grid.hex(pos);
                        }
                        for edge in HexEdge::ALL {
                            let edge_pos = HexEdgePos::from((pos, edge));
                            if grid.has_edge(edge_pos) {
                                grid.edge(edge_pos);
                            }
                        }
                        for corner in HexCorner::ALL {
                            dbg!(corner);
                            let corner_pos = HexCornerPos::from((pos, corner));
                            if grid.has_corner(corner_pos) {
                                grid.corner(corner_pos);
                            }
                        }
                    }
                }
            }
        }
    }
}
