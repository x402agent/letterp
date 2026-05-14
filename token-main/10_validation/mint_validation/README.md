# Mint Validation

Validate mint account properties before operating on them.
Covers initialization status, decimals, supply, and authority checks.

## Planned API
```rust
pub fn assert_mint_initialized(mint: &AccountInfo) -> PTokenResult<()>
pub fn assert_mint_authority(mint: &AccountInfo, authority: &Pubkey) -> PTokenResult<()>
pub fn assert_decimals(mint: &AccountInfo, expected: u8) -> PTokenResult<()>
```

> 🚧 Coming Soon
