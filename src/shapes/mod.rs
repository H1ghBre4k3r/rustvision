mod line;
mod rect;

use crate::image::Image;

pub use self::line::*;
pub use self::rect::*;

pub trait Shape<const COLS: usize, const ROWS: usize> {
    /// Draw this shape.
    fn draw(&self, img: &mut Image<COLS, ROWS>);
}
