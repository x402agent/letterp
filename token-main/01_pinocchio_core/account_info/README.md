# Account Info

Raw AccountInfo struct access using Pinocchio's account model.
Provides helpers for reading key, lamports, data, owner, and executable flag
directly from the account's memory-mapped byte slice.

## Planned API
```rust
pub fn get_account_data(account: &AccountInfo) -> &[u8]
pub fn get_account_lamports(account: &AccountInfo) -> u64
pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> PTokenResult<()>
```

> 🚧 Coming Soon
