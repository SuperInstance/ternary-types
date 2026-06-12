//! A heap-allocated vector of trits with arithmetic operations.

extern crate alloc;
use alloc::vec::Vec;
use alloc::vec;

use crate::Ternary;
use crate::Ternary::{Negative, Neutral, Positive};

/// A heap-allocated vector of trits.
///
/// Provides arithmetic operations, dot products, and conversions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TritVector {
    data: Vec<Ternary>,
}

impl TritVector {
    /// Create a new `TritVector` from a slice of `Ternary` values.
    pub fn new(trits: &[Ternary]) -> Self {
        Self { data: trits.to_vec() }
    }

    /// Create a `TritVector` of length `n` filled with `Neutral`.
    pub fn zeros(len: usize) -> Self {
        Self { data: vec![Neutral; len] }
    }

    /// Create a `TritVector` of length `n` filled with `Positive`.
    pub fn ones(len: usize) -> Self {
        Self { data: vec![Positive; len] }
    }

    /// The length of this vector.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Is this vector empty?
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get the trit at index `i`.
    pub fn get(&self, i: usize) -> Option<Ternary> {
        self.data.get(i).copied()
    }

    /// Set the trit at index `i`.
    pub fn set(&mut self, i: usize, t: Ternary) {
        if let Some(e) = self.data.get_mut(i) {
            *e = t;
        }
    }

    /// Return a reference to the underlying slice.
    pub fn as_slice(&self) -> &[Ternary] {
        &self.data
    }

    /// Convert to a `Vec<i8>`.
    pub fn to_i8_vec(&self) -> Vec<i8> {
        self.data.iter().map(|&t| i8::from(t)).collect()
    }

    /// Convert to a `Vec<f64>`.
    pub fn to_f64_vec(&self) -> Vec<f64> {
        self.data.iter().map(|&t| i8::from(t) as f64).collect()
    }

    /// Dot product of two `TritVector`s.
    ///
    /// Returns a `Ternary` (sum mod 3 via balanced addition).
    pub fn dot(&self, other: &TritVector) -> Ternary {
        let mut sum = Neutral;
        for (a, b) in self.data.iter().zip(other.data.iter()) {
            sum = sum + (*a * *b);
        }
        sum
    }

    /// Element-wise addition.
    pub fn add(&self, other: &TritVector) -> Self {
        let data: Vec<Ternary> = self.data.iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a + b)
            .collect();
        Self { data }
    }

    /// Element-wise multiplication.
    pub fn mul(&self, other: &TritVector) -> Self {
        let data: Vec<Ternary> = self.data.iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a * b)
            .collect();
        Self { data }
    }

    /// Negate all elements.
    pub fn negate(&self) -> Self {
        let data: Vec<Ternary> = self.data.iter().map(|&t| -t).collect();
        Self { data }
    }

    /// Popcount of non-zero trits (L0 "norm").
    pub fn count_nonzero(&self) -> usize {
        self.data.iter().filter(|t| t.is_nonzero()).count()
    }

    /// Hamming distance: number of positions where trits differ.
    pub fn hamming_distance(&self, other: &TritVector) -> usize {
        self.data.iter()
            .zip(other.data.iter())
            .filter(|(a, b)| a != b)
            .count()
    }

    /// Pack this vector into a `PackedTrits` representation.
    pub fn pack(&self) -> crate::PackedTrits {
        crate::PackedTrits::pack(&self.data)
    }
}

impl From<Vec<Ternary>> for TritVector {
    fn from(v: Vec<Ternary>) -> Self {
        Self { data: v }
    }
}

impl From<TritVector> for Vec<Ternary> {
    fn from(v: TritVector) -> Self {
        v.data
    }
}

impl From<Vec<i8>> for TritVector {
    fn from(v: Vec<i8>) -> Self {
        let data: Vec<Ternary> = v.iter().map(|&x| {
            match x {
                -1 => Negative,
                0  => Neutral,
                1  => Positive,
                _ => panic!("invalid trit value: {x}"),
            }
        }).collect();
        Self { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_product() {
        let a = TritVector::new(&[Positive, Negative, Neutral]);
        let b = TritVector::new(&[Positive, Positive, Negative]);
        // dot = (+1*+1) + (-1*+1) + (0*-1) = +1 + -1 + 0 = 0 (Neutral)
        assert_eq!(a.dot(&b), Neutral);
    }

    #[test]
    fn add_vectors() {
        let a = TritVector::new(&[Positive, Negative, Positive]);
        let b = TritVector::new(&[Negative, Positive, Positive]);
        let c = a.add(&b);
        // +1 + -1 = 0, -1 + +1 = 0, +1 + +1 = -1
        assert_eq!(c.as_slice(), &[Neutral, Neutral, Negative]);
    }

    #[test]
    fn hamming_distance() {
        let a = TritVector::new(&[Positive, Negative, Neutral]);
        let b = TritVector::new(&[Positive, Neutral, Neutral]);
        assert_eq!(a.hamming_distance(&b), 1);
    }

    #[test]
    fn count_nonzero() {
        let v = TritVector::new(&[Positive, Neutral, Negative, Neutral, Positive]);
        assert_eq!(v.count_nonzero(), 3);
    }

    #[test]
    fn from_i8_vec() {
        let v = TritVector::from(vec![1i8, 0, -1, 1]);
        assert_eq!(v.as_slice(), &[Positive, Neutral, Negative, Positive]);
    }

    #[test]
    fn pack_unpack() {
        let v = TritVector::new(&[Negative, Positive, Negative, Neutral]);
        let packed = v.pack();
        let unpacked = packed.unpack();
        assert_eq!(unpacked, v.as_slice());
    }
}
