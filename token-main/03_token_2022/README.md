# 03 - Token-2022 Base

Base Token-2022 operations that are shared by extension-enabled mints and accounts.

## Code Map
- Primary source: `ptoken-sdk/src/token_2022`
- Crate entry: `ptoken-sdk/src/lib.rs`

## Local Rules
- Extension space must be calculated before account creation.
- Reallocation must preserve owner, signer, and rent requirements.
- Close-account behavior follows Token-2022 rules, including extension-specific constraints.

## Review Checklist
- Keep account ordering explicit in docs and code.
- Prefer project errors over generic `ProgramError` until the Solana boundary.
- Update the matching example or test when behavior changes.
