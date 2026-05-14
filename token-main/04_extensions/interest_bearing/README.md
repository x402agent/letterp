# Interest Bearing Extension

Tokens accumulate interest over time at a configurable rate.
The UI amount grows continuously; raw on-chain balance stays fixed.

## Key Parameters
- `rate_authority` — Can update the interest rate
- `rate` — Annual interest rate in basis points (signed, allows negative rates)
- `initialization_timestamp` — Unix timestamp when interest began

## UI Amount Formula
```
ui_amount = raw_amount * e^(rate * time_elapsed_years)
```

> 🚧 Coming Soon
