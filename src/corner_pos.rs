use crate::{
    HexCoord,
    corner::{HexCorner, NormHexCorner},
    delta::HexDelta,
    id::HexId,
    pos::HexPos,
};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, Sub},
};

/// A struct representing a position of a corner of a hexagon in a hexagonal
/// grid.
///
/// Each corner has three possible representations, which are equivalent. The
/// `norm` method returns a normalized representation of the corner position,
/// which is unique for each corner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexCornerPos<C = HexCorner>
where
    C: Into<HexCorner>,
{
    pub pos: HexPos,
    pub corner: C,
}

impl<C> HexCornerPos<C>
where
    C: Into<HexCorner>,
{
    /// Returns the u coordinate of the hexagon position.
    pub fn u(self) -> HexCoord {
        self.pos.u()
    }

    /// Returns the v coordinate of the hexagon position.
    pub fn v(&self) -> HexCoord {
        self.pos.v()
    }

    /// Returns the `HexPos` of the hexagon position.
    pub fn pos(self) -> HexPos {
        self.pos
    }

    /// Returns the `HexCorner` of the corner position.
    pub fn corner(self) -> C {
        self.corner
    }

    /// Returns the `HexPos` and `HexCorner` of the corner position as a tuple.
    pub fn pos_corner(self) -> (HexPos, C) {
        (self.pos, self.corner)
    }

    /// Returns the u and v coordinates of the hexagon position and the
    /// `HexCorner` of the corner position as a tuple.
    pub fn u_v_corner(self) -> (HexCoord, HexCoord, C) {
        (self.pos.u(), self.pos.v(), self.corner)
    }
}

impl HexCornerPos {
    /// Returns a normalized representation of the corner position, which is
    /// unique for each corner.
    pub fn norm(self) -> HexCornerPos<NormHexCorner> {
        let pos = self.pos;
        match self.corner {
            HexCorner::Right => (pos + HexDelta::new(1, -1), NormHexCorner::TopLeft),
            HexCorner::TopRight => (pos, NormHexCorner::TopRight),
            HexCorner::TopLeft => (pos, NormHexCorner::TopLeft),
            HexCorner::Left => (pos + HexDelta::new(-1, -1), NormHexCorner::TopRight),
            HexCorner::BottomLeft => (pos + HexDelta::new(0, -2), NormHexCorner::TopLeft),
            HexCorner::BottomRight => (pos + HexDelta::new(0, -2), NormHexCorner::TopRight),
        }
        .into()
    }

    /// Returns the three equivalent representations of the corner position, for which
    /// `norm` returns the same value.
    pub fn variants(self) -> [Self; 3] {
        let (pos, corner) = self.norm().pos_corner();
        match corner {
            NormHexCorner::TopRight => [
                (pos, HexCorner::TopRight).into(),
                (pos + HexDelta::new(1, 1), HexCorner::Left).into(),
                (pos + HexDelta::new(0, 2), HexCorner::BottomRight).into(),
            ],
            NormHexCorner::TopLeft => [
                (pos, HexCorner::TopLeft).into(),
                (pos + HexDelta::new(0, 2), HexCorner::BottomLeft).into(),
                (pos + HexDelta::new(-1, 1), HexCorner::Right).into(),
            ],
        }
    }
}

impl<C> From<(HexCoord, HexCoord, C)> for HexCornerPos<C>
where
    C: Into<HexCorner>,
{
    fn from((u, v, corner): (HexCoord, HexCoord, C)) -> Self {
        Self {
            pos: HexPos::new(u, v),
            corner,
        }
    }
}

impl<C> From<(HexPos, C)> for HexCornerPos<C>
where
    C: Into<HexCorner>,
{
    fn from((pos, corner): (HexPos, C)) -> Self {
        Self { pos, corner }
    }
}

impl<C> Display for HexCornerPos<C>
where
    C: Into<HexCorner> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {:?})", self.pos.u(), self.pos.v(), self.corner)
    }
}

impl<C> Add<HexDelta> for HexCornerPos<C>
where
    C: Into<HexCorner>,
{
    type Output = Self;

    fn add(self, delta: HexDelta) -> Self::Output {
        Self {
            pos: self.pos + delta,
            corner: self.corner,
        }
    }
}

impl<C> Sub<HexDelta> for HexCornerPos<C>
where
    C: Into<HexCorner>,
{
    type Output = Self;

    fn sub(self, delta: HexDelta) -> Self::Output {
        Self {
            pos: self.pos - delta,
            corner: self.corner,
        }
    }
}

impl HexId for HexCornerPos {
    fn pos(&self) -> HexPos {
        self.pos
    }

    fn corners(&self) -> impl Iterator<Item = HexCorner> + '_ {
        std::iter::once(self.corner)
    }

    fn edges(&self) -> impl Iterator<Item = crate::HexEdge> + '_ {
        std::iter::empty()
    }

    fn rotate_around(self, center: HexPos, steps: HexCoord) -> Self {
        Self {
            pos: self.pos.rotate_around(center, steps),
            corner: self.corner.rotate(steps),
        }
    }

    fn shift(self, delta: HexDelta) -> Self {
        Self {
            pos: self.pos.shift(delta),
            corner: self.corner,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants() {
        for corner in HexCorner::ALL {
            let corner_pos = HexCornerPos::from((0, 0, corner));
            for variant in corner_pos.variants() {
                assert_eq!(variant.norm(), corner_pos.norm());
            }
        }
    }
}
