# Burn

Permanently destroy tokens, reducing the mint's total supply.
Requires the token account owner or an approved delegate.

## Planned API
```rust
pub fn burn(
    account: &AccountInfo,
    mint: &AccountInfo,
    authority: &AccountInfo,
    amount: u64,
) -> ProgramResult
```

> 🚧 Coming Soon
