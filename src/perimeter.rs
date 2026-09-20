use crate::{
    edge::HexEdge,
    pos::{HexPos, HexPosContainer},
};
use std::collections::HashSet;

pub struct HexPerimeterIterator<'c, C: HexPosContainer> {
    container: &'c C,
    hex_iter: Option<C::Iterator>,
    edges_seen: HashSet<(HexPos, HexEdge)>,
    #[cfg(debug_assertions)]
    items_produced: usize,
    state: HexPerimeterIteratorState,
}

enum HexPerimeterIteratorState {
    FindStartingHex,
    YieldOne {
        starting_hex: HexPos,
        starting_edge: HexEdge,
    },
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
            hex_iter: Some(container.iter_hexes()),
            edges_seen: HashSet::new(),
            state: HexPerimeterIteratorState::FindStartingHex,
            #[cfg(debug_assertions)]
            items_produced: 0,
        }
    }

    pub fn new_from(starting_hex: HexPos, starting_edge: HexEdge, container: &'c C) -> Self {
        Self {
            container,
            hex_iter: None,
            edges_seen: HashSet::new(),
            state: HexPerimeterIteratorState::YieldOne {
                starting_hex,
                starting_edge,
            },
            #[cfg(debug_assertions)]
            items_produced: 0,
        }
    }

    fn increment_items_produced(&mut self) {
        #[cfg(debug_assertions)]
        {
            self.items_produced += 1;
            debug_assert!(
                self.items_produced <= 6 * self.container.iter_hexes().count(),
                "Too many edges in perimeter, possible infinite loop"
            );
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
                let starting_hex = self.hex_iter.as_mut()?.next()?;
                let exterior_edge = HexEdge::ALL
                    .into_iter()
                    .find(|&edge| self.is_exterior_edge(starting_hex, edge));
                if let Some(starting_edge) = exterior_edge
                    && self.edges_seen.insert((starting_hex, starting_edge))
                {
                    self.state = HexPerimeterIteratorState::YieldOne {
                        starting_hex,
                        starting_edge,
                    };
                    return self.next();
                }
            },
            HexPerimeterIteratorState::YieldOne {
                starting_hex,
                starting_edge,
            } => {
                self.state = HexPerimeterIteratorState::TraverseBoundary {
                    starting_hex,
                    starting_edge,
                    current_hex: starting_hex,
                    current_edge: starting_edge,
                };
                self.increment_items_produced();
                Some((starting_hex, starting_edge))
            }
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
                    self.increment_items_produced();
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
            debug_assert!(
                result.len() <= 6 * container.iter_hexes().count(),
                "Too many edges in perimeter, possible infinite loop"
            );
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
    use crate::{grid::HexGrid, grid_size::HexGridSize};
    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;

    #[derive(Debug, Clone)]
    struct TestHexPosContainer {
        set: HashSet<HexPos>,
        size: HexGridSize,
    }

    impl Arbitrary for TestHexPosContainer {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let size = HexGridSize::arbitrary(g);
            let mut set = HashSet::new();
            for pos in HexGrid::<()>::new_with_defaults(size).iter_hexes() {
                if bool::arbitrary(g) {
                    set.insert(pos);
                }
            }
            TestHexPosContainer { set, size }
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let vec = self.set.clone();
            Box::new(self.size.shrink().map(move |size| {
                TestHexPosContainer {
                    set: vec
                        .iter()
                        .filter(|&&pos| size.contains_hex(pos))
                        .copied()
                        .collect(),
                    size,
                }
            }))
        }
    }

    #[quickcheck]
    fn perimeter(arb: TestHexPosContainer) {
        let expected_perimeter_edges = perimeter_edges(&arb.set);
        let actual_perimeter_edges = HexPerimeterIterator::new(&arb.set).collect::<Vec<_>>();
        assert_eq!(expected_perimeter_edges, actual_perimeter_edges);
    }
}
