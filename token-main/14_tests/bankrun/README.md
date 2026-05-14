# Bankrun Tests

Runtime-style test plan for transaction flows.

## Implementation Source
- `ptoken-sdk/src/tests/bankrun.rs`

## Contract Notes
- Useful for CPI-heavy modules.
- Keep fixtures deterministic.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
