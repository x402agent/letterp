# Transfer Hook

Token-2022 transfer hook configuration.

## Implementation Source
- `ptoken-sdk/src/extensions/transfer_hook.rs`

## Contract Notes
- Hook program ID must be reviewed as part of mint config.
- Hook account metas must be deterministic.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
