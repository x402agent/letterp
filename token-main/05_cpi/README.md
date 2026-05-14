# 05 — CPI Helpers

Cross-Program Invocation helpers for calling Solana token programs
from within a Pinocchio-based program. All calls use `invoke` or
`invoke_signed` with explicit account metas.

## Sub-modules
- `invoke_token_program/` — CPI into SPL Token classic
- `invoke_token_2022/` — CPI into Token-2022 program
- `invoke_associated_token/` — CPI into ATA program
- `invoke_system_program/` — CPI into System program (create accounts, transfer SOL)

> 🚧 Coming Soon
