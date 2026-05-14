# {{project_name}}

Pinocchio vault starter for native Solana programs.

This template targets the vendored Pinocchio `0.11` tree in this repo and uses
path dependencies into `../../pinocchio/pinocchio-main`.

The template demonstrates:

- one-byte instruction discriminators;
- `TryFrom` account and instruction validation;
- PDA vault state;
- deposit and withdraw instruction structure;
- explicit safety notes for account data parsing.

Before deployment, add Mollusk or SBF tests for signer checks, PDA derivation, rent, close behavior, malformed input, and CPI failure paths.

## Instruction Shape

| Discriminator | Instruction | Purpose |
| ---: | --- | --- |
| `0` | `deposit` | Parses a non-zero amount and increments vault accounting. |
| `1` | `withdraw` | Parses a non-zero amount and decrements vault accounting with an insufficient-funds check. |

## State

`src/state.rs` defines a zero-copy `Vault` account with authority, mint, amount, and bump. The starter tracks accounting only; token or SOL movement must be implemented with explicit CPI and PDA signer checks before deployment.
