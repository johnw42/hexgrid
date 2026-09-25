use crate::delta::HexDelta;
use crate::id::HexId;
use crate::{Cartesian, Distance, HexCoord, corner::HexCorner, edge::HexEdge};
use std::f32::consts::{FRAC_PI_3, FRAC_PI_6};
use std::fmt::Display;
use std::ops::{Add, Sub};

/// Value returned by [`HexPos::nearest_corner`].
pub struct NearestCorner {
    /// The nearest corner of the hex to the given point.
    pub corner: HexCorner,
    /// The distance from the given point to the nearest corner of the hex.
    pub distance: Distance,
    /// The Cartesian coordinates of the nearest corner of the hex to the given point.
    pub point: Cartesian,
}

/// Value returned by [`HexPos::nearest_edge`].
pub struct NearestEdge {
    /// The nearest edge of the hex to the given point.
    pub edge: HexEdge,
    /// The distance from the given point to the nearest edge of the hex.
    pub distance: Distance,
}

/// A position in a hexagonal grid, represented by two rectangular coordinates
/// (u, v), where u + v is always even.  The origin (0, 0) is at the bottom left
/// corner of the grid, following typical math conventions.  The u coordinate
/// increases to the right, and the v coordinate increases upwards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexPos(HexCoord, HexCoord);

impl Display for HexPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl HexPos {
    /// Creates a new `HexPos` with the given u and v values.  The sum of u and
    /// v must be even, otherwise this function will panic.
    pub const fn new(u: HexCoord, v: HexCoord) -> Self {
        assert!((u + v) % 2 == 0, "u + v must be even");
        HexPos(u, v)
    }

    /// Returns the u coordinate of the hexagon position.
    pub const fn u(self) -> HexCoord {
        self.0
    }

    /// Returns the v coordinate of the hexagon position.
    pub const fn v(self) -> HexCoord {
        self.1
    }

    /// Returns the u and v coordinates of the hexagon position as a tuple.
    pub const fn u_v(self) -> (HexCoord, HexCoord) {
        (self.0, self.1)
    }

    /// Convertes Cartesian coordinates (x, y) to the nearest hexagonal grid
    /// position.  The origin (0, 0) is at the center of the hexagon at (0, 0),
    /// and the width of the hexagon is 1.0 unit.
    pub fn from_center((x, y): Cartesian) -> Self {
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

    /// Gets the Cartesian coordinates of the center of this hexagon, assuming
    /// the width of the hexagon is 1.0 unit.
    pub fn center_pos(self) -> Cartesian {
        let HexPos(u, v) = self;
        let y_scale = (3.0_f32).sqrt() / 2.0;
        let x_scale = 1.5;
        (u as f32 * x_scale, v as f32 * y_scale)
    }

    /// Returns the Cartesian coordinates of the six corners of this hexagon, in
    /// counter-clockwise order, starting with the right corner.  The width of
    /// the hexagon is assumed to be 1.0 unit.
    pub fn corners_pos(self) -> [Cartesian; 6] {
        let (cx, cy) = self.center_pos();
        let mut corners = [(0.0, 0.0); 6];
        for (i, corner) in corners.iter_mut().enumerate() {
            let angle = (i as f32) * std::f32::consts::FRAC_PI_3;
            *corner = (cx + angle.cos(), cy + angle.sin());
        }
        corners
    }

    /// Returns the Cartesian coordinates of a specific corner of this hexagon,
    /// assuming the width of the hexagon is 1.0 unit.
    pub fn corner_pos(self, corner: HexCorner) -> Cartesian {
        let (cx, cy) = self.center_pos();
        let angle = corner.to_angle();
        (cx + angle.cos(), cy + angle.sin())
    }

    /// Gets the neighboring hex position in the given direction.
    pub fn neighbor(self, edge: impl Into<HexEdge>) -> Self {
        let HexPos(u, v) = self;
        match edge.into() {
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
        // TODO: Rewrite using HexDelta.
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

    /// Gets the two neighboring hex positions that share the given corner of
    /// this position, paired with the corner relative to that neighbor.
    pub fn neighbors_at_corner(self, corner: HexCorner) -> [(Self, HexCorner); 2] {
        let HexPos(u, v) = self;
        match corner {
            HexCorner::Right => [
                (HexPos(u + 1, v - 1), HexCorner::TopLeft),
                (HexPos(u + 1, v + 1), HexCorner::BottomLeft),
            ],
            HexCorner::TopRight => [
                (HexPos(u + 1, v + 1), HexCorner::Left),
                (HexPos(u, v + 2), HexCorner::BottomRight),
            ],
            HexCorner::TopLeft => [
                (HexPos(u, v + 2), HexCorner::BottomLeft),
                (HexPos(u - 1, v + 1), HexCorner::Right),
            ],
            HexCorner::Left => [
                (HexPos(u - 1, v + 1), HexCorner::BottomRight),
                (HexPos(u - 1, v - 1), HexCorner::Right),
            ],
            HexCorner::BottomLeft => [
                (HexPos(u - 1, v - 1), HexCorner::Right),
                (HexPos(u, v - 2), HexCorner::TopLeft),
            ],
            HexCorner::BottomRight => [
                (HexPos(u, v - 2), HexCorner::TopRight),
                (HexPos(u + 1, v - 1), HexCorner::Left),
            ],
        }
    }

    /// Give a point in Cartesian coordinates, returns the nearest corner of
    /// this hexagon to that point, and the distance to that corner.
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

    /// Gets the nearest corner of this hex to the given point, the distance to
    /// that corner, and the Cartesian coordinates of that corner.
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
}

impl From<(HexCoord, HexCoord)> for HexPos {
    fn from((u, v): (HexCoord, HexCoord)) -> Self {
        Self::new(u, v)
    }
}

impl Add<HexDelta> for HexPos {
    type Output = Self;

    fn add(self, other: HexDelta) -> Self {
        HexPos(self.0 + other.du(), self.1 + other.dv())
    }
}

impl Sub<HexDelta> for HexPos {
    type Output = Self;

    fn sub(self, other: HexDelta) -> Self {
        HexPos(self.0 - other.du(), self.1 - other.dv())
    }
}

impl HexId for HexPos {
    fn pos(&self) -> HexPos {
        *self
    }

    fn corners(&self) -> impl Iterator<Item = HexCorner> + '_ {
        std::iter::empty()
    }

    fn edges(&self) -> impl Iterator<Item = HexEdge> + '_ {
        std::iter::empty()
    }

    fn rotate_around(self, center: HexPos, steps: HexCoord) -> Self {
        (self - center).rotated(steps) + center
    }

    fn shift(self, delta: HexDelta) -> Self {
        self + delta
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{container::HexPosContainer, grid_size::HexGridSize};
    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;
    use std::collections::HashSet;

    impl Arbitrary for HexPos {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let u = HexCoord::arbitrary(g) % 64;
            let v = HexCoord::arbitrary(g) % 64;
            let u = if (u + v) % 2 == 0 { u } else { u + 1 };
            HexPos::new(u, v)
        }
    }

    #[quickcheck]
    fn hex_pos_iterator(size: HexGridSize) {
        let expected = (0..size.height())
            .flat_map(|v| {
                (0..size.width())
                    .filter(move |&u| (u + v) % 2 == 0)
                    .map(move |u| HexPos::new(u, v))
            })
            .collect::<HashSet<_>>();
        let actual = size.iter_hexes().collect::<HashSet<_>>();
        assert_eq!(expected, actual);
    }
}
