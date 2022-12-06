//! Module for working with polygons in 2D.
use crate::{color::Color, image::Image, vec::Vec2d};

use super::{Line, Shape};

/// Struct representing a polygon.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Polygon {
    points: Vec<Vec2d>,
    filled: bool,
    color: Color,
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    pub y2: f64,
    pub x: f64,
    pub delta: f64,
}

impl Edge {
    fn from(first: &Vec2d, next: &Vec2d) -> Self {
        Self {
            y2: next.y,
            x: first.x,
            delta: (next.x - first.x) / (next.y - first.y),
        }
    }
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

    fn fill(&self, img: &mut Image) {
        let HEIGHT = img.rows();
        let WIDTH = img.cols();

        let mut edge_table: Vec<Vec<Edge>> = vec![vec![]];

        for i in 0..HEIGHT {
            let mut edges = vec![];

            for (j, current) in self.points.iter().enumerate() {
                if current.x as usize == i {
                    let prev = self.points[if j > 0 { j - 1 } else { self.points.len() - 1 }];
                    let next = self.points[if j < self.points.len() - 1 { j + 1 } else { 0 }];

                    if current.y < prev.y {
                        edges.push(Edge::from(&current, &prev));
                    }

                    if current.y < next.y {
                        edges.push(Edge::from(&current, &prev));
                    }
                }
            }

            edge_table.push(edges);
        }

        let mut active_edges: Vec<Edge> = vec![];

        for y in 0..HEIGHT {
            active_edges = active_edges
                .into_iter()
                .map(|mut edge| {
                    edge.x += edge.delta;
                    edge
                })
                .filter(|edge| edge.y2 as usize > y)
                .collect();

            edge_table[y]
                .iter()
                .for_each(|elem| active_edges.push(*elem));

            active_edges.sort_by(|first, second| first.x.partial_cmp(&second.x).unwrap());
            // TODO: Implement me pls! :<
            todo!();
        }
    }
}

impl Shape for Polygon {
    fn draw(&self, img: &mut Image) {
        if self.filled {
            self.fill(img);
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
