# Close Account

Close a token account with zero balance, returning the rent lamports
to a destination account.

## Planned API
```rust
pub fn close_account(
    account: &AccountInfo,
    destination: &AccountInfo,
    owner: &AccountInfo,
) -> ProgramResult
```

> 🚧 Coming Soon
