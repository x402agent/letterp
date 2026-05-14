# Kani Verification

This repository includes Kani proof harnesses for pure arithmetic, fee, agent-policy, x402, bonding-curve, and perpetual-risk helpers.

## Install

```sh
cargo install kani-verifier --locked
cargo kani setup
```

## Run Proofs

```sh
cargo kani -p ptoken-sdk --harness safe_arithmetic_matches_checked_operations
cargo kani -p ptoken-sdk --harness division_helpers_are_total_and_precise
cargo kani -p ptoken-sdk --harness standard_fee_calculations_do_not_truncate_or_exceed_caps
cargo kani -p ptoken-sdk --harness fee_calculations_cover_u64_extremes
cargo kani -p ptoken-sdk --harness extended_fee_calculations_cover_high_bps_edges
cargo kani -p ptoken-sdk --harness bounded_u64_helpers_match_mathematical_intent
cargo kani -p ptoken-sdk --harness decimal_multiplier_is_total_and_monotonic_until_saturation
cargo kani -p ptoken-sdk --harness agent_policy_rejects_vacuous_trading_authority
cargo kani -p ptoken-sdk --harness x402_receipts_only_unlock_matching_unexpired_payments
cargo kani -p ptoken-sdk --harness bonding_curve_quotes_are_monotonic_for_bounded_inputs
cargo kani -p ptoken-sdk --harness perpetual_position_math_preserves_side_direction
```

To run every harness:

```sh
cargo kani -p ptoken-sdk
```

## Non-Vacuity Checks

Every harness uses `kani::cover!` for expected success, failure, capped, uncapped, exact, rounded, saturated, zero-value, accepted, rejected, profitable, losing, flat-curve, and increasing-curve paths. Treat any `UNSATISFIED` or `UNREACHABLE` cover result as a proof-quality failure until the harness preconditions or implementation are reviewed.

For source coverage:

```sh
cargo kani -p ptoken-sdk --coverage -Z source-coverage
```

## Current Proof Scope

The Kani configuration intentionally focuses on pure modules. Solana CPI and extension modules compile in the normal Rust build, but they are excluded from Kani because their useful invariants depend on runtime account state, external program semantics, and syscall behavior. Move logic into pure helpers first, prove those helpers, then call them from CPI-facing code.

Current local result:

```text
Complete - 11 successfully verified harnesses, 0 failures, 11 total.
```
