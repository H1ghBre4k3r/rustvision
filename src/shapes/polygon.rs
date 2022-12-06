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
        // TODO: Somehow this does not correctly fill the provided
        // We should investigate this...

        let mut edge_table: Vec<Vec<Edge>> = vec![vec![]];

        // iterate over all "y"s in order to create ET
        for i in 0..img.rows() {
            let mut edges = vec![];

            // iterator over all vertices
            for (j, current) in self.points.iter().enumerate() {
                // ...and check, if they start in this row
                if current.y as usize == i {
                    // get previous and next vertex
                    let prev = self.points[if j > 0 { j - 1 } else { self.points.len() - 1 }];
                    let next = self.points[if j < self.points.len() - 1 { j + 1 } else { 0 }];

                    /*
                     * only add edge if other vertices lie strictly above the current vertex
                     * this prevents us from adding edges twice (e.g., if y1 == y2)
                     * and removes the edge case (no pun intended) of delta_y = 0
                     */
                    if current.y < prev.y {
                        edges.push(Edge::from(&current, &prev));
                    }

                    if current.y < next.y {
                        edges.push(Edge::from(&current, &next));
                    }
                }
            }

            edge_table.push(edges);
        }

        let mut active_edges: Vec<Edge> = vec![];

        for y in 0..img.rows() {
            // adjust x values of
            active_edges = active_edges
                .into_iter()
                .map(|mut edge| {
                    edge.x += edge.delta;
                    edge
                })
                .filter(|edge| edge.y2 >= y as f64)
                .collect();

            // add starting edges to the AET
            edge_table[y]
                .iter()
                .for_each(|elem| active_edges.push(*elem));

            // sort EAT by X
            active_edges.sort_by(|first, second| first.x.partial_cmp(&second.x).unwrap());

            let mut in_shape = false;

            // index for keeping track of current edge
            let mut index = 0;

            for x in 0..img.cols() {
                while index < active_edges.len() {
                    let next_edge = active_edges[index];

                    // if we cross an edge, adjust parity
                    if x == (if !in_shape {
                        next_edge.x.ceil() as usize
                    } else {
                        next_edge.x.ceil() as usize
                    }) {
                        in_shape = !in_shape;
                    } else {
                        break;
                    }
                    index += 1;
                }

                if in_shape {
                    img.set(x, y, &self.color);
                }
            }
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
