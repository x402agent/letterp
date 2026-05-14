# P Agent Token Workflow

`p-agent-token` is the native Pinocchio draft for first-party P Agents in this repo.

## Flow

1. Mint or select a Metaplex Core asset.
2. Derive the asset signer PDA with `["mpl-core-execute", asset]`.
3. Initialize P Agent state with owner, Core asset, metadata hash, and executive policy.
4. Initialize or validate a p-token mint.
5. Bind the mint one-way to the agent state.
6. Route creator fees to the asset signer PDA or creator vault.
7. Buy, sell, graduate, and delegate through reviewable policy.

## Program Surface

| Disc | Instruction | Purpose |
| ---: |-------------|---------|
| `0` | `initialize_agent` | Create or validate agent state. |
| `1` | `initialize_agent_mint` | Create or validate the p-token mint. |
| `2` | `bind_agent_token` | Permanently bind token mint to agent state. |
| `3` | `delegate_executor` | Record an execution delegate. |
| `4` | `buy` | Buy along the launch curve. |
| `5` | `sell` | Sell along the launch curve. |
| `6` | `graduate` | Mark the launch as graduated. |

## Templates

Start from [`../../templates/p-agent-token`](../../templates/p-agent-token), or use the promoted draft at [`../../p-agent-token`](../../p-agent-token).

## Workbench

Run:

```bash
npm run ptoken:launcher
```

Use the Agent and Program tabs to create unsigned planning artifacts.
