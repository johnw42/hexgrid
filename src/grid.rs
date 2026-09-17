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
                    corners: [
                        c(pos, HexCorner::Right),
                        c(pos, HexCorner::TopLeft),
                        c(pos, HexCorner::TopLeft),
                    ],
                };
                grid.hexes.push(hex);
            }
            for u in 0..(height / 2 + (width + 1) / 2) {
                grid.bottom_left_edges
                    .push(e(HexPos::new(u, u % 2), HexEdge::BottomLeft));
                grid.bottom_right_edges
                    .push(e(HexPos::new(u, u % 2), HexEdge::BottomRight));
            }
            for u in 0..width {
                grid.bottom_edges
                    .push(e(HexPos::new(u, u % 2), HexEdge::Bottom));
            }
            for i in 0..(height + 1) / 2 {
                grid.left_corners
                    .push(c(HexPos::new(0, 2 * i), HexCorner::Left));
            }
            for u in 0..(width + 1) {
                grid.bottom_left_corners
                    .push(c(HexPos::new(u, u % 2), HexCorner::BottomLeft));
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
            HexCorner::Left => &self.left_corners[index],
            HexCorner::BottomLeft => &self.bottom_left_corners[index],
            HexCorner::BottomRight => &self.bottom_right_corners[index],
            _ => &self.hexes[index].corners[corner as usize],
        }
    }

    pub fn corner_mut(&mut self, pos: HexPos, corner: HexCorner) -> &mut C {
        let (index, corner) = self.corner_index(pos, corner);
        match corner {
            HexCorner::Left => &mut self.left_corners[index],
            HexCorner::BottomLeft => &mut self.bottom_left_corners[index],
            HexCorner::BottomRight => &mut self.bottom_right_corners[index],
            _ => &mut self.hexes[index].corners[corner as usize],
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
            HexEdge::BottomLeft if pos.u() == 0 => ((pos.v() / 2) as usize, edge),
            HexEdge::BottomLeft if pos.v() == 0 => ((pos.u() / 2 + self.height / 2) as usize, edge),
            HexEdge::Bottom if pos.v() <= 1 => (pos.u() as usize, edge),
            HexEdge::BottomRight if pos.u() == self.width - 1 => {
                ((self.width / 2 + pos.v() / 2) as usize, edge)
            }
            HexEdge::BottomRight if pos.v() == 0 => ((pos.u() / 2) as usize, edge),
            HexEdge::BottomLeft | HexEdge::Bottom | HexEdge::BottomRight => {
                let neighbor = pos.neighbor(edge);
                (self.index(neighbor), edge.opposite())
            }
            _ => (self.index(pos), edge),
        }
    }

    pub fn corner_index(&self, pos: HexPos, corner: HexCorner) -> (usize, HexCorner) {
        match corner {
            HexCorner::Left if pos.u() == 0 => ((pos.v() / 2) as usize, corner),
            HexCorner::Left if pos.v() == 0 => {
                self.corner_index(pos.neighbor(HexEdge::TopLeft), HexCorner::BottomRight)
            }
            HexCorner::Left => {
                self.corner_index(pos.neighbor(HexEdge::BottomLeft), HexCorner::TopRight)
            }
            HexCorner::BottomLeft if pos.v() < 2 => (pos.u() as usize, corner),
            HexCorner::BottomLeft => {
                self.corner_index(pos.neighbor(HexEdge::Bottom), HexCorner::TopLeft)
            }
            HexCorner::BottomRight if pos.v() < 2 => (pos.u() as usize, corner),
            HexCorner::BottomRight => {
                self.corner_index(pos.neighbor(HexEdge::Bottom), HexCorner::TopRight)
            }
            _ => (self.index(pos), corner),
        }
    }

    pub fn perimeter(&self) -> HexGridPerimeterIterator {
        HexGridPerimeterIterator::new(self.width, self.height)
    }
}

#[derive(PartialEq, Eq, Debug)]
enum PerimeterTraveral {
    Right,
    Up,
    Left,
    Down,
    Done,
}

pub struct HexGridPerimeterIterator(HexGridPerimeterIteratorData);

enum HexGridPerimeterIteratorData {
    Small(HexPosIterator),
    Large {
        u: HexCoord,
        v: HexCoord,
        width: HexCoord,
        height: HexCoord,
        traveral: PerimeterTraveral,
    },
}

impl HexGridPerimeterIterator {
    pub fn new(width: HexCoord, height: HexCoord) -> Self {
        if width < 3 || height < 4 {
            Self(HexGridPerimeterIteratorData::Small(HexPosIterator::new(
                width, height,
            )))
        } else {
            Self(HexGridPerimeterIteratorData::Large {
                u: 0,
                v: 0,
                width,
                height,
                traveral: PerimeterTraveral::Right,
            })
        }
    }
}

impl Iterator for HexGridPerimeterIterator {
    type Item = HexPos;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            HexGridPerimeterIteratorData::Small(iter) => iter.next(),
            HexGridPerimeterIteratorData::Large {
                u,
                v,
                width,
                height,
                traveral,
            } => {
                let pos = HexPos::new(*u, *v);
                if *traveral == PerimeterTraveral::Right {
                    if *v == 0 {
                        *u += 1;
                        *v = 1;
                    } else {
                        debug_assert_eq!(*v, 1);
                        *u += 1;
                        *v = 0;
                    }
                    if *u >= *width {
                        *traveral = PerimeterTraveral::Up;
                        (*u, *v) = pos.into();
                    }
                }
                if *traveral == PerimeterTraveral::Up {
                    *v += 2;
                    if *v >= *height {
                        *traveral = PerimeterTraveral::Left;
                        (*u, *v) = pos.into();
                    }
                }
                if *traveral == PerimeterTraveral::Left {
                    if *v >= *height {
                        *u -= 1;
                        *v -= 1;
                    } else {
                        *u -= 1;
                        *v += 1;
                    }
                    if *u < 0 {
                        *traveral = PerimeterTraveral::Down;
                        (*u, *v) = pos.into();
                    }
                }
                if *traveral == PerimeterTraveral::Down {
                    *v -= 2;
                    if *u == 0 && *v == 0 {
                        *traveral = PerimeterTraveral::Done;
                    }
                }

                if *traveral == PerimeterTraveral::Done {
                    None
                } else {
                    Some(pos)
                }
            }
        }
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
    fn perimeter() {
        for (width, height) in iter_valid_sizes() {
            eprintln!("perimeter: size: {:?}", (width, height));
            let grid = HexGrid::<i32>::new_with_defaults(width, height);
            // grid.perimeter()
            //     .for_each(|pos| assert!(grid.has_hex(pos), "pos: {:?}", pos));
            assert_eq!(
                grid.perimeter().count(),
                grid.perimeter().collect::<HashSet<_>>().len(),
            );
        }
    }
}
