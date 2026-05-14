# 15 - Docs

Operator and developer references for the LetterP token SDK.

## Code Map
- Primary source: `15_docs`
- Crate entry: `ptoken-sdk/src/lib.rs`

## Local Rules
- Docs describe how this repository is wired, not a generic SPL tutorial.
- Kani verification docs are maintained as a runnable checklist.
- Migration and extension references should call out account-size and authority changes.
- Agent, x402, bonding-curve, and perpetual primitives are documented in `agent_x402_curves_perps.md`.

## Review Checklist
- Keep account ordering explicit in docs and code.
- Prefer project errors over generic `ProgramError` until the Solana boundary.
- Update the matching example or test when behavior changes.
