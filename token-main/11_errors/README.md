# 11 - Errors

SDK error enums for core token operations and Token-2022 extensions.

## Code Map
- Primary source: `ptoken-sdk/src/errors`
- Crate entry: `ptoken-sdk/src/lib.rs`

## Local Rules
- Errors are intentionally specific enough for tests and audits to identify failure causes.
- Program conversion uses custom error codes for Solana runtime compatibility.
- Do not collapse validation, authority, and arithmetic failures into one catch-all error.

## Review Checklist
- Keep account ordering explicit in docs and code.
- Prefer project errors over generic `ProgramError` until the Solana boundary.
- Update the matching example or test when behavior changes.
