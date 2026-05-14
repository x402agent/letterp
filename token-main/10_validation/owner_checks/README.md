# Owner Checks

Assert that an account is owned by a specific program. Prevents spoofing
attacks where malicious accounts mimic legitimate ones.

## Planned API
```rust
pub fn assert_owned_by_token_program(account: &AccountInfo) -> PTokenResult<()>
pub fn assert_owned_by_token_2022(account: &AccountInfo) -> PTokenResult<()>
pub fn assert_owned_by(account: &AccountInfo, program: &Pubkey) -> PTokenResult<()>
```

> 🚧 Coming Soon
