use crate::{color::Color, vec::Vec2d};

/// Struct representing a polygon.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Polygon {
    points: Vec<Vec2d>,
    filled: bool,
    color: Color,
}

impl Polygon {
    /// Create a new polygon with the specified points.
    fn from_points(points: Vec<Vec2d>) -> Self {
        Self {
            points,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{color::Color, vec2};

    use super::*;

    #[test]
    fn test_polygon_from() {
        let vecs = vec![vec2![2.0, 10.0], vec2![10.0, 15.0], vec2![16.0, 20.0]];
        assert_eq!(
            Polygon::from_points(vecs.clone()),
            Polygon {
                points: vecs,
                filled: false,
                color: Color::default()
            }
        )
    }
}
