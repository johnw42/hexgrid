use crate::{
    HexCoord,
    pos::{HexPos, HexPosContainer, HexPosIterator},
};
#[cfg(test)]
use quickcheck::Arbitrary;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexGridSize {
    width: HexCoord,
    height: HexCoord,
}

impl HexGridSize {
    pub const fn new(width: HexCoord, height: HexCoord) -> Result<Self, &'static str> {
        if width < 0 || height < 0 {
            return Err("Width and height must be non-negative");
        }
        if height == 0 && width != 0 {
            return Err("Width must be 0 if height is 0");
        }
        if width == 0 && height != 0 {
            return Err("Height must be 0 if width is 0");
        }
        if width > 1 && height <= 1 {
            return Err("Height must be greater than 1 if width is greater than 1");
        }
        Ok(Self { width, height })
    }

    pub const fn width(&self) -> HexCoord {
        self.width
    }

    pub const fn height(&self) -> HexCoord {
        self.height
    }
}

impl From<HexGridSize> for (HexCoord, HexCoord) {
    fn from(size: HexGridSize) -> Self {
        (size.width, size.height)
    }
}

#[cfg(test)]
impl Arbitrary for HexGridSize {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        loop {
            let (width, height) = if bool::arbitrary(g) {
                // Make sure small sizes are checked more often, since they are
                // more likely to have edge cases.
                (HexCoord::arbitrary(g) % 3, HexCoord::arbitrary(g) % 3)
            } else {
                (HexCoord::arbitrary(g) % 64, HexCoord::arbitrary(g) % 64)
            };
            if let Ok(size) = HexGridSize::new(width, height) {
                return size;
            }
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let (width, height) = (self.width, self.height);
        Box::new(
            (0..=width)
                .flat_map(move |w| (0..=height).filter_map(move |h| HexGridSize::new(w, h).ok())),
        )
    }
}

impl HexPosContainer for HexGridSize {
    type Iterator = HexPosIterator;

    fn contains_hex(&self, pos: HexPos) -> bool {
        pos.u() >= 0 && pos.u() < self.width && pos.v() >= 0 && pos.v() < self.height
    }

    fn iter_hexes(&self) -> Self::Iterator {
        HexPosIterator::new(*self)
    }
}
