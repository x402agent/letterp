# Perps Source

Source implementation for `@x402pt/perps`.

## Files

| File | Purpose |
|------|---------|
| `index.ts` | Package barrel export. |
| `drift-adapter.ts` | Drift market config generator and Adrena placeholder config generator. |

## Behavior

`generatePerpMarketConfig` uses the graduated curve state to produce reserve and margin settings for a speculative Drift market configuration. If no oracle is supplied, it falls back to the system program ID as a stub oracle placeholder.

`generateAdrenaMarketConfig` returns a plain object with token mint, initial price, max leverage, liquidation fee ratio, and protocol marker.
