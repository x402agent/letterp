# Mint Creation

Initialize a new SPL Token mint account using Pinocchio CPI into the Token program.
Covers mint authority, freeze authority, and decimal configuration.

## Planned API
```rust
pub fn initialize_mint(
    mint: &AccountInfo,
    mint_authority: &Pubkey,
    freeze_authority: Option<&Pubkey>,
    decimals: u8,
) -> ProgramResult
```

> 🚧 Coming Soon
