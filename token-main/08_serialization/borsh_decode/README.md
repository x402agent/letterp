# Borsh Decode

Borsh payload decoding for structured instruction data.

## Implementation Source
- `ptoken-sdk/src/serialization/borsh_decode.rs`

## Contract Notes
- Reject trailing or missing fields according to caller policy.
- Map decode failures to SDK errors.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
