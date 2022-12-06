//! Lines in 2D.

use crate::{color::Color, image::Image, vec::Vec2d};

use super::Shape;

/// Struct for representing a line in 2D space with a given color.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Line {
    start: Vec2d,
    end: Vec2d,
    color: Color,
}

impl Line {
    /// Create a new line with given start and end point.
    pub fn new(start: Vec2d, end: Vec2d) -> Self {
        Self {
            start,
            end,
            ..Default::default()
        }
    }

    /// Add a color to this line.
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Shape for Line {
    fn draw(&self, img: &mut Image) {
        // Some quick algorithms for drawing fancy lines
        let p0 = self.start;
        let p1 = self.end;

        let x0 = p0.x;
        let y0 = p0.y;

        let x1 = p1.x;
        let y1 = p1.y;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        let sx = (x1 - x0).signum();
        let sy = (y1 - y0).signum();

        let mut x = x0;
        let mut y = y0;

        if dx >= dy {
            let mut e = 2.0 * dy - dx;

            while x != x1 || y != y1 {
                img.set(x as usize, y as usize, &self.color);

                x += sx;
                if e <= 0.0 {
                    e += 2.0 * dy;
                } else {
                    y += sy;
                    e += 2.0 * dy - 2.0 * dx;
                }
            }
        } else {
            let mut e = 2.0 * dx - dy;

            while x != x1 || y != y1 {
                img.set(x as usize, y as usize, &self.color);

                y += sy;
                if e <= 0.0 {
                    e += 2.0 * dx;
                } else {
                    x += sx;
                    e += 2.0 * dx - 2.0 * dy;
                }
            }
        }
        img.set(x as usize, y as usize, &self.color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{rgb, vec2};

    #[test]
    fn test_line_new() {
        let start = vec2![10.0, 20.0];
        let end = vec2![15.0, 10.0];
        assert_eq!(
            Line::new(start, end),
            Line {
                start,
                end,
                color: Color::default()
            }
        );
    }

    #[test]
    fn test_line_with_color() {
        let start = vec2![10.0, 20.0];
        let end = vec2![15.0, 10.0];
        let color = rgb!(255, 42, 17);
        assert_eq!(
            Line::new(start, end).with_color(color),
            Line { start, end, color }
        );
    }
}
