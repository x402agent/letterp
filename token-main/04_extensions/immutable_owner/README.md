# Immutable Owner

Token-2022 extension that prevents owner reassignment.

## Implementation Source
- `ptoken-sdk/src/extensions/immutable_owner.rs`

## Contract Notes
- Initialize before account initialization.
- Useful for ATAs and custody accounts.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
