# 12 - Constants

Program IDs, seed bytes, and SDK-level constants used by the modules.

## Code Map
- Primary source: `ptoken-sdk/src/constants`
- Crate entry: `ptoken-sdk/src/lib.rs`

## Local Rules
- Program IDs should be literal and reviewed before release.
- Seed constants are byte slices so PDA derivation does not allocate.
- Avoid duplicating IDs or seeds inside instruction modules.

## Review Checklist
- Keep account ordering explicit in docs and code.
- Prefer project errors over generic `ProgramError` until the Solana boundary.
- Update the matching example or test when behavior changes.
