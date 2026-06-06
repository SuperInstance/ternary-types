# ternary-types

*The foundational types for the entire ternary ecosystem. Trit, TritVec, TritMatrix, and the arithmetic that makes {-1, 0, +1} work correctly.*

## Why This Exists

Every ternary crate in the SuperInstance ecosystem — from neural network layers to distributed consensus — needs the same basic types and operations. This crate is where they live. The critical insight: ternary arithmetic is *not* just `(a + b + 3) % 3 - 1`. That formula gives wrong results for edge cases. Correct ternary addition requires explicit match arms on all 9 pairs of {-1, 0, +1}.

This crate exists because that bug was discovered the hard way, and it needs to live in exactly one place.

## The Bug That Started It

```rust
// WRONG — looks correct, isn't
fn trit_add(a: i8, b: i8) -> i8 {
    ((a + b + 3) % 3 - 1) as i8  // Fails: (1 + 1) = 2, (2+3)%3-1 = -1. WRONG.
}

// CORRECT — explicit match on all 9 cases
fn trit_add(a: Trit, b: Trit) -> Trit {
    match (a, b) {
        (-1, -1) => 1,   // -1 + -1 = -2, wraps to +1 in Z₃
        (-1,  0) => -1,
        (-1,  1) => 0,
        ( 0, -1) => -1,
        ( 0,  0) => 0,
        ( 0,  1) => 1,
        ( 1, -1) => 0,
        ( 1,  0) => 1,
        ( 1,  1) => -1,  // +1 + +1 = 2, wraps to -1 in Z₃
        _ => 0,
    }
}
```

## Types

- **`Trit`** — Type alias for `i8`, constrained to {-1, 0, +1}
- **`TritVec`** — Vector of trits with arithmetic operations (add, mul, dot product, XNOR+popcount)
- **`TritMatrix`** — 2D matrix of trits with matmul (XNOR-based), transpose
- **`trit_add / trit_mul`** — Correct Z₃ arithmetic via explicit match
- **`xnor_popcount`** — The core operation for ternary neural networks: count matching trits

## Usage

```rust
use ternary_types::*;

// Core arithmetic
assert_eq!(trit_add(1, 1), -1);  // Z₃: 1+1 = -1
assert_eq!(trit_mul(-1, 0), 0);

// Vector operations
let a = TritVec::from(vec![1, -1, 0, 1]);
let b = TritVec::from(vec![1, 1, -1, 1]);
let dot = a.dot(&b);  // XNOR + popcount based
let sum = a.add(&b);  // Element-wise Z₃ addition

// Matrix operations
let m = TritMatrix::from_rows(&[vec![1, -1], vec![0, 1]]);
let v = TritVec::from(vec![-1, 1]);
let result = m.mul_vec(&v);  // Ternary matmul
```

## Related Crates

- `ternary-core` — Higher-level algorithms built on these types
- `ternary-pack` — Packing trits into u32 for storage
- `ternary-matmul` — Optimized ternary matrix multiplication
- `ternary-conv` — Ternary convolution layers
