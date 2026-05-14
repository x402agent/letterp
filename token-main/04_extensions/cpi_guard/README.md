# CPI Guard

Token-2022 CPI guard extension helpers.

## Implementation Source
- `ptoken-sdk/src/extensions/cpi_guard.rs`

## Contract Notes
- Use for accounts that should reject unexpected CPI paths.
- Document any instruction that intentionally requires CPI.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
