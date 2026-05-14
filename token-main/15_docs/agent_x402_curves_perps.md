# Agent, x402, Bonding Curve, and Perpetual Primitives

## Scope

The SDK now includes pure primitives for the market and agent layer requested for LetterP:

- `ptoken-sdk/src/agent.rs`
- `ptoken-sdk/src/x402.rs`
- `ptoken-sdk/src/bonding_curve.rs`
- `ptoken-sdk/src/perpetuals.rs`

These modules are designed for reuse by future Solana programs and off-chain p-agent runners. They do not claim that a deployed on-chain market exists yet.

## Agent Policy

Agent policies bind an agent id, owner, capability flags, spending limit, and risk limit. Trading capabilities require a nonzero spending limit. Risk limits above 10,000 bps are rejected.

## x402

The x402 helper models the facts a program should verify after an HTTP-native payment flow settles: asset, amount, recipient, route hash, status, and expiry. It deliberately verifies receipts rather than performing HTTP negotiation on-chain.

## Bonding Curves

The bonding-curve module supports:

- Linear marginal pricing.
- Total buy quotes over a linear curve.
- Constant-product buy and sell quotes with fee handling.

All quote paths use checked arithmetic and u128 intermediates.

## Perpetuals

The perpetuals module supports:

- Long and short position state.
- Leverage in basis points.
- Mark-price PnL.
- Maintenance-margin liquidation checks.
- Signed funding payments.

## Verification

Kani harnesses cover valid and invalid agent policies, accepted and rejected x402 receipts, flat and increasing curve paths, zero and nonzero curve buys, profitable and losing perpetual positions, and funding sign direction.
