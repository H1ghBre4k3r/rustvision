//! Module for working with images.

mod image;

use crate::ppm::parse_ppm6;

pub use self::image::*;

/// Try to parse a given image from a vector of u8. The format will be determined from the magic
/// constant at the head of the file.
pub fn parse_image(inp: Vec<u8>) -> Image {
    let mut lines = inp.split(|val| *val == '\n' as u8);

    let Some(magic_constant) = lines.next() else {
        panic!("No magic constant provided");
    };

    let Ok(magic_constant) = std::str::from_utf8(magic_constant) else {
        panic!("Invalid magic constant provided");
    };

    match magic_constant.trim() {
        "P3" => panic!("PPM ASCII currently not supported"),
        "P6" => parse_ppm6(inp),
        x => panic!("Magic constant '{}' currently not supported", x),
    }
}

/// Macro to load an image from the disk and automatically parse it depending on the provided magic
/// constant.
#[macro_export]
macro_rules! load_image {
    ($file_name:expr) => {
        $crate::image::parse_image($crate::load_file!($file_name));
    };
}
