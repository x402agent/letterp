# LetterP Token SDK

LetterP Token SDK is a Solana token toolkit for p-agents and ptoken programs. It provides explicit SPL Token, Token-2022, PDA, ATA, serialization, validation, arithmetic, agent policy, x402 receipt, bonding-curve, and perpetual-risk primitives, with Kani proofs for the pure safety-critical paths.

The repository is written to be reviewed line by line. Source lives in `ptoken-sdk/src`; the numbered folders document module intent, invariants, account assumptions, and audit hooks. The current workspace also includes minimal Solana program crates, generated devnet program IDs, open-source project files, and deployment cost tooling.

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
| Validation | `10_validation`, `ptoken-sdk/src/validation` |
| Errors | `11_errors`, `ptoken-sdk/src/errors` |
| Constants | `12_constants`, `ptoken-sdk/src/constants` |
| Examples | `13_examples`, `examples/*` |
| Tests | `14_tests`, Rust unit tests, doctests, Kani harnesses |
| Docs | `15_docs`, `ARCHITECTURE.md`, `COMING_SOON.md` |
| Agent and market primitives | `ptoken-sdk/src/agent.rs`, `x402.rs`, `bonding_curve.rs`, `perpetuals.rs` |
| Program entrypoints | `programs/p_agent`, `programs/p_token`, `programs/x402_gateway`, `programs/bonding_curve`, `programs/perpetuals` |
| Formal proofs | `ptoken-sdk/src/kani_verification.rs`, `15_docs/KANI_VERIFICATION.md` |
| Devnet program IDs | `program-ids/devnet/programs.toml` |
| Open-source files | `LICENSE`, `CONTRIBUTING.md`, `SECURITY.md` |

## Workspace Layout

| Path | Purpose |
|------|---------|
| `ptoken-sdk` | Main Rust SDK crate. Default features keep pure primitives lightweight; `full` enables runtime, SPL Token, Token-2022, and extension helpers. |
| `programs` | Minimal Solana entrypoint crates for p-agent, p-token, x402 gateway, bonding curve, and perpetuals program identities. |
| `program-ids` | Public devnet program IDs and local-only ignored keypair location. |
| `examples` | Compile-checked example crates for minting, confidential minting, metadata, transfer hooks, and transfer fees. |
| `scripts` | Operational helper scripts, including mainnet rent estimation. |
| `15_docs` | Verification notes, migration material, extension cookbook, Token Classic and Token-2022 references, and agent/x402/market primitive docs. |

## SDK Modules

| Module | Purpose |
|--------|---------|
| `agent` | Capability gates, agent identity metadata, spend limits, and risk limits for p-agent execution. |
| `x402` | HTTP-native payment intent and receipt verification primitives for agent-accessible paid endpoints. |
| `bonding_curve` | Linear and constant-product quote math with bounded arithmetic and monotonicity proofs. |
| `perpetuals` | Position notional, leverage, liquidation, funding, and side-aware PnL helpers. |
| `math` | Checked arithmetic, fee math, decimal helpers, and bounded `u64` utilities. |
| `token_classic` | SPL Token classic instruction/account helpers. |
| `token_2022` | Token-2022 helpers and extension-aware account flows. |
| `extensions` | Confidential transfer, transfer fee, transfer hook, metadata, immutable owner, permanent delegate, required memo, CPI guard, and related Token-2022 extension support. |
| `cpi` | CPI wrappers for system, token, Token-2022, and associated token operations. |
| `pda` | PDA derivation, bump management, and validation helpers. |
| `serialization` | Borsh and SPL pack/unpack helpers. |
| `validation` | Signer, owner, mint, and account-state checks. |

## Quick Check

```bash
cargo check -p ptoken-sdk --lib
cargo test -p ptoken-sdk --lib
cargo kani -p ptoken-sdk
```

Full workspace checks:

```bash
cargo check --workspace
cargo test --workspace
```

Current local verification result:

```text
Complete - 11 successfully verified harnesses, 0 failures, 11 total.
```

The Kani harnesses use `kani::cover!` on success and failure paths so proofs are not vacuous.

## Agent, x402, Curves, and Perps

The SDK includes pure primitives for:

- p-agent capability gates and spend/risk limits.
- x402 payment intent and receipt verification for HTTP-native agent payments and paid API access.
- Linear and constant-product bonding-curve quote math.
- Perpetual position PnL, leverage, liquidation, and funding math.

These are SDK primitives with minimal program entrypoints under `programs/`. The generated devnet IDs reserve the intended program identities; mainnet deployment should happen only after the final audited instruction sets are built and reproducibly verified.

## Program Crates

