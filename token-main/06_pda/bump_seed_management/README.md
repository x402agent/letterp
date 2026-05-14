# Bump Seed Management

Helpers for storing and reusing PDA bumps.

## Implementation Source
- `ptoken-sdk/src/pda/bump_seed_management.rs`

## Contract Notes
- Persist bump when the account must sign later.
- Never brute-force bumps inside repeated hot paths.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
