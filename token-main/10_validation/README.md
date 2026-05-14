# 10 — Validation

Account and instruction validation helpers for safe Pinocchio program development.
All checks return typed PToken errors rather than panicking.

## Sub-modules
- `signer_checks/` — Assert required accounts are signers
- `owner_checks/` — Assert accounts are owned by expected programs
- `mint_validation/` — Validate mint account state and properties
- `account_state_checks/` — Validate token account state (initialized, frozen, etc.)

> 🚧 Coming Soon
