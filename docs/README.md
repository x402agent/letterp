# Docs

Protocol and agent design notes for the P Token launchpad workspace.

## Documents

| File | Contents |
|------|----------|
| `PROTOCOL.md` | Bonding-curve math, default parameters, account layout, instruction discriminators, PDA seeds, SDK exports, and x402 payment flow. |
| `P_AGENTS.md` | P Agent lifecycle, Metaplex Core Execute flow, PDA table, SDK reference, fee routing, registration JSON, and environment setup. |

## Related Project Folders

| Folder | Contents |
|--------|----------|
| `../templates` | Reusable Pinocchio and p-token launcher starters. |
| `../p-agent-token` | Adapted Pinocchio P Agent token scaffold. |
| `../p-token-launcher` | Unsigned browser/API workbench for p-token launch planning. |

## Reading Order

1. Read `PROTOCOL.md` first for the launchpad primitives and byte-level program contract.
2. Read `P_AGENTS.md` next for how Core assets become keyless agent wallets on top of those primitives.
3. Use `../p-token-launcher` to produce unsigned launch plans before wiring signer or deployment flows.
4. Use `../p-agent-token` as the current Pinocchio scaffold for the on-chain agent-token shape.

These docs describe the TypeScript reference implementation and the expected Pinocchio on-chain program interface.
