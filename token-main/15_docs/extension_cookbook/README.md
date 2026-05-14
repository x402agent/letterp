# Extension Cookbook

Recipes for combining Token-2022 extensions safely.

## Implementation Source
- `15_docs`

## Contract Notes
- Show compatible combinations first.
- Call out authority conflicts and account sizing.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
