# LetterP Token SDK

LetterP Token SDK is a Solana token toolkit for p-agents and ptoken programs. It provides explicit SPL Token, Token-2022, PDA, ATA, serialization, validation, arithmetic, agent policy, x402 receipt, bonding-curve, and perpetual-risk primitives.

The repository is written to be reviewed line by line. Source lives in `ptoken-sdk/src`; the numbered folders document module intent, invariants, account assumptions, and audit hooks.

## What Is Included

| Area | Path |
|------|------|
| Runtime helpers | `01_pinocchio_core`, `ptoken-sdk/src/pinocchio_core` |
| SPL Token classic | `02_token_classic`, `ptoken-sdk/src/token_classic` |
| Token-2022 | `03_token_2022`, `ptoken-sdk/src/token_2022` |
| Token-2022 extensions | `04_extensions`, `ptoken-sdk/src/extensions` |
| CPI and ATA helpers | `05_cpi`, `07_associated_token`, `ptoken-sdk/src/cpi`, `ptoken-sdk/src/associated_token` |
| PDA utilities | `06_pda`, `ptoken-sdk/src/pda` |
| Serialization | `08_serialization`, `ptoken-sdk/src/serialization` |
| Verified math | `09_math`, `ptoken-sdk/src/math` |
| Agent and market primitives | `ptoken-sdk/src/agent.rs`, `x402.rs`, `bonding_curve.rs`, `perpetuals.rs` |
| Formal proofs | `ptoken-sdk/src/kani_verification.rs`, `15_docs/KANI_VERIFICATION.md` |
| Devnet program IDs | `program-ids/devnet/programs.toml` |

## Quick Check

```bash
cargo check -p ptoken-sdk --lib
cargo test -p ptoken-sdk --lib
cargo kani -p ptoken-sdk
```

Current local verification result:

```text
Complete - 11 successfully verified harnesses, 0 failures, 11 total.
```

The Kani harnesses use `kani::cover!` on success and failure paths so proofs are not vacuous.

## Agent, x402, Curves, and Perps

The SDK includes pure primitives for:

- p-agent capability gates and spend/risk limits.
- x402 payment intent and receipt verification for HTTP-native agent payments.
- Linear and constant-product bonding-curve quote math.
- Perpetual position PnL, leverage, liquidation, and funding math.

These are SDK primitives, not deployed programs yet. The generated devnet IDs reserve the intended program identities; mainnet deployment should happen only after audited SBF programs are built from these interfaces.

## Devnet Program IDs

Generated public IDs are in `program-ids/devnet/programs.toml`. Private keypairs are local-only under `program-ids/devnet/keypairs/` and ignored by git.

## Mainnet Deployment Cost Estimate

Solana upgradeable program deployment cost is dominated by rent-exempt lamports for the ProgramData account. The exact cost requires final `.so` byte sizes. Current CLI rent checks on May 14, 2026:

| ProgramData bytes | Rent-exempt minimum |
|-------------------|---------------------|
| 200,000 | 1.39289088 SOL |
| 500,000 | 3.48089088 SOL |
| 1,000,000 | 6.96089088 SOL |
| 2,000,000 | 13.92089088 SOL |

For five new programs, multiply by the final binary sizes. Transaction fees for deployment writes are additional and usually much smaller than rent, but they vary with chunking, signatures, and priority fees.

## Open-Source Readiness

The crate compiles and tests locally. Before a public release, add final license text, choose deployment authority custody, run a third-party security review, and publish reproducible SBF build artifacts for each deployed program.
