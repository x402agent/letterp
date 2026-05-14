# {{project_name}}

Pinocchio escrow starter for make/take/refund token swaps.

The starter mirrors the common escrow flow:

- `Make`: maker defines terms and deposits token A into a vault.
- `Take`: taker sends token B to the maker and receives token A.
- `Refund`: maker cancels and receives token A back.

This is a scaffold, not an audited program. Fill in token CPI transfers, PDA seed checks, ATA policy, close behavior, and tests before deployment.

## Instruction Shape

| Discriminator | Instruction | Purpose |
| ---: | --- | --- |
| `0` | `make` | Maker defines swap terms and deposits token A into a vault. |
| `1` | `take` | Taker accepts the swap, paying token B and receiving token A. |
| `2` | `refund` | Maker cancels an open escrow and reclaims token A. |

## State

`src/state.rs` defines an `Escrow` account with seed, maker, two mints, expected receive amount, and PDA bump. The instruction modules currently validate account count, signer presence, and basic amount shape.
