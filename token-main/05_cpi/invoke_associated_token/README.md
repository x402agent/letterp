# Invoke Associated Token

Associated Token Account CPI adapters.

## Implementation Source
- `ptoken-sdk/src/cpi/invoke_associated_token.rs`

## Contract Notes
- Use idempotent creation when replays are possible.
- Wallet, mint, and token program ID define the ATA.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
