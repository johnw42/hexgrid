//    1,3   3,3   5,3
//
// 0,2   2,2   4,2
//
//    1,1   3,1   5,1
//
// 0,0   2,0   4,0

use std::{f32::consts::FRAC_PI_3, fmt::Display};

use crate::{HexCoord, corner::HexCorner, edge::HexEdge};

/// A position in a hexagonal grid, represented by two coordinates (u, v), where
/// u + v is always even.  The origin (0, 0) is at the bottom left corner of the
/// grid, following typical math conventions.  The u coordinate increases to the
/// right, and the v coordinate increases upwards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexPos(HexCoord, HexCoord);

impl Display for HexPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

pub struct HexPosRange {
    current: HexPos,
    left: HexCoord,
    right: HexCoord,
    bottom: HexCoord,
}

impl Iterator for HexPosRange {
    type Item = HexPos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.1 > self.bottom {
            return None;
        }
        let result = self.current;
        self.current.0 += 1;
        if self.current.0 + 1 > self.right {
            self.current.0 = self.left;
            self.current.1 += 1;
        }
        if (self.current.0 + self.current.1) % 2 != 0 {
            self.current.0 += 1;
        }
        Some(result)
    }
}

impl HexPos {
    pub fn new(x: HexCoord, y: HexCoord) -> Self {
        assert!((x + y) % 2 == 0, "x + y must be even");
        HexPos(x, y)
    }

    pub fn u(self) -> HexCoord {
        self.0
    }

    pub fn v(self) -> HexCoord {
        self.1
    }

    pub fn range(left: HexCoord, top: HexCoord, right: HexCoord, bottom: HexCoord) -> HexPosRange {
        HexPosRange {
            current: HexPos::new(left, top),
            left,
            right,
            bottom,
        }
    }

    pub fn from_center((x, y): (f32, f32)) -> Self {
        // Fractional axial coordinates for flat-topped hexes (R = 1.0)
        let frac_q = (2.0 / 3.0) * x;
        let frac_r = (-1.0 / 3.0) * x + (3.0_f32.sqrt() / 3.0) * y;
        let frac_s = -frac_q - frac_r;

        // Round to nearest integer cube coordinates
        let mut q = frac_q.round();
        let mut r = frac_r.round();
        let s = frac_s.round();

        // Fix rounding errors so that q + r + s == 0
        let q_diff = (q - frac_q).abs();
        let r_diff = (r - frac_r).abs();
        let s_diff = (s - frac_s).abs();

        if q_diff > r_diff && q_diff > s_diff {
            q = -r - s;
        } else if r_diff > s_diff {
            r = -q - s;
        } else {
            // s = -q - r;
        }

        // Convert to HexPos coordinates
        let q = q as HexCoord;
        let r = r as HexCoord;
        HexPos::new(q, 2 * r + q)
    }

    pub fn center(&self) -> (f32, f32) {
        let HexPos(x, y) = *self;

        let y_scale = (3.0_f32).sqrt() / 2.0;
        let x_scale = 1.5;
        (x_scale * (x as f32), y as f32 * y_scale)
    }

    pub fn corners(&self) -> [(f32, f32); 6] {
        let (cx, cy) = self.center();
        let mut corners = [(0.0, 0.0); 6];
        for (i, corner) in corners.iter_mut().enumerate() {
            let angle = (i as f32) * std::f32::consts::FRAC_PI_3;
            *corner = (cx + angle.cos(), cy + angle.sin());
        }
        corners
    }

    // pub fn shift(&self, direction: HexPoint) -> Self {
    //     self.shift_by(1, direction)
    // }

    // pub fn shift_by(&self, distance: i32, direction: HexPoint) -> Self {
    //     let HexPos(x, y) = *self;
    //     match direction {
    //         HexPoint::Right => HexPos(x + 2 * distance, y),
    //         HexPoint::UpRight => HexPos(x + distance, y + distance),
    //         HexPoint::UpLeft => HexPos(x - distance, y + distance),
    //         HexPoint::Left => HexPos(x - 2 * distance, y),
    //         HexPoint::DownLeft => HexPos(x - distance, y - distance),
    //         HexPoint::DownRight => HexPos(x + distance, y - distance),
    //     }
    // }

    // pub fn adjacent(&self) -> [HexPos; 6] {
    //     [
    //         self.shift(HexPoint::Right),
    //         self.shift(HexPoint::UpRight),
    //         self.shift(HexPoint::UpLeft),
    //         self.shift(HexPoint::Left),
    //         self.shift(HexPoint::DownLeft),
    //         self.shift(HexPoint::DownRight),
    //     ]
    // }

    pub fn adjacent(self, edge: HexEdge) -> Self {
        let HexPos(x, y) = self;
        match edge {
            HexEdge::TopRight => HexPos(x + 1, y + 1),
            HexEdge::Top => HexPos(x, y + 2),
            HexEdge::TopLeft => HexPos(x - 1, y + 1),
            HexEdge::BottomLeft => HexPos(x - 1, y - 1),
            HexEdge::Bottom => HexPos(x, y - 2),
            HexEdge::BottomRight => HexPos(x + 1, y - 1),
        }
    }

    pub fn nearest_edge(self, point: (f32, f32)) -> HexEdge {
        let (cx, cy) = self.center();
        let dx = point.0 - cx;
        let dy = point.1 - cy;
        let angle = dy.atan2(dx);
        let index =
            ((angle + std::f32::consts::FRAC_PI_6) / std::f32::consts::FRAC_PI_3).round() as i32;
        HexEdge::all()[index.rem_euclid(6) as usize]
    }

    pub fn nearest_corner(self, point: (f32, f32)) -> HexCorner {
        let (cx, cy) = self.center();
        let dx = point.0 - cx;
        let dy = point.1 - cy;
        let angle = dy.atan2(dx);
        let index = ((angle + std::f32::consts::FRAC_PI_6 / 2.0) / std::f32::consts::FRAC_PI_3)
            .round() as i32;
        HexCorner::all()[index.rem_euclid(6) as usize]
    }
}
