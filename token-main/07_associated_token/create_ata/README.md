# Create ATA

Associated Token Account creation helper.

## Implementation Source
- `ptoken-sdk/src/associated_token/create_ata.rs`

## Contract Notes
- Create with explicit payer and wallet.
- Use the expected token program ID for classic vs Token-2022.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
