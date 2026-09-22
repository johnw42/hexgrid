use crate::delta::HexDelta;
use crate::{HexCoord, corner::HexCorner, edge::HexEdge, pos::HexPos};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, Sub};

pub trait HexId:
    Clone + Debug + PartialEq + Eq + Hash + Add<HexDelta, Output = Self> + Sub<HexDelta, Output = Self>
{
    fn pos(&self) -> HexPos;
    fn corners(&self) -> impl Iterator<Item = HexCorner> + '_;
    fn edges(&self) -> impl Iterator<Item = HexEdge> + '_;

    fn u(&self) -> HexCoord {
        self.pos().u()
    }

    fn v(&self) -> HexCoord {
        self.pos().v()
    }

    fn u_v(&self) -> (HexCoord, HexCoord) {
        self.pos().u_v()
    }

    fn rotated_around(self, center: HexPos, steps: HexCoord) -> Self;
}
