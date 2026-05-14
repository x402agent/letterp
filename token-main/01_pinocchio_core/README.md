# 01 - Pinocchio Core

Runtime-facing primitives for LetterP token programs. These helpers keep instruction dispatch, account reads, and fixed-layout token parsing explicit instead of hiding behavior behind framework macros.

## Code Map
- Primary source: `ptoken-sdk/src/pinocchio_core`
- Crate entry: `ptoken-sdk/src/lib.rs`

## Local Rules
- Use byte-slice readers for SPL Token account fields that are stable by offset.
- Keep entrypoint dispatch small: validate discriminants first, then hand off to module code.
- Do not allocate for reads that can be performed directly from account data.

## Review Checklist
- Keep account ordering explicit in docs and code.
- Prefer project errors over generic `ProgramError` until the Solana boundary.
- Update the matching example or test when behavior changes.
