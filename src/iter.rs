//! Iterator utilities for trit sequences.

use crate::Ternary;

/// An iterator adapter that yields trits from a byte slice.
///
/// Each byte in the source is mapped:
/// - bytes < 85 → Negative
/// - bytes in [85, 170] → Neutral
/// - bytes > 170 → Positive
pub struct TritIterator<'a> {
    bytes: core::slice::Iter<'a, u8>,
}

impl<'a> TritIterator<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes: bytes.iter() }
    }
}

impl<'a> Iterator for TritIterator<'a> {
    type Item = Ternary;

    fn next(&mut self) -> Option<Self::Item> {
        self.bytes.next().map(|&b| {
            if b < 85 {
                Ternary::Negative
            } else if b > 170 {
                Ternary::Positive
            } else {
                Ternary::Neutral
            }
        })
    }
}

/// Convert an iterator of `Ternary` back into bytes (for serialization).
pub fn trits_to_bytes<I>(trits: I) -> Vec<u8>
where
    I: IntoIterator<Item = Ternary>,
{
    trits.into_iter().map(|t| match t {
        Ternary::Negative => 0u8,
        Ternary::Neutral => 128u8,
        Ternary::Positive => 255u8,
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_mapping() {
        let bytes = vec![0u8, 128u8, 200u8, 85u8];
        let trits: Vec<Ternary> = TritIterator::new(&bytes).collect();
        assert_eq!(trits[0], Ternary::Negative);
        assert_eq!(trits[1], Ternary::Neutral);
        assert_eq!(trits[2], Ternary::Positive);
        assert_eq!(trits[3], Ternary::Neutral);
    }

    #[test]
    fn roundtrip() {
        let trits = vec![Ternary::Negative, Ternary::Neutral, Ternary::Positive];
        let bytes = trits_to_bytes(trits.clone());
        let back: Vec<Ternary> = TritIterator::new(&bytes).collect();
        assert_eq!(trits, back);
    }
}
