use crate::{
    HexCoord,
    corner::{HexCorner, NormHexCorner},
    delta::HexDelta,
    id::HexId,
    pos::HexPos,
};
use std::{
    fmt::Display,
    ops::{Add, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexCornerPos {
    pub pos: HexPos,
    pub corner: HexCorner,
}

impl HexCornerPos {
    pub fn u(self) -> HexCoord {
        self.pos.u()
    }

    pub fn v(self) -> HexCoord {
        self.pos.v()
    }

    pub fn pos(self) -> HexPos {
        self.pos
    }

    pub fn corner(self) -> HexCorner {
        self.corner
    }

    pub fn pos_corner(self) -> (HexPos, HexCorner) {
        (self.pos, self.corner)
    }

    pub fn u_v_corner(self) -> (HexCoord, HexCoord, HexCorner) {
        (self.pos.u(), self.pos.v(), self.corner)
    }

    pub fn norm(self) -> (HexPos, NormHexCorner) {
        let pos = self.pos;
        match self.corner {
            HexCorner::Right => (pos, NormHexCorner::Right),
            HexCorner::TopRight => (pos, NormHexCorner::TopRight),
            HexCorner::TopLeft => (pos + HexDelta::new(-1, 1), NormHexCorner::Right),
            HexCorner::Left => (pos + HexDelta::new(-1, -1), NormHexCorner::TopRight),
            HexCorner::BottomLeft => (pos + HexDelta::new(-1, -1), NormHexCorner::Right),
            HexCorner::BottomRight => (pos + HexDelta::new(0, -2), NormHexCorner::TopRight),
        }
    }

    pub fn variants(self) -> [Self; 3] {
        let (pos, corner) = self.norm();
        match corner {
            NormHexCorner::Right => [
                (pos, HexCorner::Right).into(),
                (pos + HexDelta::new(1, 1), HexCorner::BottomLeft).into(),
                (pos + HexDelta::new(1, -1), HexCorner::TopLeft).into(),
            ],
            NormHexCorner::TopRight => [
                (pos, HexCorner::TopRight).into(),
                (pos + HexDelta::new(0, 2), HexCorner::BottomRight).into(),
                (pos + HexDelta::new(1, 1), HexCorner::Left).into(),
            ],
        }
    }
}

impl<C> From<(HexCoord, HexCoord, C)> for HexCornerPos
where
    C: Into<HexCorner>,
{
    fn from((u, v, corner): (HexCoord, HexCoord, C)) -> Self {
        Self {
            pos: HexPos::new(u, v),
            corner: corner.into(),
        }
    }
}

impl<C> From<(HexPos, C)> for HexCornerPos
where
    C: Into<HexCorner>,
{
    fn from((pos, corner): (HexPos, C)) -> Self {
        Self {
            pos,
            corner: corner.into(),
        }
    }
}

impl Display for HexCornerPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {:?})", self.u(), self.v(), self.corner())
    }
}

impl Add<HexDelta> for HexCornerPos {
    type Output = Self;

    fn add(self, delta: HexDelta) -> Self::Output {
        Self {
            pos: self.pos + delta,
            corner: self.corner,
        }
    }
}

impl Sub<HexDelta> for HexCornerPos {
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
