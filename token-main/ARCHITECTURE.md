# LetterP Token SDK Architecture

## Design Goal
LetterP token code is organized so security-sensitive behavior is visible at the call site. Account order, signer requirements, program IDs, PDA seeds, arithmetic rules, and extension authority rules are documented in the numbered folders and implemented in `ptoken-sdk/src`.

## Layers
| Layer | Path | Responsibility |
|-------|------|----------------|
| Source crate | `ptoken-sdk/src` | Rust modules imported by examples and downstream programs |
| Numbered notes | `01_*` through `15_*` | Project-owned documentation for each source area |
| Examples | `examples/*` | Small instruction processors that demonstrate SDK calls |
| Formal proofs | `ptoken-sdk/src/kani_verification.rs` | Bounded proofs for arithmetic and fee invariants |

## Module Boundaries
Runtime helpers live under `pinocchio_core`; they should parse bytes and accounts without deciding token policy. Token Classic, Token-2022, extension, CPI, ATA, and PDA modules build specific instruction flows. Validation modules perform preflight checks. Math modules are pure and are the first target for formal verification.

## Account Data Policy
Fixed SPL Token fields are read through named offsets in `zero_copy_layout.rs`. Dynamic Token-2022 extension data should use SPL extension APIs rather than hand-rolled offsets unless the layout is explicitly documented and tested.

## Extension Layout
Token-2022 extensions are appended after the fixed-size mint or account base data:
```
[base data][extension type][extension length][extension data]
```
Every helper that allocates or reallocates extension accounts must calculate space before CPI and document the enabled extension set.

## Verification Policy
Pure arithmetic belongs behind helpers and gets a Kani harness before it is reused by CPI-facing code. Harnesses must contain `kani::cover!` checks for success and failure paths so a proof cannot pass only because assumptions made the path unreachable.
