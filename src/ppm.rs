//! Utility macros for saving images in PPM format.

use crate::{color::Color, image::Image};

/// Trait for transforming any values into their PNM represenataion.
pub trait PNM {
    /// Convert a value into its P3 represenataion.
    fn to_pnm_p3(&self) -> String;

    /// Convert a value into its P6 represenataion.
    fn to_pnm_p6(&self) -> Vec<u8>;
}

/// Macro for saving an image in its ASCII PPM represenataion to the disk.
#[macro_export]
macro_rules! save_pnm_p3 {
    ($file_name:expr, $img:ident) => {
        $crate::write_file!($file_name, $crate::ppm::PNM::to_pnm_p3(&$img).as_bytes());
    };
}

/// Macro for saving an image in its binary PPM represenataion to the disk.
#[macro_export]
macro_rules! save_pnm_p6 {
    ($file_name:expr, $img:ident) => {
        $crate::write_file!($file_name, &$crate::ppm::PNM::to_pnm_p6(&$img));
    };
}

/// Parse a vector of u8 into a valid PPM6 image. This function will panic, if the format is not
/// valid.
pub fn parse_ppm6(inp: Vec<u8>) -> Image {
    let mut iter = inp.iter();
    let Some(first_line_break) = iter.position(|&item| item == b'\n') else {
        panic!("No valid magic constant");
    };

    let Some(second_line_break) = iter.position(|&item| item == b'\n') else {
        panic!("No dimensions provided");
    };

    let second_line_break = second_line_break + first_line_break + 1;

    let dimensions = &inp[first_line_break + 1..second_line_break];

    let Ok(dimensions) = std::str::from_utf8(dimensions) else {
        panic!("Dimensions need to be UTF8");
    };

    let Some((left, right)) = dimensions.trim().split_once(' ') else {
        panic!("Dimensions to not have valid format!");
    };

    let Ok(width) = left.parse::<usize>() else {
        panic!("Width does not have valid format! ({})", left);
    };

    let Ok(height) = right.parse::<usize>() else {
        panic!("Height does not have valid format! ({})", right);
    };

    let Some(third_line_break) = iter.position(|&item| item == b'\n') else {
        panic!("No max value provided");
    };

    let third_line_break = third_line_break + second_line_break + 1;

    let max_value = &inp[second_line_break + 1..third_line_break];

    let Ok(max_value) = std::str::from_utf8(max_value) else {
        panic!("Max value needs to be UTF8");
    };

    let Ok(_max_value) = max_value.parse::<usize>() else {
        panic!("Max value does not have a valid format! ({})", max_value);
    };

    let after_third_line_break = third_line_break + 1;

    assert_eq!(inp.len() - after_third_line_break, width * height * 3);

    let mut image = Image::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let start = 3 * (y * width + x) + after_third_line_break;
            image.set(x, y, &Color::from_u8_array(&inp[start..start + 3]));
        }
    }
    image
}
