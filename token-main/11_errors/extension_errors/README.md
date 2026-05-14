# Extension Errors

Extension-specific errors for Token-2022 features.

## Implementation Source
- `ptoken-sdk/src/errors/extension_errors.rs`

## Contract Notes
- Use when the extension layer can fail before CPI.
- Avoid masking extension config mistakes as generic invalid data.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
