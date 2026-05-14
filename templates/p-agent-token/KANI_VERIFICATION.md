# Kani Verification

This crate has Kani proof harnesses in `src/kani_verification.rs`.

Run all proofs:

```sh
cargo kani
```

Run individual harnesses:

```sh
cargo kani --harness agent_flags_are_one_way_and_independent
cargo kani --harness state_layout_lengths_match_struct_sizes
cargo kani --harness buy_data_accepts_exactly_nonzero_u64_payloads
cargo kani --harness sell_data_accepts_exactly_nonzero_u64_payloads
cargo kani --harness curve_decoding_and_fee_math_are_total
cargo kani --harness curve_fee_math_handles_u64_extremes
cargo kani --harness constant_product_quotes_are_bounded_and_total
cargo kani --harness instruction_discriminators_are_unique
```

The harnesses use `kani::cover!` for both accepted and rejected parser paths, bound and unbound flag states, standard and high fee basis-point states, zero and nonzero fee amounts, defined and undefined constant-product quotes, and discriminator reachability. Treat any unsatisfied cover as a failed proof review, even if assertions pass.
