# u64 Helpers

Common range, clamp, basis-point, and ceil-division helpers.

## Implementation Source
- `ptoken-sdk/src/math/u64_helpers.rs`

## Contract Notes
- Ceil division avoids `a + b - 1` overflow.
- Max checks compare against mathematical u128 addition.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
