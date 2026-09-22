use crate::{
    HexCoord,
    corner::{HexCorner, NormHexCorner},
    pos::HexPos,
};
use std::fmt::Display;

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
            HexCorner::TopLeft => (pos.shift(-1, 1), NormHexCorner::Right),
            HexCorner::Left => (pos.shift(-1, -1), NormHexCorner::TopRight),
            HexCorner::BottomLeft => (pos.shift(-1, -1), NormHexCorner::Right),
            HexCorner::BottomRight => (pos.shift(0, -2), NormHexCorner::TopRight),
        }
    }

    pub fn variants(self) -> [Self; 3] {
        let (pos, corner) = self.norm();
        match corner {
            NormHexCorner::Right => [
                (pos, HexCorner::Right).into(),
                (pos.shift(1, 1), HexCorner::BottomLeft).into(),
                (pos.shift(1, -1), HexCorner::TopLeft).into(),
            ],
            NormHexCorner::TopRight => [
                (pos, HexCorner::TopRight).into(),
                (pos.shift(0, 2), HexCorner::BottomRight).into(),
                (pos.shift(1, 1), HexCorner::Left).into(),
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
