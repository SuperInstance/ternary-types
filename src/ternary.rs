//! The core `Ternary` enum and its operations.
//!
//! This module was the original content of `ternary-types` v0.1.0 and remains
//! the bedrock type for the entire fleet.

use core::fmt;

/// The three possible states of a balanced ternary digit (trit).
///
/// `Negative` represents `-1`, `Neutral` represents `0`, and `Positive` represents `+1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Ternary {
    /// Represents the value `-1`.
    Negative,
    /// Represents the value `0` (the neutral / zero state).
    Neutral,
    /// Represents the value `+1`.
    Positive,
}

// ---------------------------------------------------------------------------
// Display
// ---------------------------------------------------------------------------

impl fmt::Display for Ternary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Negative => write!(f, "-1"),
            Self::Neutral  => write!(f, "0"),
            Self::Positive => write!(f, "+1"),
        }
    }
}

// ---------------------------------------------------------------------------
// From<i8>
// ---------------------------------------------------------------------------

impl TryFrom<i8> for Ternary {
    type Error = TernaryError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            -1 => Ok(Self::Negative),
            0  => Ok(Self::Neutral),
            1  => Ok(Self::Positive),
            _  => Err(TernaryError::InvalidConversion(value)),
        }
    }
}

// ---------------------------------------------------------------------------
// From<Ternary> for i8
// ---------------------------------------------------------------------------

impl From<Ternary> for i8 {
    fn from(value: Ternary) -> Self {
        match value {
            Ternary::Negative => -1,
            Ternary::Neutral  => 0,
            Ternary::Positive => 1,
        }
    }
}

// ---------------------------------------------------------------------------
// TryFrom<i32> / i64 / i16 / isize
// ---------------------------------------------------------------------------

macro_rules! impl_try_from_int {
    ($int:ty) => {
        impl TryFrom<$int> for Ternary {
            type Error = TernaryError;

            fn try_from(value: $int) -> Result<Self, Self::Error> {
                match value {
                    -1 => Ok(Ternary::Negative),
                    0  => Ok(Ternary::Neutral),
                    1  => Ok(Ternary::Positive),
                    n if n < -1 => Err(TernaryError::Overflow(n as i64)),
                    _           => Err(TernaryError::Overflow(value as i64)),
                }
            }
        }
    };
}

impl_try_from_int!(i16);
impl_try_from_int!(i32);
impl_try_from_int!(i64);
impl_try_from_int!(isize);

// ---------------------------------------------------------------------------
// Neg
// ---------------------------------------------------------------------------

impl core::ops::Neg for Ternary {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Negative => Self::Positive,
            Self::Neutral  => Self::Neutral,
            Self::Positive => Self::Negative,
        }
    }
}

// ---------------------------------------------------------------------------
// Arithmetic operations
// ---------------------------------------------------------------------------

/// Balanced ternary addition (mod 3).
///
/// | + | -1 | 0 | +1 |
/// |---|---|---|---|
/// | -1| +1 | -1 | 0 |
/// | 0 | -1 | 0 | +1 |
/// | +1| 0 | +1 | -1 |
impl core::ops::Add for Ternary {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        use Ternary::*;
        match (self, rhs) {
            (Negative, Negative) => Positive,  // -1 + -1 = +1 (mod 3)
            (Negative, Neutral)  => Negative,
            (Negative, Positive) => Neutral,
            (Neutral, Negative)  => Negative,
            (Neutral, Neutral)   => Neutral,
            (Neutral, Positive)  => Positive,
            (Positive, Negative) => Neutral,
            (Positive, Neutral)  => Positive,
            (Positive, Positive) => Negative,  // +1 + +1 = -1 (mod 3)
        }
    }
}

/// Balanced ternary subtraction (mod 3).
impl core::ops::Sub for Ternary {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + (-rhs)
    }
}

