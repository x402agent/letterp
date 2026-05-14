# Pinocchio User Guide

This guide connects the vendored Pinocchio tree to the Letter P / Solana Clawd codebase.

## What This Repo Adds

- `p-agent-token`: a native Pinocchio P Agent token program draft.
- `programs/src`: a native Pinocchio bonding-curve launchpad draft.
- `p-token-launcher`: an unsigned browser/API workbench for p-token and P Agent exploration.
- `launchpad`: TypeScript instruction builders that mirror the native program contract.
- `agent-sdk`: Core asset, asset signer PDA, registration, and Core Execute helpers.
- `templates`: forkable Pinocchio starters for vault, escrow, p-token launcher, and p-agent-token flows.

## Read First

| Guide | Purpose |
|-------|---------|
| [`../docs/PINOCCHIO_ADAPTATION.md`](../docs/PINOCCHIO_ADAPTATION.md) | How local Pinocchio crates are wired into this repo. |
| [`../docs/PROGRAM_DRAFT.md`](../docs/PROGRAM_DRAFT.md) | Devnet/mainnet program contract and deployment model. |
| [`docs/PINOCCHIO_GUIDE.md`](docs/PINOCCHIO_GUIDE.md) | Practical Pinocchio development notes. |
| [`docs/P_TOKEN_LAUNCHES.md`](docs/P_TOKEN_LAUNCHES.md) | p-token launch and bonding curve flow. |
| [`docs/AGENT_WORKFLOWS.md`](docs/AGENT_WORKFLOWS.md) | Agent-facing discovery and registry workflows. |

## Workbench

```bash
npm run ptoken:launcher
```

Open `http://localhost:8787`, then use:

- Explore
- Pinocchio
- Launch
- Agent
- Program
- Quote
- Perp
- Inspect
- Registry

## Native Checks

```bash
CARGO_TARGET_DIR=/tmp/letterp-p-agent-token-target cargo check --manifest-path p-agent-token/Cargo.toml
CARGO_TARGET_DIR=/tmp/letterp-bonding-curve-target cargo check --manifest-path programs/src/Cargo.toml
```

## Development Rules

- Validate every account owner, signer, writable flag, and PDA seed.
- Keep instruction account parsing separate from business logic.
- Prefer explicit byte parsing until zero-copy layout safety is documented.
- Keep TypeScript client builders aligned with native discriminators and account order.
- Treat all templates as unaudited until tests and review exist.
