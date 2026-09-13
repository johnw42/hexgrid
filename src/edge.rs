use crate::corner::HexCorner;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HexEdge {
    TopRight,
    Top,
    TopLeft,
    BottomLeft,
    Bottom,
    BottomRight,
}

impl HexEdge {
    pub fn all() -> [HexEdge; 6] {
        [
            HexEdge::TopRight,
            HexEdge::Top,
            HexEdge::TopLeft,
            HexEdge::BottomLeft,
            HexEdge::Bottom,
            HexEdge::BottomRight,
        ]
    }

    pub fn ends(self) -> [HexCorner; 2] {
        match self {
            HexEdge::TopRight => [HexCorner::Right, HexCorner::TopRight],
            HexEdge::Top => [HexCorner::TopRight, HexCorner::TopLeft],
            HexEdge::TopLeft => [HexCorner::TopLeft, HexCorner::Left],
            HexEdge::BottomLeft => [HexCorner::Left, HexCorner::BottomLeft],
            HexEdge::Bottom => [HexCorner::BottomLeft, HexCorner::BottomRight],
            HexEdge::BottomRight => [HexCorner::BottomRight, HexCorner::Right],
        }
    }
}
