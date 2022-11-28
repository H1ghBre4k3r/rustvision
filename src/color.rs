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
}

#[macro_export]
macro_rules! rgb {
    () => {
        Color::default()
    };
    ($val:expr) => {
        Color::splat($val)
    };
    ($r:expr, $g:expr, $b:expr) => {
        Color::new($r, $g, $b)
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
