# Pinocchio Guide

Internal guide for the explicit runtime style used in this repo.

## Implementation Source
- `15_docs`

## Contract Notes
- Explain account order and discriminant parsing.
- Avoid framework comparisons unless they clarify migration risk.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
