# Unit Tests

Pure Rust tests for math, parsing, and fixed-layout helpers.

## Implementation Source
- `ptoken-sdk/src/tests/unit.rs`

## Contract Notes
- Keep these fast and runtime-free.
- Mirror Kani invariants with concrete examples.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
