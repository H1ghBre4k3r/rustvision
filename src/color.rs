use crate::ppm::PNM;

/// Struct for representing a regular 8bit RGB color.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn splat(val: u8) -> Self {
        Self {
            r: val,
            g: val,
            b: val,
        }
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!("({},{},{})", self.r, self.g, self.b)
    }
}

impl PNM for Color {
    fn to_pnm_p3(&self) -> String {
        format!("{} {} {}", self.r, self.g, self.b)
    }

    fn to_pnm_p6(&self) -> Vec<u8> {
        vec![self.r, self.g, self.b]
    }
}

/// Utility macro for creating an RGB color.
///
/// # Examples
///
/// You can create a default (i.e., black) color by calling this macro without any arguments:
///
/// ```rust
/// use rustvision::{rgb, color::Color};
///
/// let color = rgb!();
/// assert_eq!(color, Color { r: 0, g: 0, b: 0});
/// ```
///
/// If you want to create a color with a certain gray value (i.e., all channels have the same
/// value), you can do so by calling the macro with a single argument:
///
/// ```rust
/// use rustvision::{rgb, color::Color};
///
/// let color = rgb!(42);
/// assert_eq!(color, Color { r: 42, g: 42, b: 42});
/// ```
///
/// By calling the macro with three arguments, you create a simple color:
///
/// ```rust
/// use rustvision::{rgb, color::Color};
///
/// let color = rgb!(42, 17, 129);
/// assert_eq!(color, Color { r: 42, g: 17, b: 129});
/// ```
#[macro_export]
macro_rules! rgb {
    () => {
        $crate::color::Color::default()
    };
    ($val:expr) => {
        $crate::color::Color::splat($val)
    };
    ($r:expr, $g:expr, $b:expr) => {
        $crate::color::Color::new($r, $g, $b)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_new() {
        assert_eq!(
            Color::new(10, 42, 128),
            Color {
                r: 10,
                g: 42,
                b: 128
            }
        );
    }

    #[test]
    fn test_color_default() {
        assert_eq!(Color::default(), Color { r: 0, g: 0, b: 0 });
    }

    #[test]
    fn test_color_splat() {
        assert_eq!(
            Color::splat(17),
            Color {
                r: 17,
                g: 17,
                b: 17
            }
        );
        assert_eq!(
            Color::splat(42),
            Color {
                r: 42,
                g: 42,
                b: 42
            }
        );
    }

    #[test]
    fn test_macro_empty() {
        assert_eq!(rgb!(), Color { r: 0, g: 0, b: 0 });
    }

    #[test]
    fn test_macro_splat() {
        assert_eq!(
            rgb!(42),
            Color {
                r: 42,
                g: 42,
                b: 42
            }
        );
    }

    #[test]
    fn test_macro_three_values() {
        assert_eq!(
            rgb!(17, 42, 128),
            Color {
                r: 17,
                g: 42,
                b: 128
            }
        );
    }
}
