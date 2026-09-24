pub use crate::{
    container::HexPosContainer,
    corner::{HexCorner, NormHexCorner},
    corner_pos::HexCornerPos,
    edge::{HexEdge, NormHexEdge},
    edge_pos::HexEdgePos,
    grid::HexGrid,
    grid_size::{HexGridSize, HexGridSizeError},
    group::HexGroup,
    perimeter::HexPerimeterIterator,
    pos::{HexPos, NearestCorner, NearestEdge},
    region::{HexRegion, HexRegionIterator},
};

mod container;
mod corner;
mod corner_pos;
mod delta;
mod edge;
mod edge_pos;
mod grid;
mod grid_size;
mod group;
mod id;
mod perimeter;
mod pos;
mod region;

/// Integer type used for hex grid coordinates, and other related purposes.
pub type HexCoord = i32;

/// Cartesian distance type used for hex grid calculations and conversions.  Use implies that the
/// width of a hexagon is 1.0 unit, and the height of a hexagon is sqrt(3)/2 units.
pub type Distance = f32;

/// Type of an angle measured in sixths of a full circle, assuming the positive
/// direction is counter-clockwise.
pub type Sixths = i32;

/// A point in Cartesian coordinates, represented as a pair (x, y) of `Distance` values.
pub type Cartesian = (Distance, Distance);

/// Type of an angle measured in radians, assuming the positive direction is
/// counter-clockwise from the positive u-axis.
pub type Radians = f32;

/// The width of a hexagon in Cartesian coordinates, which is 1.0 unit.
pub const HEX_WIDTH: Distance = 1.0;

/// The height of a hexagon in Cartesian coordinates.
pub const HEX_HEIGHT: Distance = HEX_WIDTH * SQRT_3 / 2.0;

/// The Cartesian distance between the centers of two vertically adjacent hexagons in a hexagonal grid.
pub const HEX_VERTICAL_SPACING: Distance = HEX_HEIGHT;

/// The Cartesian distance between the centers of two horizontally adjacent columns of hexagons in a hexagonal grid.
pub const HEX_HORIZONTAL_SPACING: Distance = HEX_WIDTH * 1.5;

/// The square root of 3, used in hexagonal grid calculations.
#[allow(clippy::excessive_precision)]
const SQRT_3: Distance = 1.7320508075688772;
