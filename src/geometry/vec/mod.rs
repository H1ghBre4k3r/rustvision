//! Vectors in 2D space.

mod vec2d;
mod vec2i;

use std::ops::{Add, Div, Mul, Sub};

pub use self::vec2d::*;
pub use self::vec2i::*;

/// Struct for prepresenting a vector in 2D space.
/// This vector features common operation such as addition, substraction and scaling
/// (e.g., vector * scalar).
///
/// Unless you really know what you are doing, you should probably not use this struct directly.
/// Rather work with the provided utility types (such as `Vec2d`).
#[derive(Default, Debug, Eq, PartialEq, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

/// Utility macro for creating a vector from the given arguments.
///
/// # Examples
///
/// You can create default (i.e., "zero length") vectors with this macro:
///
/// ```rust
/// # #[macro_use] extern crate rustvision;
/// use rustvision::geometry::vec::*;
///
/// let v = vec2![];
/// assert_eq!(v, Vec2 { x: 0, y: 0 });
/// ```
///
/// It also supports "prefilling" all fields of a vector with a given value:
///
/// ```rust
/// # #[macro_use] extern crate rustvision;
/// use rustvision::geometry::vec::*;
///
/// let v = vec2![42];
/// assert_eq!(v, Vec2 { x: 42, y: 42 });
/// ```
///
/// Lastly, it supports intuitive vector initialization aswell:
///
/// ```rust
/// # #[macro_use] extern crate rustvision;
/// use rustvision::geometry::vec::*;
///
/// let v = vec2![42, 1337];
/// assert_eq!(v, Vec2 { x: 42, y: 1337 });
/// ```
///
/// This macro is also "type agnostic" in a way, that automatically creates a vector of the fitting
/// utility type (e.g., `Vec2d`) when called with the right arguments.
#[macro_export]
macro_rules! vec2 {
    () => {
        $crate::geometry::vec::Vec2::default()
    };
    ($val:expr) => {
        $crate::geometry::vec::Vec2 { x: $val, y: $val }
    };
    ($x:expr, $y:expr) => {
        $crate::geometry::vec::Vec2 { x: $x, y: $y }
    };
}

/// Common operations on vectors.
pub trait VectorOperations<T> {
    /// Get the length of this vector.
    fn length(&self) -> f64;

    /// Normalize this vector (i.e., set its length to 1 but preserve the direction).
    fn normalize(&self) -> Vec2<f64>;

    /// Calculate the dot product of this vector and another one.
    fn dot(&self, rhs: &Self) -> T;
}

impl<T> Add for Vec2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Vec2<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul<T> for Vec2<T>
where
    T: Mul<Output = T> + Clone + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Div<T> for Vec2<T>
where
    T: Div<Output = T> + Clone + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_add() {
        assert_eq!(
            Vec2 { x: 1, y: 0 } + Vec2 { x: 2, y: 1 },
            Vec2 { x: 3, y: 1 }
        );
    }

    #[test]
    fn test_vec_sub() {
        assert_eq!(
            Vec2 { x: 1, y: 0 } - Vec2 { x: 2, y: 1 },
            Vec2 { x: -1, y: -1 }
        );
    }

    #[test]
    fn test_vec_scale() {
        assert_eq!(Vec2 { x: 2, y: 5 } * 3, Vec2 { x: 6, y: 15 });
    }

    #[test]
    fn test_vec_div() {
        assert_eq!(Vec2 { x: 2.0, y: 5.0 } / 2.0, Vec2 { x: 1.0, y: 2.5 });
    }

    #[test]
    fn test_vec2_macro_empty() {
        assert_eq!(vec2![], Vec2 { x: 0, y: 0 });
    }

    #[test]
    fn test_vec2_macro_splat() {
        assert_eq!(vec2![42], Vec2 { x: 42, y: 42 });
    }

    #[test]
    fn test_vec2_macro_double_args() {
        assert_eq!(vec2![3, 4], Vec2 { x: 3, y: 4 });
    }
}
