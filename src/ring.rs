use crate::{HexCoord, HexEdge, HexPos};

/// An iterator that yields the positions of hexagons in a ring around a given
/// center hexagon, at a given radius.
#[derive(Debug, Clone)]
pub struct HexRingIterator {
    pos: HexPos,
    start: HexPos,
    direction: HexEdge,
    steps_remaining: HexCoord,
    done: bool,
    radius: HexCoord,
}

impl HexRingIterator {
    /// Creates a new iterator that will yield the positions of hexagons in a
    /// ring around the given `center` hexagon, at the given `radius`.  For the
    /// special case of radius 0, the iterator yields only the center hexagon.
    pub fn new(center: HexPos, radius: HexCoord) -> Self {
        assert!(radius >= 0, "radius must be non-negative");
        let mut pos = center;
        if radius == 0 {
            Self {
                pos: center,
                start: center.neighbor(HexEdge::TopRight),
                direction: HexEdge::TopRight,
                steps_remaining: 1,
                done: false,
                radius: 0,
            }
        } else {
            for _ in 0..radius {
                pos = pos.neighbor(HexEdge::TopRight);
            }
            Self {
                pos,
                start: pos,
                direction: HexEdge::TopLeft,
                steps_remaining: radius,
                done: false,
                radius,
            }
        }
    }
}

impl Iterator for HexRingIterator {
    type Item = HexPos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let result = Some(self.pos);
        if self.steps_remaining == 0 {
            self.direction = self.direction.rotate(1);
            self.steps_remaining = self.radius;
        }
        self.steps_remaining -= 1;
        self.pos = self.pos.neighbor(self.direction);
        if self.pos == self.start {
            self.done = true;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn ring_iterator0(center: HexPos) {
        let mut iter = HexRingIterator::new(center, 0);
        assert_eq!(iter.next(), Some(center));
        assert_eq!(iter.next(), None);
    }

    #[quickcheck]
    fn ring_iterator1(center: HexPos) {
        let ring_positions = HexRingIterator::new(center, 1).take(7).collect::<Vec<_>>();
        let expected_positions = HexEdge::ALL
            .into_iter()
            .map(|edge| center.neighbor(edge))
            .collect::<Vec<_>>();
        assert_eq!(ring_positions, expected_positions);
    }

    #[quickcheck]
    fn ring_iterator2(center: HexPos) {
        let (u, v) = center.u_v();
        let radius = 2;
        let ring_positions = HexRingIterator::new(center, radius)
            .take((6 * radius + 1) as usize)
            .collect::<Vec<_>>();
        let expected_positions = vec![
            HexPos::new(u + 2, v + 2),
            HexPos::new(u + 1, v + 3),
            HexPos::new(u, v + 4),
            HexPos::new(u - 1, v + 3),
            HexPos::new(u - 2, v + 2),
            HexPos::new(u - 2, v),
            HexPos::new(u - 2, v - 2),
            HexPos::new(u - 1, v - 3),
            HexPos::new(u, v - 4),
            HexPos::new(u + 1, v - 3),
            HexPos::new(u + 2, v - 2),
            HexPos::new(u + 2, v),
        ];
        assert_eq!(ring_positions, expected_positions);
    }

    #[quickcheck]
    fn ring_iterator_size(center: HexPos, radius: u8) {
        let radius = radius as HexCoord;
        let ring_positions = HexRingIterator::new(center, radius).collect::<Vec<_>>();
        let expected_size = if radius == 0 { 1 } else { 6 * radius };
        assert_eq!(ring_positions.len(), expected_size as usize);
    }
}
