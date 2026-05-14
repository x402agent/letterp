# Pack/Unpack

SPL Pack trait helpers for account state types.

## Implementation Source
- `ptoken-sdk/src/serialization/pack_unpack.rs`

## Contract Notes
- Import Pack wherever associated LEN is used.
- Keep unpack errors visible to callers.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
