# Freeze and Thaw

Classic SPL Token freeze-authority helpers.

## Implementation Source
- `ptoken-sdk/src/token_classic/freeze_thaw.rs`

## Contract Notes
- Freeze authority comes from mint configuration.
- Frozen accounts must fail transfer validation.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
