# Confidential Transfer

Token-2022 confidential-transfer configuration helpers.

## Implementation Source
- `ptoken-sdk/src/extensions/confidential_transfer.rs`

## Contract Notes
- Treat cryptographic public keys as typed data, not raw strings.
- Audit authority and auditor key updates separately.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
