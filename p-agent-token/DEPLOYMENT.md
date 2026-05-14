# P Agent Token Deployment

Deployment checklist for the Pinocchio P Agent Token program.

## Current Status

The program currently defines the entrypoint, instruction discriminators, zero-copy state layouts, basic signer checks, amount parsing, bound flag updates, and graduation flag updates. It does not yet move funds, mint tokens, burn tokens, enforce all PDA seeds, or migrate liquidity.

## Local Checks

```bash
cd p-agent-token
cargo check
```

Expected today: `cargo check` succeeds, with Pinocchio macro warnings on standard host toolchains about `target_os = "solana"`.

## Devnet Draft

1. Set cluster and IDs:

```bash
solana config set --url devnet
export SOLANA_NETWORK=solana-devnet
export P_TOKEN_PROGRAM_ID=ptok6rngomXrDbWf5v5Mkmu5CEbB51hzSCPDoj9DrvF
```

2. Build with the Solana SBF toolchain for the selected Pinocchio version:

```bash
cd p-agent-token
cargo build-sbf
```

3. Deploy with an explicit program keypair:

```bash
solana program deploy \
  --program-id target/deploy/p_agent_token-keypair.json \
  target/deploy/p_agent_token.so
```

4. Export the deployed ID:

```bash
export P_AGENT_TOKEN_PROGRAM_ID=<devnet-program-id>
```

5. Use the workbench:

```bash
npm run ptoken:launcher
```

Open `http://localhost:8787`, draft an agent plan, draft a devnet program plan, and inspect any created mint.

## Mainnet Draft

Before mainnet:

- Replace placeholder program IDs in docs and environment examples.
- Freeze instruction discriminators and account layouts.
- Add complete PDA seed checks to all instructions.
- Implement CPI transfers, p-token mint/burn, reserve custody, fee withdrawal, and graduation.
- Add SBF or Mollusk tests for malformed accounts, duplicate accounts, wrong token program, arithmetic overflow, slippage, and graduation thresholds.
- Put upgrade authority behind a multisig.
- Run an external review for the exact deployed commit.

Mainnet deployment shape:

```bash
solana config set --url mainnet-beta
export SOLANA_NETWORK=solana-mainnet
export P_AGENT_TOKEN_PROGRAM_ID=<mainnet-program-id>
export P_TOKEN_LAUNCHPAD_PROGRAM_ID=<mainnet-launchpad-program-id>
```

Agent execution policy on mainnet should default to review-required until automated policy enforcement, spend caps, monitoring, and emergency pause behavior are implemented.

## Published Contract

| Item | Value |
|------|-------|
| Crate | `p-agent-token` |
| Current instruction count | `7` |
| State accounts | `AgentState`, `CurveState` |
| Identity dependency | Metaplex Core asset signer PDA |
| Token dependency | p-token/SPL-compatible token program |
| Workbench | `p-token-launcher` |
