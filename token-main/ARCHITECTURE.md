# LetterP Token SDK Architecture

## Design Goal

LetterP keeps security-sensitive token and agent behavior visible at the call site. Account order, signer requirements, program IDs, PDA seeds, arithmetic rules, payment receipts, curve pricing, and perpetual risk checks are all represented as small SDK primitives.

## Layers

| Layer | Responsibility |
|-------|----------------|
| `ptoken-sdk/src` | Rust modules imported by examples and downstream programs |
| `01_*` through `15_*` | Project-owned notes for each source area |
| `examples/*` | Small instruction processors that demonstrate SDK calls |
| `program-ids/*` | Public deployment identities, with private keypairs ignored |
| `kani_verification.rs` | Bounded proofs with non-vacuity cover checks |

## Core Boundary

Runtime helpers parse bytes and accounts without deciding protocol policy. Token Classic, Token-2022, extension, CPI, ATA, and PDA modules build specific instruction flows. Validation modules perform preflight checks. Pure math and policy modules are kept free of CPI so they can be verified by Kani.

## Agent Market Boundary

Agent, x402, bonding-curve, and perpetual modules are intentionally pure:

- `agent.rs` validates capability flags, spending limits, and risk limits.
- `x402.rs` verifies paid route facts from a settlement receipt.
- `bonding_curve.rs` quotes linear and constant-product market flows with checked u128 intermediates.
- `perpetuals.rs` computes leverage, PnL, liquidation status, and funding payments.

On-chain programs can compose these primitives, but deployment needs separate audited program entrypoints.

## Account Data Policy

Fixed SPL Token fields are read through named offsets in `zero_copy_layout.rs`. Dynamic Token-2022 extension data should use SPL extension APIs rather than hand-rolled offsets unless the layout is explicitly documented and tested.

## Verification Policy

Pure arithmetic and policy helpers get Kani harnesses before reuse by CPI-facing code. Harnesses must include `kani::cover!` checks for success and failure paths so a proof cannot pass only because assumptions made the path unreachable.
