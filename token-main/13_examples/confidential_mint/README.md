# Confidential Mint Example

Token-2022 confidential-transfer mint setup walkthrough.

## Implementation Source
- `examples/confidential_mint/src/lib.rs`

## Contract Notes
- Demonstrates extension-oriented account flow.
- Cryptographic inputs are deterministic test fixtures until the example is wired to real key material.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
