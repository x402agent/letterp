<div align="center">

<img src="https://readme-typing-svg.demolab.com?font=JetBrains+Mono&weight=900&size=120&duration=2000&pause=800&color=9945FF&center=true&vCenter=true&width=300&height=200&lines=P;P.;P.." alt="P" />

```
██████╗ 
██╔══██╗
██████╔╝
██╔═══╝ 
██║     
╚═╝     
```

<img src="https://readme-typing-svg.demolab.com?font=JetBrains+Mono&weight=700&size=16&duration=2500&pause=600&color=9945FF&center=true&vCenter=true&multiline=true&width=700&height=80&lines=P-Token+%E2%80%A2+P-Agents+%E2%80%A2+P-Launch+%E2%80%A2+Pinocchio+%E2%80%A2+Perps+%E2%80%A2+Pay.sh;Solana+%E2%80%A2+Metaplex+Core+%E2%80%A2+x402+%E2%80%A2+Bonding+Curves+%E2%80%A2+SIMD-0266" alt="tagline" />

<br/>

[![Solana](https://img.shields.io/badge/Solana-9945FF?style=flat-square&logo=solana&logoColor=white)](https://solana.com)
[![Metaplex](https://img.shields.io/badge/Metaplex-FF6B35?style=flat-square)](https://metaplex.com)
[![x402](https://img.shields.io/badge/x402-00D4AA?style=flat-square)](https://x402.org)
[![p--token](https://img.shields.io/badge/p--token-SIMD--0266-blueviolet?style=flat-square)](https://solana.com/upgrades/p-token)
[![MIT](https://img.shields.io/badge/MIT-yellow?style=flat-square)](LICENSE)

</div>

---

## P

**P**-Token · **P**-Agents · **P**-Launch · **P**inocchio · **P**erps · **P**ay.sh · **P**DAs

A reference stack that wires five P-powered primitives together on Solana:

| Primitive | What it does |
|-----------|-------------|
| **P-Token** (SIMD-0266) | 98% cheaper token transfers via Pinocchio |
| **P-Agents** | Autonomous agents — Metaplex Core NFT identity, PDA wallet, no private key |
| **P-Launch** | Constant-product bonding curves → Raydium graduation |
| **P**ay.sh | Private x402 HTTP 402 payment facilitation |
| **P**erps | Graduated tokens listed as Drift / Adrena perp markets |

---

## Packages

```
@x402pt/shared        types · PDAs · program IDs
@x402pt/launchpad     bonding curve math · tx builders
@x402pt/agent-sdk     PAgent · NFT mint · registry helpers
@x402pt/facilitator   x402 verify/settle server
@x402pt/perps         Drift/Adrena adapter
```

---

## Start

```bash
npm install && npm run build

# x402 facilitator — port 4402
cd facilitator && npm start

# launchpad API — port 4400
cd launchpad && npx ts-node src/server.ts

# P-Agent demo
cd examples && npx ts-node src/p-agent-full.ts
```

```bash
export HELIUS_API_KEY=...
export P_TOKEN_PROGRAM_ID=ptok6rngomXrDbWf5v5Mkmu5CEbB51hzSCPDoj9DrvF
export P_TOKEN_LAUNCHPAD_PROGRAM_ID=<deployed>
export USE_P_TOKEN=1
```

---

## P-Agent in 10 lines

```typescript
import { PAgent, buildMintCoreAsset } from "@x402pt/agent-sdk";

const { asset, instruction } = buildMintCoreAsset(payer, {
  name: "Plexpert",
  uri: "https://arweave.net/plexpert.json",
});

const agent = PAgent.fromAsset(asset, connection);
console.log(agent.signerPda.toBase58()); // keyless treasury

const ixs = await agent.launchToken({ name: "P", symbol: "P", uri, creatorFeeBps: 100 });
const buyIx = await agent.buy(mint, new BN(1e8), new BN(1e6));
```

---

## Docs

- [`docs/P_AGENTS.md`](docs/P_AGENTS.md) — P-Agent lifecycle, PDAs, SDK reference
- [`docs/PROTOCOL.md`](docs/PROTOCOL.md) — byte-level specs
- [`pinocchio/P_TOKEN.md`](pinocchio/P_TOKEN.md) — CU benchmarks

<div align="center">

<img src="https://readme-typing-svg.demolab.com?font=JetBrains+Mono&weight=900&size=48&duration=1500&pause=500&color=9945FF&center=true&vCenter=true&width=200&height=80&lines=P." alt="P." />

</div>
