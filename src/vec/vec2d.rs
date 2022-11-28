use super::{Vec2, VectorOperations};

/// Type for representing a 2D vector with f64 as fields.
pub type Vec2d = Vec2<f64>;

impl VectorOperations<f64> for Vec2d {
    fn length(&self) -> f64 {
        (self.dot(self)).sqrt()
    }

    fn normalize(&self) -> Self {
        let len = self.length();
        *self / len
    }

    fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        assert_eq!(Vec2d { x: 3.0, y: 4.0 }.length(), 5.0);
    }

    #[test]
    fn test_normalize() {
        assert_eq!(
            Vec2d { x: 2.0, y: 0.0 }.normalize(),
            Vec2d { x: 1.0, y: 0.0 }
        );
    }

    #[test]
    fn test_dot() {
        assert_eq!(
            Vec2d { x: -4.0, y: -9.0 }.dot(&Vec2d { x: -1.0, y: 2.0 }),
            -14.0
        );
    }
}
