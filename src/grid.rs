#![allow(unused)] // TODO

use crate::{
    HexCoord,
    container::HexPosContainer,
    corner::{HexCorner, HexCornerIterator, HexCornerPos, NormHexCorner},
    edge::{HexEdge, HexEdgeIterator, HexEdgePos, NormHexEdge},
    grid_size::HexGridSize,
    pos::{HexPos, HexPosIterator},
};
use std::{
    assert_matches,
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    sync::Arc,
};

pub struct HexGrid<H, E = (), C = ()> {
    size: HexGridSize,
    even_row_size: HexCoord,
    odd_row_size: HexCoord,
    hexes: Vec<Hex<H, E, C>>,
    bottom_left_edges: Vec<E>,
    bottom_edges: Vec<E>,
    bottom_right_edges: Vec<E>,
    bottom_edge_corners: Vec<C>,
    top_edge_corners: Vec<C>,
    left_edge_corners: Vec<C>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeIndex {
    Hex(usize, NormHexEdge),
    BottomLeft(usize),
    Bottom(usize),
    BottomRight(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CornerIndex {
    Hex(usize, NormHexCorner),
    Bottom(usize),
    Top(usize),
    Left(usize),
}

struct Hex<H, E, C> {
    data: H,
    edges: [E; 3],
    corners: [C; 2],
}

impl<H, E, C> HexGrid<H, E, C> {
    pub fn new(
        size: HexGridSize,
        mut hex_init: impl FnMut(HexPos) -> H,
        mut edge_init: impl FnMut(HexEdgePos) -> E,
        mut corner_init: impl FnMut(HexCornerPos) -> C,
    ) -> Self {
        let (width, height) = size.unpack();
        let even_row_size = 1 + (width - 1) / 2;
        let odd_row_size = width / 2;

        let mut grid = HexGrid {
            size,
            even_row_size,
            odd_row_size,
            hexes: Vec::new(),
            bottom_left_edges: Vec::new(),
            bottom_edges: Vec::new(),
            bottom_right_edges: Vec::new(),
            bottom_edge_corners: Vec::new(),
            top_edge_corners: Vec::new(),
            left_edge_corners: Vec::new(),
        };

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
                        corner_init(HexCornerPos::from((pos, HexCorner::Right))),
                        corner_init(HexCornerPos::from((pos, HexCorner::TopRight))),
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

            for i in 0..((width + 1) / 2) {
                grid.bottom_edge_corners
                    .push(corner_init(HexCornerPos::from((
                        2 * i - 1,
                        -1,
                        HexCorner::TopRight,
                    ))));
                grid.bottom_edge_corners
                    .push(corner_init(HexCornerPos::from((
                        2 * i - 1,
                        -1,
                        HexCorner::Right,
                    ))));
                grid.bottom_edge_corners
                    .push(corner_init(HexCornerPos::from((
                        2 * i,
                        -2,
                        HexCorner::TopRight,
                    ))));
            }
            if width % 2 == 0 {
                grid.bottom_edge_corners
                    .push(corner_init(HexCornerPos::from((
                        width - 1,
                        -1,
                        HexCorner::TopRight,
                    ))));
            }
            for i in 0..((width + height % 2) / 2) {
                grid.top_edge_corners.push(corner_init(HexCornerPos::from((
                    2 * i - height % 2,
                    height,
                    HexCorner::Right,
                ))));
            }
            for i in 1..((height + 1) / 2) {
                grid.left_edge_corners.push(corner_init(HexCornerPos::from((
                    -1,
                    2 * i - 1,
                    HexCorner::Right,
                ))));
                grid.left_edge_corners.push(corner_init(HexCornerPos::from((
                    -1,
                    2 * i - 1,
                    HexCorner::TopRight,
                ))));
            }
            if height % 2 == 0 {
                grid.left_edge_corners.push(corner_init(HexCornerPos::from((
                    -1,
                    height - 1,
                    HexCorner::Right,
                ))));
            }
        }

        if size_of::<H>() + size_of::<E>() + size_of::<C>() > 0 {
            debug_assert_eq!(grid.hexes.len(), grid.hexes.capacity());
        }

        grid
    }

    pub fn new_with_defaults(size: HexGridSize) -> Self
    where
        H: Default,
        E: Default,
        C: Default,
    {
        Self::new(size, |_| H::default(), |_| E::default(), |_| C::default())
    }

    pub fn size(&self) -> HexGridSize {
        self.size
    }

    pub fn width(&self) -> HexCoord {
        self.size.width()
    }

    pub fn height(&self) -> HexCoord {
        self.size.height()
    }

    pub fn has_hex(&self, pos: HexPos) -> bool {
        self.size.contains_hex(pos)
    }

    pub fn iter_edges(&self) -> HexEdgeIterator {
        HexEdgeIterator::new(self.size)
    }

    pub fn iter_corners(&self) -> HexCornerIterator {
        HexCornerIterator::new(self.size)
    }

    pub fn hex(&self, pos: HexPos) -> &H {
        &self.hexes[self.hex_index(pos)].data
    }

    pub fn hex_mut(&mut self, pos: HexPos) -> &mut H {
        let index = self.hex_index(pos);
        &mut self.hexes[index].data
    }

    pub fn edge(&self, pos: HexEdgePos) -> &E {
        match self.edge_index(pos) {
            EdgeIndex::Hex(index, edge) => &self.hexes[index].edges[edge as usize],
            EdgeIndex::BottomLeft(index) => &self.bottom_left_edges[index],
            EdgeIndex::Bottom(index) => &self.bottom_edges[index],
            EdgeIndex::BottomRight(index) => &self.bottom_right_edges[index],
        }
    }

    pub fn edge_mut(&mut self, pos: HexEdgePos) -> &mut E {
        match self.edge_index(pos) {
            EdgeIndex::Hex(index, edge) => &mut self.hexes[index].edges[edge as usize],
            EdgeIndex::BottomLeft(index) => &mut self.bottom_left_edges[index],
            EdgeIndex::Bottom(index) => &mut self.bottom_edges[index],
            EdgeIndex::BottomRight(index) => &mut self.bottom_right_edges[index],
        }
    }

    pub fn corner(&self, pos: HexCornerPos) -> &C {
        match self.corner_index(pos) {
            CornerIndex::Hex(index, corner) => &self.hexes[index].corners[corner as usize],
            CornerIndex::Bottom(index) => &self.bottom_edge_corners[index],
            CornerIndex::Top(index) => &self.top_edge_corners[index],
            CornerIndex::Left(index) => &self.left_edge_corners[index],
        }
    }

    pub fn corner_mut(&mut self, pos: HexCornerPos) -> &mut C {
        match self.corner_index(pos) {
            CornerIndex::Hex(index, corner) => &mut self.hexes[index].corners[corner as usize],
            CornerIndex::Bottom(index) => &mut self.bottom_edge_corners[index],
            CornerIndex::Top(index) => &mut self.top_edge_corners[index],
            CornerIndex::Left(index) => &mut self.left_edge_corners[index],
        }
    }

    fn unchecked_hex_index(&self, pos: HexPos) -> usize {
        let u = pos.u();
        let v = pos.v();
        debug_assert_eq!(
            (u + v) % 2,
            0,
            "Hex position {:?} is invalid: u + v must be even",
            pos
        );
        if v % 2 == 0 {
            (u / 2 + self.even_row_size * (v / 2) + self.odd_row_size * ((v + 1) / 2)) as usize
        } else {
            (u / 2 + self.even_row_size * ((v + 1) / 2) + self.odd_row_size * (v / 2)) as usize
        }
    }

    fn hex_index(&self, pos: HexPos) -> usize {
        assert!(
            self.has_hex(pos),
            "Hex at position {:?} does not exist",
            pos
        );
        self.unchecked_hex_index(pos)
    }

    fn edge_index(&self, edge_pos: HexEdgePos) -> EdgeIndex {
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
                _ => panic!("Invalid edge at position {:?}: {:?}", pos, edge),
            }
        }
    }

