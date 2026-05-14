# Security Policy

## Status

LetterP Token SDK is pre-mainnet software. The SDK compiles, tests, and has Kani proofs for pure policy and arithmetic primitives, but the generated program IDs should not be treated as audited production deployments.

## Reporting

Do not disclose exploitable issues publicly before maintainers have had time to assess and patch them. Open a private security advisory or contact the repository maintainers through the project-owned security channel once one is published.

Include:

- Affected commit and crate or program path.
- Minimal reproduction steps.
- Expected and observed behavior.
- Impact assessment and suggested fix, if known.

## Scope

In scope:

- Arithmetic, fee, bonding-curve, perpetual, and x402 receipt validation bugs.
- PDA seed collisions or authority bypasses.
- CPI account-ordering, signer, owner, and writable-account issues.
- Build or deployment processes that could publish unverifiable bytecode.

Out of scope:

- Issues in third-party RPC providers.
- Social engineering.
- Findings that require publishing private deploy keypairs or user secrets.
