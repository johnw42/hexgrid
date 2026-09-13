use std::f32::consts::{FRAC_PI_3, PI};

use crate::edge::HexEdge;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HexCorner {
    Right,
    TopRight,
    TopLeft,
    Left,
    BottomLeft,
    BottomRight,
}

impl From<HexCorner> for usize {
    fn from(direction: HexCorner) -> Self {
        match direction {
            HexCorner::Right => 0,
            HexCorner::TopRight => 1,
            HexCorner::TopLeft => 2,
            HexCorner::Left => 3,
            HexCorner::BottomLeft => 4,
            HexCorner::BottomRight => 5,
        }
    }
}

impl TryFrom<usize> for HexCorner {
    type Error = ();

    fn try_from(index: usize) -> Result<Self, Self::Error> {
        match index {
            0 => Ok(HexCorner::Right),
            1 => Ok(HexCorner::TopRight),
            2 => Ok(HexCorner::TopLeft),
            3 => Ok(HexCorner::Left),
            4 => Ok(HexCorner::BottomLeft),
            5 => Ok(HexCorner::BottomRight),
            _ => Err(()),
        }
    }
}

impl From<HexCorner> for i32 {
    fn from(direction: HexCorner) -> Self {
        usize::from(direction) as i32
    }
}

impl From<i32> for HexCorner {
    fn from(index: i32) -> Self {
        usize::try_from(index.rem_euclid(6))
            .unwrap()
            .try_into()
            .unwrap()
    }
}

impl HexCorner {
    pub fn all() -> [HexCorner; 6] {
        [
            HexCorner::Right,
            HexCorner::TopRight,
            HexCorner::TopLeft,
            HexCorner::Left,
            HexCorner::BottomLeft,
            HexCorner::BottomRight,
        ]
    }

    pub fn rotate(self, times: i32) -> HexCorner {
        (i32::from(self) + times).into()
    }

    pub fn to_angle(self) -> f32 {
        match self {
            HexCorner::Right => 0.0,
            HexCorner::TopRight => FRAC_PI_3,
            HexCorner::TopLeft => 2.0 * FRAC_PI_3,
            HexCorner::Left => PI,
            HexCorner::BottomLeft => -2.0 * FRAC_PI_3,
            HexCorner::BottomRight => -FRAC_PI_3,
        }
    }

    pub fn offset_from_center(self, radius: f32) -> (f32, f32) {
        let angle = self.to_angle();
        (radius * angle.cos(), radius * angle.sin())
    }

    pub fn from_angle(angle: f32) -> HexCorner {
        ((angle / FRAC_PI_3).round() as i32).rem_euclid(6).into()
    }

    pub fn adjacent_edges(self) -> [HexEdge; 2] {
        match self {
            HexCorner::Right => [HexEdge::BottomRight, HexEdge::TopRight],
            HexCorner::TopRight => [HexEdge::TopRight, HexEdge::Top],
            HexCorner::TopLeft => [HexEdge::Top, HexEdge::TopLeft],
            HexCorner::Left => [HexEdge::TopLeft, HexEdge::BottomLeft],
            HexCorner::BottomLeft => [HexEdge::BottomLeft, HexEdge::Bottom],
            HexCorner::BottomRight => [HexEdge::Bottom, HexEdge::BottomRight],
        }
    }
}
