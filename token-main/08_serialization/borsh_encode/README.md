# Borsh Encode

Serialize Rust structs to bytes using the Borsh binary format.
Used for instruction data passed to pToken instructions.

## Planned Usage
```rust
let ix_data = BorshSerialize::try_to_vec(&MyInstruction { amount: 1000 })?;
```

> 🚧 Coming Soon
