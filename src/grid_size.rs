use crate::{
    HexCoord,
    container::HexPosContainer,
    corner::HexCorner,
    corner_pos::HexCornerPos,
    edge::HexEdge,
    edge_pos::HexEdgePos,
    pos::{HexPos, HexPosIterator},
};
#[cfg(test)]
use quickcheck::Arbitrary;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexGridSize {
    width: HexCoord,
    height: HexCoord,
}

impl HexGridSize {
    pub const fn new(width: HexCoord, height: HexCoord) -> Result<Self, &'static str> {
        if width < 0 || height < 0 {
            return Err("Width and height must be non-negative");
        }
        if height == 0 && width != 0 {
            return Err("Width must be 0 if height is 0");
        }
        if width == 0 && height != 0 {
            return Err("Height must be 0 if width is 0");
        }
        if width > 1 && height <= 1 {
            return Err("Height must be greater than 1 if width is greater than 1");
        }
        Ok(Self { width, height })
    }

    pub const fn width(&self) -> HexCoord {
        self.width
    }

    pub const fn height(&self) -> HexCoord {
        self.height
    }

    pub const fn unpack(&self) -> (HexCoord, HexCoord) {
        (self.width, self.height)
    }
}

#[cfg(test)]
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
                        vec![(width - shrink_amount / 2, height - shrink_amount / 2)].into_iter()
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

impl HexPosContainer for HexGridSize {
    type Iterator<'c> = HexPosIterator;

    fn contains_hex(&self, pos: HexPos) -> bool {
        pos.u() >= 0 && pos.u() < self.width && pos.v() >= 0 && pos.v() < self.height
    }

    fn iter_hexes(&self) -> Self::Iterator<'_> {
        HexPosIterator::new(0, 0, self.width - 1, self.height - 1)
    }

    fn len(&self) -> usize {
        self.width as usize * self.height as usize / 2
            + (self.width as usize * self.height as usize % 2)
    }
}

pub struct HexEdgeIterator {
    width: HexCoord,
    edge: HexEdge,
    pos: Option<HexPos>,
    pos_iter: HexPosIterator,
}

impl HexEdgeIterator {
    pub fn new(size: HexGridSize) -> Self {
        let mut pos_iter = size.iter_hexes();
        let pos = pos_iter.next();
        Self {
            width: size.width(),
            edge: HexEdge::TopRight,
            pos,
            pos_iter,
        }
    }
}

impl Iterator for HexEdgeIterator {
    type Item = HexEdgePos;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let pos = self.pos?;
            let edge = self.edge;
            self.edge = self.edge.rotate(1);
            if self.edge == HexEdge::TopRight {
                self.pos = self.pos_iter.next();
            }
            let is_valid_edge = match edge {
                HexEdge::TopRight | HexEdge::Top | HexEdge::TopLeft => true,
                HexEdge::BottomLeft => pos.u() == 0 || pos.v() == 0,
                HexEdge::Bottom => pos.v() <= 1,
                HexEdge::BottomRight => pos.v() == 0 || pos.u() == self.width - 1,
            };
            if is_valid_edge {
                return Some(HexEdgePos::from((pos, edge)));
            }
        }
    }
}

pub struct HexCornerIterator {
    width: HexCoord,
    corner: HexCorner,
    pos: Option<HexPos>,
    pos_iter: HexPosIterator,
}

impl HexCornerIterator {
    pub fn new(size: HexGridSize) -> Self {
        let mut pos_iter = size.iter_hexes();
        let pos = pos_iter.next();
        Self {
            width: size.width(),
            corner: HexCorner::TopRight,
            pos,
            pos_iter,
        }
    }
}

impl Iterator for HexCornerIterator {
    type Item = HexCornerPos;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let pos = self.pos?;
            let corner = self.corner;
            self.corner = self.corner.rotate(1);
            if self.corner == HexCorner::TopRight {
                self.pos = self.pos_iter.next();
            }
            let is_valid_corner = match corner {
                HexCorner::Right | HexCorner::TopRight | HexCorner::TopLeft => true,
                HexCorner::Left => pos.u() == 0,
                HexCorner::BottomLeft => pos.v() == 0 && pos.v() == 0 || pos.v() == 1,
                HexCorner::BottomRight => {
                    pos.v() < 2
                        || (self.width % 2 == 0 && pos.v() == 1 && pos.u() == self.width - 1)
                }
            };
            if is_valid_corner {
                return Some(HexCornerPos::from((pos, corner)));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;
    use std::collections::HashSet;

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
    fn edge_iterator(size: HexGridSize) {
        let mut seen_edges = HashSet::new();
        for pos in size.iter_hexes() {
            for edge in HexEdge::ALL {
                seen_edges.insert(HexEdgePos::from((pos, edge)).norm());
            }
        }
        let iter_edges = HexEdgeIterator::new(size)
            .map(|e| e.norm())
            .collect::<HashSet<_>>();
        assert_eq!(seen_edges, iter_edges);
    }

    #[quickcheck]
    fn corner_iterator(size: HexGridSize) {
        let mut seen_corners = HashSet::new();
        for pos in size.iter_hexes() {
            for corner in HexCorner::ALL {
                seen_corners.insert(HexCornerPos::from((pos, corner)).norm());
            }
        }
        let iter_corners = HexCornerIterator::new(size)
            .map(|c| c.norm())
            .collect::<HashSet<_>>();
        assert_eq!(seen_corners, iter_corners);
    }
}
