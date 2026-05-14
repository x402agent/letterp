# Instruction Data

Discriminant and fixed-field parsing for compact instruction payloads.

## Implementation Source
- `ptoken-sdk/src/pinocchio_core/instruction_data.rs`

## Contract Notes
- Reject short buffers before reading.
- Keep discriminant mapping local to each program or example.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
