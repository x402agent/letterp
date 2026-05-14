# Examples

Runnable TypeScript examples for the launchpad, x402 facilitator, P Agent SDK, and perps adapter.

## Scripts

```bash
npm run build -w @x402pt/examples
npm run start:launch-token -w @x402pt/examples
npm run start:agent-launch -w @x402pt/examples
npm run start:pay-to-read -w @x402pt/examples
npm run start:memo -w @x402pt/examples
```

`src/p-agent-full.ts` is included as a full walkthrough, but `package.json` does not currently define a start script for it. After building, run it directly with:

```bash
node examples/dist/p-agent-full.js
```

## Example Files

| File | Purpose |
|------|---------|
| `src/launch-token.ts` | Calls the launchpad HTTP API, verifies returned PDAs, and demonstrates paid curve endpoints returning `402`. |
| `src/pay-to-read.ts` | Walks through the reference x402 verify and settle flow against the facilitator. |
| `src/memo.ts` | Builds a p-memo instruction using the shared SDK. |
| `src/agent-launch.ts` | Demonstrates asset signer PDA derivation and wrapping a buy instruction in Core Execute. |
| `src/p-agent-full.ts` | End-to-end P Agent walkthrough: Core asset, registration document, token launch, signer PDA, and Core Execute buy. |
| `src/index.ts` | Empty module marker for the package build. |

## Local Services

Several examples expect these services:

```bash
npm run build -w @x402pt/facilitator
npm run start -w @x402pt/facilitator

npm run build -w @x402pt/launchpad
node launchpad/dist/server.js
```

Use `LAUNCHPAD_URL` and `FACILITATOR_URL` to point examples at non-default hosts.
