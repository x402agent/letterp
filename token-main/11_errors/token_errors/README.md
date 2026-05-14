# Token Errors

Core SDK error enum for SPL Token style failures.

## Implementation Source
- `ptoken-sdk/src/errors/token_errors.rs`

## Contract Notes
- Keep variants stable for tests.
- Convert to ProgramError at the boundary.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
