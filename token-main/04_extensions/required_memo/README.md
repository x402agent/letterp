# Required Memo

Token-2022 required memo transfer behavior.

## Implementation Source
- `ptoken-sdk/src/extensions/required_memo.rs`

## Contract Notes
- Transfer flows must include memo instruction where required.
- Tests should cover missing-memo rejection.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
