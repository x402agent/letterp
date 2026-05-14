# 07 - Associated Token Accounts

ATA derivation and creation helpers for SPL Token and Token-2022 account ownership patterns.

## Code Map
- Primary source: `ptoken-sdk/src/associated_token`
- Crate entry: `ptoken-sdk/src/lib.rs`

## Local Rules
- Derive ATA addresses from wallet, token program, and mint exactly in that order.
- Use idempotent creation when account pre-existence is acceptable.
- Do not infer token program ID from mint data unless the caller explicitly opts into that behavior.

## Review Checklist
- Keep account ordering explicit in docs and code.
- Prefer project errors over generic `ProgramError` until the Solana boundary.
- Update the matching example or test when behavior changes.
