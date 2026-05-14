# 13 - Examples

Small programs that demonstrate how LetterP token helpers are expected to be called.

## Code Map
- Primary source: `examples`
- Crate entry: `ptoken-sdk/src/lib.rs`

## Local Rules
- Examples are not marketing demos; they are executable integration sketches.
- Each example keeps instruction discriminants local and simple.
- When the SDK API changes, examples should be updated in the same patch.

## Review Checklist
- Keep account ordering explicit in docs and code.
- Prefer project errors over generic `ProgramError` until the Solana boundary.
- Update the matching example or test when behavior changes.
