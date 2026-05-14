# x402-ptoken-launchpad

A reference implementation that stitches four things together on Solana:

1. **x402 facilitator** — verifies and settles HTTP 402 payment challenges (Coinbase x402 spec) using p-token transfers on Solana via Helius RPC.
2. **P-token bonding-curve launchpad** — lets anyone (or an autonomous agent) launch a token using Solana's new SIMD-0266 p-token program, with a constant-product bonding curve and a graduation path.
3. **Metaplex agent integration** — uses the Metaplex Agent Registry (`mpl-agent-registry`) so an agent's PDA wallet (the Asset Signer) can mint and own tokens it launches.
4. **Perps adapter** — exposes the launched p-token as collateral / a market on a perp DEX (Drift / Adrena adapter pattern) once the curve graduates.

The repo is a TypeScript workspace plus a thin Rust crate skeleton for the on-chain bonding curve program (Pinocchio-style, no Anchor).

---

## Why these pieces together

- **p-token (SIMD-0266)** lands on mainnet ~May 2026 and cuts CU usage by ~95–98% on every token op. A launchpad written against `@solana-program/token@0.13` (the new IDL) gets those savings for free and scales to far higher TPS per block. The token-program ID stays the same (`Tokenkeg…VQ5DA`) because p-token is a runtime swap behind a feature gate, so existing wallets and indexers keep working.
- **x402** gives the launchpad a built-in monetisation primitive: every API call an agent makes against the launchpad (launch a token, query a curve, simulate a buy) can return `402 Payment Required` and be paid for in USDC or the agent's own token. This is how an agent token earns revenue even before it graduates.
- **Metaplex Agent Registry** gives each agent a verifiable on-chain identity and a PDA wallet with no private key. The launchpad treats the agent's Asset Signer PDA as the creator, so creator fees route there automatically and the agent literally controls its own treasury via Core's `Execute` lifecycle hook.
- **Helius** is the RPC. We use it for transaction landing (staked connections), `getAsset` lookups on Core assets, and webhook subscriptions on the bonding-curve program for indexing trades.

---

## Layout

```
shared/        Types + helpers used by every package
facilitator/   Express server implementing the x402 /verify, /settle, /supported endpoints
launchpad/     Bonding-curve math, Solana transaction builders, graduation logic
agent-sdk/     Thin SDK an agent imports to launch & trade its own token
perps/         Adapter that lists a graduated token as a perp market
programs/      Rust source for the on-chain bonding-curve program (Pinocchio)
examples/      Runnable end-to-end demos
docs/          Protocol specifications and documentation
```

---

## Getting Started

```bash
# Install dependencies
npm install

# Build all packages
npm run build

# Start the facilitator (port 4402)
cd facilitator && npm start

# Start the launchpad server (port 4400)
cd launchpad && npx ts-node src/server.ts

# Run an example
cd examples && npx ts-node src/launch-token.ts
```

### Runtime configuration

The TypeScript SDK now defaults to Helius when either `HELIUS_RPC_URL` or
`HELIUS_API_KEY` is present:

```bash
export SOLANA_NETWORK=solana-devnet
export HELIUS_API_KEY=...
# or:
export HELIUS_RPC_URL="https://devnet.helius-rpc.com/?api-key=..."
```

Program ids are env-driven so the same SDK works before and after you deploy
your own Pinocchio launchpad:

```bash
export P_TOKEN_PROGRAM_ID=ptok6rngomXrDbWf5v5Mkmu5CEbB51hzSCPDoj9DrvF
export P_TOKEN_LAUNCHPAD_PROGRAM_ID=<your deployed launchpad program id>
export USE_P_TOKEN=1
```

`USE_P_TOKEN=0` switches token instructions back to classic SPL Token.

---

## End-to-end flow

```
┌─────────┐    1. mint Core asset + register identity     ┌──────────────────┐
│ Agent   │ ──────────────────────────────────────────▶  │ Metaplex Registry │
│ owner   │                                                └──────────────────┘
└─────────┘
     │
     │ 2. POST /launch  (signed by agent's PDA via Core Execute)
     ▼
┌────────────────────┐    3. CreateMint (p-token) + init bonding curve account
│ launchpad service  │ ───────────────────────────────────────────────────────▶ Solana
└────────────────────┘
     │
     │ 4. GET /curve/:mint   ←─ HTTP 402  ─→  pays via X-PAYMENT header
     ▼
┌────────────────────┐    5. /verify + /settle on chain
│ x402 facilitator   │ ──────────────────────────────────────▶ Helius RPC
└────────────────────┘
     │
     │ 6. curve fills → graduation event
     ▼
┌────────────────────┐
│ perps adapter      │  lists token as a perp market
└────────────────────┘
```

See `docs/PROTOCOL.md` for the byte-level specs and `examples/` for runnable code.
