use crate::{color::Color, image::Image, vec::Vec2d};

use super::Shape;

/// Struct representing a rectangle with given anchor and dimensions.
#[derive(Default, Clone, Copy, Debug)]
pub struct Rectangle {
    anchor: Vec2d,
    width: usize,
    height: usize,
    color: Color,
}

impl Rectangle {
    pub fn new(anchor: Vec2d, width: usize, height: usize, color: Color) -> Self {
        Self {
            anchor,
            width,
            height,
            color,
        }
    }
}

impl Shape for Rectangle {
    fn draw(&self, img: &mut Image) {
        for x in 0..self.width {
            for y in 0..self.height {
                // TODO: add proper bounds checks
                img.set(
                    self.anchor.x as usize + x,
                    self.anchor.y as usize + y,
                    &self.color,
                );
            }
        }
    }
}
