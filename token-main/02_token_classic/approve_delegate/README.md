# Approve Delegate

Classic SPL Token delegate allowance setup.

## Implementation Source
- `ptoken-sdk/src/token_classic/approve_delegate.rs`

## Contract Notes
- Allowance is raw base units.
- Delegate changes should be explicit and auditable.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
