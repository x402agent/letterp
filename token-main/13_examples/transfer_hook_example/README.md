# Transfer Hook Example

Token-2022 transfer hook configuration and invocation sketch.

## Implementation Source
- `examples/transfer_hook_example/src/lib.rs`

## Contract Notes
- Hook program behavior should be deterministic.
- Log-only hooks are examples, not policy enforcement.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
