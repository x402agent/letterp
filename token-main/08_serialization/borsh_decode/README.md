# Borsh Decode

Deserialize raw byte slices into typed Rust structs using Borsh.
Used for reading account data and instruction arguments.

## Planned Usage
```rust
let state = MyAccountState::try_from_slice(&account.data.borrow())?;
```

> 🚧 Coming Soon
