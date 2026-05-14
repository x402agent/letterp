# Mint With Extensions

Token-2022 mint creation with extension space planning.

## Implementation Source
- `ptoken-sdk/src/token_2022/mint_with_extensions.rs`

## Contract Notes
- Calculate extension space before allocation.
- Initialize extensions before mint init when required.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
