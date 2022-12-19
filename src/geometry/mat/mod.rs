use std::ops::{Add, AddAssign, Mul};

/// Struc for representing matrices with an arbitrary amount of rows and columns.
/// The matrix supports common operations, like addition and matrix multiplication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mat<T, const COLS: usize, const ROWS: usize>
where
    T: Clone + Copy,
{
    matrix: [[T; COLS]; ROWS],
}

/// Utility macro for creating a marix from the given 2D slice.
#[macro_export]
macro_rules! mat {
    () => {
        $crate::geometry::mat::Mat::default()
    };
    ($matrix:expr) => {
        $crate::geometry::mat::Mat::from($matrix)
    };
}

impl<T, const COLS: usize, const ROWS: usize> Mat<T, COLS, ROWS>
where
    T: Clone + Copy + Default,
{
    pub fn new() -> Mat<T, COLS, ROWS> {
        Self {
            matrix: [[T::default(); COLS]; ROWS],
        }
    }
}

impl<T, const COLS: usize, const ROWS: usize> Default for Mat<T, COLS, ROWS>
where
    T: Clone + Copy + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const COLS: usize, const ROWS: usize> From<[[T; COLS]; ROWS]> for Mat<T, COLS, ROWS>
where
    T: Copy,
{
    fn from(matrix: [[T; COLS]; ROWS]) -> Self {
        Self { matrix }
    }
}

impl<T, const COLS: usize, const ROWS: usize> Add for Mat<T, COLS, ROWS>
where
    T: Add<Output = T> + Copy + Default,
{
    type Output = Mat<T, COLS, ROWS>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_matrix = Mat::<T, COLS, ROWS>::new();

        for x in 0..COLS {
            for y in 0..ROWS {
                new_matrix.matrix[y][x] = self.matrix[y][x] + rhs.matrix[y][x];
            }
        }

        new_matrix
    }
}

impl<T, const COLS_1_ROWS_2: usize, const ROWS_1: usize, const COLS_2: usize>
    Mul<Mat<T, COLS_2, COLS_1_ROWS_2>> for Mat<T, COLS_1_ROWS_2, ROWS_1>
where
    T: Mul<Output = T> + Add<Output = T> + AddAssign + Copy + Default,
{
    type Output = Mat<T, COLS_2, ROWS_1>;

    fn mul(self, rhs: Mat<T, COLS_2, COLS_1_ROWS_2>) -> Self::Output {
        let mut new_matrix = Mat::<T, COLS_2, ROWS_1>::new();

        for x in 0..COLS_2 {
            for y in 0..ROWS_1 {
                for a in 0..COLS_1_ROWS_2 {
                    new_matrix.matrix[y][x] += self.matrix[y][a] * rhs.matrix[a][x];
                }
            }
        }

        new_matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mat_add_empty() {
        let a = Mat::<i32, 4, 4>::new();
        let b = Mat::<i32, 4, 4>::new();
        assert_eq!(a + b, Mat::<i32, 4, 4>::new());
    }

    #[test]
    fn test_mat_add() {
        let a = Mat::from([[1, 2], [3, 4]]);
        let b = Mat::from([[2, 4], [6, 8]]);

        assert_eq!(a + b, Mat::from([[3, 6], [9, 12]]));
    }

    #[test]
    fn test_mat_macro_empty() {
        let matrix: Mat<u64, 4, 4> = mat! {};
        assert_eq!(matrix, Mat::from([[0; 4]; 4]));
    }

    #[test]
    fn test_mat_macro() {
        let matrix = mat! {
            [[1, 2], [3, 4]]
        };

        assert_eq!(
            matrix,
            Mat {
                matrix: [[1, 2], [3, 4]]
            }
        );
    }

    #[test]
    fn test_mat_from() {
        assert_eq!(
            Mat::from([[1, 2], [3, 4]]),
            Mat {
                matrix: [[1, 2], [3, 4]]
            }
        )
    }

    #[test]
    fn test_mat_mul() {
        let a = mat! {
            [[1, 2, 3], [4, 5, 6]]
        };

        let b = mat! {
            [[1, 2, 6], [3, 4, 2], [6, 7, 1]]
        };

        let result = a * b;
        assert_eq!(
            result,
            mat! {
                [[25, 31, 13], [55, 70, 40]]
            }
        )
    }
}
