# Facilitator

Express service implementing the reference x402 facilitator endpoints for this workspace. It verifies `X-PAYMENT` headers, marks payment IDs as settled in memory, and reports supported Solana USDC payment settings.

## Endpoints

| Method | Path | Purpose |
|--------|------|---------|
| `GET` | `/supported` | Returns accepted network, asset, scheme, and amount bounds. |
| `POST` | `/verify` | Validates the request body and checks that the payment looks like a Solana transaction signature. |
| `POST` | `/settle` | Marks a payment/resource pair as settled for idempotency. |
| `GET` | `/health` | Basic service health response. |

## Build and Run

```bash
npm run build -w @x402pt/facilitator
npm run start -w @x402pt/facilitator
```

The default port is `4402`.

## Environment

| Variable | Default | Purpose |
|----------|---------|---------|
| `FACILITATOR_PORT` | `4402` | HTTP port. |
| `SOLANA_NETWORK` | `solana` | Selects mainnet or devnet USDC mint. |
| `HELIUS_RPC_URL` | unset | Preferred RPC endpoint. |
| `SOLANA_RPC_URL` | unset | Fallback RPC endpoint. |

## Production Gaps

This is a reference implementation. It does not submit transfers or confirm transaction contents on-chain. A production facilitator should verify the referenced transaction, confirm amount and recipient, enforce freshness, and store idempotency in durable storage.
