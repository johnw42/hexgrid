use crate::HexPos;

/// An iterator that yields the positions of hexagons along an approximate
/// straight line between two hexagonal grid positions.
pub struct HexLineIterator {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    step: usize,
    max_step: usize,
}

/// Creates a new iterator that will yield the positions of hexagons along an
/// approximate straight line between the given `start` and `end` hexagonal grid
/// positions.  The iterator will yield the `start` position first, and the
/// exact path is not guaranteed, but it will always be
/// a shortest path between the two positions.
impl HexLineIterator {
    pub fn new(start: HexPos, end: HexPos) -> Self {
        let (x, y) = start.cartesian_center();
        let (x_end, y_end) = end.cartesian_center();
        let steps = start.steps_to(end) as f32;
        let dx = (x_end - x) / steps;
        let dy = (y_end - y) / steps;
        Self {
            x,
            y,
            dx,
            dy,
            step: 0,
            max_step: steps as usize,
        }
    }
}

impl Iterator for HexLineIterator {
    type Item = HexPos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step > self.max_step {
            return None;
        }

        let result = Some(HexPos::nearest_from_cartesian((self.x, self.y)));

        self.x += self.dx;
        self.y += self.dy;
        self.step += 1;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn line_iterator_terminates(start: HexPos, end: HexPos) {
        dbg!(&start, &end);
        let iter = HexLineIterator::new(start, end);
        let (du, dv) = (start - end).du_dv();
        let to_take = (2 * du.abs() + dv.abs() + 1) as usize;
        let count = iter.take(to_take).count();
        assert!(
            count < to_take,
            "Line iterator from {start} to {end} produced too many hexes: {count} > {to_take}",
        );
    }
}