/// Balanced ternary multiplication.
///
/// | × | -1 | 0 | +1 |
/// |---|---|---|---|
/// | -1| +1 | 0 | -1 |
/// | 0 | 0 | 0 | 0 |
/// | +1| -1 | 0 | +1 |
impl core::ops::Mul for Ternary {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        use Ternary::*;
        match (self, rhs) {
            (Negative, Negative) => Positive,
            (Negative, Neutral)  => Neutral,
            (Negative, Positive) => Negative,
            (Neutral, _)         => Neutral,
            (Positive, Negative) => Negative,
            (Positive, Neutral)  => Neutral,
            (Positive, Positive) => Positive,
        }
    }
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Errors that can occur when converting into a [`Ternary`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TernaryError {
    /// The source integer is outside the valid range after narrowing.
    Overflow(i64),
    /// The source integer was a valid `i8`-range value that simply isn't `-1`,
    /// `0`, or `+1` (e.g. `42` or `-128`).
    InvalidConversion(i8),
}

impl fmt::Display for TernaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Overflow(v) => {
                write!(f, "ternary overflow: {v} cannot be narrowed to -1/0/1")
            }
            Self::InvalidConversion(v) => {
                write!(f, "invalid ternary conversion: {v} is not -1, 0, or 1")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TernaryError {}

// ---------------------------------------------------------------------------
// TernaryResult
// ---------------------------------------------------------------------------

/// A convenience alias for [`Result`] whose error is a [`TernaryError`].
pub type TernaryResult<T> = Result<T, TernaryError>;

// ---------------------------------------------------------------------------
// Iterator
// ---------------------------------------------------------------------------

/// An iterator that yields the three [`Ternary`] variants in order.
#[derive(Debug, Clone)]
pub struct TernaryIter {
    idx: u8,
}

impl Iterator for TernaryIter {
    type Item = Ternary;

    fn next(&mut self) -> Option<Self::Item> {
        use Ternary::*;
        let v = match self.idx {
            0 => Some(Negative),
            1 => Some(Neutral),
            2 => Some(Positive),
            _ => None,
        };
        self.idx = self.idx.saturating_add(1);
        v
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = 3usize.saturating_sub(self.idx as usize);
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for TernaryIter {}

impl Ternary {
    /// Iterate over all three [`Ternary`] variants in order.
    pub fn iter() -> TernaryIter {
        TernaryIter { idx: 0 }
    }

    /// Return the numeric value of this ternary state as an `i8`.
    pub fn to_i8(self) -> i8 {
        i8::from(self)
    }

    /// Return the numeric value as an `i32`.
    pub fn to_i32(self) -> i32 {
        i8::from(self) as i32
    }

    /// Return the numeric value as an `f64`.
    pub fn to_f64(self) -> f64 {
        i8::from(self) as f64
    }

    /// Convert from an `i8`, panicking on invalid input.
    ///
    /// Useful in const contexts where the value is guaranteed valid.
    pub fn from_i8(v: i8) -> Self {
        match v {
            -1 => Self::Negative,
            0  => Self::Neutral,
            1  => Self::Positive,
            _  => panic!("invalid trit value: {v}"),
        }
    }

    /// Is this trit non‑zero?
    pub fn is_nonzero(self) -> bool {
        self != Self::Neutral
    }

    /// Is this trit zero / neutral?
    pub fn is_zero(self) -> bool {
        self == Self::Neutral
    }

    /// Compute the signum: -1 → -1, 0 → 0, +1 → +1 (identity).
    pub fn signum(self) -> i8 {
        i8::from(self)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use Ternary::*;

    #[test]
    fn try_from_i8() {
        assert_eq!(Ternary::try_from(-1_i8), Ok(Negative));
        assert_eq!(Ternary::try_from(0_i8), Ok(Neutral));
        assert_eq!(Ternary::try_from(1_i8), Ok(Positive));
        assert_eq!(Ternary::try_from(42_i8), Err(TernaryError::InvalidConversion(42)));
    }

    #[test]
    fn into_i8() {
        assert_eq!(i8::from(Negative), -1);
        assert_eq!(i8::from(Neutral), 0);
        assert_eq!(i8::from(Positive), 1);
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", Negative), "-1");
        assert_eq!(format!("{}", Neutral), "0");
        assert_eq!(format!("{}", Positive), "+1");
    }

    #[test]
    fn neg() {
        assert_eq!(-Negative, Positive);
        assert_eq!(-Neutral, Neutral);
        assert_eq!(-Positive, Negative);
    }

    #[test]
    fn arithmetic_add() {
        assert_eq!(Negative + Negative, Positive);
        assert_eq!(Negative + Positive, Neutral);
        assert_eq!(Neutral + Positive, Positive);
        assert_eq!(Positive + Positive, Negative);
    }

    #[test]
    fn arithmetic_mul() {
        assert_eq!(Negative * Negative, Positive);
        assert_eq!(Negative * Positive, Negative);
        assert_eq!(Positive * Positive, Positive);
        assert_eq!(Negative * Neutral, Neutral);
    }

    #[test]
    fn arithmetic_sub() {
        // (-1) - (+1) = -2 ≡ +1 (mod 3)
        assert_eq!(Negative - Positive, Positive);
        // (+1) - (-1) = +2 ≡ -1 (mod 3)
        assert_eq!(Positive - Negative, Negative);
        // (+1) - (+1) = 0 (mod 3)
        assert_eq!(Positive - Positive, Neutral);
        assert_eq!(Neutral - Neutral, Neutral);
    }

    #[test]
    fn iter_all() {
        let collected: Vec<Ternary> = Ternary::iter().collect();
        assert_eq!(collected, vec![Negative, Neutral, Positive]);
    }

    #[test]
    fn to_i8_method() {
        assert_eq!(Negative.to_i8(), -1);
        assert_eq!(Neutral.to_i8(), 0);
        assert_eq!(Positive.to_i8(), 1);
    }

    #[test]
    fn is_nonzero() {
        assert!(Negative.is_nonzero());
        assert!(!Neutral.is_nonzero());
        assert!(Positive.is_nonzero());
    }

    #[test]
    fn try_from_wider_types() {
        assert_eq!(Ternary::try_from(0i32), Ok(Neutral));
        assert_eq!(Ternary::try_from(-1i64), Ok(Negative));
        assert_eq!(Ternary::try_from(1i16), Ok(Positive));
        assert_eq!(Ternary::try_from(999i32), Err(TernaryError::Overflow(999)));
        assert_eq!(Ternary::try_from(-1000i64), Err(TernaryError::Overflow(-1000)));
    }

    #[test]
    fn error_display() {
        let e = TernaryError::Overflow(999);
        assert_eq!(e.to_string(), "ternary overflow: 999 cannot be narrowed to -1/0/1");
        let e = TernaryError::InvalidConversion(42);
        assert_eq!(e.to_string(), "invalid ternary conversion: 42 is not -1, 0, or 1");
    }

    #[test]
    fn exact_size_iterator() {
        let it = Ternary::iter();
        assert_eq!(it.len(), 3);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        for t in [Negative, Neutral, Positive] {
            let json = serde_json::to_string(&t).unwrap();
            let back: Ternary = serde_json::from_str(&json).unwrap();
            assert_eq!(t, back);
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_json_exact() {
        assert_eq!(serde_json::to_string(&Negative).unwrap(), r#""Negative""#);
        assert_eq!(serde_json::to_string(&Neutral).unwrap(), r#""Neutral""#);
        assert_eq!(serde_json::to_string(&Positive).unwrap(), r#""Positive""#);
    }
}
