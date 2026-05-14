# Launchpad

TypeScript launchpad package for bonding-curve token launches, x402-gated HTTP endpoints, curve-state reads, and unsigned Solana instruction builders.

## Exports

| Area | Exports |
|------|---------|
| Curve math | `DEFAULT_CURVE`, `quoteBuy`, `quoteSell`, `applyBuy`, `applySell`, `isReadyToGraduate`, `spotPrice` |
| State reads | `makeConnection`, `fetchCurveState`, `decodeCurveState` |
| Instruction builders | `buildCreateLaunch`, `buildCreateAgentToken`, `buildBuy`, `buildSell`, `buildGraduate`, `buildClaimCreatorFees`, registration and delegation builders |
| Server | `src/server.ts` runs the launchpad HTTP API |

## HTTP API

| Method | Path | Payment | Purpose |
|--------|------|---------|---------|
| `POST` | `/launch` | No | Builds launch instructions for classic or agent token launches. |
| `GET` | `/curve/:mint` | Yes | Reads decoded curve state. |
| `GET` | `/curve/:mint/quote` | Yes | Quotes buy or sell output. |
| `POST` | `/buy` | Yes | Returns an unsigned buy instruction. |
| `POST` | `/sell` | Yes | Returns an unsigned sell instruction. |

## Build and Run

```bash
npm run build -w @x402pt/launchpad
node launchpad/dist/server.js
```

The default API port is `4400`.

## Environment

| Variable | Default | Purpose |
|----------|---------|---------|
| `PORT` | `4400` | Launchpad HTTP port. |
| `SOLANA_NETWORK` | `solana` | Selects mainnet or devnet defaults. |
| `HELIUS_RPC_URL` | unset | Preferred RPC endpoint. |
| `SOLANA_RPC_URL` | unset | Fallback RPC endpoint. |
| `HELIUS_API_KEY` | unset | Builds a Helius RPC URL when explicit URLs are not set. |
| `FACILITATOR_URL` | `http://localhost:4402` | x402 verification and settlement service. |
| `PROTOCOL_TREASURY` | system program ID | Fallback `payTo` address for paid endpoints. |

## Notes

`/launch` returns unsigned instructions and a generated mint secret in this reference implementation. Production code should use a partial-signing or wallet-mediated flow instead of returning a secret key.
