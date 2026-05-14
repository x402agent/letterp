# Open Work Register

This file tracks known work that remains after adapting the repository into the LetterP token codebase.

## Completed

- [x] Root workspace manifest resolves the SDK and example crates.
- [x] SPL metadata, Token-2022 confidential transfer, `Pack::LEN`, CPI lifetime, and PDA bump-borrow compile blockers are fixed.
- [x] Numbered module READMEs describe LetterP-specific source paths, contracts, and audit hooks.
- [x] Agent, x402, bonding-curve, and perpetual SDK primitives are implemented.
- [x] Kani verifies 11 harnesses with non-vacuity cover checks.
- [x] Devnet public program IDs are generated in `program-ids/devnet/programs.toml`.

## Remaining Before Mainnet

- [ ] Implement audited on-chain entrypoints for the generated p-agent, p-token, x402 gateway, bonding-curve, and perpetual program IDs.
- [ ] Build SBF artifacts and record final `.so` sizes for exact mainnet rent costs.
- [ ] Add reproducible build instructions and release checksums.
- [ ] Decide deployment authority custody and upgrade-authority revocation policy.
- [ ] Add third-party security review and disclosure process.

## Next Verification Targets

- [ ] Fixed-layout readers in `pinocchio_core::zero_copy_layout`.
- [ ] Account-state validation for initialized, frozen, sufficient-balance, and zero-balance checks.
- [ ] PDA seed construction and address-domain separation.
- [ ] On-chain entrypoint instruction decoding once the new programs exist.
