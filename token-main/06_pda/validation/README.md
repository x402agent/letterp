# PDA Validation

Compare provided accounts against derived PDAs.

## Implementation Source
- `ptoken-sdk/src/pda/validation.rs`

## Contract Notes
- Fail before state mutation.
- Return SDK errors with enough context for tests.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
