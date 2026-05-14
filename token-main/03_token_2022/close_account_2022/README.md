# Close Account 2022

Close Token-2022 token accounts, handling any extension cleanup required.
Some extensions (e.g. CPI Guard) impose restrictions on closing.

## Planned API
```rust
pub fn close_account_2022(
    account: &AccountInfo,
    destination: &AccountInfo,
    owner: &AccountInfo,
) -> ProgramResult
```

> 🚧 Coming Soon
