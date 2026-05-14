# Token With Metadata Example

Token-2022 embedded metadata mint walkthrough.

## Implementation Source
- `examples/token_with_metadata/src/lib.rs`

## Contract Notes
- Metadata strings should use LetterP-specific names.
- Update metadata authority paths are explicit.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
