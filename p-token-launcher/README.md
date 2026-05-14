# P Token Launcher Workbench

Local unsigned workbench for p-token launch planning. It gives humans and agents a browser UI and JSON API for launch configs, constant-product quotes, RISE-style floor modeling, perp intents, mint inspection, and WDK-style unsigned action envelopes.

This folder is the promoted version of `templates/p-token-launcher`.

## How It Fits

| Workspace area | Role |
|----------------|------|
| `launchpad` | TypeScript instruction builders and x402-gated launchpad API. |
| `agent-sdk` | Agent-side Core Execute and registration helpers. |
| `p-agent-token` | Pinocchio scaffold for the on-chain agent token program. |
| `p-token-launcher` | Unsigned planning UI/API before wallet signing, deployment, or registry updates. |

## Run

From the repo root:

```bash
npm run ptoken:launcher
```

Or from this folder:

```bash
npm start
```

Then open:

```txt
http://localhost:8787
```

## API

| Route | Purpose |
|-------|---------|
| `GET /api/health` | Runtime status, target network, p-token program ID, and supported adapters. |
| `GET /api/registry` | Reads `data/ptokens.json` when present; otherwise returns an empty registry. |
| `GET /api/examples` | Returns `launch-config.example.json` and `bonding-curve.example.json`. |
| `POST /api/launch-plan` | Builds the complete unsigned p-token launch plan. |
| `POST /api/quote` | Simulates a buy or sell against the constant-product curve. |
| `POST /api/rise-floor` | Models protocol-owned floor coverage for a RISE-style launch. |
| `POST /api/perp-plan` | Produces a planner-only perpetual trade intent and risk checks. |
| `POST /api/wdk-intent` | Emits an unsigned WDK-style Solana action envelope. |
| `POST /api/inspect` | Inspects a mint account over RPC and classifies the token program. |

## Examples

```bash
curl -s http://localhost:8787/api/launch-plan \
  -H 'content-type: application/json' \
  -d '{"symbol":"PQC","name":"Quantum Compute p-token","virtualSol":30,"virtualToken":1073000000}'
```

```bash
curl -s http://localhost:8787/api/quote \
  -H 'content-type: application/json' \
  -d '{"side":"buy","virtualSol":30,"virtualToken":1073000000,"sol":1,"feeBps":100}'
```

## Config Files

| File | Purpose |
|------|---------|
| `launch-config.example.json` | Token metadata, authorities, bonding curve, and registry tags. |
| `bonding-curve.example.json` | Curve reserves, fee settings, graduation actions, and example quote inputs. |
| `public/` | Static browser UI served by `server.mjs`. |

## Environment

| Variable | Default | Purpose |
|----------|---------|---------|
| `PORT` | `8787` | Workbench HTTP port. |
| `SOLANA_RPC_URL` | unset | Preferred RPC URL for mint inspection. |
| `HELIUS_RPC_URL` | unset | Fallback RPC URL for mint inspection. |
| `P_TOKEN_PROGRAM_ID` | `ptok6rngomXrDbWf5v5Mkmu5CEbB51hzSCPDoj9DrvF` | Program ID used to classify inspected mints. |

## Safety Boundary

The workbench does not deploy a program, create a mint, trade, borrow, repay, open perpetuals, transfer funds, or sign transactions. Its WDK output is an intent envelope for review and later wallet integration.
