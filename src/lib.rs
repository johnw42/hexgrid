#[cfg(test)]
use crate::grid_size::HexGridSize;

pub mod corner;
pub mod edge;
pub mod grid;
pub mod grid_size;
pub mod perimeter;
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

#[cfg(test)]
fn iter_valid_sizes() -> impl Iterator<Item = HexGridSize> {
    (0..=9).flat_map(|width| (0..=9).filter_map(move |height| HexGridSize::new(width, height).ok()))
}
