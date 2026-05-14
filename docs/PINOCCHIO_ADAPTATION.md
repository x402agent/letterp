# Pinocchio Adaptation

This repo vendors Anza Pinocchio under `pinocchio/pinocchio-main` and uses that local tree as the source of truth for native Solana program code.

## Local Pinocchio Crates

| Crate | Path | Used by |
|-------|------|---------|
| `pinocchio` | `pinocchio/pinocchio-main/sdk` | `p-agent-token`, `programs/src` |
| `pinocchio-system` | `pinocchio/pinocchio-main/programs/system` | `p-agent-token`, `programs/src` |
| `pinocchio-token` | `pinocchio/pinocchio-main/programs/token` | `p-agent-token`, `programs/src` |
| `pinocchio-associated-token-account` | `pinocchio/pinocchio-main/programs/associated-token-account` | `p-agent-token` |
| `pinocchio-token-2022` | `pinocchio/pinocchio-main/programs/token-2022` | Future metadata pointer, transfer fee, and extension work |
| `pinocchio-memo` | `pinocchio/pinocchio-main/programs/memo` | Future audit trail and intent memo work |

## Adapted Surfaces

### `p-agent-token`

`p-agent-token` now depends on the local Pinocchio SDK and helper crates by path. Its code has been adapted to the Pinocchio `0.11` API:

- `AccountInfo` became `AccountView`.
- `Pubkey` became `Address`.
- `ProgramError` is imported from `pinocchio::error`.
- Mutable account data uses `AccountView::try_borrow_mut`.
- The entrypoint receives `&mut [AccountView]`.

This is the native program draft for first-party P Agents.

### `programs/src`

The bonding curve launchpad already uses the local Pinocchio SDK and helper crates by path:

```toml
pinocchio = { path = "../../pinocchio/pinocchio-main/sdk" }
pinocchio-system = { path = "../../pinocchio/pinocchio-main/programs/system" }
pinocchio-token = { path = "../../pinocchio/pinocchio-main/programs/token" }
```

This remains the curve-side program surface for initialize, buy, sell, graduate, and claim-fee instructions.

### `p-token-launcher`

The workbench exposes Pinocchio integration through:

- `GET /api/pinocchio`
- `GET /api/workspace`
- `POST /api/program-draft`

These routes let the frontend show which local Pinocchio crates are backing the P Agent and bonding curve programs.

### `launchpad`

The TypeScript launchpad package mirrors the Pinocchio instruction contract. Its builders encode the discriminators and account order that the native programs must enforce.

## Checks

Run Rust checks without writing build artifacts into tracked folders:

```bash
CARGO_TARGET_DIR=/tmp/letterp-p-agent-token-target cargo check --manifest-path p-agent-token/Cargo.toml
CARGO_TARGET_DIR=/tmp/letterp-bonding-curve-target cargo check --manifest-path programs/src/Cargo.toml
```

Run the frontend workbench when Node is available:

```bash
npm run ptoken:launcher
```

Then open `http://localhost:8787` and use the Pinocchio tab.

## Remaining Program Work

- Replace all placeholder program IDs after devnet deployments.
- Implement full p-token CPI paths in `p-agent-token`.
- Implement full reserve movement and fee accounting in the bonding curve program.
- Add SBF/Mollusk tests for Pinocchio account parsing, duplicate accounts, PDA validation, and arithmetic boundaries.
- Add `pinocchio-token-2022` only when Token-2022 extensions are actually used.
