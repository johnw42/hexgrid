#![allow(unused)] // TODO

use crate::{HexCoord, corner::HexCorner, edge::HexEdge, pos::HexPos};
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    sync::Arc,
};

pub struct HexGrid<H, E, C> {
    hexes: Vec<Hex<H, E, C>>,
    bottom_right_edges: Vec<E>,
    bottom_edges: Vec<E>,
    bottom_left_edges: Vec<E>,
    bottom_right_corners: Vec<C>,
    bottom_left_corners: Vec<C>,
    left_corners: Vec<C>,
    left: HexCoord,
    bottom: HexCoord,
    right: HexCoord,
    top: HexCoord,
    origin_index: HexCoord,
    hexes_left_of_origin: HexCoord,
    hexes_right_of_origin: HexCoord,
    even_row_size: HexCoord,
    odd_row_size: HexCoord,
}

struct Hex<H, E, C> {
    data: H,
    edges: [E; 3],
    corners: [C; 3],
}

impl<H, E, C> HexGrid<H, E, C> {
    pub fn new(left: HexCoord, bottom: HexCoord, right: HexCoord, top: HexCoord) -> Self
    where
        H: Default,
        E: Default,
        C: Default,
    {
        assert!(left <= 0, "Left coordinate must be less than or equal to 0");
        assert!(
            bottom <= 0,
            "Bottom coordinate must be less than or equal to 0"
        );
        assert!(
            right >= 0,
            "Right coordinate must be greater than or equal to 0"
        );
        assert!(
            top >= 0,
            "Top coordinate must be greater than or equal to 0"
        );

        let even_row_size = 1 + (-left / 2) + right / 2;
        let odd_row_size = (-left + 1) / 2 + (right + 1) / 2;
        let hexes_left_of_origin = -left / 2;
        let hexes_right_of_origin = right / 2;
        let even_rows_below_origin = -bottom / 2;
        let odd_rows_below_origin = (-bottom + 1) / 2;
        let origin_index = hexes_left_of_origin
            + even_row_size * even_rows_below_origin
            + odd_row_size * odd_rows_below_origin;

        dbg!(
            even_row_size,
            odd_row_size,
            hexes_left_of_origin,
            hexes_right_of_origin,
            even_rows_below_origin,
            odd_rows_below_origin,
            origin_index
        );

        HexGrid {
            hexes: Vec::new(),
            bottom_right_edges: Vec::new(),
            bottom_edges: Vec::new(),
            bottom_left_edges: Vec::new(),
            bottom_right_corners: Vec::new(),
            bottom_left_corners: Vec::new(),
            left_corners: Vec::new(),
            left,
            bottom,
            right,
            top,
            origin_index,
            even_row_size,
            odd_row_size,
            hexes_left_of_origin,
            hexes_right_of_origin,
        }
    }

    pub fn top(&self) -> HexCoord {
        self.top
    }

    pub fn left(&self) -> HexCoord {
        self.left
    }

    pub fn bottom(&self) -> HexCoord {
        self.bottom
    }

    pub fn right(&self) -> HexCoord {
        self.right
    }

    pub fn width(&self) -> HexCoord {
        self.right - self.left + 1
    }

    pub fn height(&self) -> HexCoord {
        self.top - self.bottom + 1
    }

    pub fn has_hex(&self, pos: HexPos) -> bool {
        pos.u() >= self.left
            && pos.v() <= self.top
            && pos.u() <= self.right
            && pos.v() >= self.bottom
    }

    pub fn range(&self) -> impl Iterator<Item = HexPos> {
        HexPos::range(self.left, self.bottom, self.right, self.top)
    }

    pub fn hex(&self, pos: HexPos) -> &H {
        &self.get(pos).data
    }

    pub fn hex_mut(&mut self, pos: HexPos) -> &mut H {
        &mut self.get_mut(pos).data
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

    fn get(&self, pos: HexPos) -> &Hex<H, E, C> {
        assert!(
            self.has_hex(pos),
            "Hex at position {:?} does not exist",
            pos
        );
        todo!()
    }

    fn get_mut(&mut self, pos: HexPos) -> &mut Hex<H, E, C> {
        assert!(
            self.has_hex(pos),
            "Hex at position {:?} does not exist",
            pos
        );
        todo!()
    }

    pub fn index(&self, pos: HexPos) -> usize {
        // assert!(
        //     self.has_hex(pos),
        //     "Hex at position {:?} does not exist",
        //     pos
        // );
        let u = pos.u();
        let v = pos.v();
        let v_sign = if v >= 0 { 1 } else { -1 };
        let index = (self.origin_index
            + u / 2
            + self.even_row_size * (v / 2)
            + self.odd_row_size * ((v + v_sign) / 2)) as usize;
        if u == 0 && v == 0 {
            assert_eq!(index, self.origin_index as usize);
        }
        // if v % 2 == 0 {
        //     assert_eq!(
        //         self.range().nth(index),
        //         Some(pos),
        //         "pos: {:?}, index: {}, range().nth(index): {:?}",
        //         pos,
        //         index,
        //         self.range().nth(index)
        //     );
        // }
        index
    }
}

#[test]
fn test_index() {
    for left in -3..=0 {
        for bottom in -3..=0 {
            for right in 0..=3 {
                for top in 0..=3 {
                    let grid = HexGrid::<(), (), ()>::new(left, bottom, right, top);
                    for (i, pos) in grid.range().enumerate() {
                        if pos.v() % 2 == 0 {
                            assert_eq!(
                                grid.index(pos),
                                i,
                                "grid: {:?}, pos: {:?}, i: {}",
                                (left, bottom, right, top),
                                pos,
                                i
                            );
                        }
                    }
                }
            }
        }
    }
}
