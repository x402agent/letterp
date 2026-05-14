<div align="center">

```
██████╗     ██████╗     ██████╗     ██╗     ██╗     ██████╗ 
██╔══██╗    ██╔══██╗    ██╔══██╗    ██║     ██║    ██╔═══██╗
██████╔╝    ██████╔╝    ██████╔╝    ██║     ██║    ██║   ██║
██╔═══╝     ██╔═══╝     ██╔═══╝     ██║     ██║    ██║   ██║
██║         ██║         ██║         ███████╗██║    ╚██████╔╝
╚═╝         ╚═╝         ╚═╝         ╚══════╝╚═╝     ╚═════╝ 
```

**P**inocchio · **P**-Token · **P**-Agents · **P**erps · **P**DAs · **P**rotocol · **P**ay.sh

[![Solana](https://img.shields.io/badge/Solana-SIMD--0266-9945FF?style=for-the-badge&logo=solana&logoColor=white)](https://solana.com/upgrades/p-token)
[![Metaplex](https://img.shields.io/badge/Metaplex-Core%20%2B%20Genesis-FF6B35?style=for-the-badge)](https://www.metaplex.com)
[![x402](https://img.shields.io/badge/x402-HTTP%20402%20Payments-00D4AA?style=for-the-badge)](https://x402.org)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.7-3178C6?style=for-the-badge&logo=typescript&logoColor=white)](https://www.typescriptlang.org)
[![Rust](https://img.shields.io/badge/Rust-Pinocchio-CE422B?style=for-the-badge&logo=rust&logoColor=white)](https://github.com/anza-xyz/pinocchio)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](LICENSE)

> *Everything starts with **P**.*

</div>

---

## P is for Protocol

```
                        ┌──────────────────────────────────────┐
                        │                                        │
                        │   P  P  P  P  P  P  P  P  P  P       │
                        │   │                                    │
                        │   │  THE LETTER P STACK ON SOLANA     │
                        │   │                                    │
       ┌────────────────▼────────────────────────────────────┐  │
       │  P-AGENTS  ←  Autonomous AI agents, PDA wallets     │  │
       │  P-TOKENS  ←  SIMD-0266, 98% cheaper transfers     │  │
       │  P-LAUNCH  ←  Bonding curves + graduation           │  │
       │  P-ERPS    ←  Perp markets on graduated tokens      │  │
       │  PAY.SH    ←  Private x402 facilitator              │  │
       │  PINOCCHIO ←  Zero-copy native Solana programs      │  │
       │  PDAs      ←  Program Derived Addresses everywhere  │  │
       └─────────────────────────────────────────────────────┘
```

Five P-powered primitives. One repo. Infinite **P**ossibilities.

---

## P is for Primitives

### P-Token (SIMD-0266)

The Pinocchio token **P**rogram that makes every transfer nearly free:

| Operation | SPL Token | P-Token | Savings |
|-----------|-----------|---------|---------|
| TransferChecked | 6,200 CU | 105 CU | **98.3%** |
| Transfer | 4,645 CU | 76 CU | **98.4%** |
| MintTo | 4,128 CU | 2,012 CU | **51.3%** |
| Burn | 4,753 CU | 1,884 CU | **60.4%** |
| Approve | 2,904 CU | 124 CU | **95.7%** |

Same token **P**rogram ID (`Tokenkeg…VQ5DA`). Same ATAs. Same wallets. Just **P**ure P-erformance.

```bash
export USE_P_TOKEN=1
export P_TOKEN_PROGRAM_ID=ptok6rngomXrDbWf5v5Mkmu5CEbB51hzSCPDoj9DrvF
```

### P-Agents

Autonomous AI agents that own their own on-chain wallets — **P**owered by Metaplex Core:

```
Agent owner mints Core NFT
         │
         ▼
  Asset Signer PDA  ←── No private key. Ever.
  seeds: [b"mpl-core-execute", asset_pubkey]
         │
         ├── Holds SOL + tokens (the agent treasury)
         ├── Signs via Core Execute CPI
         ├── Launches P-Tokens (creator fees → PDA)
         ├── Buys/sells on bonding curves
         └── Graduates to Perps
```

```typescript
import { PAgent } from "@x402pt/agent-sdk";

const agent = PAgent.fromAsset(assetAddress, connection);

// The agent's wallet — no private key required
console.log(agent.signerPda.toBase58());

// Launch a token FROM the agent
const ixs = await agent.launchToken({
  name: "Plexpert",
  symbol: "PLEX",
  uri: "https://arweave.net/plexpert-metadata.json",
  creatorFeeBps: 100,
});
```

### P-Launch (Bonding Curves)

Pump-style constant-**P**roduct bonding curves — written in **P**inocchio, launched in seconds:

```
k = virtual_token_reserves × virtual_sol_reserves

Buy:   tokens_out = token_reserves − (k / (sol_reserves + sol_in))
Sell:  sol_out    = sol_reserves   − (k / (token_reserves + tokens_in))

Initial reserves:  30 SOL virtual · 793.1B tokens virtual
Fee:               1% (100 bps) → creator PDA
Graduation:        85 SOL raised → auto-migrates to Raydium CPMM
```

### P-erps

Once a **P**-token graduates from the bonding curve, the **P**erps adapter lists it as a market on Drift or Adrena — giving holders leveraged exposure without leaving the **P** ecosystem.

### **P**ay.sh

**P**rivate x402 **P**ayment facilitation. Every launchpad API call can return `402 Payment Required`. Agents **P**ay in USDC or their own token. **P**ay.sh blinds the payer from the resource server:

```
Agent → pay.sh relay → resource endpoint
          ↕ blind signature
    Solana p-token USDC settlement (105 CU)
```

---

## P is for Programs

```
P-TOKEN LAUNCHPAD PROGRAM
  ID:     ptok6rngomXrDbWf5v5Mkmu5CEbB51hzSCPDoj9DrvF (feature-gated)
  Lang:   Rust + Pinocchio (zero-copy, no Anchor)
  PDAs:   [b"global"]
          [b"bonding-curve", mint]
          [b"bonding-curve", mint, b"vault"]
          [b"agent", owner]
          [b"agent-token", mint]
          [b"creator-vault", creator]
          [b"exec-delegation", agent, delegate]

METAPLEX CORE
  ID:     CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d
  PDAs:   [b"mpl-core-execute", asset]  ← agent wallet

METAPLEX AGENT REGISTRY
  ID:     (see @metaplex-foundation/mpl-agent-registry)
  PDAs:   [b"agent-identity-v1", asset]

CORE CANDY MACHINE
  ID:     CMACYFENjoBMHzapRXyo1JZkVS6EtaDDzkjMrmQLvr4J
  Guard:  CMAGAKJ67e9hRZgfC5SFTbZH8MgEmtqazKXjmkaJjWTJ
```

---

## P is for Project Layout

```
letterp/
├── shared/          ← P-shared types, PDAs, program IDs
├── facilitator/     ← x402 /verify /settle /supported — P-ayment server
├── launchpad/       ← Bonding curve math + Solana tx builders
├── agent-sdk/       ← P-Agent class, NFT minting, registry helpers
│   └── src/
│       ├── p-agent.ts       ← PAgent lifecycle class
│       ├── p-nft.ts         ← Core NFT minting (agent identity)
│       ├── p-registry.ts    ← ERC-8004 agent registration helpers
│       ├── execute.ts       ← Core Execute CPI wrapper
│       └── client.ts        ← HTTP client with x402 auto-pay
├── perps/           ← P-erp market adapter (Drift / Adrena)
├── programs/        ← Rust on-chain program (P-inocchio style)
├── pinocchio/       ← P-inocchio program map + p-token docs
├── examples/        ← Runnable end-to-end demos
└── docs/            ← P-rotocol specs and deep dives
    ├── PROTOCOL.md
    └── P_AGENTS.md  ← P-Agent infrastructure guide
```

---

## P is for Prerequisites

**P**ick your environment:

```bash
# P-Token program (mainnet — feature-gated)
export P_TOKEN_PROGRAM_ID=ptok6rngomXrDbWf5v5Mkmu5CEbB51hzSCPDoj9DrvF

# Your deployed launchpad program (after running: solana program deploy)
export P_TOKEN_LAUNCHPAD_PROGRAM_ID=<your-program-id>

# Helius RPC (recommended for staked connections + getAsset)
export HELIUS_API_KEY=your-helius-api-key
# or:
export HELIUS_RPC_URL=https://devnet.helius-rpc.com/?api-key=...

# Network
export SOLANA_NETWORK=solana-devnet   # or solana-mainnet

# Enable p-token (default: 1)
export USE_P_TOKEN=1

# pay.sh facilitator (optional)
export FACILITATOR_SECRET_KEY=<base58-secret-key>
export USDC_MINT=4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU
```

---

## P is for Putting it Together

```bash
# 1. Install
npm install

# 2. Build all packages
npm run build

# 3. Start the x402 facilitator (port 4402)
cd facilitator && npm start

# 4. Start the launchpad API (port 4400)
cd launchpad && npx ts-node src/server.ts

# 5. Run the P-Agent full example
cd examples && npx ts-node src/p-agent-full.ts

# 6. Run the classic token launch example
cd examples && npx ts-node src/launch-token.ts
```

---

## P is for the Path (End-to-End Flow)

```
┌─────────┐
│  Agent  │  1. Owner mints Core NFT (MPL Core)
│  owner  │     → gets asset address + Asset Signer PDA
└────┬────┘
     │
     │  2. registerIdentityV1 → Agent PDA on-chain
     │     URI points to ERC-8004 registration JSON
     ▼
┌──────────────────┐
│ Metaplex Registry │  Asset is now a verifiable P-Agent
└────────┬─────────┘
         │
         │  3. POST /launch  (signed via Core Execute)
         │     name, symbol, uri, creatorFeeBps
         ▼
┌───────────────────┐  4. CreateMint (p-token) + init bonding curve
│  Launchpad server │ ─────────────────────────────────────▶ Solana
└───────┬───────────┘
        │
        │  5. Buyers hit GET /curve/:mint
        │     → HTTP 402 → pay 0.001 USDC → get curve data
        ▼
┌───────────────────┐  6. x402 /verify + /settle via Helius
│  pay.sh / x402    │ ─────────────────────────────────────▶ Helius RPC
└───────┬───────────┘
        │
        │  7. Curve fills (85 SOL raised)
        │     → graduation event fires
        ▼
┌───────────────────┐  8. Seed Raydium CPMM pool
│  Graduation       │ ─────────────────────────────────────▶ Raydium
└───────┬───────────┘
        │
        │  9. List as perp market
        ▼
┌───────────────────┐
│  Perps adapter    │  P-Token is now a perp market on Drift/Adrena
└───────────────────┘
```

---

## P is for Packages

| Package | What it does |
|---------|-------------|
| `@x402pt/shared` | Types, program IDs, PDA derivation — the **P**illars |
| `@x402pt/launchpad` | Bonding curve math, tx builders, graduation — the **P**ump |
| `@x402pt/agent-sdk` | PAgent class, NFT mint, registry — the **P**erson |
| `@x402pt/facilitator` | x402 verify/settle server — the **P**aymaster |
| `@x402pt/perps` | Drift/Adrena market adapter — the **P**erp |
| `@x402pt/examples` | End-to-end demos — the **P**layground |

---

## P is for P-Agent SDK Quick Reference

```typescript
import { PAgent, buildMintCoreAsset, buildAgentRegistrationDoc } from "@x402pt/agent-sdk";

// ── Mint an agent identity (Core NFT) ─────────────────────────────────
const { asset, assetSigner, instruction, signers } = buildMintCoreAsset(payer, {
  name: "Plexpert",
  uri: "https://arweave.net/plexpert.json",
  collection: collectionAddress,
});

// ── Create a P-Agent from the asset ───────────────────────────────────
const agent = PAgent.fromAsset(asset, connection);
console.log("P-Agent wallet:", agent.signerPda.toBase58()); // no private key!

// ── Register on-chain identity ─────────────────────────────────────────
const registerIx = await agent.register("https://arweave.net/plexpert-reg.json");

// ── Launch a P-Token ──────────────────────────────────────────────────
const launchIxs = await agent.launchToken({
  name: "Plexpert Token",
  symbol: "PLEX",
  uri: "https://arweave.net/plex-meta.json",
  creatorFeeBps: 100,    // 1% → routes to agent.signerPda automatically
});

// ── Buy from bonding curve (agent pays from its PDA treasury) ─────────
const buyIx = await agent.buy(mint, new BN(100_000_000), new BN(1_000_000));

// ── Delegate execution to an operator wallet ──────────────────────────
const currentSlot = await connection.getSlot();
const delegateIx = await agent.delegateTo(operatorWallet, currentSlot + 5000);

// ── Build the ERC-8004 registration document ──────────────────────────
const doc = buildAgentRegistrationDoc(asset.toBase58(), {
  name: "Plexpert",
  description: "A P-powered agent for Metaplex protocols",
  image: "https://arweave.net/plexpert-avatar.png",
  model: "claude-sonnet-4-6",
  capabilities: ["trade", "launch", "distribute"],
  endpoint: "https://api.example.com/plexpert",
  services: [
    { name: "A2A", endpoint: "https://api.example.com/plexpert/agent-card.json", version: "0.3.0" },
    { name: "MCP", endpoint: "https://api.example.com/plexpert/mcp", version: "2025-06-18" },
  ],
  active: true,
  registrations: [{ agentId: asset.toBase58(), agentRegistry: "solana:101:metaplex" }],
  supportedTrust: ["reputation", "crypto-economic"],
});
```

---

## P is for P-Agent vs Genesis

| Feature | Metaplex Genesis | P-Token Launch Pad |
|---------|-----------------|-------------------|
| Token **P**rogram | SPL Token | P-Token (98% cheaper) |
| Agent identity | MPL Core asset | MPL Core asset + PDA registry |
| Creator fee routing | Asset Signer PDA | Asset Signer PDA |
| **P**rogram dependency | Metaplex API (hosted) | Self-hosted launchpad |
| DEX graduation | Raydium CPMM | Any DEX (Raydium / Orca / Meteora) |
| **P**erps integration | ❌ | ✅ Drift + Adrena adapter |
| **P**ay.sh | ❌ | ✅ Private x402 facilitator |
| Batch fee distribution | Sequential SPL | P-Token batch opcode 25 |
| CU: batch 10 recipients | 62,000 | 1,250 (98% savings) |

---

## P is for Pinocchio (The On-Chain Program)

The bonding curve **P**rogram is written in native Rust using Pinocchio — zero dependencies, zero-copy, zero compromise:

```toml
[dependencies]
pinocchio       = { path = "pinocchio/pinocchio-main/sdk" }
pinocchio-token = { path = "pinocchio/pinocchio-main/programs/token" }

[profile.release]
opt-level = "z"    # P-erformance
lto       = true   # P-recision
```

```bash
# Build the on-chain program
cd programs
cargo build-sbf

# Deploy to devnet
solana program deploy \
  --program-id target/deploy/x402_bonding_curve-keypair.json \
  target/deploy/x402_bonding_curve.so
```

---

## P is for Protocol Documentation

| Doc | Contents |
|-----|----------|
| [`docs/PROTOCOL.md`](docs/PROTOCOL.md) | Byte-level instruction specs, account layouts, error codes |
| [`docs/P_AGENTS.md`](docs/P_AGENTS.md) | P-Agent infrastructure guide — lifecycle, PDAs, SDK reference |
| [`pinocchio/P_TOKEN.md`](pinocchio/P_TOKEN.md) | P-Token CU benchmarks and Pinocchio zero-copy design |
| [`pinocchio/README.md`](pinocchio/README.md) | Pinocchio program map and MCP tools |
| [`pinocchio/docs/P_TOKEN_LAUNCHES.md`](pinocchio/docs/P_TOKEN_LAUNCHES.md) | Bonding curve planning and launch workflow |
| [`pinocchio/docs/P_AGENT_TOKEN.md`](pinocchio/docs/P_AGENT_TOKEN.md) | P-Token agent token planning and templates |

---

## P is for Possible (Roadmap)

- [x] **P**-Token support (SIMD-0266, feature-gated)
- [x] **P**-Agent class with Core Execute CPI wrapper
- [x] **P**-Launch bonding curve (constant-product, pump-compatible)
- [x] **P**ay.sh private x402 facilitator
- [x] **P**erps adapter (Drift / Adrena)
- [x] **P**inocchio on-chain program skeleton
- [x] Metaplex Core NFT minting for agent identity
- [x] ERC-8004 agent registration document builder
- [ ] **P**-Token batch fee distribution (opcode 25) on mainnet
- [ ] Core Candy Machine integration (batch-mint agent NFT collections)
- [ ] **P**-Token Token-2022 extension support (metadata pointer, transfer fee)
- [ ] Confidential transfers via **P**ay.sh + SPL CT
- [ ] On-chain governance via agent token voting
- [ ] MCP server for agent discovery and launchpad interaction

---

## P is for Provenance

Built on:

- [**P**-Token / SIMD-0266](https://solana.com/upgrades/p-token) — Pinocchio token program by Febo (Anza)
- [**P**inocchio](https://github.com/anza-xyz/pinocchio) — Zero-dependency native Solana programs
- [Metaplex Core](https://developers.metaplex.com/core) — Single-account NFT standard
- [Metaplex Agent Registry](https://developers.metaplex.com/agents) — On-chain agent identity
- [Metaplex Genesis](https://developers.metaplex.com/genesis) — Bonding curve token launches
- [x402](https://x402.org) — HTTP 402 payment protocol
- [Helius](https://helius.dev) — Solana RPC with staked connections + `getAsset`
- [Raydium CPMM](https://raydium.io) — Post-graduation DEX liquidity

---

<div align="center">

**P** · **P** · **P** · **P** · **P** · **P** · **P** · **P** · **P** · **P** · **P** · **P** · **P** · **P** · **P**

*The only letter that matters on Solana.*

[![Built for Solana](https://img.shields.io/badge/Built%20for-Solana-9945FF?style=flat-square&logo=solana)](https://solana.com)
[![Powered by P-Token](https://img.shields.io/badge/Powered%20by-P--Token-blueviolet?style=flat-square)](https://solana.com/upgrades/p-token)
[![MIT License](https://img.shields.io/badge/license-MIT-green?style=flat-square)](LICENSE)

</div>
