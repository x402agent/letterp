# Account State Checks

Validate token account state before transfer, burn, or close operations.
Checks initialization, frozen status, delegate, and balance.

## Planned API
```rust
pub fn assert_account_initialized(account: &AccountInfo) -> PTokenResult<()>
pub fn assert_account_not_frozen(account: &AccountInfo) -> PTokenResult<()>
pub fn assert_sufficient_balance(account: &AccountInfo, amount: u64) -> PTokenResult<()>
```

> 🚧 Coming Soon
