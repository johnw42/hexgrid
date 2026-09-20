use crate::{
    edge::HexEdge,
    pos::{HexPos, HexPosContainer},
};
use std::collections::HashSet;

pub struct HexPerimeterIterator<'c, C: HexPosContainer> {
    container: &'c C,
    hex_iter: C::Iterator,
    edges_seen: HashSet<(HexPos, HexEdge)>,
    state: HexPerimeterIteratorState,
}

enum HexPerimeterIteratorState {
    FindStartingHex,
    TraverseBoundary {
        starting_hex: HexPos,
        starting_edge: HexEdge,
        current_hex: HexPos,
        current_edge: HexEdge,
    },
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
                    self.state = HexPerimeterIteratorState::TraverseBoundary {
                        starting_hex,
                        starting_edge,
                        current_hex: starting_hex,
                        current_edge: starting_edge,
                    };
                    return Some((starting_hex, starting_edge));
                }
            },
            HexPerimeterIteratorState::TraverseBoundary {
                starting_hex,
                starting_edge,
                current_hex,
                current_edge,
            } => {
                let mut hex = current_hex;
                let mut edge = current_edge;

                edge = edge.rotate(1);
                if !self.is_exterior_edge(hex, edge) {
                    hex = hex.neighbor(edge);
                    edge = edge.rotate(-2);
                }

                if hex == starting_hex && edge == starting_edge {
                    // Completed one full perimeter, find next starting point
                    self.state = HexPerimeterIteratorState::FindStartingHex;
                    self.next()
                } else if self.edges_seen.insert((hex, edge)) {
                    // Found new edge, continue traversal
                    self.state = HexPerimeterIteratorState::TraverseBoundary {
                        starting_hex,
                        starting_edge,
                        current_hex: hex,
                        current_edge: edge,
                    };
                    Some((hex, edge))
                } else {
                    // Edge already seen but not back at start - something is wrong
                    // This shouldn't happen if the algorithm is correct
                    self.state = HexPerimeterIteratorState::FindStartingHex;
                    self.next()
                }
            }
        }
    }
}

pub fn perimeter_edges<C: HexPosContainer>(container: &C) -> Vec<(HexPos, HexEdge)> {
    let is_exterior_edge = |pos: HexPos, edge: HexEdge| !container.contains_hex(pos.neighbor(edge));

    let mut edges_seen = HashSet::new();
    let mut result = Vec::new();

    for starting_hex in container.iter_hexes() {
        let exterior_edge = HexEdge::ALL
            .into_iter()
            .find(|&edge| is_exterior_edge(starting_hex, edge));

        let Some(starting_edge) = exterior_edge else {
            continue;
        };
        if !edges_seen.insert((starting_hex, starting_edge)) {
            continue;
        }

        // We have a new starting point on the perimeter
        let mut edge = starting_edge;
        let mut hex = starting_hex;
        loop {
            result.push((hex, edge));
            edges_seen.insert((hex, edge));
            edge = edge.rotate(1);
            if !is_exterior_edge(hex, edge) {
                hex = hex.neighbor(edge);
                edge = edge.rotate(-2);
            }
            if hex == starting_hex && edge == starting_edge {
                break;
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
