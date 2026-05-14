# Zero-Copy Layout

Fixed-offset readers for SPL Token mint and account data.

## Implementation Source
- `ptoken-sdk/src/pinocchio_core/zero_copy_layout.rs`

## Contract Notes
- Offsets must match SPL Token layout.
- Readers return errors instead of slicing blindly.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
