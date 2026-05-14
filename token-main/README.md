# pToken SDK

A high-performance Solana token SDK built on Pinocchio — zero-dependency, close-to-the-metal Rust primitives for SPL Token and Token-2022 programs.

## Structure

| Module | Description |
|--------|-------------|
| `01_pinocchio_core` | Pinocchio runtime: accounts, instructions, syscalls, zero-copy |
| `02_token_classic` | SPL Token (original program) operations |
| `03_token_2022` | Token-2022 base operations |
| `04_extensions` | All 14 Token-2022 extensions |
| `05_cpi` | Cross-Program Invocation helpers |
| `06_pda` | Program Derived Address utilities |
| `07_associated_token` | Associated Token Account (ATA) wrappers |
| `08_serialization` | Borsh encode/decode and pack/unpack |
| `09_math` | Safe checked arithmetic and decimal helpers |
| `10_validation` | Signer, owner, mint, and state checks |
| `11_errors` | Custom error types for tokens and extensions |
| `12_constants` | Program IDs, seeds, and defaults |
| `13_examples` | Runnable example programs |
| `14_tests` | Unit, integration, and bankrun test suites |
| `15_docs` | Guides, references, and migration docs |

## Status
> 🚧 Coming Soon — All modules are under active development.

## Formal Verification
Kani proof harnesses live in `ptoken-sdk/src/kani_verification.rs` and currently cover token arithmetic, fee, range, and decimal multiplier invariants. See `15_docs/KANI_VERIFICATION.md` for install and run commands, including non-vacuity checks with `kani::cover!`.

## Target Programs
- **SPL Token**: `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`
- **Token-2022**: `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb`
- **ATA Program**: `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJe1bN`
- **Meteora**: `4xCUgkuWmpk1gmrcT9PmmCPVY8kFnHKCsMoLsRrNSPL`
- **pumpswap**: `7hynuVWWjwUPHEqYYYWCDUGctSR8PretCD8AU1F3pump`
