# Reallocate

Token-2022 account reallocation helpers.

## Implementation Source
- `ptoken-sdk/src/token_2022/reallocate.rs`

## Contract Notes
- Check owner and signer before realloc.
- Rent must be recalculated after size changes.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
