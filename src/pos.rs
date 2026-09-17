//    1,3   3,3   5,3
//
// 0,2   2,2   4,2
//
//    1,1   3,1   5,1
//
// 0,0   2,0   4,0

use crate::{Cartesian, Distance, HexCoord, corner::HexCorner, edge::HexEdge};
use std::f32::consts::{FRAC_PI_3, FRAC_PI_6};
use std::fmt::Display;

pub struct NearestCorner {
    pub corner: HexCorner,
    pub distance: Distance,
    pub point: Cartesian,
}

pub struct NearestEdge {
    pub edge: HexEdge,
    pub distance: Distance,
}

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

impl From<HexPos> for (HexCoord, HexCoord) {
    fn from(pos: HexPos) -> Self {
        (pos.0, pos.1)
    }
}

impl HexPos {
    pub fn new(u: HexCoord, v: HexCoord) -> Self {
        assert!((u + v) % 2 == 0, "u + v must be even");
        HexPos(u, v)
    }

    pub fn u(self) -> HexCoord {
        self.0
    }

    pub fn v(self) -> HexCoord {
        self.1
    }

    pub fn shift(self, du: HexCoord, dv: HexCoord) -> Self {
        let HexPos(u, v) = self;
        HexPos::new(u + du, v + dv)
    }

