# Transfer Fee

Token-2022 fee configuration and withdrawal helpers.

## Implementation Source
- `ptoken-sdk/src/extensions/transfer_fee.rs`

## Contract Notes
- Use basis points and max fee caps together.
- Math helpers prove cap and saturation behavior with Kani.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
