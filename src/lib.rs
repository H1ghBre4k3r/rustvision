//! # Rustvision
//!
//! Simple (and very early) project for interacting with images, vertices, and polygons (and some other stuff related to computer vision).
//!
//! **Note:** This project is in a very early state and the documentation is far from perfect. Feel free to open PRs with improvements!
//!
//! ## Installation
//!
//! Simply add this crate to your `Cargo.toml` (either via `cargo add` or manually).
//!
//! ## Usage
//!
//! The usage of this library is very straight forward. First you need to create a new image.
//!
//! ```rust
//! use rustvision::image::Image;
//!
//! fn main() {
//!     let mut img = Image::new(400, 400);
//! }
//! ```
//!
//! ### Rectangle
//!
//! To work with a simple rectangle, you can import and then create one:
//!
//! ```rust
//! use rustvision::{image::Image, shapes::Rectangle, rgb, vec2};
//!
//! // ...
//!     let rect = Rectangle::new(vec2![50.0, 40.0], 100, 70, rgb!(255, 0, 0));
//! // ...
//! ```
//!
//! This will create a new instance of a rectangle at the coordinates x-y-coordinates `(50.0, 40.0)` with a width of 100, a height of 70 and the color red.
//!
//! You will notice the usage of two macros here: `vec2![]` and `rgb!()`. `vec2![]` is a utility macro to create a 2D vector with the specified coordinates, whereas `rgb!()` creates a new RGB color for you. For more information on these have a look at the code (or later documentation).
//!
//! If you want to draw this created rectangle to the image, you can do so by calling `img.draw`:
//!
//! ```rust
//! // ...
//!     img.draw(&rect);
//! // ...
//! ```
//!
//! This will draw the rectangle to the provided image. To finally save the image to your disk, you can use another utility macro:
//!
//! ```rust
//! use rustvision::{image::Image, shapes::Rectangle, rgb, vec2, save_pnm_p6};
//!
//! // ...
//!     save_pnm_p6!("path_to_image.ppm", img);
//! // ...
//! ```
//!
//! The image will be saved in the P6 (binary PPM) representation. If you prefer the ASCII representation (P3), this library provides another utility macro as well.
//!
//! ### Polygons
//!
//! Similar to rectangles, you can create more complex polygons:
//!
//! ```rust
//! use rustvision::{image::Image, shapes::Polygon, vec2};
//!
//! // ...
//!     let mut polygon = Polygon::from_points(vec![
//!         vec2![20.0, 250.0],
//!         vec2![50.0, 350.0],
//!         vec2![80.0, 280.0],
//!         vec2![110.0, 350.0],
//!         vec2![140.0, 250.0],
//!         vec2![110.0, 300.0],
//!         vec2![80.0, 250.0],
//!         vec2![50.0, 300.0],
//!     ]);
//! // ...
//! ```
//!
//! Notice that this "constructor" does not take a color. By default, this polygon will be black. If you want to specify a color, you can do so with the respective method:
//!
//! ```rust
//! use rustvision::{image::Image, shapes::Polygon, rgb, vec2};
//!
//! // ...
//!     polygon.set_color(rgb!(0, 255, 0));
//! // ...
//! ```
//!
//! This will color the polygon green. Painting this polygon now would result in only the outline being drawn. To fill it, you need to manually set the flag:
//!
//! ```rust
//! // ...
//!     polygon.set_filled(true);
//! // ...
//! ```
//!
//! Similar to a rectangle, you can draw this polygon to an image:
//!
//! ```rust
//! // ...
//!     img.draw(&polygon);
//! // ...
//!    ```
pub mod color;
pub mod image;
pub mod ppm;
pub mod shapes;
pub mod vec;
