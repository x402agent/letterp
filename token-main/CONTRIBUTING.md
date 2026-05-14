# Contributing

## Development

Run the core checks before opening a PR:

```bash
cargo fmt --check
cargo check --workspace
cargo test --workspace
cargo kani -p ptoken-sdk
```

Keep changes small and reviewable. Security-sensitive behavior should be visible in the function signature or validated through a named helper.

## Verification Rules

Pure math, policy, and parsing helpers should get focused unit tests and, when practical, Kani harnesses. Kani proofs must include `kani::cover!` paths for success and failure cases so the proof is not vacuous.

## Program Rules

Do not commit private keypairs. Only commit public program IDs, reproducible build metadata, and source code. Deployment authority custody and upgrade authority changes must be documented before mainnet release.

## Style

- Prefer checked arithmetic and explicit error returns.
- Keep account ordering documented near instruction handlers.
- Do not hide signer, owner, or PDA assumptions behind broad helper names.
- Avoid unrelated refactors in security-sensitive modules.
