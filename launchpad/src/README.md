# Launchpad Source

Source implementation for `@x402pt/launchpad`.

## Files

| Path | Purpose |
|------|---------|
| `index.ts` | Public package exports. |
| `server.ts` | Express launchpad API and x402 middleware. |
| `state-reader.ts` | RPC connection helper and raw curve account decoder. |
| `curves/constant-product.ts` | Constant-product quote, simulation, graduation, and spot-price helpers. |
| `programs/launchpad-ix.ts` | Solana instruction builders for launch, trade, registration, delegation, graduation, claims, and p-token batch distribution. |

## Implementation Boundaries

- Curve math is pure and lives under `curves/`.
- RPC decoding lives in `state-reader.ts` and must match the on-chain account layout documented in `docs/PROTOCOL.md`.
- Instruction encoding lives under `programs/` and should stay aligned with the Pinocchio program instruction discriminators.
- `server.ts` adapts those primitives into HTTP endpoints and delegates paid-route verification to the facilitator.
