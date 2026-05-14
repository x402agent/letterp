# Burn

Classic SPL Token burn helper surface.

## Implementation Source
- `ptoken-sdk/src/token_classic/burn.rs`

## Contract Notes
- Burn authority must be owner or approved delegate.
- Check available balance before invoking token program.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
