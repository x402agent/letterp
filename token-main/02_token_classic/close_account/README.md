# Close Account

Classic SPL Token close-account flow.

## Implementation Source
- `ptoken-sdk/src/token_classic/close_account.rs`

## Contract Notes
- Require zero balance before close.
- Destination receives remaining lamports.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
