# Classic Token Reference

Reference notes for SPL Token classic flows used by LetterP.

## Implementation Source
- `15_docs`

## Contract Notes
- Focus on instructions wrapped by this SDK.
- Call out required accounts and authorities.

## Audit Hooks
- Check signer, owner, and writable requirements before CPI or state mutation.
- Add or update unit tests for pure logic and integration tests for account flow.
- If arithmetic is involved, mirror the invariant in `ptoken-sdk/src/kani_verification.rs`.
