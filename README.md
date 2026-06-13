# Ternary Types

A **ternary type** is a lightweight, dependency-free Rust enum representing the three balanced-ternary states: `Negative (−1)`, `Neutral (0)`, and `Positive (+1)`. This crate provides the foundational type used across the SuperInstance ecosystem, with optional `serde` serialization and comprehensive conversion traits.

## Why It Matters

Every ternary crate in SuperInstance needs a common type for trit values. Without a shared definition, each crate reimplements the enum, leading to incompatibilities and conversion boilerplate. `ternary-types` is that shared definition — a single, minimal crate that depends on nothing (not even `serde` by default). It provides the three-state enum, conversions to/from all common integer types, negation, display, and error handling. The `#[no_std]` support makes it usable in embedded contexts and WASM, and the optional `serde` feature allows wire serialization when needed. This crate is the type-theoretic foundation stone of the entire ternary stack.

## How It Works

### The Ternary Enum

```rust
pub enum Ternary {
    Negative,  // -1
    Neutral,   //  0
    Positive,  // +1
}
```

### Conversion Table

| From → To | `Ternary` | `i8` |
|-----------|----------|------|
| `Negative` | `Ternary::Negative` | `−1` |
| `Neutral` | `Ternary::Neutral` | `0` |
| `Positive` | `Ternary::Positive` | `+1` |

### Trait Implementations

- **`From<Ternary>` for `i8`**: Infallible — every `Ternary` maps to exactly one `i8`.
- **`TryFrom<i8>` for `Ternary`**: Fails for any value not in {−1, 0, 1}. Returns `TernaryError::InvalidConversion(value)`.
- **`TryFrom<i16/i32/i64/isize>`**: Guards against overflow before narrowing to `i8`.
- **`Neg`**: `−Negative = Positive`, `−Positive = Negative`, `−Neutral = Neutral`.
- **`Display`**: `"−1"`, `"0"`, `"+1"`.

### Derives

`Ternary` derives: `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`. The ordering is `Negative < Neutral < Positive`, consistent with the integer mapping.

### Serde Support

Behind the `serde` feature flag:
```toml
[dependencies]
ternary-types = { version = "0.1", features = ["serde"] }
```

Serializes as the string representation: `"−1"`, `"0"`, `"+1"`. Or optionally as integers via custom serializers.

### Complexity

All operations are O(1). `Copy` makes passing trivially cheap (1 byte).

## Quick Start

```rust
use ternary_types::Ternary;

fn main() {
    let a = Ternary::Positive;
    let b = Ternary::Negative;

    // Convert to integer
    assert_eq!(i8::from(a), 1);
    assert_eq!(i8::from(b), -1);

    // Convert from integer
    let c = Ternary::try_from(0_i8).unwrap();
    assert_eq!(c, Ternary::Neutral);

    // Negation
    assert_eq!(-a, Ternary::Negative);
    assert_eq!(-b, Ternary::Positive);
    assert_eq!(-Ternary::Neutral, Ternary::Neutral);

    // Ordering
    assert!(Ternary::Negative < Ternary::Neutral);
    assert!(Ternary::Neutral < Ternary::Positive);

    // Display
    println!("{}", a);  // "+1"
    println!("{}", b);  // "-1"

    // Error on invalid conversion
    assert!(Ternary::try_from(5_i8).is_err());
}
```

```bash
cargo build
cargo test
cargo build --features serde
```

## API

| Item | Kind | Description |
|------|------|-------------|
| `Ternary` | enum | `Negative`, `Neutral`, `Positive` |
| `TernaryError` | enum | `InvalidConversion(i8)`, `Overflow(i64)` |
| `From<Ternary>` for `i8` | trait impl | Infallible conversion |
| `TryFrom<i8/i16/i32/i64/isize>` | trait impl | Fallible conversion |
| `Neg` for `Ternary` | trait impl | Negation |
| `Display` for `Ternary` | trait impl | `"-1"`, `"0"`, `"+1"` |
| `serde::{Serialize, Deserialize}` | feature | Optional, behind `"serde"` flag |

## Architecture Notes

`ternary-types` defines the fundamental value domain of γ + η = C: `{−1, 0, +1} = {η, neutral, γ}`. Every other ternary crate depends on this definition. The type is deliberately minimal — no arithmetic, no collections, no algorithms — because those belong in separate crates. This separation ensures that the type definition is stable and universal. See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

1. Knuth, D. E. (1981). *TAOCP Vol. 2*, Section 4.1. — Balanced ternary number system.
2. Rust Reference. "Enumerations." — The `enum` construct and derive macros.
3. Serde. (2024). *Serde JSON*. — Serialization framework for Rust.

## License

MIT
