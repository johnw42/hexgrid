#![allow(unused)] // TODO

use crate::{
    HexCoord,
    corner::{HexCorner, HexCornerIterator, HexPosWithCorner},
    edge::{HexEdge, HexEdgeIterator, HexPosWithEdge},
    grid_size::HexGridSize,
    perimeter::HexGridPerimeterIterator,
    pos::{HexPos, HexPosContainer, HexPosIterator},
};
use std::{
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
    top_left_corners: Vec<C>,
    left_corners: Vec<C>,
    bottom_left_corners: Vec<C>,
    bottom_right_corners: Vec<C>,
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
        mut edge_init: impl FnMut(HexPosWithEdge) -> E,
        mut corner_init: impl FnMut(HexPosWithCorner) -> C,
    ) -> Self {
        let (width, height) = size.into();
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
            top_left_corners: Vec::new(),
            left_corners: Vec::new(),
            bottom_left_corners: Vec::new(),
            bottom_right_corners: Vec::new(),
        };

        if width > 0 && height > 0 {
            grid.hexes
                .reserve_exact(grid.unchecked_hex_index(HexPos::new(height % 2, height)));
            for pos in grid.iter_hexes() {
                let hex = Hex {
                    data: hex_init(pos),
                    edges: [
                        edge_init(HexPosWithEdge::new(pos, HexEdge::TopRight)),
                        edge_init(HexPosWithEdge::new(pos, HexEdge::Top)),
                        edge_init(HexPosWithEdge::new(pos, HexEdge::TopLeft)),
                    ],
                    corners: [
                        corner_init(HexPosWithCorner::new(pos, HexCorner::Right)),
                        corner_init(HexPosWithCorner::new(pos, HexCorner::TopRight)),
                    ],
                };
                grid.hexes.push(hex);
            }
            for i in 0..((height + 1) / 2) {
                grid.bottom_left_edges.push(edge_init(HexPosWithEdge::new(
                    HexPos::new(0, i * 2),
                    HexEdge::BottomLeft,
                )));
            }
            for i in 1..((width + 1) / 2) {
                grid.bottom_left_edges.push(edge_init(HexPosWithEdge::new(
                    HexPos::new(i * 2, 0),
                    HexEdge::BottomLeft,
                )));
            }

            for i in 0..((width + 1) / 2) {
                grid.bottom_right_edges.push(edge_init(HexPosWithEdge::new(
                    HexPos::new(i * 2, 0),
                    HexEdge::BottomRight,
                )));
            }
            if width % 2 == 0 {
                for i in 0..(height / 2) {
                    grid.bottom_right_edges.push(edge_init(HexPosWithEdge::new(
                        HexPos::new(width - 1, i * 2 + 1),
                        HexEdge::BottomRight,
                    )));
                }
            } else {
                for i in 1..((height + 1) / 2) {
                    grid.bottom_right_edges.push(edge_init(HexPosWithEdge::new(
                        HexPos::new(width - 1, i * 2),
                        HexEdge::BottomRight,
                    )));
                }
            }
            for u in 0..width {
                grid.bottom_edges.push(edge_init(HexPosWithEdge::new(
                    HexPos::new(u, u % 2),
                    HexEdge::Bottom,
                )));
            }
            for i in 0..(height + 1) / 2 {
                grid.top_left_corners
                    .push(corner_init(HexPosWithCorner::new(
                        HexPos::new(0, 2 * i),
                        HexCorner::TopLeft,
                    )));
            }
            if height % 2 == 0 {
                for i in 0..(width / 2) {
                    grid.top_left_corners
                        .push(corner_init(HexPosWithCorner::new(
                            HexPos::new(2 * i + 1, height - 1),
                            HexCorner::TopLeft,
                        )));
                }
            } else {
                for i in 1..((width + 1) / 2) {
                    grid.top_left_corners
                        .push(corner_init(HexPosWithCorner::new(
                            HexPos::new(2 * i, height - 1),
                            HexCorner::TopLeft,
                        )));
                }
            }
            for i in 0..(height + 1) / 2 {
                grid.left_corners.push(corner_init(HexPosWithCorner::new(
                    HexPos::new(0, 2 * i),
                    HexCorner::Left,
                )));
            }
            for i in 0..((height + 1) / 2) {
                grid.bottom_left_corners
                    .push(corner_init(HexPosWithCorner::new(
                        HexPos::new(0, 2 * i),
                        HexCorner::BottomLeft,
                    )));
            }
            for i in 1..((width + 1) / 2) {
                grid.bottom_left_corners
                    .push(corner_init(HexPosWithCorner::new(
                        HexPos::new(i * 2, 0),
                        HexCorner::BottomLeft,
                    )));
            }
            for u in 0..width {
                grid.bottom_right_corners
                    .push(corner_init(HexPosWithCorner::new(
                        HexPos::new(u, u % 2),
                        HexCorner::BottomRight,
                    )));
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

    pub fn edge(&self, pos: HexPos, edge: HexEdge) -> &E {
        let (index, edge) = self.edge_index(pos, edge);
        match edge {
            HexEdge::TopRight | HexEdge::Top | HexEdge::TopLeft => {
                &self.hexes[index].edges[edge as usize]
            }
            HexEdge::BottomLeft => &self.bottom_left_edges[index],
            HexEdge::Bottom => &self.bottom_edges[index],
            HexEdge::BottomRight => &self.bottom_right_edges[index],
        }
    }

    pub fn edge_mut(&mut self, pos: HexPos, edge: HexEdge) -> &mut E {
        let (index, edge) = self.edge_index(pos, edge);
        match edge {
            HexEdge::TopRight | HexEdge::Top | HexEdge::TopLeft => {
                &mut self.hexes[index].edges[edge as usize]
            }
            HexEdge::BottomLeft => &mut self.bottom_left_edges[index],
            HexEdge::Bottom => &mut self.bottom_edges[index],
            HexEdge::BottomRight => &mut self.bottom_right_edges[index],
        }
    }

    pub fn corner(&self, pos: HexPos, corner: HexCorner) -> &C {
        let (index, corner) = self.corner_index(pos, corner);
        match corner {
            HexCorner::Right | HexCorner::TopRight => &self.hexes[index].corners[corner as usize],
            HexCorner::TopLeft => &self.top_left_corners[index],
            HexCorner::Left => &self.left_corners[index],
            HexCorner::BottomLeft => &self.bottom_left_corners[index],
            HexCorner::BottomRight => &self.bottom_right_corners[index],
        }
    }

    pub fn corner_mut(&mut self, pos: HexPos, corner: HexCorner) -> &mut C {
        let (index, corner) = self.corner_index(pos, corner);
        match corner {
            HexCorner::Right | HexCorner::TopRight => {
                &mut self.hexes[index].corners[corner as usize]
            }
            HexCorner::TopLeft => &mut self.top_left_corners[index],
            HexCorner::Left => &mut self.left_corners[index],
            HexCorner::BottomLeft => &mut self.bottom_left_corners[index],
            HexCorner::BottomRight => &mut self.bottom_right_corners[index],
        }
    }

    fn unchecked_hex_index(&self, pos: HexPos) -> usize {
        let u = pos.u();
        let v = pos.v();
        if v % 2 == 0 {
            debug_assert_eq!(u % 2, 0, "Even row {} must have even u", v);
            (u / 2 + self.even_row_size * (v / 2) + self.odd_row_size * ((v + 1) / 2)) as usize
        } else {
            debug_assert_eq!(u % 2, 1, "Odd row {} must have odd u", v);
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

    fn edge_index(&self, pos: HexPos, edge: HexEdge) -> (usize, HexEdge) {
        let (u, v) = pos.into();
        let (width, height) = self.size.into();

        match edge {
            HexEdge::TopRight | HexEdge::Top | HexEdge::TopLeft => (self.hex_index(pos), edge),
            HexEdge::BottomLeft if u == 0 => ((v / 2) as usize, edge),
            HexEdge::BottomLeft if v == 0 => ((u / 2 + (height + 1) / 2 - 1) as usize, edge),
            HexEdge::Bottom if v <= 1 => (u as usize, edge),
            HexEdge::BottomRight if v == 0 => ((u / 2) as usize, edge),
            HexEdge::BottomRight if u == width - 1 => ((width / 2 + v / 2) as usize, edge),
            HexEdge::BottomLeft | HexEdge::Bottom | HexEdge::BottomRight => {
                let neighbor = pos.neighbor(edge);
                (self.hex_index(neighbor), edge.opposite())
            }
        }
    }

    fn corner_index(&self, pos: HexPos, corner: HexCorner) -> (usize, HexCorner) {
        let (u, v) = pos.into();
        let (width, height) = self.size.into();

        match corner {
            HexCorner::Right | HexCorner::TopRight => (self.hex_index(pos), corner),
            HexCorner::TopLeft if u == 0 => ((v / 2) as usize, corner),
            HexCorner::TopLeft if v == height - 1 => ((height / 2 + u / 2) as usize, corner),
            HexCorner::TopLeft => {
                self.corner_index(pos.neighbor(HexEdge::TopLeft), HexCorner::Right)
            }
            HexCorner::Left if u == 0 => ((v / 2) as usize, corner),
            HexCorner::Left if v == 0 => {
                self.corner_index(pos.neighbor(HexEdge::TopLeft), HexCorner::BottomRight)
            }
            HexCorner::Left => {
                self.corner_index(pos.neighbor(HexEdge::BottomLeft), HexCorner::TopRight)
            }
            HexCorner::BottomLeft if u == 0 => ((v / 2) as usize, corner),
            HexCorner::BottomLeft if v == 0 => (((height + 1) / 2 + u / 2 - 1) as usize, corner),
            HexCorner::BottomLeft => {
                self.corner_index(pos.neighbor(HexEdge::BottomLeft), HexCorner::Right)
            }
            HexCorner::BottomRight if v < 2 => (u as usize, corner),
            HexCorner::BottomRight => {
                self.corner_index(pos.neighbor(HexEdge::Bottom), HexCorner::TopRight)
            }
        }
    }

    pub fn perimeter(&self) -> HexGridPerimeterIterator {
        HexGridPerimeterIterator::new(self.size)
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
    type Iterator = HexPosIterator;

    fn contains_hex(&self, pos: HexPos) -> bool {
        self.size.contains_hex(pos)
    }

    fn iter_hexes(&self) -> Self::Iterator {
        self.size.iter_hexes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{corner::HexPosWithCorner, edge::HexPosWithEdge, iter_valid_sizes};
    use std::collections::HashSet;

    #[test]
    fn index() {
        for size in iter_valid_sizes() {
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
    }

    #[test]
    fn vec_dimensions() {
        for size in iter_valid_sizes() {
            eprintln!("size: {:?}", size);
            let grid = HexGrid::<i32>::new_with_defaults(size);
            let mut hex_grid_size = 0;
            let mut bottom_left_edges_size = 0;
            let mut bottom_edges_size = 0;
            let mut bottom_right_edges_size = 0;
            let mut top_left_corners_size = 0;
            let mut left_corners_size = 0;
            let mut bottom_left_corners_size = 0;
            let mut bottom_right_corners_size = 0;

            for pos in grid.iter_hexes() {
                let index = grid.hex_index(pos);
                hex_grid_size = hex_grid_size.max(index + 1);

                for edge in HexEdge::ALL {
                    let (index, index_edge) = grid.edge_index(pos, edge);
                    match index_edge {
                        HexEdge::BottomLeft => {
                            bottom_left_edges_size = bottom_left_edges_size.max(index + 1);
                        }
                        HexEdge::Bottom => {
                            bottom_edges_size = bottom_edges_size.max(index + 1);
                        }
                        HexEdge::BottomRight => {
                            bottom_right_edges_size = bottom_right_edges_size.max(index + 1);
                        }
                        _ => {}
                    }
                }

                for corner in HexCorner::ALL {
                    let (index, index_corner) = grid.corner_index(pos, corner);
                    match index_corner {
                        HexCorner::TopLeft => {
                            top_left_corners_size = top_left_corners_size.max(index + 1);
                        }
                        HexCorner::Left => {
                            left_corners_size = left_corners_size.max(index + 1);
                        }
                        HexCorner::BottomLeft => {
                            bottom_left_corners_size = bottom_left_corners_size.max(index + 1);
                        }
                        HexCorner::BottomRight => {
                            bottom_right_corners_size = bottom_right_corners_size.max(index + 1);
                        }
                        _ => {}
                    }
                }
            }

            assert_eq!(hex_grid_size, grid.hexes.len());
            assert_eq!(bottom_left_edges_size, grid.bottom_left_edges.len());
            assert_eq!(bottom_edges_size, grid.bottom_edges.len());
            assert_eq!(bottom_right_edges_size, grid.bottom_right_edges.len());
            assert_eq!(top_left_corners_size, grid.top_left_corners.len());
            assert_eq!(left_corners_size, grid.left_corners.len());
            assert_eq!(bottom_left_corners_size, grid.bottom_left_corners.len());
            assert_eq!(bottom_right_corners_size, grid.bottom_right_corners.len());
        }
    }

    #[test]
    fn edge_order() {
        for size in iter_valid_sizes() {
            eprintln!("size: {:?}", size);
            let grid = HexGrid::new(size, |_| (), |edge| edge, |_| ());
            for pos in grid.iter_hexes() {
                for edge in HexEdge::ALL {
                    assert_eq!(
                        HexPosWithEdge::new(pos, edge),
                        *grid.edge(pos, edge),
                        "pos: {:?}, edge: {:?}",
                        pos,
                        edge
                    );
                }
            }
        }
    }

    #[test]
    fn corner_order() {
        for size in iter_valid_sizes() {
            eprintln!("size: {:?}", size);
            let grid = HexGrid::new(size, |_| (), |edge| (), |corner| corner);
            for pos in grid.iter_hexes() {
                for corner in HexCorner::ALL {
                    assert_eq!(
                        HexPosWithCorner::new(pos, corner),
                        *grid.corner(pos, corner)
                    );
                }
            }
        }
    }
}
