# Mint Close Authority

Token-2022 mint close-authority helpers.

## Implementation Source
- `ptoken-sdk/src/extensions/mint_close_authority.rs`

## Contract Notes
- Closing a mint is destructive and should be gated.
- Authority may be absent by design.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
