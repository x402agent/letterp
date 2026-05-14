# Mint Validation

Mint account length, initialization, and decimal checks.

## Implementation Source
- `ptoken-sdk/src/validation/mint_validation.rs`

## Contract Notes
- Validate mint length before reading offsets.
- Decimals should match instruction expectations.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
