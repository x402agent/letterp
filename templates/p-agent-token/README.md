# {{project_name}}

Pinocchio p-agent-token starter.

This template is for a faster agent token path:

- p-token mint/account operations for lower compute-unit cost.
- Pinocchio zero-copy account validation and state access.
- MPL Core / Agent Registry compatible identity metadata.
- One-way agent-token binding similar to Metaplex agent token linking.
- Constant-product launch curve planning.

This template targets the vendored Pinocchio `0.11` tree in this repo and uses
`AccountView`, `Address`, `ProgramResult`, and `pinocchio::error::ProgramError`.
It also includes the same Kani proof harnesses used by the promoted
`../../p-agent-token` crate so generated agent-token programs start with a
formal verification baseline.

This scaffold is intentionally incomplete and unaudited. It defines the public
program shape and state contract, but you must implement CPI transfers, PDA
signer seeds, curve reserve custody, fee distribution, graduation, and tests
before deployment.

## Planned Instructions

| Discriminator | Instruction | Purpose |
| ---: | --- | --- |
| `0` | `initialize_agent` | Create zero-copy agent state and point to agent metadata/Core asset. |
| `1` | `initialize_agent_mint` | Create or validate the p-token mint for the agent. |
| `2` | `bind_agent_token` | Permanently bind the p-token mint to the agent state. |
| `3` | `delegate_executor` | Record an executive wallet allowed to operate the agent. |
| `4` | `buy` | Buy along the launch curve. |
| `5` | `sell` | Sell along the launch curve. |
| `6` | `graduate` | Freeze the launch curve and prepare AMM migration. |

## Adapted Version

This repo includes an adapted copy at `../../p-agent-token`. Use that folder as
the concrete reference for crate naming, source layout, and example config.

## Planner

Use `../../p-token-launcher` for unsigned launch planning and curve quotes. The
program template itself does not include a JavaScript CLI.

## Formal Verification

Kani proof harnesses cover the template's pure invariants: one-way flags, state
layout constants, buy/sell amount parsing, fee basis-point math,
constant-product quotes, byte decoding, and instruction discriminator
uniqueness. See [`KANI_VERIFICATION.md`](KANI_VERIFICATION.md).

## Security Checklist

- Validate every account owner, signer, writable flag, and PDA bump.
- Keep p-token mint owner checks explicit.
- Keep token state zero-copy; avoid owned token account deserialization in hot paths.
- Make `bind_agent_token` one-way after finalization.
- Separate curve reserves from agent operating balances.
- Test malformed accounts, duplicate accounts, incorrect token programs, overflow,
  fee math, and graduation boundaries.
- Treat p-token as unaudited unless your chosen deployment has an independent
  review for the exact commit and program id.
