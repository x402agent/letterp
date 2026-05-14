# Syscalls

Thin runtime wrappers for logging and invocation boundaries.

## Implementation Source
- `ptoken-sdk/src/pinocchio_core/syscalls.rs`

## Contract Notes
- Keep wrappers boring and direct.
- Do not hide signer-seed or account-order requirements.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
