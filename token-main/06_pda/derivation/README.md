# PDA Derivation

Wrappers around `Pubkey::find_program_address` and `Pubkey::create_program_address`
for deriving PDAs with common pToken seed patterns.

## Common Seeds Used in pToken
- `[b"mint", user_pubkey]` — User-specific mint PDA
- `[b"vault", mint_pubkey]` — Protocol vault PDA
- `[b"metadata", mint_pubkey]` — Metadata PDA

> 🚧 Coming Soon
