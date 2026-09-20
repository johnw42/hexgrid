use crate::{
    HexCoord,
    grid::HexGridSize,
    pos::{HexPos, HexPosIterator},
};

#[derive(PartialEq, Eq, Debug)]
enum PerimeterTraveral {
    Right,
    Up,
    Left,
    Down,
    Done,
}

pub struct HexGridPerimeterIterator(HexGridPerimeterIteratorData);

enum HexGridPerimeterIteratorData {
    Small(HexPosIterator),
    Large {
        u: HexCoord,
        v: HexCoord,
        width: HexCoord,
        height: HexCoord,
        traveral: PerimeterTraveral,
    },
}

impl HexGridPerimeterIterator {
    pub fn new(size: HexGridSize) -> Self {
        if size.width() < 3 || size.height() < 4 {
            Self(HexGridPerimeterIteratorData::Small(HexPosIterator::new(
                size,
            )))
        } else {
            Self(HexGridPerimeterIteratorData::Large {
                u: 0,
                v: 0,
                width: size.width(),
                height: size.height(),
                traveral: PerimeterTraveral::Right,
            })
        }
    }
}

impl Iterator for HexGridPerimeterIterator {
    type Item = HexPos;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            HexGridPerimeterIteratorData::Small(iter) => iter.next(),
            HexGridPerimeterIteratorData::Large {
                u,
                v,
                width,
                height,
                traveral,
            } => {
                let pos = HexPos::new(*u, *v);
                if *traveral == PerimeterTraveral::Right {
                    if *v == 0 {
                        *u += 1;
                        *v = 1;
                    } else {
                        debug_assert_eq!(*v, 1);
                        *u += 1;
                        *v = 0;
                    }
                    if *u >= *width {
                        *traveral = PerimeterTraveral::Up;
                        (*u, *v) = pos.into();
                    }
                }
                if *traveral == PerimeterTraveral::Up {
                    *v += 2;
                    if *v >= *height {
                        *traveral = PerimeterTraveral::Left;
                        (*u, *v) = pos.into();
                    }
                }
                if *traveral == PerimeterTraveral::Left {
                    if *v >= *height {
                        *u -= 1;
                        *v -= 1;
                    } else {
                        *u -= 1;
                        *v += 1;
                    }
                    if *u < 0 {
                        *traveral = PerimeterTraveral::Down;
                        (*u, *v) = pos.into();
                    }
                }
                if *traveral == PerimeterTraveral::Down {
                    *v -= 2;
                    if *u == 0 && *v == 0 {
                        *traveral = PerimeterTraveral::Done;
                    }
                }

                if *traveral == PerimeterTraveral::Done {
                    None
                } else {
                    Some(pos)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{grid::HexGrid, iter_valid_sizes};
    use std::collections::HashSet;

    #[test]
    fn perimeter() {
        for size in iter_valid_sizes() {
            let grid = HexGrid::<i32>::new_with_defaults(size);
            // TODO
            // grid.perimeter()
            //     .for_each(|pos: HexPos| assert!(grid.has_hex(pos), "pos: {:?}", pos));
            assert_eq!(
                grid.perimeter().count(),
                grid.perimeter().collect::<HashSet<_>>().len(),
            );
        }
    }
}
