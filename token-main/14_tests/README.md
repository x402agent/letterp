# 14 - Tests

Unit, integration, and runtime-oriented test documentation for the SDK.

## Code Map
- Primary source: `ptoken-sdk/src/tests`
- Crate entry: `ptoken-sdk/src/lib.rs`

## Local Rules
- Unit tests cover pure math and byte parsing without Solana runtime setup.
- Integration tests should exercise instruction-level behavior once compile blockers are removed.
- Formal proofs live beside source because arithmetic invariants are part of test coverage.

## Review Checklist
- Keep account ordering explicit in docs and code.
- Prefer project errors over generic `ProgramError` until the Solana boundary.
- Update the matching example or test when behavior changes.
