# Facilitator Source

Implementation source for `@x402pt/facilitator`.

## Files

| File | Purpose |
|------|---------|
| `index.ts` | Exports the server module. |
| `server.ts` | Express app, request schemas, endpoint handlers, network configuration, and in-memory settlement tracking. |

## Request Flow

1. Launchpad sends a `payment` value and `requirements` object to `/verify`.
2. The facilitator validates the payload and checks the signature-like payment string.
3. Launchpad sends the same payload to `/settle`.
4. The facilitator records a resource/payment ID in memory and returns a settlement response.

`server.ts` starts listening as a side effect when loaded. Keep that in mind if importing it from tests or other tooling.