    fn corner_index(&self, corner_pos: HexCornerPos) -> CornerIndex {
        let (pos, corner) = corner_pos.norm();
        let (u, v) = pos.u_v();
        let (width, height) = self.size.unpack();
        if self.has_hex(pos) {
            CornerIndex::Hex(self.hex_index(pos), corner)
        } else if v == -1 {
            if width % 2 == 0 && u == width - 1 {
                assert_matches!(corner, NormHexCorner::TopRight);
                CornerIndex::Bottom(((width + 1) / 2) as usize * 3)
            } else {
                match corner {
                    NormHexCorner::Right => CornerIndex::Bottom(((u + 1) / 2) as usize * 3 + 1),
                    NormHexCorner::TopRight => CornerIndex::Bottom(((u + 1) / 2) as usize * 3),
                }
            }
        } else if v == -2 {
            assert_matches!(corner, NormHexCorner::TopRight);
            CornerIndex::Bottom((u / 2) as usize * 3 + 2)
        } else if v == height {
            assert_matches!(corner, NormHexCorner::Right);
            CornerIndex::Top(((u + 1) / 2) as usize)
        } else {
            assert_eq!(u, -1);
            match corner {
                NormHexCorner::Right => CornerIndex::Left(((v - 1) / 2) as usize * 2),
                NormHexCorner::TopRight => CornerIndex::Left(((v - 1) / 2) as usize * 2 + 1),
            }
        }
    }
}

impl<H, E, C> Default for HexGrid<H, E, C> {
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
        = HexPosIterator
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
    use crate::{corner::HexCornerPos, edge::HexEdgePos};
    use quickcheck_macros::quickcheck;
    use std::collections::HashSet;

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

    #[quickcheck]
    fn vec_dimensions(size: HexGridSize) {
        let grid = HexGrid::<i32>::new_with_defaults(size);
        let mut hex_grid_size = 0;
        let mut bottom_left_edges_size = 0;
        let mut bottom_edges_size = 0;
        let mut bottom_right_edges_size = 0;
        let mut bottom_edge_corners_size = 0;
        let mut top_edge_corners_size = 0;
        let mut left_edge_corners_size = 0;

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
                    CornerIndex::Bottom(index) => {
                        bottom_edge_corners_size = bottom_edge_corners_size.max(index + 1);
                    }
                    CornerIndex::Top(index) => {
                        top_edge_corners_size = top_edge_corners_size.max(index + 1);
                    }
                    CornerIndex::Left(index) => {
                        left_edge_corners_size = left_edge_corners_size.max(index + 1);
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
        assert_eq!(top_edge_corners_size, grid.top_edge_corners.len());
        assert_eq!(left_edge_corners_size, grid.left_edge_corners.len());
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
                assert_eq!(
                    HexCornerPos::from((pos, corner)).norm(),
                    grid.corner((pos, corner).into()).norm(),
                );
            }
        }
    }
}
