# Token-2022 Close Account

Close behavior for Token-2022 accounts.

## Implementation Source
- `ptoken-sdk/src/token_2022/close_account_2022.rs`

## Contract Notes
- Extension state may add pre-close requirements.
- Keep lamport destination explicit.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
