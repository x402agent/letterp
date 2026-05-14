# Basic Mint Example

Classic mint, ATA, mint-to, transfer, and burn walkthrough.

## Implementation Source
- `examples/basic_mint/src/lib.rs`

## Contract Notes
- Uses local discriminants for simple dispatch.
- Should be run after SDK compile blockers are resolved.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
