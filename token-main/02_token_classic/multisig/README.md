# Multisig

Classic SPL Token multisig account creation and authority use.

## Implementation Source
- `ptoken-sdk/src/token_classic/multisig.rs`

## Contract Notes
- Threshold and signer count must be checked.
- Signer account order matters for SPL Token.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
