/// Macro for loading a file from the disk.
#[macro_export]
macro_rules! load_file {
    ($file_name:expr) => {
        std::fs::read($file_name).expect("Should have been able to read the file")
    };
}

/// Macro for writing a file to the disk.
#[macro_export]
macro_rules! write_file {
    ($file_name:expr, $content:expr) => {
        use std::io::prelude::*;
        let mut file = std::fs::File::create($file_name).expect("File could not be opened");
        file.write_all($content).expect("Could not write to file");
    };
}

/// Enum representing all supported files types of this library.
pub enum FileTypes {
    PPM3,
    PPM6,
}

/// Error when an unsupported image is provided.
#[derive(Debug)]
pub struct InvalidMagicConstantError(String);

impl TryFrom<&[u8]> for FileTypes {
    type Error = InvalidMagicConstantError;

    fn try_from(magic_constant: &[u8]) -> Result<Self, Self::Error> {
        use FileTypes::*;

        match magic_constant {
            b"P3" => Ok(PPM3),
            b"P6" => Ok(PPM6),
            x => Err(InvalidMagicConstantError(format!(
                "Magic constant '{:?}' currently not supported",
                x
            ))),
        }
    }
}
