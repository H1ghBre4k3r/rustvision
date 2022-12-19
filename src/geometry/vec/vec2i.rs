use crate::{geometry::mat::HomogeneousMatrix, mat, vec2};

use super::{Vec2, VectorOperations};

/// Type for representing a 2D vector with i64 as fields.
pub type Vec2i = Vec2<i64>;

impl VectorOperations<i64> for Vec2i {
    fn length(&self) -> f64 {
        ((self.dot(self)) as f64).sqrt()
    }

    fn normalize(&self) -> Vec2<f64> {
        vec2![self.x as f64, self.y as f64] / self.length()
    }

    fn dot(&self, rhs: &Self) -> i64 {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl From<Vec2i> for HomogeneousMatrix<i64, 1> {
    fn from(vector: Vec2i) -> Self {
        mat! { [[vector.x], [vector.y], [1]]}
    }
}

#[cfg(test)]
mod tests {
    use crate::vec2;

    use super::*;

    #[test]
    fn test_vec2i_macro_empty() {
        assert_eq!(vec2![], Vec2i { x: 0, y: 0 });
    }

    #[test]
    fn test_vec2i_macro_splat() {
        assert_eq!(vec2![3], Vec2i { x: 3, y: 3 });
    }

    #[test]
    fn test_vec2i_macro_double_args() {
        assert_eq!(vec2![3, 4], Vec2i { x: 3, y: 4 });
    }

    #[test]
    fn test_length() {
        assert_eq!(Vec2i { x: 3, y: 4 }.length(), 5.0);
    }

    #[test]
    fn test_dot() {
        assert_eq!(Vec2i { x: -4, y: -9 }.dot(&Vec2i { x: -1, y: 2 }), -14);
    }

    #[test]
    fn test_hom_matrix_from_vec2i() {
        assert_eq!(
            HomogeneousMatrix::from(Vec2i { x: 42, y: 17 }),
            HomogeneousMatrix::from([[42], [17], [1]])
        );
    }
}