| Program | Path | Devnet ID |
|---------|------|-----------|
| p-agent | `programs/p_agent` | `FaXsrwC4bZnprnoMtvPibxLyJjhPSwC7pURsZK7T49Gg` |
| p-token | `programs/p_token` | `7BNvimHVAW7KHzG33RFYoQLEQMZZ1yk8MbaDETwujptY` |
| x402 gateway | `programs/x402_gateway` | `5hsc8ptpLrCYfeEZypEm4NtDjjMpsSwWVLLnEwLtzeMF` |
| bonding curve | `programs/bonding_curve` | `2yVjXFU9cLM79DDpLHvAsCWgZaxU2cNRpd9nBhj7tC3m` |
| perpetuals | `programs/perpetuals` | `GikZAua12fZz7rNNBPeSp9PSGP5RpH2Drje7x75Kq4wX` |

## Devnet Program IDs

Generated public IDs are in `program-ids/devnet/programs.toml`. Private keypairs are local-only under `program-ids/devnet/keypairs/` and ignored by git.

SDK constants are available in `ptoken-sdk/src/constants/program_ids.rs`:

```rust
LETTERP_P_AGENT_DEVNET_ID
LETTERP_P_TOKEN_DEVNET_ID
LETTERP_X402_GATEWAY_DEVNET_ID
LETTERP_BONDING_CURVE_DEVNET_ID
LETTERP_PERPETUALS_DEVNET_ID
```

## Program Builds

Host workspace builds pass with:

```bash
cargo check --workspace
```

Workspace tests pass with:

```bash
cargo test --workspace
```

SBF build tooling is available locally through `cargo build-sbf`. The current local Agave 3.1.11 SBF toolchain builds the minimal program crates, but emits compatibility warnings for the Solana 1.18 dependency stack. Use a Solana build toolchain matched to the dependency line before deploying, or upgrade the Solana/SPL dependency set and rerun all tests and Kani proofs.

Build individual SBF crates with:

```bash
cargo build-sbf --manifest-path programs/p_agent/Cargo.toml
cargo build-sbf --manifest-path programs/p_token/Cargo.toml
cargo build-sbf --manifest-path programs/x402_gateway/Cargo.toml
cargo build-sbf --manifest-path programs/bonding_curve/Cargo.toml
cargo build-sbf --manifest-path programs/perpetuals/Cargo.toml
```

## Formal Verification

Kani proofs cover:

| Harness area | What it checks |
|--------------|----------------|
| Checked arithmetic | SDK helpers match Rust checked operations and avoid overflow surprises. |
| Division helpers | Total behavior and precision on bounded inputs. |
| Fee calculations | Cap handling, high-bps edges, truncation behavior, and `u64` extremes. |
| Decimal helpers | Monotonic multiplier behavior until saturation. |
| Agent policy | Vacuous trading authority is rejected and valid authority paths are reachable. |
| x402 receipts | Only matching, unexpired receipts unlock payment-gated access. |
| Bonding curves | Quotes remain monotonic for bounded inputs. |
| Perpetuals | Side-aware position math preserves profitable, losing, and neutral directionality. |

Run every harness with:

```bash
cargo kani -p ptoken-sdk
```

Run source coverage with:

```bash
cargo kani -p ptoken-sdk --coverage -Z source-coverage
```

Treat any unsatisfied `kani::cover!` result as a proof-quality failure, even when assertions pass.

## Mainnet Deployment Cost Estimate

Solana upgradeable program deployment cost is dominated by rent-exempt lamports for the ProgramData account. The exact cost requires final `.so` byte sizes. Current CLI rent checks on May 14, 2026:

| ProgramData bytes | Rent-exempt minimum |
|-------------------|---------------------|
| 200,000 | 1.39289088 SOL |
| 500,000 | 3.48089088 SOL |
| 1,000,000 | 6.96089088 SOL |
| 2,000,000 | 13.92089088 SOL |

For five new programs, multiply by the final binary sizes. Transaction fees for deployment writes are additional and usually much smaller than rent, but they vary with chunking, signatures, and priority fees.

Re-run the estimate with:

```bash
scripts/estimate-mainnet-rent.sh 200000 500000 1000000 2000000
```

## Open-Source Readiness

The crate compiles and tests locally, and the repository now includes MIT license text, contribution notes, and a security policy. Generated build output under `target/` is ignored and removed from the git index. Devnet private keypairs are ignored; only public program IDs should be committed.

Before mainnet, choose deployment authority custody, align the SBF toolchain with the Solana dependency line, run a third-party security review, publish reproducible SBF build artifacts for each deployed program, and rerun:

```bash
cargo check --workspace
cargo test --workspace
cargo kani -p ptoken-sdk
```
