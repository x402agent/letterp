# Idempotent ATA Create

Replay-safe ATA creation helper.

## Implementation Source
- `ptoken-sdk/src/associated_token/idempotent_create.rs`

## Contract Notes
- Safe when account may already exist.
- Still validate owner and mint after creation.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
