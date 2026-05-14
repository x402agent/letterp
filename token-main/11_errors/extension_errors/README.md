# Extension Errors

Error variants specific to Token-2022 extension operations.

## Planned Error Variants
```rust
pub enum ExtensionError {
    ExtensionNotFound,
    ExtensionAlreadyInitialized,
    InvalidExtensionType,
    InvalidFeeConfig,
    ConfidentialTransferDisabled,
    TransferHookFailed,
    MemoRequired,
    CpiGuardViolation,
    ImmutableOwnerViolation,
    NonTransferable,
}
```

> 🚧 Coming Soon
