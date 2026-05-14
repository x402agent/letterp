# 09 - Math

Token amount arithmetic, decimal conversion, fee calculation, and u64 helpers.

## Code Map
- Primary source: `ptoken-sdk/src/math`
- Crate entry: `ptoken-sdk/src/lib.rs`

## Local Rules
- Checked operations return `PTokenError` instead of panicking.
- Fee calculations use u128 intermediates and clamp before casting back to u64.
- Kani harnesses cover overflow, underflow, zero divisors, caps, and decimal saturation.

## Review Checklist
- Keep account ordering explicit in docs and code.
- Prefer project errors over generic `ProgramError` until the Solana boundary.
- Update the matching example or test when behavior changes.
