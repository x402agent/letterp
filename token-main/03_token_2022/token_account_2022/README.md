# Token-2022 Account

Token-2022 account creation and initialization.

## Implementation Source
- `ptoken-sdk/src/token_2022/token_account_2022.rs`

## Contract Notes
- Account size depends on enabled extensions.
- Use Token-2022 program ID consistently.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
