# LetterP Token SDK

LetterP Token SDK is the local Solana token toolkit for this repository. It keeps SPL Token, Token-2022, PDA, ATA, serialization, validation, and arithmetic behavior explicit so instruction code can be reviewed line by line.

## Structure

| Module | Description |
|--------|-------------|
| `01_pinocchio_core` | Runtime-facing account, entrypoint, instruction-data, syscall, and fixed-layout readers |
| `02_token_classic` | Original SPL Token instruction and CPI helpers |
| `03_token_2022` | Token-2022 mint, account, close, and reallocation helpers |
| `04_extensions` | Token-2022 extension constructors and authority-aware update helpers |
| `05_cpi` | System, Token, Token-2022, and ATA CPI adapters |
| `06_pda` | PDA derivation, bump, and validation helpers |
| `07_associated_token` | ATA derivation and creation helpers |
| `08_serialization` | Borsh and fixed-width instruction encoding helpers |
| `09_math` | Checked arithmetic, decimal, fee, and u64 helpers with Kani proofs |
| `10_validation` | Reusable signer, owner, mint, and account-state checks |
| `11_errors` | SDK error types and ProgramError conversion |
| `12_constants` | Canonical program IDs and PDA seed bytes |
| `13_examples` | LetterP example programs for mint, fee, metadata, hook, and confidential flows |
| `14_tests` | Unit, integration, and runtime test plans |
| `15_docs` | Verification, migration, extension, and token references |

## Implementation Status
The repository has two layers:
- `ptoken-sdk/src` contains the Rust crate code that examples import.
- Numbered folders contain project-owned module notes that map back to the Rust source and document invariants, authority assumptions, and audit hooks.

The arithmetic and documentation surfaces are wired and Kani verified. The broader Solana CPI/extension crate still has compile blockers tracked in `COMING_SOON.md`, mostly around dependency imports, SPL Token-2022 type drift, CPI lifetimes, and one PDA temporary borrow.

## Formal Verification
Kani proof harnesses live in `ptoken-sdk/src/kani_verification.rs` and currently cover token arithmetic, fee, range, and decimal multiplier invariants. See `15_docs/KANI_VERIFICATION.md` for install and run commands, including non-vacuity checks with `kani::cover!`.

## Target Programs
- **SPL Token**: `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`
- **Token-2022**: `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb`
- **ATA Program**: `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJe1bN`
