# Token-2022 Reference

Reference notes for Token-2022 base and extension flows.

## Implementation Source
- `15_docs`

## Contract Notes
- Document extension initialization ordering.
- Call out reallocation and rent changes.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
