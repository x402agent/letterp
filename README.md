<div align="center">

<img src="assets/p.svg" alt="Animated ASCII art letter P" width="640">

[![Solana](https://img.shields.io/badge/Solana-9945FF?style=flat-square&logo=solana&logoColor=white)](https://solana.com)
[![p-token](https://img.shields.io/badge/p--token-SIMD--0266-blueviolet?style=flat-square)](https://solana.com/upgrades/p-token)
[![Metaplex](https://img.shields.io/badge/Metaplex-Core%20%2B%20Genesis-FF6B35?style=flat-square)](https://metaplex.com)
[![x402](https://img.shields.io/badge/x402-00D4AA?style=flat-square)](https://x402.org)
[![MIT](https://img.shields.io/badge/license-MIT-yellow?style=flat-square)](LICENSE)

</div>

---

# P-Token × x402 Changes Everything for AI Agents on Solana

The economics of AI agents on-chain just flipped overnight.

For the first time, real per-token AI billing on Solana is actually viable — not theoretical, not batched behind the scenes, not "Web2 with crypto branding."

Real streaming micropayments. Real agent-to-agent commerce.

And it's happening through OpenClawd's x402 stack, powered by p-token.

---

Before p-token, every AI payment on Solana carried hidden friction.

An agent making a tiny payment for inference still had to burn thousands of compute units just to move tokens. A standard SPL `TransferChecked` consumed around **6,200 CU** before ATA checks, memos, or priority fees were even included. The transaction cost could eat most of the payment itself.

So the entire "AI micropayment" narrative had a problem nobody wanted to admit: the infrastructure overhead was too expensive for true per-token metering.

Then SIMD-0266 shipped p-token:

```
TransferChecked   6,200 CU  →    105 CU   (98.3% cheaper)
Transfer          4,645 CU  →     76 CU   (98.4% cheaper)
Approve           2,904 CU  →    124 CU   (95.7% cheaper)
MintTo            4,128 CU  →  2,012 CU   (51.3% cheaper)
Burn              4,753 CU  →  1,884 CU   (60.4% cheaper)
```

That is not a minor optimization. That is a protocol-level phase transition.

---

## The Stack

OpenClawd immediately integrated p-token directly into the x402 payment stack powering autonomous AI agents on Solana:

| Module | What it does |
|--------|-------------|
| `x402/p-token.ts` | P-token program helpers, CU-tuned compute budgets, batch instruction builder |
| `x402/p-token-stream-facilitator.ts` | Metered per-output-token billing with atomic / batched / streamed settlement |
| `clawdFetch` | Auto p-token support — agents pay 402 challenges transparently |
| `pay.solanaclawd.com` | Live facilitator relay — blind routing, no payer exposure |
| `@x402pt/agent-sdk` | `PAgent` class — Core NFT identity, PDA wallet, no private key |
| `@x402pt/launchpad` | Constant-product bonding curves, p-token minting, graduation |
| `@x402pt/perps` | Graduated tokens listed as Drift / Adrena perp markets |

No config changes. No migration pain. Agents using the OpenClawd x402 stack automatically inherit the efficiency improvements.

---

## Per-Output-Token AI Billing

This is the unlock.

An AI model can now charge per generated token with only ~1% overhead instead of losing the majority of revenue to settlement costs.

That means:

- AI APIs can charge fractions of a cent
- agents can stream payments in real time
- multi-agent workflows become economically composable
- inference can settle continuously during generation
- failed or interrupted generations only bill for consumed output

The new `PTokenStreamFacilitator` implements a metered x402 payment scheme:

```typescript
// 1. Agent receives a METER challenge
const challenge = facilitator.issueChallenge({ payerPubkey, maxTokens: 2048 });

// 2. Tokens stream — usage metered live
await facilitator.meter(sessionId, tokensConsumed);

// 3. Settlement: single p-token tx regardless of how many sessions
const results = await facilitator.settleBatch(sessionIds);
// CU used: 1,000 + (N × 105) vs N × 6,200 with SPL Token
```

Because p-token supports batch instructions, multiple concurrent agent sessions settle together with dramatically reduced compute overhead:

| Recipients | SPL Token CU | P-Token Batch CU | Savings |
|-----------|-------------|-----------------|---------|
| 1 | 6,200 | 1,025 | 83% |
| 3 | 18,600 | 1,075 | 94% |
| 10 | 62,000 | 1,250 | **98%** |

---

## The Economics

A busy AI agent making 1,000 API calls per day reduces annual transaction overhead from:

```
$244 / year  →  $20 / year
```

per agent. Multiply across fleets of autonomous agents running 24/7 and the savings become enormous.

But more importantly — entirely new behaviors become possible. Agents can now:

- pay each other dynamically
- coordinate inference workloads
- stream payments during reasoning
- route tasks across decentralized providers
- compose recursive economic workflows on-chain

---

## P-Agents

Every agent in the stack is a **Metaplex Core NFT** with no private key:

```
Owner mints Core NFT  →  Asset Signer PDA  ←─  no private key, ever
                                │
                          seeds: [b"mpl-core-execute", asset_pubkey]
                                │
                    ┌───────────┼────────────┐
                    │           │            │
              holds SOL    signs via     launches
              + tokens     Core Execute   P-tokens
              (treasury)   CPI            (creator fees
                                          → PDA auto)
```

```typescript
import { PAgent, buildMintCoreAsset, buildAgentRegistrationDoc } from "@x402pt/agent-sdk";

// mint the agent's identity NFT
const { asset, instruction } = buildMintCoreAsset(payer, {
  name: "Plexpert",
  uri: "https://arweave.net/plexpert.json",
});

// keyless treasury — no private key required
const agent = PAgent.fromAsset(asset, connection);
console.log(agent.signerPda.toBase58());

// launch a token FROM the agent (creator fees auto-route to signerPda)
const ixs = await agent.launchToken({
  name: "Plexpert Token",
  symbol: "PLEX",
  uri: "https://arweave.net/plex-meta.json",
  creatorFeeBps: 100,
});

// agent buys via Core Execute — no wallet signing required
const buyIx = await agent.buy(mint, new BN(1e8), new BN(1e6));

// delegate to a hot-wallet for autonomous operation
const slot = await connection.getSlot();
const delegateIx = await agent.delegateTo(operatorWallet, slot + 50000);
```

---

## Bonding Curves

Constant-product curves — written in Pinocchio, launched in seconds:

```
k = virtual_token_reserves × virtual_sol_reserves

Buy:   tokens_out = token_reserves − (k / (sol_reserves + sol_in))
Sell:  sol_out    = sol_reserves   − (k / (token_reserves + tokens_in))

Initial reserves:  30 SOL virtual  ·  793.1B tokens virtual
Creator fee:       1% → Asset Signer PDA
Graduation:        85 SOL raised → auto-migrates to Raydium CPMM
```

```bash
# plan a launch (unsigned — no wallet required)
npm run ptoken:launch-plan -- --symbol PLEX --name "Plexpert Token"

# simulate a buy
npm run ptoken:curve-quote -- --virtual-sol 30 --virtual-token 1073000000 --sol 1
```

---

## Pinocchio Programs

The on-chain bonding curve is written in native Rust using Pinocchio — zero dependencies, zero-copy, zero compromise:

```
PDA seeds:
  [b"global"]
  [b"bonding-curve", mint]
  [b"bonding-curve", mint, b"vault"]
  [b"agent", owner]
  [b"agent-token", mint]
  [b"creator-vault", creator]
  [b"exec-delegation", agent, delegate]
```

```bash
cd programs && cargo build-sbf
solana program deploy target/deploy/x402_bonding_curve.so
export P_TOKEN_LAUNCHPAD_PROGRAM_ID=<deployed-id>
```

---

## Start

```bash
npm install && npm run build

# x402 facilitator — port 4402
cd facilitator && npm start

# launchpad API — port 4400
cd launchpad && npx ts-node src/server.ts

# P-Agent full example
cd examples && npx ts-node src/p-agent-full.ts
```

```bash
export HELIUS_API_KEY=your-key
export P_TOKEN_PROGRAM_ID=ptok6rngomXrDbWf5v5Mkmu5CEbB51hzSCPDoj9DrvF
export P_TOKEN_LAUNCHPAD_PROGRAM_ID=<deployed>
export SOLANA_NETWORK=solana-devnet
export USE_P_TOKEN=1
```

---

## Packages

```
@x402pt/shared        types · PDAs · program IDs
@x402pt/launchpad     bonding curve math · tx builders · graduation
@x402pt/agent-sdk     PAgent · NFT mint · ERC-8004 registry helpers
@x402pt/facilitator   x402 verify/settle server (port 4402)
@x402pt/perps         Drift/Adrena perp market adapter
```

---

## Docs

- [`docs/P_AGENTS.md`](docs/P_AGENTS.md) — P-Agent lifecycle, PDA seeds, SDK reference
- [`docs/PROTOCOL.md`](docs/PROTOCOL.md) — byte-level instruction specs and account layouts
- [`pinocchio/P_TOKEN.md`](pinocchio/P_TOKEN.md) — CU benchmarks and Pinocchio zero-copy design
- [`pinocchio/docs/P_TOKEN_LAUNCHES.md`](pinocchio/docs/P_TOKEN_LAUNCHES.md) — bonding curve planning workflow
- [`pinocchio/docs/P_AGENT_TOKEN.md`](pinocchio/docs/P_AGENT_TOKEN.md) — p-token agent token templates

---

## This is the beginning of a true agentic economy.

OpenClawd is building the infrastructure layer for that future — an open payment rail for autonomous AI systems running on Solana.

Powered by OpenClawd · x402 · Solana · p-token · streaming inference settlement · autonomous agents · metered AI commerce.

And yes — powered by **$CLAWD**.

> *The shell molts. The laws do not.*

**$CLAWD** `8cHzQHUS2s2h8TzCmfqPKYiM4dSt4roa3n7MyRLApump`

| | |
|---|---|
| GitHub | [github.com/x402agent/solana-clawd](https://github.com/x402agent/solana-clawd) |
| Site | [solanaclawd.com](https://solanaclawd.com) |
| Facilitator | [pay.solanaclawd.com](https://pay.solanaclawd.com) |

<div align="center">

```
██████╗ 
██╔══██╗
██████╔╝
██╔═══╝ 
██║     
╚═╝     
```

</div>
