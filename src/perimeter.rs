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

    pub fn new_from(starting_hex: HexPos, container: &'c C) -> Self {
        let starting_edge = HexEdge::ALL
            .into_iter()
            .find(|&edge| !container.contains_hex(starting_hex.neighbor(edge)))
            .expect("Starting hex must have at least one exterior edge");
        Self {
            container,
            hex_iter: None,
            edges_seen: [(starting_hex, starting_edge)].into_iter().collect(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{grid::HexGrid, grid_size::HexGridSize};
    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;

    // Reference implementation.
    fn perimeter_edges<C: HexPosContainer>(container: &C) -> Vec<(HexPos, HexEdge)> {
        let is_exterior_edge =
            |pos: HexPos, edge: HexEdge| !container.contains_hex(pos.neighbor(edge));

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

    #[derive(Debug, Clone)]
    struct TestHexPosContainer {
        hexes: HashSet<HexPos>,
        size: HexGridSize,
    }

    impl Arbitrary for TestHexPosContainer {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let size = HexGridSize::arbitrary(g);
            let mut hexes = HashSet::new();
            for pos in HexGrid::<()>::new_with_defaults(size).iter_hexes() {
                if bool::arbitrary(g) {
                    hexes.insert(pos);
                }
            }
            TestHexPosContainer { hexes, size }
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let vec = self.hexes.clone();
            Box::new(self.size.shrink().map(move |size| {
                TestHexPosContainer {
                    hexes: vec
                        .iter()
                        .filter(|&&pos| size.contains_hex(pos))
                        .copied()
                        .collect(),
                    size,
                }
            }))
        }
    }

    #[derive(Debug, Clone)]
    struct ContiguousHexPosContainer {
        hexes: Vec<HexPos>,
    }

    impl ContiguousHexPosContainer {
        fn fill(&mut self, g: &mut quickcheck::Gen, size: HexGridSize) {
            let last = *self.hexes.last().unwrap();
            let neighbors = HexEdge::ALL
                .into_iter()
                .map(|edge| last.neighbor(edge))
                .filter(|pos| size.contains_hex(*pos))
                .filter(|pos| !self.hexes.contains(pos))
                .collect::<Vec<_>>();
            if neighbors.is_empty() {
                return;
            }
            if let Some(next) = g.choose(&neighbors) {
                self.hexes.push(*next);
            }
        }
    }

    impl Arbitrary for ContiguousHexPosContainer {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut result = ContiguousHexPosContainer { hexes: Vec::new() };
            let size = HexGridSize::arbitrary(g);
            result.hexes.push(HexPos::new(0, 0));
            result.fill(g, size);
            result
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let mut items = Vec::new();
            let mut hexes = self.hexes.clone();
            while hexes.len() > 2 {
                hexes.pop();
                items.push(ContiguousHexPosContainer {
                    hexes: hexes.clone(),
                });
            }
            Box::new(items.into_iter())
        }
    }

    #[quickcheck]
    fn perimeter(arb: TestHexPosContainer) {
        let expected_perimeter_edges = perimeter_edges(&arb.hexes);
        let actual_perimeter_edges = HexPerimeterIterator::new(&arb.hexes).collect::<Vec<_>>();
        assert_eq!(expected_perimeter_edges, actual_perimeter_edges);
    }

    #[quickcheck]
    fn perimeter_from(arb: ContiguousHexPosContainer) {
        let expected_perimeter_edges = perimeter_edges(&arb.hexes.as_slice());
        let actual_perimeter_edges =
            HexPerimeterIterator::new_from(HexPos::new(0, 0), &arb.hexes.as_slice())
                .collect::<Vec<_>>();
        assert_eq!(expected_perimeter_edges, actual_perimeter_edges);
    }
}
