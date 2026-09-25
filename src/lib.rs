//! A crate for working with rectangular grids tiled with hexagons, such as
//! might commonly be used to represent the map in a strategy game like
//! Civilization or Settlers of Catan.
//!
//! The core type of this trait is [`HexGrid`], which represents a rectangular
//! grid of hexagons along with their edges and corners.
//!
//! A `HexGrid` is indexable by [`HexPos`], which represents the position of a
//! hexagon using two coordinates, (u, v), where u + v must be even.
//!
//! Differences between two `HexPos` values can be represented as a
//! [`HexDelta`], which represents a vector in the hexagonal grid that can be
//! scaled and rotated.
//!
//! Corners and edges are represented by [`HexCorner`] and [`HexEdge`],
//! respectively, which are enums with six variants corresponding to the six
//! corners and edges of a hexagon.  The are combined with a `HexPos` to form
//! [`HexCornerPos`] and [`HexEdgePos`], which represent the position of a
//! corner or edge in the hexagonal grid.
//!
//! An arbitrary group of [`HexPos`] values can be represented as a
//! [`HexGroup`], allowing groups of positions to be translated and rotated as a
//! unit.
//!
//! Some special iterators are provided for particular traversals of the
//! hexagonal grid:
//!
//! * [`HexRegionIterator`] iterates over all hexagonal grid positions in a
//!   rectangular region defined by minimum and maximum u and v coordinates.
//! * [`HexRingIterator`] iterates over the positions of hexagons in a ring
//!   around a given center hexagon, at a given radius.
//! * [`HexPerimeterIterator`] iterates the boundary of a group of hexagons.
//!
//! Various methods are provided for converting between hexagonal grid
//! coordinates and Cartesian coordinates and angles, following math
//! conventions, the positive u/x direction is to the right, the positive v/y
//! direction is upward, and angles are measured counter-clockwise from the
//! positive u/x axis.  The most noteworthy methods are
//!
//! * [`HexPos::from_center`], for finding the hexagonal grid position of a
//!   hexagon given its Cartesian center coordinates.
//! * [`HexPos::corner_pos`], for finding the Cartesian coordinates of a the
//!   corners of a hexagon.
//! * [`HexRegionIterator::cartesian`], for iterating over all hexagonal grid
//!   positions that intersect a rectangular region defined by minimum and
//!   maximum Cartesian coordinates.

pub use crate::{
    container::HexPosContainer,
    corner::{HexCorner, NormHexCorner},
    corner_pos::HexCornerPos,
    delta::HexDelta,
    edge::{HexEdge, NormHexEdge},
    edge_pos::HexEdgePos,
    grid::HexGrid,
    grid_size::{HexGridSize, HexGridSizeError},
    group::HexGroup,
    perimeter::HexPerimeterIterator,
    pos::{HexPos, NearestCorner, NearestEdge},
    region::{HexRegion, HexRegionIterator},
    ring::HexRingIterator,
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
mod ring;

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
