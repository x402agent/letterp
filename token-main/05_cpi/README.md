# 05 - CPI Helpers

Cross-program invocation adapters for System, SPL Token, Token-2022, and Associated Token Account programs.

## Code Map
- Primary source: `ptoken-sdk/src/cpi`
- Crate entry: `ptoken-sdk/src/lib.rs`

## Local Rules
- CPI helpers should not make business decisions; they should assemble instructions and account arrays.
- Signer seeds must be passed only by callers that already validated the PDA authority.
- Prefer checked Token-2022 instructions where the token program exposes them.

## Review Checklist
- Keep account ordering explicit in docs and code.
- Prefer project errors over generic `ProgramError` until the Solana boundary.
- Update the matching example or test when behavior changes.
