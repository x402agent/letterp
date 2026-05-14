# Revoke

Remove any previously approved delegate authority from a token account.
After revocation, only the owner can transfer tokens.

## Planned API
```rust
pub fn revoke(
    source: &AccountInfo,
    owner: &AccountInfo,
) -> ProgramResult
```

> 🚧 Coming Soon
