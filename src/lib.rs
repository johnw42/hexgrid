//! A crate for working with rectangular grids tiled with hexagons, such as
//! might commonly be used to represent the map in a strategy game like
//! Civilization or Settlers of Catan.
//!
//! ## Notational Conventions
//!
//! The names and documentation in this crate follow a few notational
//! conventions that are worth explaining up front.  Everything follows math
//! conventions, where the positive u/x direction is to the right, the positive
//! v/y direction is upward, and angles are measured counter-clockwise from the
//! positive u/x axis.  Hexagons are flat-topped, with the top and bottom edges
//! horizontal, and the left and right corners pointing directly left and right.
//! The six corners of a hexagon are named counter-clockwise starting from the
//! right corner, and the six edges of a hexagon are named counter-clockwise
//! starting from the bottom-right edge.  The naming convention is consistent
//! with the [`HexCorner`] and [`HexEdge`] enums.
//!
//! ### Coordinate Systems
//!
//! Coordinates described as "Cartesian" are in the usual (x, y) coordinate
//! system with real-valued coordinates.  These are useful to translating pixel
//! coordinates into hexagonal grid coordinates, and vice versa.  In the
//! Cartesian coordinate system, the width of a hexagon is `HEX_WIDTH`, which is
//! defined as 1.0, and the height of a hexagon is `HEX_HEIGHT`, which is
//! defined as `sqrt(3)/2` units.  The distance between two vertically adjacent
//! hexagons is `HEX_VERTICAL_SPACING`, which is equal to `HEX_HEIGHT`, and the
//! distance between two horizontally adjacent columns of hexagons is
//! `HEX_HORIZONTAL_SPACING`, which is equal to `HEX_WIDTH * 1.5`.
//!
//! To avoid confusion, the names x and y are reserved for Cartesian
//! coordinates, and the names u and v are used for hexagonal grid coordinates.
//!
//! All non-Cartesian coordinate systems use a hexagonal grid with
//! integer-valued coordinates usually subject to some constraints.
//!
//! The primary coordinate system used in this crate is defined by the
//! [`HexPos`] type, which represents the position of a hexagon in a rectangular
//! grid of hexagons, and [`HexDelta`], which represents a difference of
//! positions.  The u/du coordinate increases to the right, and the v/dv
//! coordinate increases upward.  A full set of arithmetic operations is
//! provided by these types.
//!
//! For compatibility with other hexagonal grid libraries, the [`CubicPos`] type
//! is also provided, which represents the position of a hexagon in a cubic
//! coordinate system, where each hexagon has coordinates (q, r, s) such that q
//! \+ r + s = 0.  Arithmetic on this type is convenient but not provided by this
//! crate, on the assumption that cubic coordinates will only be used on
//! conjunction with other crates that provide their own cubic or axial
//! coordinate arithmetic.
//!
//! Lastly, offset coordinates are provided through the [`OffsetPos`] type,
//! which represents the position of a hexagon in an offset coordinate system,
//! where the rows are staggered.  Arithmetic on this type is not provided
//! because offset coordinates are not suitable for performing arithmetic
//! operations.
//!
//! ## Important Types
//!
//! The core type of this trait is [`HexGrid`], which represents a rectangular
//! grid of hexagons along with their edges and corners.
//!
//! Corners and edges are represented by [`HexCorner`] and [`HexEdge`],
//! respectively, which are enums with six variants corresponding to the six
//! corners and edges of a hexagon.  The are combined with a `HexPos` to form
//! [`HexCornerPos`] and [`HexEdgePos`], which represent the position of a
//! corner or edge in the hexagonal grid.
//!
//! Some special iterators are provided for particular traversals of the
//! hexagonal grid:
//!
//! * [`RingIterator`] iterates over the positions of hexagons in a ring around
//!   a given center hexagon, at a given radius.
//! * [`DiskIterator`] iterates over the positions of hexagons in a disk around
//!   a given center hexagon, starting from the center and expanding outward.
//! * [`PerimeterIterator`] iterates the boundary of a group of hexagons.
//! * [`LineIterator`] iterates the straightest possible line between two
//!   hexagons.
//!
//! Various methods are provided for converting between hexagonal grid
//! coordinates and Cartesian coordinates and angles, following math
//! conventions, the positive u/x direction is to the right, the positive v/y
//! direction is upward, and angles are measured counter-clockwise from the
//! positive u/x axis.  The most noteworthy methods are
//!
//! * [`HexPos::nearest_from_cartesian`], for finding the hexagonal grid
//!   position of a hexagon given its Cartesian coordinates.
//! * [`HexPos::cartesian_corner`], for finding the Cartesian coordinates of a
//!   the corners of a hexagon.
//! * [`HexRectangle::cartesian`], for iterating over all hexagonal grid
//!   positions that intersect a rectangular region defined by minimum and
//!   maximum Cartesian coordinates.

pub use crate::{
    container::HexPosContainer,
    corner::{HexCorner, NormHexCorner},
    corner_pos::HexCornerPos,
    cubic::CubicPos,
    delta::HexDelta,
    edge::{HexEdge, NormHexEdge},
    edge_pos::HexEdgePos,
    grid::HexGrid,
    grid_size::{HexGridSize, HexGridSizeError},
    line::LineIterator,
    offset::OffsetPos,
    perimeter::PerimeterIterator,
    pos::{HexPos, NearestCorner, NearestEdge},
    rectangle::{HexRectangle, HexRectangleIterator},
    ring::{DiskIterator, RingIterator},
};

mod container;
mod corner;
mod corner_pos;
mod cubic;
mod delta;
mod edge;
mod edge_pos;
mod grid;
mod grid_size;
mod line;
mod offset;
mod perimeter;
mod pos;
mod rectangle;
mod ring;

#[cfg(test)]
mod setdiff;

/// Integer type used for hex grid coordinates, and other related purposes.
pub type HexCoord = i32;

/// Cartesian distance type used for hex grid calculations and conversions.  Use implies that the
/// width of a hexagon is 1.0 unit, and the height of a hexagon is sqrt(3)/2 units.
pub type Real = f64;

/// Type of an angle measured in sixths of a full circle, assuming the positive
/// direction is counter-clockwise.
pub type Sixths = i32;

/// A point in Cartesian coordinates, represented as a pair (x, y) of `Distance` values.
pub type Cartesian = (Real, Real);

/// Type of an angle measured in radians, assuming the positive direction is
/// counter-clockwise from the positive u-axis.
pub type Radians = Real;

/// The width of a hexagon in Cartesian coordinates, which is 1.0 unit.
pub const HEX_WIDTH: Real = 1.0;

/// The height of a hexagon in Cartesian coordinates.  Equal to sqrt(3)/2 units.
pub const HEX_HEIGHT: Real = SQRT_3 / 2.0;

/// The Cartesian distance between the centers of two vertically adjacent hexagons in a hexagonal grid.
pub const HEX_VERTICAL_SPACING: Real = HEX_HEIGHT;

/// The Cartesian distance between the centers of two horizontally adjacent columns of hexagons in a hexagonal grid.
pub const HEX_HORIZONTAL_SPACING: Real = HEX_WIDTH * 1.5;

/// The sqrt(3), used in hexagonal grid calculations.  Needed because f32::sqrt is not a const function.
#[allow(clippy::excessive_precision)]
const SQRT_3: Real = 1.7320508075688772;
