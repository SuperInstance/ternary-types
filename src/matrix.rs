//! A 2D matrix of trits with basic operations.

extern crate alloc;
use alloc::vec::Vec;
use alloc::vec;

use crate::Ternary;
use crate::Ternary::{Negative, Neutral, Positive};
use crate::TritVector;

/// A 2D matrix of trits stored in row-major order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TernaryMatrix {
    rows: usize,
    cols: usize,
    data: Vec<Ternary>,
}

impl TernaryMatrix {
    /// Create a new matrix filled with `Neutral`.
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![Neutral; rows * cols],
        }
    }

    /// Create a matrix from a flat slice (row-major).
    pub fn from_slice(rows: usize, cols: usize, data: &[Ternary]) -> Self {
        assert_eq!(data.len(), rows * cols, "data length must match rows * cols");
        Self {
            rows,
            cols,
            data: data.to_vec(),
        }
    }

    /// Number of rows.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Number of columns.
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Get the trit at position `(r, c)`.
    pub fn get(&self, r: usize, c: usize) -> Option<Ternary> {
        if r >= self.rows || c >= self.cols {
            return None;
        }
        self.data.get(r * self.cols + c).copied()
    }

    /// Set the trit at position `(r, c)`.
    pub fn set(&mut self, r: usize, c: usize, t: Ternary) {
        if r < self.rows && c < self.cols {
            self.data[r * self.cols + c] = t;
        }
    }

    /// Get a row as a `TritVector`.
    pub fn row(&self, r: usize) -> Option<TritVector> {
        if r >= self.rows {
            return None;
        }
        let start = r * self.cols;
        Some(TritVector::new(&self.data[start..start + self.cols]))
    }

    /// Get a column as a `TritVector`.
    pub fn col(&self, c: usize) -> Option<TritVector> {
        if c >= self.cols {
            return None;
        }
        let mut col_data = Vec::with_capacity(self.rows);
        for r in 0..self.rows {
            col_data.push(self.data[r * self.cols + c]);
        }
        Some(TritVector::new(&col_data))
    }

    /// Matrix-matrix multiplication (mod 3).
    ///
    /// `C[i][j] = dot(A.row(i), B.col(j))`
    pub fn matmul(&self, other: &TernaryMatrix) -> Self {
        assert_eq!(self.cols, other.rows, "incompatible matrix dimensions");
        let mut result = TernaryMatrix::new(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let a_row = self.row(i).unwrap();
                let b_col = other.col(j).unwrap();
                result.set(i, j, a_row.dot(&b_col));
            }
        }
        result
    }

    /// Matrix-vector multiplication.
    pub fn matvec(&self, vec: &TritVector) -> TritVector {
        assert_eq!(self.cols, vec.len(), "incompatible dimensions");
        let mut result = Vec::with_capacity(self.rows);
        for i in 0..self.rows {
            let row = self.row(i).unwrap();
            result.push(row.dot(vec));
        }
        TritVector::new(&result)
    }

    /// Transpose the matrix.
    pub fn transpose(&self) -> Self {
        let mut result = TernaryMatrix::new(self.cols, self.rows);
        for r in 0..self.rows {
            for c in 0..self.cols {
                result.set(c, r, self.get(r, c).unwrap());
            }
        }
        result
    }

    /// The raw underlying data slice.
    pub fn as_slice(&self) -> &[Ternary] {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Ternary::{Negative, Positive};

    #[test]
    fn identity_matmul() {
        // 2x2 identity: [[+1, 0], [0, +1]]
        let mut eye = TernaryMatrix::new(2, 2);
        eye.set(0, 0, Positive);
        eye.set(1, 1, Positive);

        let mut a = TernaryMatrix::new(2, 2);
        a.set(0, 0, Positive);
        a.set(0, 1, Negative);
        a.set(1, 0, Neutral);
        a.set(1, 1, Positive);

        let b = a.matmul(&eye);
        assert_eq!(a, b);
    }

    #[test]
    fn transpose_symmetry() {
        let mut m = TernaryMatrix::new(2, 3);
        m.set(0, 0, Positive);
        m.set(0, 1, Negative);
        m.set(1, 2, Positive);

        let t = m.transpose();
        assert_eq!(t.rows(), 3);
        assert_eq!(t.cols(), 2);
        assert_eq!(t.get(0, 0), Some(Positive));
        assert_eq!(t.get(1, 0), Some(Negative));
        assert_eq!(t.get(2, 1), Some(Positive));
    }

    #[test]
    fn matvec_product() {
        let mut m = TernaryMatrix::new(2, 3);
        m.set(0, 0, Positive);
        m.set(0, 1, Negative);
        m.set(0, 2, Neutral);
        m.set(1, 0, Neutral);
        m.set(1, 1, Positive);
        m.set(1, 2, Negative);

        let v = TritVector::new(&[Positive, Neutral, Positive]);
        let result = m.matvec(&v);
        // row0: +1*+1 + -1*0 + 0*+1 = +1
        // row1: 0*+1 + +1*0 + -1*+1 = -1
        assert_eq!(result.as_slice(), &[Positive, Negative]);
    }
}
