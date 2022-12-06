//! Several shapes and geometric primitives to work with.

mod line;
mod polygon;
mod rect;

use crate::image::Image;

pub use self::line::*;
pub use self::polygon::*;
pub use self::rect::*;

pub trait Shape {
    /// Draw this shape.
    fn draw(&self, img: &mut Image);
}
