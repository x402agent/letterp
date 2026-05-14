# Invoke System Program

System Program CPI adapters.

## Implementation Source
- `ptoken-sdk/src/cpi/invoke_system_program.rs`

## Contract Notes
- Account creation, transfer, allocate, and assign calls require exact account ordering.
- Signed calls must receive caller-owned seeds.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
