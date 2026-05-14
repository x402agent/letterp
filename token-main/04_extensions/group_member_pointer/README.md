# Group Member Pointer

Pointer from a mint/account into group membership metadata.

## Implementation Source
- `ptoken-sdk/src/extensions/group_member_pointer.rs`

## Contract Notes
- Pointer authority must be explicit.
- Do not assume metadata account ownership without validation.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
