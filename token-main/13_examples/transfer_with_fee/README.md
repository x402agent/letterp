# Transfer With Fee Example

Token-2022 transfer-fee configuration and transfer sketch.

## Implementation Source
- `examples/transfer_with_fee/src/lib.rs`

## Contract Notes
- Basis-point calculations are backed by math helpers.
- Fee withdrawal should be tested separately.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
