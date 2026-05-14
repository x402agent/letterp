# Interest Bearing

Interest-rate extension helpers for Token-2022 mints.

## Implementation Source
- `ptoken-sdk/src/extensions/interest_bearing.rs`

## Contract Notes
- Rates require authority-controlled updates.
- UI amount conversion must account for accrued interest outside base math helpers.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
