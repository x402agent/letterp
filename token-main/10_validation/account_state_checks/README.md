# Account State Checks

Token account initialized, frozen, balance, and zero-balance checks.

## Implementation Source
- `ptoken-sdk/src/validation/account_state_checks.rs`

## Contract Notes
- Read SPL state from fixed offsets.
- Close paths must require zero balance.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
