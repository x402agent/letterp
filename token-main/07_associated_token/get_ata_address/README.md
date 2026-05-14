# Get ATA Address

Deterministic ATA derivation.

## Implementation Source
- `ptoken-sdk/src/associated_token/get_ata_address.rs`

## Contract Notes
- Derivation is pure and side-effect free.
- Use this before create/idempotent-create flows.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
