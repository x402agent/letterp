# Open Work Register

This file tracks known work that remains after adapting the scaffold into the LetterP token codebase. It is intentionally concrete: each item points to a code surface, not a generic future feature.

## Verified
- [x] `09_math` overflow, underflow, divisor, fee cap, decimal multiplier, and range invariants are covered by Kani.
- [x] Numbered module READMEs now describe LetterP-specific source paths, contracts, and audit hooks.
- [x] Root workspace manifest resolves the SDK and example crates.

## Compile Blockers
- [ ] Add the missing `spl-token-metadata-interface` dependency or route metadata helpers through the Token-2022 re-export if the selected SPL version supports it.
- [ ] Import `solana_program::program_pack::Pack` wherever SPL `LEN` associated constants are used.
- [ ] Update confidential-transfer helper signatures to match `spl-token-2022` 3.x typed public key and ciphertext arguments.
- [ ] Give CPI helper functions a shared `AccountInfo<'a>` lifetime where cloned account arrays require invariant lifetimes.
- [ ] Fix `ptoken-sdk/src/pda/derivation.rs` so the bump byte array lives long enough for `create_program_address`.

## Next Verification Targets
- [ ] Fixed-layout readers in `pinocchio_core::zero_copy_layout`.
- [ ] Account-state validation for initialized, frozen, sufficient-balance, and zero-balance checks.
- [ ] PDA seed construction once the temporary bump borrow is fixed.
