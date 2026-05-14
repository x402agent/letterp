# Transfer

Transfer tokens from one account to another. Supports both direct owner
transfers and delegate-authorized transfers.

## Planned API
```rust
pub fn transfer(
    source: &AccountInfo,
    destination: &AccountInfo,
    authority: &AccountInfo,
    amount: u64,
) -> ProgramResult
```

> 🚧 Coming Soon
