# Pinocchio Program Map

One-by-one map of vendored Pinocchio helper crates and the local Solana Clawd adaptation.

| Program | Vendored source | Local adaptation |
|---------|-----------------|------------------|
| SDK | `pinocchio-main/sdk` | Core native-program API for `AccountView`, `Address`, `ProgramResult`, and entrypoints. Used by `p-agent-token` and `programs/src`. |
| System | `pinocchio-main/programs/system` | Account creation, SOL movement, rent funding, and PDA-owned state helpers. |
| Token | `pinocchio-main/programs/token` | SPL-compatible token CPI helper surface used by p-token/SPL fallback flows. |
| Token-2022 | `pinocchio-main/programs/token-2022` | Reserved for future metadata pointer, transfer fee, transfer hook, and extension-aware p-token launch paths. |
| Associated Token Account | `pinocchio-main/programs/associated-token-account` | ATA creation and validation helper surface for launch, vault, escrow, and agent flows. |
| Memo | `pinocchio-main/programs/memo` | Reserved for launch annotations and agent-readable trace markers. |

## Local Surfaces

| Surface | Purpose |
|---------|---------|
| `p-agent-token` | First-party P Agent program draft using local Pinocchio path dependencies. |
| `programs/src` | Bonding-curve launchpad program draft using local Pinocchio path dependencies. |
| `p-token-launcher` | Workbench exposing `GET /api/pinocchio` and `POST /api/program-draft`. |
| `launchpad` | TypeScript instruction builders mirroring native discriminators and account order. |
| `templates` | Forkable starter code already aligned with local Pinocchio `0.11` APIs. |
