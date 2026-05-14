# Token Account

Create and initialize a token account to hold a specific mint.
Supports both explicit account creation and ATA-style derivation.

## Planned API
```rust
pub fn initialize_account(
    account: &AccountInfo,
    mint: &AccountInfo,
    owner: &Pubkey,
) -> ProgramResult
```

> 🚧 Coming Soon
