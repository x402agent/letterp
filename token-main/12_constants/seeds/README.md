# Seeds

PDA seed byte constants.

## Implementation Source
- `ptoken-sdk/src/constants/seeds.rs`

## Contract Notes
- Changing a seed changes addresses.
- Document migrations for any seed update.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
