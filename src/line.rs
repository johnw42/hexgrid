use crate::HexPos;

/// An iterator that yields the positions of hexagons along an approximate
/// straight line between two hexagonal grid positions.
pub struct HexLineIterator {
    current: HexPos,
    end: HexPos,
    done: bool,
}

/// Creates a new iterator that will yield the positions of hexagons along an
/// approximate straight line between the given `start` and `end` hexagonal grid
/// positions.  The iterator will yield the `start` position first, and the
/// exact path is not guaranteed, but it will always be
/// a shortest path between the two positions.
impl HexLineIterator {
    pub fn new(start: HexPos, end: HexPos) -> Self {
        Self {
            current: start,
            end,
            done: false,
        }
    }
}

impl Iterator for HexLineIterator {
    type Item = HexPos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result = Some(self.current);

        if self.current == self.end {
            self.done = true;
        } else {
            self.current = self.current.neighbor(self.current.direction_to(self.end));
        }

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
