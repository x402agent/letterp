# Migration Guide

Migration notes from classic SPL Token flows to Token-2022 flows.

## Implementation Source
- `15_docs`

## Contract Notes
- Identify changed program IDs, account sizes, and instructions.
- Keep compatibility risks explicit.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
