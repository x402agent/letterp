# Metadata Pointer

Pointer to off-account metadata for Token-2022 mints.

## Implementation Source
- `ptoken-sdk/src/extensions/metadata_pointer.rs`

## Contract Notes
- Pointer and metadata address can differ.
- Validate metadata account owner when reading data.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
