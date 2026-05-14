# 10 - Validation

Reusable account, signer, owner, mint, and state checks for the token modules.

## Code Map
- Primary source: `ptoken-sdk/src/validation`
- Crate entry: `ptoken-sdk/src/lib.rs`

## Local Rules
- Validation functions should be pure checks with no CPI side effects.
- Use direct layout readers where SPL Token fixed offsets are sufficient.
- Return SDK errors that callers can convert into `ProgramError`.

## Review Checklist
- Keep account ordering explicit in docs and code.
- Prefer project errors over generic `ProgramError` until the Solana boundary.
- Update the matching example or test when behavior changes.
