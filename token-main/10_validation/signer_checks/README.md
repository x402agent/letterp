# Signer Checks

Assert that specific accounts in an instruction have signed the transaction.
Essential for authorization before any state-modifying operation.

## Planned API
```rust
pub fn assert_signer(account: &AccountInfo) -> PTokenResult<()>
pub fn assert_signers(accounts: &[&AccountInfo]) -> PTokenResult<()>
```

> 🚧 Coming Soon
