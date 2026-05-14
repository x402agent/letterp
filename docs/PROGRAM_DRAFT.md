# P Agent Program Draft

This draft defines the repo target for native Pinocchio-programmed agents on Solana. The model is intentionally close to Metaplex Core agents where identity is an asset and execution flows through an asset signer PDA, but the launch, token binding, curve, fees, and delegation state are owned by this repo.

## Goals

- Create a first-party P Agent program that binds a Metaplex Core asset to one p-token mint.
- Keep the agent wallet keyless by using the Core asset signer PDA for execution.
- Route creator fees and launch revenue to agent-controlled PDAs.
- Support reviewable delegation for executives, operators, and automation.
- Keep the same account and instruction contracts available on devnet and mainnet.

## Program Set

| Program | Path | Purpose |
|---------|------|---------|
| P Agent Token | `p-agent-token` | Agent state, Core asset binding, token binding, executive delegation, curve flags. |
| Launchpad Curve | `programs/src` | Bonding curve initialization, buy, sell, graduation, and creator fee claims. |
| TypeScript SDK | `agent-sdk`, `launchpad`, `shared` | Client instruction builders, PDA helpers, Core Execute wrapping, state decoding. |
| Workbench | `p-token-launcher` | Unsigned exploration, planning, inspection, and deployment draft generation. |

All native program surfaces use the vendored Pinocchio tree at `pinocchio/pinocchio-main`. See `PINOCCHIO_ADAPTATION.md`.

## Account Model

| Account | Seeds | Owner | Purpose |
|---------|-------|-------|---------|
| Asset Signer | `["mpl-core-execute", asset]` | Metaplex Core | Keyless wallet for the Core asset. |
| Agent State | `["agent", asset_signer]` | P Agent Token | Owner, Core asset, token mint, executive, metadata hash, flags. |
| Agent Token | `["agent-token", mint]` | Launchpad or P Agent Token | One-way mint binding and launch metadata. |
| Bonding Curve | `["bonding-curve", mint]` | Launchpad Curve | Constant-product reserves and fee settings. |
| Curve Vault | `["bonding-curve", mint, "vault"]` | Launchpad Curve | SOL reserve custody and fee custody. |
| Creator Vault | `["creator-vault", asset_signer]` | Launchpad Curve | Agent creator fee accounting. |
| Execution Delegation | `["exec-delegation", agent, delegate]` | P Agent Token | Slot-expiring operator authorization. |

## Instruction Contract

| Disc | Instruction | Owner Program | Required Result |
| ---: |-------------|---------------|-----------------|
| `0` | `initialize_agent` | P Agent Token | Create and initialize agent state for an owner and Core asset. |
| `1` | `initialize_agent_mint` | P Agent Token | Create or validate the p-token mint used by the agent. |
| `2` | `bind_agent_token` | P Agent Token | Permanently bind a mint to the agent state. |
| `3` | `delegate_executor` | P Agent Token | Record a delegate and expiry slot. |
| `4` | `buy` | Launchpad Curve or P Agent Token wrapper | Buy against the curve through signer PDA policy. |
| `5` | `sell` | Launchpad Curve or P Agent Token wrapper | Sell against the curve through signer PDA policy. |
| `6` | `graduate` | Launchpad Curve | Disable curve trades and prepare AMM/perps migration. |

## Devnet Contract

Devnet is the iteration target:

- Program IDs may be replaced frequently.
- Upgrade authority can be a development key or small multisig.
- Agents must default to `review-required` execution.
- The workbench may emit unsigned intent envelopes only.
- x402 verification may use the reference facilitator.

Required devnet environment:

```bash
export SOLANA_NETWORK=solana-devnet
export P_TOKEN_PROGRAM_ID=ptok6rngomXrDbWf5v5Mkmu5CEbB51hzSCPDoj9DrvF
export P_AGENT_TOKEN_PROGRAM_ID=<devnet-agent-program>
export P_TOKEN_LAUNCHPAD_PROGRAM_ID=<devnet-launchpad-program>
export FACILITATOR_URL=http://localhost:4402
```

## Mainnet Contract

Mainnet should be treated as a frozen interface:

- Discriminators and account layouts are versioned and documented before deploy.
- Upgrade authority is a multisig or governance-controlled authority.
- Program IDs are published in `shared`, docs, and deployment notes.
- Agent execution remains review-required until policy, limits, and monitoring exist.
- Facilitator settlement verifies transaction contents, recipient, amount, freshness, and idempotency.

Required mainnet environment:

```bash
export SOLANA_NETWORK=solana-mainnet
export P_TOKEN_PROGRAM_ID=<audited-p-token-program>
export P_AGENT_TOKEN_PROGRAM_ID=<mainnet-agent-program>
export P_TOKEN_LAUNCHPAD_PROGRAM_ID=<mainnet-launchpad-program>
export FACILITATOR_URL=<production-facilitator>
```

## Implementation Order

1. Lock account layout constants in Rust and TypeScript.
2. Implement PDA derivation checks in every instruction.
3. Implement initialization and one-way token binding.
4. Implement p-token CPI mint, burn, and transfer paths.
5. Implement curve reserve accounting and fee accounting on-chain.
6. Add Core Execute policy checks and delegation expiry.
7. Add SBF/Mollusk tests for malformed accounts and boundary math.
8. Deploy devnet, inspect via the workbench, then freeze mainnet artifacts.
