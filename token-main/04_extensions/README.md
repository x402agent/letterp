# 04 - Token-2022 Extensions

Extension-specific constructors and update helpers for Token-2022 mints and accounts.

## Code Map
- Primary source: `ptoken-sdk/src/extensions`
- Crate entry: `ptoken-sdk/src/lib.rs`

## Local Rules
- Each extension README documents the authority, account layout, and CPI risk for that feature.
- Extension initialization should happen before mint initialization unless SPL docs require otherwise.
- Combinations must be reviewed for conflicting authorities and account-size changes.

## Review Checklist
- Keep account ordering explicit in docs and code.
- Prefer project errors over generic `ProgramError` until the Solana boundary.
- Update the matching example or test when behavior changes.
