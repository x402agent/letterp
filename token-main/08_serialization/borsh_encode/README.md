# Borsh Encode

Borsh payload encoding for structured instruction data.

## Implementation Source
- `ptoken-sdk/src/serialization/borsh_encode.rs`

## Contract Notes
- Use only for schemas that are part of the contract.
- Keep primitive fixed-width helpers for hot paths.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
