# P Agent Token

Pinocchio program scaffold for a faster P Agent token path. This folder is the promoted version of `templates/p-agent-token`: it has a concrete crate name and example config, but it is still a scaffold rather than a deployable audited program.

## How It Fits

`p-agent-token` is the on-chain companion to the TypeScript packages:

| Workspace area | Role |
|----------------|------|
| `agent-sdk` | Builds Core asset, registration, and Core Execute instructions for P Agents. |
| `launchpad` | Builds TypeScript launch, buy, sell, fee, and graduation instructions. |
| `p-agent-token` | Defines the Pinocchio state and instruction shell those client builders are expected to target. |
| `p-token-launcher` | Provides an unsigned planning UI/API for humans and agents before signing or deployment. |

## Pinocchio Integration

This crate uses the vendored Pinocchio tree under `../pinocchio/pinocchio-main`:

```toml
pinocchio = { path = "../pinocchio/pinocchio-main/sdk", features = ["cpi"] }
pinocchio-system = { path = "../pinocchio/pinocchio-main/programs/system" }
pinocchio-token = { path = "../pinocchio/pinocchio-main/programs/token" }
pinocchio-associated-token-account = { path = "../pinocchio/pinocchio-main/programs/associated-token-account" }
```

The program is adapted to the Pinocchio `0.11` API with `AccountView`, `Address`, `ProgramResult`, and `pinocchio::error::ProgramError`.

## Program Shape

| Discriminator | Instruction | Current behavior |
| ---: | --- | --- |
| `0` | `initialize_agent` | Validates account shape and owner signer. |
| `1` | `initialize_agent_mint` | Validates account shape and owner signer. |
| `2` | `bind_agent_token` | Marks the agent state as permanently bound. |
| `3` | `delegate_executor` | Validates account shape and owner signer. |
| `4` | `buy` | Parses a non-zero lamport amount; transfer and curve math are still TODO. |
| `5` | `sell` | Parses a non-zero token amount; transfer and curve math are still TODO. |
| `6` | `graduate` | Marks the agent state as graduated. |

## State

`src/state.rs` defines:

| State | Purpose |
|-------|---------|
| `AgentState` | Owner, Core asset, bound token mint, executive wallet, metadata hash, flags, and bump. |
| `CurveState` | Mint, vault, virtual reserves, real reserves, fees, flags, and bump. |

Both structs use zero-copy loading and exact account-data length checks.

## Example Config

`agent-token.example.json` describes the launch contract shared by the planner, agent metadata, and program implementation:

```json
{
  "standard": "p-agent-token-v1",
  "token": { "symbol": "PCLAWD" },
  "bondingCurve": { "type": "constant-product", "graduationSol": 85 }
}
```

## Build

```bash
cd p-agent-token
cargo check
```

For SBF builds, use the Solana toolchain appropriate for your validator and Pinocchio version.

## Formal Verification

Kani proof harnesses cover current pure program invariants: one-way flags, state layout constants, buy/sell amount parsing, fee basis-point math, constant-product quotes, byte decoding, and instruction discriminator uniqueness. See [`KANI_VERIFICATION.md`](KANI_VERIFICATION.md).

## Deployment Draft

See [`DEPLOYMENT.md`](DEPLOYMENT.md) for the devnet and mainnet deployment checklist.

## Security Work Before Deployment

- Validate every account owner, signer, writable flag, and PDA bump.
- Enforce token program IDs and p-token mint/account ownership explicitly.
- Implement CPI transfers and signer seeds for reserve custody.
- Keep curve reserves separate from agent operating balances.
- Make `bind_agent_token` one-way after finalization.
- Implement overflow-safe fee math, graduation thresholds, reserve migration, and close/refund behavior.
- Add Mollusk or SBF tests for malformed accounts, duplicate accounts, wrong token programs, overflow, fee math, and graduation boundaries.
