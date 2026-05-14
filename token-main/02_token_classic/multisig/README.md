# Multisig

Create and use M-of-N multisig authorities for mint, freeze, or transfer operations.
Supports up to 11 signers with a configurable threshold.

## Planned API
```rust
pub fn initialize_multisig(
    multisig: &AccountInfo,
    signers: &[&Pubkey],
    m: u8,
) -> ProgramResult
```

> 🚧 Coming Soon
