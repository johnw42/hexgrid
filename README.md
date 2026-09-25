# hexgridrect

A crate for working with rectangular grids tiled with hexagons, such as might
commonly be used to represent the map in a strategy game like Civilization or
Settlers of Catan.

Refer to the crate-level documention for reference, and the included example
code to show how this crate can be used in a real application.

## Compaison to Other Crates

Several existing crates implement similar functionality, the most mature of
which appears to be [`HexGridSpiral`](https://crates.io/crates/hexgridspiral).
The main distinguising features of this crate compare to `HexGridSpiral` are:

- Use of rectagular coordinates
- Use of a rectangular grid
- Support for attaching data to edges and corners of hexagons

<img src="thumbnail.png"/>