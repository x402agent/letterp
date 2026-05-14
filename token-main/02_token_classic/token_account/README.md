# Token Account

Classic SPL Token account creation, initialization, and state operations.

## Implementation Source
- `ptoken-sdk/src/token_classic/token_account.rs`

## Contract Notes
- Use SPL account length from Pack once compile imports are fixed.
- Separate account setup from transfers.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
