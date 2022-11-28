use crate::color::Color;

/// Struct for representing an image.
pub struct Image {
    cols: usize,
    rows: usize,
    pixels: Vec<Vec<Color>>,
}

impl Image {
    /// Create a new image with the specified width and height, where all pixels are filled with
    /// black.
    fn new(cols: usize, rows: usize) -> Self {
        let pixels = vec![vec![Color::default(); rows]; cols];
        Self { cols, rows, pixels }
    }

    /// Get the number of columns of this image.
    fn cols(&self) -> usize {
        self.cols
    }

    /// Get the number of rows of this image.
    fn rows(&self) -> usize {
        self.rows
    }

    /// Get the color at the specified index, or None, if the index it out of bounds.
    fn get(&self, x: usize, y: usize) -> Option<Color> {
        let Some(column) = self.pixels.get(x) else {
             return None;
        };
        return column.get(y).map(|color| *color);
    }

    /// Set the color of a pixel at the specified coordinates.
    fn set(&mut self, x: usize, y: usize, color: &Color) {
        let Some(column) = self.pixels.get_mut(x) else {
             return;
        };
        if y < column.len() {
            column[y] = *color;
        } else {
            // TODO: Return error if out of bounds
        }
    }

    /// Fill the entire image with one color.
    fn fill(&mut self, color: &Color) {
        self.pixels = vec![vec![*color; self.rows]; self.cols];
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
        assert_eq!(img.pixels, vec![vec![Color::splat(0); 17]; 42]);
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
        assert_eq!(img.pixels, vec![vec![rgb!(17, 120, 42); 17]; 42]);
    }
}
