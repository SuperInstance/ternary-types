//! # ternary-types
//!
//! **Central dependency hub for the SuperInstance ternary fleet.**
//!
//! Core types shared across all 24+ ternary crates in the math stack:
//!
//! | Type | Description |
//! |------|-------------|
//! | [`Ternary`] | The three balanced states `{-1, 0, +1}` |
//! | [`TritVector`] | Compact packed vector of trits (2 bits/trit) |
//! | [`TernaryMatrix`] | 2D matrix of trits |
//! | [`PackedTrits`] | Low-level packed trit representation |
//!
//! Feature gates:
//! - `serde` — `Serialize`/`Deserialize` for `Ternary`
//! - `std` — `std::error::Error` impls (on by default)
//! - `packed` — Unsafe bit-level packed operations

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "packed", feature(core_intrinsics))]

#[cfg(feature = "std")]
extern crate std;

mod ternary;
mod trit_vector;
mod packed;
mod matrix;
mod convert;
mod iter;

pub use ternary::{Ternary, TernaryError, TernaryResult, TernaryIter};
pub use trit_vector::TritVector;
pub use packed::PackedTrits;
pub use matrix::TernaryMatrix;
pub use iter::TritIterator;

/// Pre-defined trit values for convenience.
pub mod trits {
    pub use crate::Ternary::{Negative, Neutral, Positive};

    /// Short alias for `Ternary::Negative`
    pub const N: super::Ternary = super::Ternary::Negative;
    /// Short alias for `Ternary::Neutral`
    pub const Z: super::Ternary = super::Ternary::Neutral;
    /// Short alias for `Ternary::Positive`
    pub const P: super::Ternary = super::Ternary::Positive;
}

/// The ring of integers modulo 3 (balanced), i.e. `{-1, 0, +1}`.
///
/// This is the underlying algebraic structure for all ternary operations.
pub mod z3 {
    pub use crate::Ternary::{Negative, Neutral, Positive};
}

/// Re-export the core `Ternary` at crate root for ergonomics.
pub use Ternary::{Negative, Neutral, Positive};
