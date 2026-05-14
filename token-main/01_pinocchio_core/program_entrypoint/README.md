# Program Entrypoint

Lightweight dispatch pattern for Solana programs using this SDK.

## Implementation Source
- `ptoken-sdk/src/pinocchio_core/program_entrypoint.rs`

## Contract Notes
- Entry points should parse once and delegate.
- Business logic belongs in instruction modules, not the entrypoint.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
