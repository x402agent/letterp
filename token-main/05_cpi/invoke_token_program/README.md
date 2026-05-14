# Invoke SPL Token

Classic SPL Token CPI adapters.

## Implementation Source
- `ptoken-sdk/src/cpi/invoke_token_program.rs`

## Contract Notes
- Keep wrappers close to SPL instruction signatures.
- Prefer checked variants where available.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
