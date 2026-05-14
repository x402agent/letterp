# Agent SDK

TypeScript helpers for treating a Metaplex Core asset as a P Agent. The SDK derives the asset signer PDA, builds registration metadata, wraps launchpad instructions in Core Execute, and provides a small HTTP client for the launchpad API.

## Files

| File | Purpose |
|------|---------|
| `src/index.ts` | Package barrel export. |
| `src/p-agent.ts` | `PAgent` class for registration, token launch, buy, sell, and delegation instructions. |
| `src/execute.ts` | Low-level Metaplex Core Execute wrapper. |
| `src/p-nft.ts` | Core asset mint instruction builder and agent collection PDA helper. |
| `src/p-registry.ts` | ERC-8004-style agent registration document builder, fetcher, and validator. |
| `src/client.ts` | Launchpad HTTP client with a reference x402 retry flow. |

## Build

```bash
npm run build -w @x402pt/agent-sdk
```

## Basic Use

```ts
import { Connection, PublicKey } from "@solana/web3.js";
import { PAgent } from "@x402pt/agent-sdk";

const connection = new Connection("https://api.devnet.solana.com", "confirmed");
const agent = PAgent.fromAsset(new PublicKey("<core-asset>"), connection);

console.log(agent.signerPda.toBase58());

const launchIxs = await agent.launchToken({
  name: "Agent Token",
  symbol: "AGNT",
  uri: "https://example.com/token.json",
  creatorFeeBps: 100,
});
```

## Notes

- `PAgent` returns unsigned `TransactionInstruction` objects. Callers still assemble, sign, and submit transactions.
- The asset signer PDA has no private key. Agent actions are wrapped through Metaplex Core Execute.
- `LaunchpadClient` currently uses a reference dummy payment signature for x402 retries; production clients should create and submit a real USDC transfer.
