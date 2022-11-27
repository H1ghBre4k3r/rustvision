use std::ops::{Add, Div, Mul, Sub};

/// Struct for prepresenting a vector in 2D space.
/// This vector features common operation such as addition, substraction and scaling
/// (e.g., vector * scalar).
///
/// Unless you really know what you are doing, you should probably not use this struct directly.
/// Rather work with the provided utility types (such as `Vec2d`).
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

/// Type for representing a 2D vector with f64 as fields.
pub type Vec2d = Vec2<f64>;

pub trait VectorOperations<T> {
    /// Get the length of this vector.
    fn length(&self) -> T;

    /// Normalize this vector (i.e., set its length to 1 but preserve the direction).
    fn normalize(&self) -> Self;
}

impl VectorOperations<f64> for Vec2d {
    fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    fn normalize(&self) -> Self {
        let len = self.length();
        self.clone() / len
    }
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
}
