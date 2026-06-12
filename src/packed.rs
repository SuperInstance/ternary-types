//! Low-level packed trit representation using 2 bits per trit.
//!
//! A `u64` can hold 32 trits when packed densely.

use crate::Ternary;
use crate::Ternary::{Negative, Neutral, Positive};

/// Number of trits that fit in a single `u64`.
pub const TRITS_PER_U64: usize = 32;

/// A packed representation of trits stored in a `u64`.
///
/// Each trit uses 2 bits:
///
/// | Bits | Trit      |
/// |------|-----------|
/// | `00` | `Neutral` |
/// | `01` | `Positive` |
/// | `10` | `Negative` |
/// | `11` | (invalid) |
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PackedTrits(pub u64, pub u8); // (packed bits, count)

impl PackedTrits {
    /// Create a new packed trit set from a raw u64.
    ///
    /// `count` must not exceed `TRITS_PER_U64`.
    pub fn new(packed: u64, count: u8) -> Self {
        Self(packed, count.min(TRITS_PER_U64 as u8))
    }

    /// Pack up to 32 trits into a single `u64`.
    pub fn pack(trits: &[Ternary]) -> Self {
        let count = trits.len().min(TRITS_PER_U64);
        let mut packed: u64 = 0;
        for (i, t) in trits[..count].iter().enumerate() {
            let bits = encode_trit(*t);
            packed |= (bits as u64) << (i * 2);
        }
        Self(packed, count as u8)
    }

    /// Unpack back into a `Vec<Ternary>`.
    pub fn unpack(&self) -> Vec<Ternary> {
        let mut out = Vec::with_capacity(self.1 as usize);
        for i in 0..self.1 {
            let bits = (self.0 >> (i as usize * 2)) & 0b11;
            out.push(decode_trit(bits));
        }
        out
    }

    /// Access the trit at index `i`.
    pub fn get(&self, i: usize) -> Option<Ternary> {
        if i >= self.1 as usize {
            return None;
        }
        let bits = (self.0 >> (i * 2)) & 0b11;
        Some(decode_trit(bits))
    }

    /// Set the trit at index `i`.
    pub fn set(&mut self, i: usize, t: Ternary) {
        if i >= self.1 as usize {
            return;
        }
        let bits = encode_trit(t) as u64;
        let shift = i * 2;
        self.0 = (self.0 & !(0b11 << shift)) | (bits << shift);
    }

    /// The number of packed trits.
    pub fn len(&self) -> usize {
        self.1 as usize
    }

    /// Is this empty?
    pub fn is_empty(&self) -> bool {
        self.1 == 0
    }

    /// The raw packed bits.
    pub fn bits(&self) -> u64 {
        self.0
    }
}

/// Encode a trit to 2-bit representation.
#[inline]
fn encode_trit(t: Ternary) -> u8 {
    match t {
        Neutral  => 0b00,
        Positive => 0b01,
        Negative => 0b10,
    }
}

/// Decode 2 bits to a trit. Invalid patterns decode as `Neutral`.
#[inline]
fn decode_trit(bits: u64) -> Ternary {
    match bits {
        0b00 => Neutral,
        0b01 => Positive,
        0b10 => Negative,
        _    => Neutral, // 0b11 is invalid; silently correct
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let trits = vec![Negative, Neutral, Positive, Neutral, Negative];
        let packed = PackedTrits::pack(&trits);
        assert_eq!(packed.len(), 5);
        assert_eq!(packed.unpack(), trits);
    }

    #[test]
    fn get_set() {
        let mut packed = PackedTrits::pack(&[Neutral, Neutral, Neutral]);
        packed.set(0, Positive);
        packed.set(2, Negative);
        assert_eq!(packed.get(0), Some(Positive));
        assert_eq!(packed.get(1), Some(Neutral));
        assert_eq!(packed.get(2), Some(Negative));
    }

    #[test]
    fn max_trits() {
        let trits: Vec<Ternary> = (0..32).map(|i| {
            match i % 3 {
                0 => Negative,
                1 => Neutral,
                _ => Positive,
            }
        }).collect();
        let packed = PackedTrits::pack(&trits);
        assert_eq!(packed.len(), 32);
        assert_eq!(packed.unpack(), trits);
    }

    #[test]
    fn empty() {
        let p = PackedTrits::new(0, 0);
        assert!(p.is_empty());
        assert!(p.unpack().is_empty());
    }
}
