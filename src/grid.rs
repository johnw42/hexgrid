#![allow(unused)] // TODO

use crate::{HexCoord, corner::HexCorner, edge::HexEdge, pos::HexPos};

pub struct HexGrid<H, E, C> {
    hexes: Vec<H>,
    edges: Vec<E>,
    corners: Vec<C>,
    left: HexCoord,
    top: HexCoord,
    right: HexCoord,
    bottom: HexCoord,
}

impl<H, E, C> HexGrid<H, E, C> {
    pub fn new(left: HexCoord, top: HexCoord, right: HexCoord, bottom: HexCoord) -> Self
    where
        H: Default,
        E: Default,
        C: Default,
    {
        assert!(
            left <= right,
            "Left coordinate must be less than or equal to right coordinate"
        );
        assert!(
            top <= bottom,
            "Top coordinate must be less than or equal to bottom coordinate"
        );

        let hexes = Vec::new();
        let edges = Vec::new();
        let corners = Vec::new();
        HexGrid {
            hexes,
            edges,
            corners,
            left,
            top,
            right,
            bottom,
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
        self.bottom - self.top + 1
    }

    pub fn has_hex(&self, pos: HexPos) -> bool {
        pos.u() >= self.left
            && pos.v() >= self.top
            && pos.u() <= self.right
            && pos.v() <= self.bottom
    }

    pub fn range(&self) -> impl Iterator<Item = HexPos> {
        HexPos::range(self.left, self.top, self.right, self.bottom)
    }

    pub fn hex(&self, pos: HexPos) -> &H {
        assert!(
            self.has_hex(pos),
            "Hex at position {:?} does not exist",
            pos
        );
        todo!()
    }

    pub fn hex_mut(&mut self, pos: HexPos) -> &mut H {
        assert!(
            self.has_hex(pos),
            "Hex at position {:?} does not exist",
            pos
        );
        todo!()
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
}
