# Mint Creation

Classic SPL Token mint allocation and initialization.

## Implementation Source
- `ptoken-sdk/src/token_classic/mint_creation.rs`

## Contract Notes
- Allocate rent-exempt space before init.
- Validate mint and authority accounts before CPI.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
