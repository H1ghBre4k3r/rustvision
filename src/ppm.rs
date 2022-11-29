/// Trait for transforming any values into their PNM represenataion.
pub trait PNM {
    /// Convert a value into its P3 represenataion.
    fn to_pnm_p3(&self) -> String;

    /// Convert a value into its P6 represenataion.
    fn to_pnm_p6(&self) -> Vec<u8>;
}

/// Macro for saving an image in its PPM represenataion to the disk.
#[macro_export]
macro_rules! save_pnm_p3 {
    ($file_name:expr, $img:ident) => {
        use std::io::prelude::*;
        let mut file = std::fs::File::create($file_name).expect("File could not be opened");
        file.write_all($img.to_pnm_p3().as_bytes())
            .expect("Could not write to file");
    };
}

#[macro_export]
macro_rules! save_pnm_p6 {
    ($file_name:expr, $img:ident) => {
        use std::io::prelude::*;
        let mut file = std::fs::File::create($file_name).expect("File could not be opened");
        file.write_all(&$img.to_pnm_p6())
            .expect("Could not write to file");
    };
}
