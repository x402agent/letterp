# Invoke Token-2022

Token-2022 CPI adapters.

## Implementation Source
- `ptoken-sdk/src/cpi/invoke_token_2022.rs`

## Contract Notes
- Token-2022 supports additional checked and extension-aware flows.
- Do not mix Tokenkeg and Token-2022 program IDs.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