    pub fn in_range(self, width: HexCoord, height: HexCoord) -> bool {
        let HexPos(u, v) = self;
        u >= 0 && u < width && v >= 0 && v < height
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

    pub fn center_pos(self) -> Cartesian {
        let HexPos(u, v) = self;
        let y_scale = (3.0_f32).sqrt() / 2.0;
        let x_scale = 1.5;
        (u as f32 * x_scale, v as f32 * y_scale)
    }

    pub fn corners_pos(self) -> [Cartesian; 6] {
        let (cx, cy) = self.center_pos();
        let mut corners = [(0.0, 0.0); 6];
        for (i, corner) in corners.iter_mut().enumerate() {
            let angle = (i as f32) * std::f32::consts::FRAC_PI_3;
            *corner = (cx + angle.cos(), cy + angle.sin());
        }
        corners
    }

    pub fn corner_pos(self, corner: HexCorner) -> Cartesian {
        let (cx, cy) = self.center_pos();
        let angle = corner.to_angle();
        (cx + angle.cos(), cy + angle.sin())
    }

    /// Gets the neighboring hex in the given direction.
    pub fn neighbor(self, edge: HexEdge) -> Self {
        let HexPos(u, v) = self;
        match edge {
            HexEdge::TopRight => HexPos(u + 1, v + 1),
            HexEdge::Top => HexPos(u, v + 2),
            HexEdge::TopLeft => HexPos(u - 1, v + 1),
            HexEdge::BottomLeft => HexPos(u - 1, v - 1),
            HexEdge::Bottom => HexPos(u, v - 2),
            HexEdge::BottomRight => HexPos(u + 1, v - 1),
        }
    }

    /// Returns true iff the given position is a neighbor of this position.
    pub fn is_neighbor(self, other: HexPos) -> bool {
        self.neighbor_edge(other).is_some()
    }

    /// Gets the edge of this position that is shared by a neighboring position,
    /// if any.  Returns None if the other position is not a neighbor.
    pub fn neighbor_edge(self, other: Self) -> Option<HexEdge> {
        let HexPos(u1, v1) = self;
        let HexPos(u2, v2) = other;
        let du = u2 - u1;
        let dv = v2 - v1;
        match (du, dv) {
            (1, 1) => Some(HexEdge::TopRight),
            (0, 2) => Some(HexEdge::Top),
            (-1, 1) => Some(HexEdge::TopLeft),
            (-1, -1) => Some(HexEdge::BottomLeft),
            (0, -2) => Some(HexEdge::Bottom),
            (1, -1) => Some(HexEdge::BottomRight),
            _ => None,
        }
    }

    pub fn neighbors_at_corner(self, corner: HexCorner) -> [(Self, HexCorner); 2] {
        // TODO: Do this without iterating over all edges.
        let mut result = [(self, corner); 2];
        let mut num_found = 0;
        for edge in HexEdge::ALL {
            if edge.ends()[0] == corner {
                let neighbor = self.neighbor(edge);
                let neighbor_corner = edge.opposite().ends()[0];
                result[num_found] = (neighbor, neighbor_corner);
                num_found += 1;
            }
        }
        debug_assert_eq!(num_found, 2, "Corner {:?} should have 2 neighbors", corner);
        result
    }

    pub fn nearest_edge(self, point: Cartesian) -> NearestEdge {
        let (cx, cy) = self.center_pos();
        let (px, py) = point;
        let dx = point.0 - cx;
        let dy = point.1 - cy;
        let angle = dy.atan2(dx);
        let steps = ((angle + FRAC_PI_6) / FRAC_PI_3).round() as i32;
        let edge = HexEdge::TopRight.rotate(steps - 1);
        let corner = edge.ends()[0];
        let (cpx, cpy) = self.corner_pos(corner);
        let slope = match edge {
            HexEdge::TopRight | HexEdge::BottomLeft => -FRAC_PI_3.tan(),
            HexEdge::Top | HexEdge::Bottom => 0.0,
            HexEdge::TopLeft | HexEdge::BottomRight => FRAC_PI_3.tan(),
        };
        let distance = (slope * (px - cpx) - (py - cpy)).abs() / (slope * slope + 1.0).sqrt();
        NearestEdge { edge, distance }
    }

    /// Gets the nearest corner of this hex to the given point, and the distance to that corner.
    pub fn nearest_corner(self, point: Cartesian) -> NearestCorner {
        let (cx, cy) = self.center_pos();
        let (px, py) = point;
        let dx = px - cx;
        let dy = py - cy;
        let angle = dy.atan2(dx);
        let steps = ((angle + FRAC_PI_6 / 2.0) / FRAC_PI_3).round() as i32;
        let corner = HexCorner::Right.rotate(steps);
        let (cpx, cpy) = self.corner_pos(corner);
        let distance = (cpx - px).hypot(cpy - py);
        NearestCorner {
            corner,
            distance,
            point: (cpx, cpy),
        }
    }

    pub fn norm_edge(self, edge: HexEdge) -> (HexPos, HexEdge) {
        match edge {
            HexEdge::TopRight | HexEdge::Top | HexEdge::TopLeft => (self, edge),
            _ => {
                let neighbor = self.neighbor(edge);
                (neighbor, edge.opposite())
            }
        }
    }

    pub fn norm_corner(self, corner: HexCorner) -> (HexPos, HexCorner) {
        match corner {
            HexCorner::Right | HexCorner::TopRight | HexCorner::TopLeft => (self, corner),
            _ => self.neighbors_at_corner(corner)[0],
        }
    }
}

pub struct HexPosIterator {
    u: HexCoord,
    v: HexCoord,
    width: HexCoord,
    height: HexCoord,
}

impl HexPosIterator {
    pub fn new(width: HexCoord, height: HexCoord) -> Self {
        Self {
            u: 0,
            v: 0,
            width,
            height,
        }
    }
}

impl Iterator for HexPosIterator {
    type Item = HexPos;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;
        while result.is_none() && self.v < self.height && self.u < self.width {
            if (self.u + self.v) % 2 != 0 {
                self.u += 1;
            }
            if self.u < self.width {
                result = Some(HexPos::new(self.u, self.v));
            }
            self.u += 1;
            if self.u >= self.width {
                self.u = 0;
                self.v += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn hex_pos_iterator() {
        for width in 0..9 {
            for height in 0..9 {
                let expected = (0..height)
                    .flat_map(|v| {
                        (0..width)
                            .filter(move |&u| (u + v) % 2 == 0)
                            .map(move |u| HexPos::new(u, v))
                    })
                    .collect::<HashSet<_>>();
                let actual = HexPosIterator::new(width, height).collect::<HashSet<_>>();
                assert_eq!(expected, actual);
            }
        }
    }
}
