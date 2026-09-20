use crate::{
    edge::{self, HexEdge, HexPosWithEdge},
    pos::{HexPos, HexPosContainer},
};
use std::{collections::HashSet, hash::Hash};

// pub struct HexPerimeterIterator<'c, C: HexPosContainer> {
//     container: &'c C,
//     hex_iter: C::Iterator,
//     last_pos: Option<HexPos>,
//     next_edge: HexEdge,
//     hexes_seen: HashSet<HexPos>,
// }

// impl<'c, C> HexPerimeterIterator<'c, C>
// where
//     C: HexPosContainer,
// {
//     pub fn new(container: &'c C) -> Self {
//         Self {
//             container,
//             hex_iter: container.iter_hexes(),
//             last_pos: None,
//             next_edge: HexEdge::TopRight, // arbitrary, not used
//             hexes_seen: HashSet::new(),
//         }
//     }

//     fn is_exterior_edge(&self, pos: HexPosWithEdge) -> bool {
//         !self.container.contains_hex(pos.pos().neighbor(pos.edge()))
//     }

//     fn exterior_edge(&self, pos: HexPos) -> Option<HexEdge> {
//         for edge in HexEdge::ALL {
//             let neighbor_pos = pos.neighbor(edge);
//             if !self.container.contains_hex(neighbor_pos) {
//                 return Some(edge);
//             }
//         }
//         None
//     }
// }

// impl<'c, C> Iterator for HexPerimeterIterator<'c, C>
// where
//     C: HexPosContainer,
// {
//     type Item = HexPosWithEdge;

//     fn next(&mut self) -> Option<Self::Item> {
//         while self.last_pos.is_none() {
//             self.last_pos = self.hex_iter.next();
//             if let Some(pos) = self.last_pos
//                 && let Some(edge) = self.exterior_edge(pos)
//             {
//                 if self.hexes_seen.contains(&pos) {
//                     self.last_pos = None;
//                     continue;
//                 }
//                 self.next_edge = edge;
//                 break;
//             }
//         }
//         if let Some(pos) = self.last_pos {
//             let result = Some(HexPosWithEdge::new(pos, self.next_edge));
//             self.hexes_seen.insert(pos);
//             let starting_edge = self.next_edge;
//             loop {
//                 self.next_edge = self.next_edge.rotate(1);
//                 debug_assert!(
//                     self.next_edge != starting_edge,
//                     "No exterior edge found for pos: {:?}",
//                     pos
//                 );
//                 if self.is_exterior_edge(HexPosWithEdge::new(pos, self.next_edge)) {
//                     break;
//                 }
//             }
//             result
//         } else {
//             None
//         }
//     }
// }

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
    // use crate::{grid::HexGrid, iter_valid_sizes};
    // use std::collections::HashSet;

    // #[test]
    // fn perimeter() {
    //     for size in iter_valid_sizes() {
    //         let grid = HexGrid::<i32>::new_with_defaults(size);
    //         // TODO
    //         // grid.perimeter()
    //         //     .for_each(|pos: HexPos| assert!(grid.has_hex(pos), "pos: {:?}", pos));
    //         assert_eq!(
    //             grid.perimeter().count(),
    //             grid.perimeter().collect::<HashSet<_>>().len(),
    //         );
    //     }
    // }
}
