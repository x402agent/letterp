# CPI Guard Extension

Protects token accounts from being manipulated by CPI (cross-program invocations).
When enabled, the token account can only be operated on by the owner directly,
not by programs calling on their behalf.

## Protected Operations
- Transfer
- Burn
- Approve
- Close
- SetAuthority

> 🚧 Coming Soon
