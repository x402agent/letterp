# Permanent Delegate

Token-2022 permanent delegate configuration.

## Implementation Source
- `ptoken-sdk/src/extensions/permanent_delegate.rs`

## Contract Notes
- Permanent delegate bypasses normal approval lifecycle.
- Use only when recovery or compliance flows require it.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
