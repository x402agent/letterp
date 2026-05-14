# Checked Arithmetic

Safe add, sub, mul, div, and muldiv operations.

## Implementation Source
- `ptoken-sdk/src/math/checked_arithmetic.rs`

## Contract Notes
- Overflow and underflow return SDK errors.
- Kani covers success and failure paths.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
