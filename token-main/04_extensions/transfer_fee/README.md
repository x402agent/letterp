# Transfer Fee Extension

Charge an automatic fee on every token transfer. Fees accumulate in a
designated fee account and can be harvested by the fee authority.

## Key Parameters
- `transfer_fee_config_authority` — Can update the fee config
- `withdraw_withheld_authority` — Can collect accumulated fees
- `transfer_fee_basis_points` — Fee as basis points (1 bp = 0.01%)
- `maximum_fee` — Cap on fee per transfer (in raw token units)

## Fee Calculation
```
fee = min(amount * basis_points / 10_000, maximum_fee)
```

> 🚧 Coming Soon
