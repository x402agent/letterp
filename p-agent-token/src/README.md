# P Agent Token Source

Pinocchio source for the promoted P Agent token scaffold.

## Files

| Path | Purpose |
|------|---------|
| `lib.rs` | Program entrypoint and one-byte instruction discriminator dispatch. |
| `state.rs` | Zero-copy `AgentState` and `CurveState` layouts. |
| `errors.rs` | Program-specific error codes and conversion helpers. |
| `instructions/mod.rs` | Instruction module exports. |
| `instructions/helpers.rs` | Shared signer and amount validation helpers. |
| `instructions/initialize_agent.rs` | Agent state initialization account parser. |
| `instructions/initialize_agent_mint.rs` | Agent mint initialization account parser. |
| `instructions/bind_agent_token.rs` | One-way bound flag update. |
| `instructions/delegate_executor.rs` | Executive wallet delegation account parser. |
| `instructions/buy.rs` | Buy instruction parser for lamports-in. |
| `instructions/sell.rs` | Sell instruction parser for tokens-in. |
| `instructions/graduate.rs` | Graduation flag update shell. |

## Current Status

This source defines account and instruction shape. The transfer CPIs, reserve accounting, PDA seed enforcement, token custody, and AMM migration logic still need implementation before deployment.
