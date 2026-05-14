# Owner Checks

Program and account-owner validation helpers.

## Implementation Source
- `ptoken-sdk/src/validation/owner_checks.rs`

## Contract Notes
- Distinguish token account owner from Solana account owner.
- Owner mismatch should be explicit.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
