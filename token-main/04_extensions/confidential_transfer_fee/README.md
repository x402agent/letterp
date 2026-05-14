# Confidential Transfer Fee

Withheld-fee configuration for confidential transfers.

## Implementation Source
- `ptoken-sdk/src/extensions/confidential_transfer_fee.rs`

## Contract Notes
- Fee authority and withdraw authority are independent.
- Use typed ElGamal public keys.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
