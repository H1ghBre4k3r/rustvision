//! Module for working with images.

mod representation;

use crate::{files::FileTypes, ppm::parse_ppm6};

pub use self::representation::*;

/// Try to parse a given image from a vector of u8. The format will be determined from the magic
/// constant at the head of the file.
pub fn parse_image(inp: Vec<u8>) -> Image {
    let mut lines = inp.split(|val| *val == b'\n');

    let Some(magic_constant) = lines.next() else {
        panic!("No magic constant provided");
    };

    let Ok(magic_constant) = FileTypes::try_from(magic_constant) else {
        panic!("Invalid magic constant provided");
    };

    match magic_constant {
        FileTypes::PPM3 => panic!("PPM ASCII currently not supported"),
        FileTypes::PPM6 => parse_ppm6(inp),
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
