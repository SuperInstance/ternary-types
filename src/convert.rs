//! Conversion traits between `Ternary` and common numeric types.

extern crate alloc;
use alloc::vec::Vec;

use crate::Ternary;
use crate::TritVector;

/// A trait for types that can be converted to/from `Ternary`.
pub trait TernaryConvertible: Sized {
    /// Convert a `Ternary` to `Self`.
    fn from_ternary(t: Ternary) -> Self;

    /// Convert `Self` to a `Ternary`.
    fn to_ternary(&self) -> Ternary;
}

impl TernaryConvertible for i8 {
    fn from_ternary(t: Ternary) -> Self {
        i8::from(t)
    }

    fn to_ternary(&self) -> Ternary {
        Ternary::try_from(*self).unwrap_or(Ternary::Neutral)
    }
}

impl TernaryConvertible for f64 {
    fn from_ternary(t: Ternary) -> Self {
        t.to_f64()
    }

    fn to_ternary(&self) -> Ternary {
        if *self < -0.5 {
            Ternary::Negative
        } else if *self > 0.5 {
            Ternary::Positive
        } else {
            Ternary::Neutral
        }
    }
}

impl TernaryConvertible for bool {
    fn from_ternary(t: Ternary) -> Self {
        t.is_nonzero()
    }

    fn to_ternary(&self) -> Ternary {
        if *self { Ternary::Positive } else { Ternary::Neutral }
    }
}

/// Convert a slice of f64 values to a TritVector via threshold quantization.
pub fn f64_slice_to_trits(values: &[f64]) -> TritVector {
    let data: Vec<Ternary> = values.iter().map(|&v| {
        if v < -0.5 {
            Ternary::Negative
        } else if v > 0.5 {
            Ternary::Positive
        } else {
            Ternary::Neutral
        }
    }).collect();
    TritVector::new(&data)
}

/// Convert a slice of i8 values to a TritVector (clamping invalid values to Neutral).
pub fn i8_slice_to_trits(values: &[i8]) -> TritVector {
    let data: Vec<Ternary> = values.iter().map(|&v| {
        Ternary::try_from(v).unwrap_or(Ternary::Neutral)
    }).collect();
    TritVector::new(&data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f64_conversion() {
        assert_eq!(TernaryConvertible::to_ternary(&0.7_f64), Ternary::Positive);
        assert_eq!(TernaryConvertible::to_ternary(&(-0.7_f64)), Ternary::Negative);
        assert_eq!(TernaryConvertible::to_ternary(&0.0_f64), Ternary::Neutral);
        assert_eq!(TernaryConvertible::to_ternary(&0.4_f64), Ternary::Neutral);
    }

    #[test]
    fn f64_slice() {
        let values = vec![1.2, -0.8, 0.1, -3.0];
        let v = f64_slice_to_trits(&values);
        assert_eq!(v.as_slice(), &[
            Ternary::Positive,
            Ternary::Negative,
            Ternary::Neutral,
            Ternary::Negative,
        ]);
    }
}
