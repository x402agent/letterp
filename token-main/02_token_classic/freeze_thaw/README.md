# Freeze & Thaw

Freeze a token account to prevent transfers, or thaw it to re-enable them.
Requires the mint's freeze authority.

## Planned API
```rust
pub fn freeze_account(account: &AccountInfo, mint: &AccountInfo, authority: &AccountInfo) -> ProgramResult
pub fn thaw_account(account: &AccountInfo, mint: &AccountInfo, authority: &AccountInfo) -> ProgramResult
```

> 🚧 Coming Soon
