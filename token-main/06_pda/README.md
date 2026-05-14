# 06 - PDA Utilities

Program-derived-address helpers for deterministic account derivation and bump handling.

## Code Map
- Primary source: `ptoken-sdk/src/pda`
- Crate entry: `ptoken-sdk/src/lib.rs`

## Local Rules
- Seed bytes are centralized in `ptoken-sdk/src/constants/seeds.rs`.
- Validation should compare derived addresses to provided accounts before state mutation.
- Bump seeds are part of the account contract and should be persisted when replay is required.

## Review Checklist
- Keep account ordering explicit in docs and code.
- Prefer project errors over generic `ProgramError` until the Solana boundary.
- Update the matching example or test when behavior changes.
