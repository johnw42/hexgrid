#![allow(unused)] // TODO

use crate::{
    HexCoord,
    corner::{HexCorner, HexCornerIterator},
    edge::{HexEdge, HexEdgeIterator},
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
    width: HexCoord,
    height: HexCoord,
    even_row_size: HexCoord,
    odd_row_size: HexCoord,
}

struct Hex<H, E, C> {
    data: H,
    edges: [E; 3],
    corners: [C; 3],
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
                    corners: [
                        c(pos, HexCorner::Right),
                        c(pos, HexCorner::TopLeft),
                        c(pos, HexCorner::TopLeft),
                    ],
                };
                grid.hexes.push(hex);
            }
            //TODO
            for u in 0..(height / 2 + (width + 1) / 2) {
                grid.bottom_left_edges
                    .push(e(HexPos::new(u, u % 2), HexEdge::BottomLeft));
            }
            for u in 0..width {
                grid.bottom_edges
                    .push(e(HexPos::new(u, u % 2), HexEdge::Bottom));
                grid.bottom_right_edges
                    .push(e(HexPos::new(u, u % 2), HexEdge::BottomRight));
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
        assert!(
            self.has_hex(pos),
            "Hex at position {:?} does not exist",
            pos
        );
        todo!()
    }

    pub fn corner_mut(&mut self, pos: HexPos, corner: HexCorner) -> &mut C {
        assert!(
            self.has_hex(pos),
            "Hex at position {:?} does not exist",
            pos
        );
        todo!()
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
            HexEdge::BottomLeft if pos.v() == 0 => ((pos.u() / 2 + self.height / 2) as usize, edge),
            HexEdge::BottomLeft if pos.u() == 0 => ((pos.v() / 2) as usize, edge),
            HexEdge::Bottom if pos.v() <= 1 => (pos.u() as usize, edge),
            HexEdge::BottomRight if pos.v() == 0 || (pos.v() == 1 && pos.u() == self.width - 1) => {
                (pos.u() as usize, edge)
            }
            HexEdge::BottomLeft | HexEdge::Bottom | HexEdge::BottomRight => {
                let neighbor = pos.neighbor(edge);
                (self.index(neighbor), edge.opposite())
            }
            _ => (self.index(pos), edge),
        }
    }
}

#[test]
fn test_index() {
    for width in 0..=9 {
        for height in 0..=9 {
            if validate_grid_size(width, height).is_ok() {
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
    }
}
