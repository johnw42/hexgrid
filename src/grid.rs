#![allow(unused)] // TODO

use crate::{
    HexCoord,
    corner::{HexCorner, HexCornerIterator},
    edge::{HexEdge, HexEdgeIterator},
    perimeter::HexGridPerimeterIterator,
    pos::{HexPos, HexPosIterator},
    validate_grid_size,
};
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    sync::Arc,
};

pub struct HexGrid<H, E = (), C = ()> {
    hexes: Vec<Hex<H, E, C>>,
    bottom_left_edges: Vec<E>,
    bottom_edges: Vec<E>,
    bottom_right_edges: Vec<E>,
    top_left_corners: Vec<C>,
    left_corners: Vec<C>,
    bottom_left_corners: Vec<C>,
    bottom_right_corners: Vec<C>,
    width: HexCoord,
    height: HexCoord,
    even_row_size: HexCoord,
    odd_row_size: HexCoord,
}

struct Hex<H, E, C> {
    data: H,
    edges: [E; 3],
    corners: [C; 2],
}

impl<H, E, C> HexGrid<H, E, C> {
    pub fn new(
        width: HexCoord,
        height: HexCoord,
        mut h: impl FnMut(HexPos) -> H,
        mut e: impl FnMut(HexPos, HexEdge) -> E,
        mut c: impl FnMut(HexPos, HexCorner) -> C,
    ) -> Self
    where
        H: Default,
        E: Default,
        C: Default,
    {
        assert_eq!(validate_grid_size(width, height), Ok(()));

        let even_row_size = 1 + (width - 1) / 2;
        let odd_row_size = width / 2;

        let mut grid = HexGrid {
            hexes: Vec::new(),
            bottom_left_edges: Vec::new(),
            bottom_edges: Vec::new(),
            bottom_right_edges: Vec::new(),
            top_left_corners: Vec::new(),
            left_corners: Vec::new(),
            bottom_left_corners: Vec::new(),
            bottom_right_corners: Vec::new(),
            width,
            height,
            even_row_size,
            odd_row_size,
        };

        if width > 0 && height > 0 {
            grid.hexes.reserve_exact(grid.hex_range().count());
            for pos in grid.hex_range() {
                let hex = Hex {
                    data: h(pos),
                    edges: [
                        e(pos, HexEdge::TopRight),
                        e(pos, HexEdge::Top),
                        e(pos, HexEdge::TopLeft),
                    ],
                    corners: [c(pos, HexCorner::Right), c(pos, HexCorner::TopRight)],
                };
                grid.hexes.push(hex);
            }
            for i in 0..((height + 1) / 2) {
                grid.bottom_left_edges
                    .push(e(HexPos::new(0, i * 2), HexEdge::BottomLeft));
            }
            for i in 1..((width + 1) / 2) {
                grid.bottom_left_edges
                    .push(e(HexPos::new(i * 2, 0), HexEdge::BottomLeft));
            }

            for i in 0..((width + 1) / 2) {
                grid.bottom_right_edges
                    .push(e(HexPos::new(0, i * 2), HexEdge::BottomRight));
            }
            if width % 2 == 0 {
                for i in 0..(height / 2) {
                    grid.bottom_right_edges
                        .push(e(HexPos::new(width - 1, i * 2 + 1), HexEdge::BottomRight));
                }
            } else {
                for i in 1..((height + 1) / 2) {
                    grid.bottom_right_edges
                        .push(e(HexPos::new(width - 1, i * 2), HexEdge::BottomRight));
                }
            }
            for u in 0..width {
                grid.bottom_edges
                    .push(e(HexPos::new(u, u % 2), HexEdge::Bottom));
            }
            for i in 0..(height + 1) / 2 {
                grid.top_left_corners
                    .push(c(HexPos::new(0, 2 * i), HexCorner::TopLeft));
            }
            if height % 2 == 0 {
                for i in 0..(width / 2) {
                    grid.top_left_corners
                        .push(c(HexPos::new(2 * i + 1, height - 1), HexCorner::TopLeft));
                }
            } else {
                for i in 1..((width + 1) / 2) {
                    grid.top_left_corners
                        .push(c(HexPos::new(2 * i, height - 1), HexCorner::TopLeft));
                }
            }
            for i in 0..(height + 1) / 2 {
                grid.left_corners
                    .push(c(HexPos::new(0, 2 * i), HexCorner::Left));
            }
            for i in 0..((height + 1) / 2) {
                grid.bottom_left_corners
                    .push(c(HexPos::new(0, 2 * i), HexCorner::BottomLeft));
            }
            for i in 1..((width + 1) / 2) {
                grid.bottom_left_corners
                    .push(c(HexPos::new(i * 2, 0), HexCorner::BottomLeft));
            }
            for u in 0..width {
                grid.bottom_right_corners
                    .push(c(HexPos::new(u, u % 2), HexCorner::BottomRight));
            }
        }

        if size_of::<H>() + size_of::<E>() + size_of::<C>() > 0 {
            debug_assert_eq!(grid.hexes.len(), grid.hexes.capacity());
        }

        grid
    }

    pub fn new_with_defaults(width: HexCoord, height: HexCoord) -> Self
    where
        H: Default,
        E: Default,
        C: Default,
    {
        Self::new(
            width,
            height,
            |_| H::default(),
            |_, _| E::default(),
            |_, _| C::default(),
        )
    }

