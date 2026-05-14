# Perps

Adapter package for generating perps-market configuration from graduated launchpad bonding curves.

## Exports

| Export | Purpose |
|--------|---------|
| `generatePerpMarketConfig` | Builds a Drift-style perp market config from a graduated `BondingCurveState`. |
| `generateAdrenaMarketConfig` | Builds a placeholder Adrena-style config from a graduated `BondingCurveState`. |
| `PerpMarketConfig` | Type for generated Drift market parameters. |

## Build

```bash
npm run build -w @x402pt/perps
```

## Notes

Both adapters require `state.graduated === true`. The Drift adapter prepares market parameters; it does not submit governance or market-creation transactions. The Adrena adapter is intentionally a stub until a stable Adrena SDK interface is available.
