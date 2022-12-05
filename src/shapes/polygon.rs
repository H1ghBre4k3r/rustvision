use crate::vec::Vec2d;

/// Struct representing a polygon.
pub struct Polygon {
    points: Vec<Vec2d>,
}

impl Polygon {
    /// Create a new polygon with the specified points.
    fn from_points(points: Vec<Vec2d>) -> Self {
        Self { points }
    }
}
