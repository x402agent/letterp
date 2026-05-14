# Integration Tests

Instruction-level test plans for SDK flows.

## Implementation Source
- `ptoken-sdk/src/tests/integration.rs`

## Contract Notes
- Use Solana program-test once compile blockers are cleared.
- Cover authority and account-order failures.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
