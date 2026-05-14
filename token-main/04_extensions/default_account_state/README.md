# Default Account State

Token-2022 default account-state extension helpers.

## Implementation Source
- `ptoken-sdk/src/extensions/default_account_state.rs`

## Contract Notes
- Frozen defaults are useful for gated activation.
- State transitions require the configured authority.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
