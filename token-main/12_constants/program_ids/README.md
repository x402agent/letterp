# Program IDs

Canonical program IDs used by the SDK.

## Implementation Source
- `ptoken-sdk/src/constants/program_ids.rs`

## Contract Notes
- Tokenkeg, Token-2022, ATA, System, and other IDs are centralized.
- Review literals before deploy.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
