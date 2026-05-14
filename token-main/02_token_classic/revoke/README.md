# Revoke Delegate

Classic SPL Token delegate removal.

## Implementation Source
- `ptoken-sdk/src/token_classic/revoke.rs`

## Contract Notes
- Revocation should require owner authority.
- Post-revoke tests should reject delegate transfers.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
