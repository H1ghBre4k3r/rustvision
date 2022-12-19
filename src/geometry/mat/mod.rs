//! Module for interacting with all kind of matrices;
mod basic;

pub use self::basic::*;

/// Type representing a homogeneous matrix (i.e, a matrix with 3 rows).
pub type HomogeneousMatrix<T, const COLS: usize> = Mat<T, COLS, 3>;
