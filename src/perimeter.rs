use crate::{
    edge::{self, HexEdge, HexPosWithEdge},
    pos::{HexPos, HexPosContainer},
};
use std::{collections::HashSet, hash::Hash};

pub struct HexPerimeterIterator<'c, C: HexPosContainer> {
    container: &'c C,
    hex_iter: C::Iterator,
    edges_seen: HashSet<(HexPos, HexEdge)>,
    state: HexPerimeterIteratorState,
}

enum HexPerimeterIteratorState {
    FindStartingHex,
    TraverseBoundary(HexPos, HexEdge),
}

impl<'c, C> HexPerimeterIterator<'c, C>
where
    C: HexPosContainer,
{
    pub fn new(container: &'c C) -> Self {
        Self {
            container,
            hex_iter: container.iter_hexes(),
            edges_seen: HashSet::new(),
            state: HexPerimeterIteratorState::FindStartingHex,
        }
    }

    fn is_exterior_edge(&self, pos: HexPos, edge: HexEdge) -> bool {
        !self.container.contains_hex(pos.neighbor(edge))
    }
}

impl<'c, C> Iterator for HexPerimeterIterator<'c, C>
where
    C: HexPosContainer,
{
    type Item = (HexPos, HexEdge);

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            HexPerimeterIteratorState::FindStartingHex => loop {
                let starting_hex = self.hex_iter.next()?;
                let exterior_edge = HexEdge::ALL
                    .into_iter()
                    .find(|&edge| self.is_exterior_edge(starting_hex, edge));
                if let Some(starting_edge) = exterior_edge
                    && self.edges_seen.insert((starting_hex, starting_edge))
                {
                    self.state =
                        HexPerimeterIteratorState::TraverseBoundary(starting_hex, starting_edge);
                    return Some((starting_hex, starting_edge));
                }
            },
            HexPerimeterIteratorState::TraverseBoundary(mut hex, mut edge) => {
                edge = edge.rotate(1);
                if !self.is_exterior_edge(hex, edge) {
                    hex = hex.neighbor(edge);
                    edge = edge.rotate(-2);
                }
                if self.edges_seen.insert((hex, edge)) {
                    Some((hex, edge))
                } else {
                    self.state = HexPerimeterIteratorState::FindStartingHex;
                    self.next()
                }
            }
        }
    }
}

pub fn perimeter_edges<C: HexPosContainer>(container: &C) -> Vec<(HexPos, HexEdge)> {
    let is_exterior_edge = |pos: HexPos, edge: HexEdge| !container.contains_hex(pos.neighbor(edge));

    let mut hex_iter = container.iter_hexes();
    let mut edges_seen = HashSet::new();
    let mut result = Vec::new();
    'regions: loop {
        // Find an arbitrary starting hex on the perimeter.
        let (starting_hex, starting_edge) = 'find_boundary: loop {
            let Some(starting_hex) = hex_iter.next() else {
                break 'regions;
            };
            let exterior_edge = HexEdge::ALL
                .into_iter()
                .find(|&edge| !container.contains_hex(starting_hex.neighbor(edge)));
            if let Some(starting_edge) = exterior_edge
                && edges_seen.insert((starting_hex, starting_edge))
            {
                break 'find_boundary (starting_hex, starting_edge);
            }
        };

        let mut edge = starting_edge;
        let mut hex = starting_hex;
        'traverse_boundary: loop {
            result.push((hex, edge));
            edge = edge.rotate(1);
            if !is_exterior_edge(hex, edge) {
                hex = hex.neighbor(edge);
                edge = edge.rotate(-2);
            }
            if hex == starting_hex && edge == starting_edge {
                break 'traverse_boundary;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::iter_valid_sizes;
    // use crate::{grid::HexGrid, iter_valid_sizes};
    // use std::collections::HashSet;

    #[test]
    fn perimeter() {
        for size in iter_valid_sizes() {
            let expected_perimeter_edges = perimeter_edges(&size);
            let actual_perimeter_edges = HexPerimeterIterator::new(&size).collect::<Vec<_>>();
            assert_eq!(expected_perimeter_edges, actual_perimeter_edges);
        }
    }
}
