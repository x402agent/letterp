# Signer Checks

Signer and multisig threshold validation.

## Implementation Source
- `ptoken-sdk/src/validation/signer_checks.rs`

## Contract Notes
- Single-signer and multisig paths should be separate.
- Missing signer errors should be deterministic.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
