# Transfer

Classic SPL Token transfer helper surface.

## Implementation Source
- `ptoken-sdk/src/token_classic/transfer.rs`

## Contract Notes
- Transfer amount is raw base units.
- Validate mint and owner relationships before CPI.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
