use crate::{
    HexCoord,
    edge::HexEdge,
    pos::{HexPos, HexPosIterator},
};
use std::f32::consts::{FRAC_PI_3, PI};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HexCorner {
    Right,
    TopRight,
    TopLeft,
    Left,
    BottomLeft,
    BottomRight,
}

impl HexCorner {
    pub const ALL: [HexCorner; 6] = [
        HexCorner::Right,
        HexCorner::TopRight,
        HexCorner::TopLeft,
        HexCorner::Left,
        HexCorner::BottomLeft,
        HexCorner::BottomRight,
    ];

    pub fn touches_edge(self, edge: HexEdge) -> bool {
        edge.touches_corner(self)
    }

    pub fn rotate(self, steps: i32) -> HexCorner {
        Self::ALL[(self as i32 + steps).rem_euclid(6) as usize]
    }

    pub fn opposite(self) -> Self {
        self.rotate(3)
    }

    pub fn steps_to(self, other: HexCorner) -> i32 {
        let diff = (other as i32 - self as i32).rem_euclid(6);
        if diff > 3 { diff - 6 } else { diff }
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
        Self::Right.rotate((angle / FRAC_PI_3).round() as i32)
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

pub struct HexCornerIterator {
    width: HexCoord,
    corner: HexCorner,
    pos: Option<HexPos>,
    pos_iter: HexPosIterator,
}

impl HexCornerIterator {
    pub fn new(width: HexCoord, height: HexCoord) -> Self {
        let mut pos_iter = HexPosIterator::new(width, height);
        let pos = pos_iter.next();
        Self {
            width,
            corner: HexCorner::TopRight,
            pos,
            pos_iter,
        }
    }
}

impl Iterator for HexCornerIterator {
    type Item = (HexPos, HexCorner);
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
                HexCorner::BottomLeft => pos.v() == 0,
                HexCorner::BottomRight => {
                    pos.v() == 0
                        || (self.width % 2 == 0 && pos.v() == 1 && pos.u() == self.width - 1)
                }
            };
            if is_valid_corner {
                return Some((pos, corner));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotation() {
        for corner in HexCorner::ALL {
            for steps in -10..=10 {
                let rotated = corner.rotate(steps);
                assert!(
                    HexCorner::ALL.contains(&rotated),
                    "corner: {:?}, steps: {}, rotated: {:?}",
                    corner,
                    steps,
                    rotated
                );
                let steps_back = rotated.steps_to(corner);
                assert_eq!(
                    rotated.rotate(steps_back),
                    corner,
                    "corner: {:?}, steps: {}, rotated: {:?}, steps_back: {}",
                    corner,
                    steps,
                    rotated,
                    steps_back
                );
            }
        }
    }
}
