use crate::{color::Color, ppm::PNM, shapes::Shape, vec::Vec2d};

/// Struct for representing an image.
pub struct Image<const COLS: usize, const ROWS: usize> {
    pixels: [[Color; COLS]; ROWS],
}

impl<const COLS: usize, const ROWS: usize> Image<COLS, ROWS> {
    /// Create a new image with the specified width and height, where all pixels are filled with
    /// black.
    pub fn new() -> Self {
        let pixels = [[Color::default(); COLS]; ROWS];
        Self { pixels }
    }

    /// Get the number of columns of this image.
    pub fn cols(&self) -> usize {
        COLS
    }

    /// Get the number of rows of this image.
    pub fn rows(&self) -> usize {
        ROWS
    }

    /// Get the color at the specified index, or None, if the index it out of bounds.
    pub fn get(&self, x: usize, y: usize) -> Option<Color> {
        let Some(row) = self.pixels.get(y) else {
             return None;
        };
        return row.get(x).copied();
    }

    /// Set the color of a pixel at the specified coordinates.
    pub fn set(&mut self, x: usize, y: usize, color: &Color) {
        let Some(row) = self.pixels.get_mut(y) else {
             return;
        };
        if x < COLS {
            row[x] = *color;
        } else {
            // TODO: Return error if out of bounds
        }
    }

    /// Get the color at a specified pixel.
    /// The semantics are the same as for Image::get.
    pub fn at(&self, vector: Vec2d) -> Option<Color> {
        match (vector.x, vector.y) {
            (x, y) if x >= 0.0 && y >= 0.0 => self.get(x as usize, y as usize),
            _ => None,
        }
    }

    /// Fill the entire image with one color.
    pub fn fill_with(&mut self, color: &Color) {
        self.pixels = [[*color; COLS]; ROWS];
    }

    /// Draw a given shape to the picture.
    pub fn draw(&mut self, shape: &dyn Shape<COLS, ROWS>) {
        shape.draw(self);
    }
}

impl<const COLS: usize, const ROWS: usize> PNM for Image<COLS, ROWS> {
    fn to_pnm_p3(&self) -> String {
        // add magic number
        let mut ppm = "P3\n".to_string();
        // add dimensions of the picture
        ppm.push_str(&format!("{} {}\n", COLS, ROWS));
        // add max value
        ppm.push_str(&format!("{}\n", u8::max_value()));

        // add rows after each other
        for row in &self.pixels {
            let mut row_str = "".to_string();
            for pixel in row {
                row_str.push_str(&format!("{} ", pixel.to_pnm_p3()));
            }
            ppm.push_str(&format!("{}\n", row_str));
        }
        ppm
    }

    fn to_pnm_p6(&self) -> Vec<u8> {
        // add magic number
        let mut ppm = "P6\n".to_string();
        // add dimensions of the picture
        ppm.push_str(&format!("{} {}\n", COLS, ROWS));
        // add max value
        ppm.push_str(&format!("{}\n", u8::max_value()));
        let mut ppm = ppm.as_bytes().to_owned();
        for row in &self.pixels {
            let mut row_vec = vec![];
            for pixel in row {
                row_vec.append(&mut pixel.to_pnm_p6());
            }
            ppm.append(&mut row_vec);
        }
        ppm
    }
}

#[cfg(test)]
mod tests {
    use crate::{rgb, vec2};

    use super::*;

    #[test]
    fn test_image_new() {
        let img = Image::<42, 17>::new();
        assert_eq!(img.cols(), 42);
        assert_eq!(img.rows(), 17);
        assert_eq!(img.pixels, [[Color::splat(0); 42]; 17]);
    }

    #[test]
    fn test_image_set() {
        let mut img = Image::<42, 17>::new();
        img.set(10, 10, &rgb!(42, 42, 17));
        let pixel = img.get(10, 10);
        assert_eq!(pixel, Some(rgb!(42, 42, 17)));
        let pixel = img.at(vec2![10.0]);
        assert_eq!(pixel, Some(rgb!(42, 42, 17)));
    }

    #[test]
    fn test_image_get_out_of_bounds() {
        let img = Image::<42, 17>::new();
        let pixel = img.get(100, 100);
        assert_eq!(pixel, None);
    }

    #[test]
    fn test_image_at_out_of_bounds() {
        let img = Image::<42, 17>::new();
        let pixel = img.at(vec2![100.0]);
        assert_eq!(pixel, None);
    }

    #[test]
    fn test_image_fill() {
        let mut img = Image::<42, 17>::new();
        img.fill_with(&rgb!(17, 120, 42));
        assert_eq!(img.pixels, [[rgb!(17, 120, 42); 42]; 17]);
    }
}
