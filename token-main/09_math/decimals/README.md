# Decimals

Raw amount, UI amount, decimal multiplier, and fee helpers.

## Implementation Source
- `ptoken-sdk/src/math/decimals.rs`

## Contract Notes
- Decimal multipliers saturate above u64 precision.
- Fee calculations use u128 intermediates.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
