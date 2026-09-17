pub mod corner;
pub mod edge;
pub mod grid;
pub mod pos;

pub type HexCoord = i32;
pub type Distance = f32;
pub type Cartesian = (Distance, Distance);

pub const HEX_WIDTH: Distance = 1.0;
pub const HEX_HEIGHT: Distance = HEX_WIDTH * SQRT_3 / 2.0;
pub const HEX_VERTICAL_SPACING: Distance = HEX_HEIGHT;
pub const HEX_HORIZONTAL_SPACING: Distance = HEX_WIDTH * 1.5;

#[allow(clippy::excessive_precision)]
const SQRT_3: Distance = 1.7320508075688772;

pub fn validate_grid_size(width: HexCoord, height: HexCoord) -> Result<(), &'static str> {
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
    Ok(())
}

#[cfg(test)]
fn iter_valid_sizes() -> impl Iterator<Item = (HexCoord, HexCoord)> {
    (0..=9)
        .flat_map(|width| (0..=9).map(move |height| (width, height)))
        .filter(|&(width, height)| validate_grid_size(width, height).is_ok())
}
