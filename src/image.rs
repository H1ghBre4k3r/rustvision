use crate::{color::Color, ppm::PNM};

/// Struct for representing an image.
pub struct Image {
    cols: usize,
    rows: usize,
    pixels: Vec<Vec<Color>>,
}

impl Image {
    /// Create a new image with the specified width and height, where all pixels are filled with
    /// black.
    pub fn new(cols: usize, rows: usize) -> Self {
        let pixels = vec![vec![Color::default(); cols]; rows];
        Self { cols, rows, pixels }
    }

    /// Get the number of columns of this image.
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Get the number of rows of this image.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Get the color at the specified index, or None, if the index it out of bounds.
    pub fn get(&self, x: usize, y: usize) -> Option<Color> {
        let Some(row) = self.pixels.get(y) else {
             return None;
        };
        return row.get(x).map(|color| *color);
    }

    /// Set the color of a pixel at the specified coordinates.
    pub fn set(&mut self, x: usize, y: usize, color: &Color) {
        let Some(row) = self.pixels.get_mut(y) else {
             return;
        };
        if x < row.len() {
            row[x] = *color;
        } else {
            // TODO: Return error if out of bounds
        }
    }

    /// Fill the entire image with one color.
    pub fn fill(&mut self, color: &Color) {
        self.pixels = vec![vec![*color; self.cols]; self.rows];
    }
}

impl PNM for Image {
    fn to_pnm_p3(&self) -> String {
        // add magic number
        let mut ppm = "P3\n".to_string();
        // add dimensions of the picture
        ppm.push_str(&format!("{} {}\n", self.cols, self.rows));
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
        return ppm;
    }
}

#[cfg(test)]
mod tests {
    use crate::rgb;

    use super::*;

    #[test]
    fn test_image_new() {
        let img = Image::new(42, 17);
        assert_eq!(img.cols(), 42);
        assert_eq!(img.rows(), 17);
        assert_eq!(img.pixels, vec![vec![Color::splat(0); 42]; 17]);
    }

    #[test]
    fn test_image_set() {
        let mut img = Image::new(42, 17);
        img.set(10, 10, &rgb!(42, 42, 17));
        let pixel = img.get(10, 10);
        assert_eq!(pixel, Some(rgb!(42, 42, 17)));
    }

    #[test]
    fn test_image_fill() {
        let mut img = Image::new(42, 17);
        img.fill(&rgb!(17, 120, 42));
        assert_eq!(img.pixels, vec![vec![rgb!(17, 120, 42); 42]; 17]);
    }
}
