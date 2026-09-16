#![allow(unused)] // TODO

use crate::{HexCoord, corner::HexCorner, edge::HexEdge, pos::HexPos};
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    sync::Arc,
};

pub struct HexGrid<H, E = (), C = ()> {
    hexes: Vec<Hex<H, E, C>>,
    bottom_right_edges: Vec<E>,
    bottom_edges: Vec<E>,
    bottom_left_edges: Vec<E>,
    bottom_right_corners: Vec<C>,
    bottom_left_corners: Vec<C>,
    left_corners: Vec<C>,
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
        let even_row_size = 1 + (width - 1) / 2;
        let odd_row_size = width / 2;

        let mut grid = HexGrid {
            hexes: Vec::new(),
            bottom_right_edges: Vec::new(),
            bottom_edges: Vec::new(),
            bottom_left_edges: Vec::new(),
            bottom_right_corners: Vec::new(),
            bottom_left_corners: Vec::new(),
            left_corners: Vec::new(),
            width,
            height,
            even_row_size,
            odd_row_size,
        };

        if width > 0 && height > 0 {
            // let last_hex_pos = if (width + height) % 2 == 0 {
            //     HexPos::new(width - 1, height - 1)
            // } else {
            //     HexPos::new(width - 2, height - 1)
            // };
            // let last_index = grid.index(last_hex_pos);
            // grid.hexes.reserve(last_index + 1);
            for pos in grid.range() {
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
        }

        //debug_assert_eq!(grid.hexes.len(), grid.hexes.capacity());

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
        pos.u() >= 0 && pos.v() >= 0 && pos.v() < self.height && pos.u() < self.width
    }

    pub fn range(&self) -> impl Iterator<Item = HexPos> + 'static {
        HexPos::range(self.width, self.height)
    }

    pub fn hex(&self, pos: HexPos) -> &H {
        &self.hexes[self.index(pos)].data
    }

    pub fn hex_mut(&mut self, pos: HexPos) -> &mut H {
        let index = self.index(pos);
        &mut self.hexes[index].data
    }

    pub fn edge(&self, pos: HexPos, edge: HexEdge) -> &E {
        assert!(
            self.has_hex(pos),
            "Hex at position {:?} does not exist",
            pos
        );
        todo!()
    }
    pub fn edge_mut(&mut self, pos: HexPos, edge: HexEdge) -> &mut E {
        assert!(
            self.has_hex(pos),
            "Hex at position {:?} does not exist",
            pos
        );
        todo!()
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
}

#[test]
fn test_index() {
    for width in 0..=9 {
        for height in 0..=9 {
            let grid = HexGrid::<()>::new_with_defaults(width, height);
            for (i, pos) in grid.range().enumerate() {
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
