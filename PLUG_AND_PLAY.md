# PLUG_AND_PLAY — Types

> Lightweight balanced ternary enum: Negative, Neutral, Positive

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
ternary-types = { git = "https://github.com/SuperInstance/ternary-types" }
```

Use in your code:

```rust
use ternary_types::Ternary;

let t = Ternary::Positive;
assert_eq!(i8::from(t), 1);
assert_eq!(Ternary::try_from(0_i8).unwrap(), Ternary::Neutral);
```

## 📚 Available Documentation

| Document | Description |
|----------|-------------|
| `docs/FROM_BINARY.md` | Understanding ternary concepts as a binary programmer |
| `docs/MIGRATION.md` | Version migration guide |
| `docs/FUTURE-INTEGRATION.md` | Planned features and roadmap |

## 🔗 Integration

This crate is part of the [SuperInstance ternary fleet](https://github.com/SuperInstance). It uses the canonical `Ternary` type from `ternary-types` for cross-crate compatibility.

## 📄 License

MIT
