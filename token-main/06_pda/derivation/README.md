# PDA Derivation

Address derivation helpers for LetterP seed contracts.

## Implementation Source
- `ptoken-sdk/src/pda/derivation.rs`

## Contract Notes
- Seeds are byte slices and must stay stable.
- Bump handling should avoid temporary borrowed arrays.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
