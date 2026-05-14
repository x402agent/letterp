# Account Info

Borrowing and account metadata helpers used before instruction logic touches account data.

## Implementation Source
- `ptoken-sdk/src/pinocchio_core/account_info.rs`

## Contract Notes
- Keep lamport/data borrows scoped tightly.
- Expose signer, writable, owner, and key checks without cloning more than necessary.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
