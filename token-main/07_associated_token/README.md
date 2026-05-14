# 07 — Associated Token Account (ATA)

Helpers for the Associated Token Account program.
ATA Program ID: `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJe1bN`

ATAs are deterministically derived token accounts — one per (wallet, mint) pair.

## Sub-modules
- `create_ata/` — Create a new ATA
- `get_ata_address/` — Derive the ATA address off-chain
- `idempotent_create/` — Create ATA only if it doesn't exist

> 🚧 Coming Soon
