# Kani Verification

This repository includes Kani proof harnesses for the pure arithmetic helpers that sit under token amount, fee, and supply calculations.

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
```

To run every harness:

```sh
cargo kani -p ptoken-sdk
```

## Non-Vacuity Checks

Every harness uses `kani::cover!` for expected success, failure, capped, uncapped, exact, rounded, saturated, and zero-value paths. Treat any `UNSATISFIED` or `UNREACHABLE` cover result as a proof-quality failure until the harness preconditions or implementation are reviewed.

For source coverage:

```sh
cargo kani -p ptoken-sdk --coverage -Z source-coverage
```

## Current Proof Scope

The Kani configuration intentionally compiles the arithmetic and error modules under `cfg(kani)`. Several Solana CPI and extension modules are still under active development and currently do not compile in the normal Rust build, so they are excluded from the first verification slice. Extend the `cfg(kani)` surface only after those modules compile normally and have clear, local invariants to prove.
