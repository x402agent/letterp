# Token Errors

Error enum covering all core SPL Token and Token-2022 operation failures.

## Planned Error Variants
```rust
pub enum PTokenError {
    NotInitialized,
    AlreadyInitialized,
    InvalidMint,
    InvalidOwner,
    InvalidDelegate,
    InsufficientBalance,
    AccountFrozen,
    MintMismatch,
    OwnerMismatch,
    FixedSupply,
    ArithmeticOverflow,
}
```

> 🚧 Coming Soon
