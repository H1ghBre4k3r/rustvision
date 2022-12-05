use crate::{color::Color, image::Image, vec::Vec2d};

use super::{Line, Shape};

/// Struct representing a polygon.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Polygon {
    points: Vec<Vec2d>,
    filled: bool,
    color: Color,
}

impl Polygon {
    /// Create a new polygon with the specified points.
    pub fn from_points(points: Vec<Vec2d>) -> Self {
        Self {
            points,
            ..Default::default()
        }
    }

    /// Set the filled status of this polygon.
    pub fn set_filled(&mut self, filled: bool) {
        self.filled = filled;
    }

    /// Set the color of this polygon.
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl Shape for Polygon {
    fn draw(&self, img: &mut Image) {
        if self.filled {
            todo!();
        }

        for (i, point) in self.points.iter().enumerate() {
            let line =
                Line::new(*point, self.points[(i + 1) % self.points.len()]).with_color(self.color);
            img.draw(&line);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{color::Color, rgb, vec2};

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

    #[test]
    fn test_set_filled() {
        let mut polygon = Polygon::default();

        polygon.set_filled(true);
        assert!(polygon.filled);

        polygon.set_filled(false);
        assert!(!polygon.filled);
    }

    #[test]
    fn test_set_color() {
        let mut polygon = Polygon::default();
        let color = rgb!(17, 42, 137);

        polygon.set_color(color);
        assert_eq!(polygon.color, color);
    }
}
