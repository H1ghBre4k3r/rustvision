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