    pub fn width(&self) -> HexCoord {
        self.width
    }

    pub fn height(&self) -> HexCoord {
        self.height
    }

    pub fn has_hex(&self, pos: HexPos) -> bool {
        pos.in_range(self.width, self.height)
    }

    pub fn hex_range(&self) -> HexPosIterator {
        HexPosIterator::new(self.width, self.height)
    }

    pub fn edge_range(&self) -> HexEdgeIterator {
        HexEdgeIterator::new(self.width, self.height)
    }

    pub fn corner_range(&self) -> HexCornerIterator {
        HexCornerIterator::new(self.width, self.height)
    }

    pub fn hex(&self, pos: HexPos) -> &H {
        &self.hexes[self.index(pos)].data
    }

    pub fn hex_mut(&mut self, pos: HexPos) -> &mut H {
        let index = self.index(pos);
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

    fn index(&self, pos: HexPos) -> usize {
        assert!(
            self.has_hex(pos),
            "Hex at position {:?} does not exist",
            pos
        );
        let u = pos.u();
        let v = pos.v();
        if v % 2 == 0 {
            (u / 2 + self.even_row_size * (v / 2) + self.odd_row_size * ((v + 1) / 2)) as usize
        } else {
            (u / 2 + self.even_row_size * ((v + 1) / 2) + self.odd_row_size * (v / 2)) as usize
        }
    }

    pub fn edge_index(&self, pos: HexPos, edge: HexEdge) -> (usize, HexEdge) {
        match edge {
            HexEdge::TopRight | HexEdge::Top | HexEdge::TopLeft => (self.index(pos), edge),
            HexEdge::BottomLeft if pos.u() == 0 => ((pos.v() / 2) as usize, edge),
            HexEdge::BottomLeft if pos.v() == 0 => {
                ((pos.u() / 2 + (self.height + 1) / 2 - 1) as usize, edge)
            }
            HexEdge::Bottom if pos.v() <= 1 => (pos.u() as usize, edge),
            HexEdge::BottomRight if pos.v() == 0 => ((pos.u() / 2) as usize, edge),
            HexEdge::BottomRight if pos.u() == self.width - 1 => {
                ((self.width / 2 + pos.v() / 2) as usize, edge)
            }
            HexEdge::BottomLeft | HexEdge::Bottom | HexEdge::BottomRight => {
                let neighbor = pos.neighbor(edge);
                (self.index(neighbor), edge.opposite())
            }
        }
    }

    pub fn corner_index(&self, pos: HexPos, corner: HexCorner) -> (usize, HexCorner) {
        match corner {
            HexCorner::Right | HexCorner::TopRight => (self.index(pos), corner),
            HexCorner::TopLeft if pos.u() == 0 => ((pos.v() / 2) as usize, corner),
            HexCorner::TopLeft if pos.v() == self.height - 1 => {
                ((self.height() / 2 + pos.u() / 2) as usize, corner)
            }
            HexCorner::TopLeft => {
                self.corner_index(pos.neighbor(HexEdge::TopLeft), HexCorner::Right)
            }
            HexCorner::Left if pos.u() == 0 => ((pos.v() / 2) as usize, corner),
            HexCorner::Left if pos.v() == 0 => {
                self.corner_index(pos.neighbor(HexEdge::TopLeft), HexCorner::BottomRight)
            }
            HexCorner::Left => {
                self.corner_index(pos.neighbor(HexEdge::BottomLeft), HexCorner::TopRight)
            }
            HexCorner::BottomLeft if pos.u() == 0 => ((pos.v() / 2) as usize, corner),
            HexCorner::BottomLeft if pos.v() == 0 => {
                (((self.height + 1) / 2 + pos.u() / 2 - 1) as usize, corner)
            }
            HexCorner::BottomLeft => {
                self.corner_index(pos.neighbor(HexEdge::BottomLeft), HexCorner::Right)
            }
            HexCorner::BottomRight if pos.v() < 2 => (pos.u() as usize, corner),
            HexCorner::BottomRight => {
                self.corner_index(pos.neighbor(HexEdge::Bottom), HexCorner::TopRight)
            }
        }
    }

    pub fn perimeter(&self) -> HexGridPerimeterIterator {
        HexGridPerimeterIterator::new(self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::iter_valid_sizes;
    use std::collections::HashSet;

    #[test]
    fn index() {
        for (width, height) in iter_valid_sizes() {
            let grid = HexGrid::<i32>::new_with_defaults(width, height);
            for (i, pos) in grid.hex_range().enumerate() {
                assert_eq!(
                    grid.index(pos),
                    i,
                    "size: {:?}, pos: {:?}, i: {}",
                    (width, height),
                    pos,
                    i
                );
            }
        }
    }

    #[test]
    fn vec_dimensions() {
        for (width, height) in iter_valid_sizes() {
            eprintln!("size: {:?}", (width, height));
            let grid = HexGrid::<i32>::new_with_defaults(width, height);
            let mut hex_grid_size = 0;
            let mut bottom_left_edges_size = 0;
            let mut bottom_edges_size = 0;
            let mut bottom_right_edges_size = 0;
            let mut top_left_corners_size = 0;
            let mut left_corners_size = 0;
            let mut bottom_left_corners_size = 0;
            let mut bottom_right_corners_size = 0;

            for pos in grid.hex_range() {
                let index = grid.index(pos);
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
}
