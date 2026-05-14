# Group Pointer

Pointer into token group metadata.

## Implementation Source
- `ptoken-sdk/src/extensions/group_pointer.rs`

## Contract Notes
- Keep group address optional only where Token-2022 allows it.
- Changing pointers is authority-sensitive.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
