use crate::{color::Color, vec::Vec2d};

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
