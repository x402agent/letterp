# Examples Source

Source files for the `@x402pt/examples` package. These files are meant to be read as workflows as much as they are meant to be run.

## Files

| File | Flow |
|------|------|
| `launch-token.ts` | Launch request through the HTTP API, local PDA verification, and unpaid reads that trigger x402 responses. |
| `pay-to-read.ts` | Manual x402 flow: request, inspect `PaymentRequirements`, call `/verify`, call `/settle`, and retry with `X-PAYMENT`. |
| `memo.ts` | Builds a LetterP memo instruction with signer account metadata. |
| `agent-launch.ts` | Simulated Core asset agent, asset signer PDA fee routing, and Core Execute wrapping. |
| `p-agent-full.ts` | Complete SDK-level P Agent flow using `PAgent`, Core asset mint instruction building, registration JSON, and wrapped launch/buy instructions. |
| `index.ts` | Empty export so the examples package compiles cleanly as a module. |

The examples use placeholder public keys in several places. Replace them with real wallet keys and submitted transactions before using the flows against a live cluster.
