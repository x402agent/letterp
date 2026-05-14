# 02 - SPL Token Classic

Wrappers for the original SPL Token program. This layer builds instructions and CPI account lists while preserving the exact account ordering expected by Tokenkeg.

## Code Map
- Primary source: `ptoken-sdk/src/token_classic`
- Crate entry: `ptoken-sdk/src/lib.rs`

## Local Rules
- Mint and account initialization must use rent-exempt allocation before token initialization.
- Transfer, burn, approve, revoke, freeze, and close helpers should remain thin CPI surfaces.
- Authority checks belong in validation helpers before the CPI boundary.

## Review Checklist
- Keep account ordering explicit in docs and code.
- Prefer project errors over generic `ProgramError` until the Solana boundary.
- Update the matching example or test when behavior changes.
