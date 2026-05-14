# Pinocchio / p-token Support

This folder contains agent-readable documentation, helpers, and detection utilities for Pinocchio programs and the p-token program on Solana.

## What is Pinocchio?

Pinocchio is a zero-dependency, `no_std` Rust library by Anza (Agave client team) for building native Solana programs. It replaces `solana-program` with **zero-copy `AccountInfo`** — a raw pointer into the runtime input buffer instead of an owned, heap-allocated struct.

**Key benefits:**
- ~75–90% fewer compute units per instruction (vs SPL Token)
- Smaller `.so` binary on-chain
- No external crate dependencies
- Full control over heap allocator and panic handler

## Folder Structure

```
pinocchio/
├── README.md               ← this file (agent entrypoint)
├── PROGRAMS.md             ← known Pinocchio programs + their on-chain IDs
├── P_TOKEN.md              ← p-token deep dive: CU comparison, API, detection
├── AGENT_HELPERS.md        ← how CLAWD agents should query p-token accounts
└── examples/
    ├── create_token.rs     ← Token2022 mint with metadata using pinocchio-token
    ├── escrow_state.rs     ← zero-copy #[repr(C)] state pattern
    ├── entrypoint.rs       ← all three entrypoint macro patterns
    └── try_from_accounts.rs← TryFrom account validation pattern
```

## Quick Reference: Program IDs

| Program | Address | Note |
|---------|---------|------|
| SPL Token | `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA` | Canonical fungible token |
| Token-2022 | `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb` | Token Extensions |
| System Program | `11111111111111111111111111111111` | Core system |
| p-token | Under active development — see `PROGRAMS.md` | Pinocchio replacement for SPL Token |

## How CLAWD Agents Use This

Agents can call `trpc.agents.metaplex.inspectAccount({ address })` to:
1. Fetch an account's `owner` field via Helius RPC
2. Match it against known program IDs in `PROGRAMS.md`
3. Flag accounts owned by unknown programs as "possible Pinocchio program"
4. Show CU comparison data from `P_TOKEN.md`

See `AGENT_HELPERS.md` for the full query pattern.

## Links

- [Pinocchio GitHub](https://github.com/anza-xyz/pinocchio)
- [p-token GitHub](https://github.com/febo/p-token)
- [pinocchio-token crate](https://crates.io/crates/pinocchio-token)
- [Helius Guide](https://www.helius.dev/blog/pinocchio)
- [Blueshift Pinocchio Course](https://blueshift.gg/courses/pinocchio)
- [CU Optimization Article](https://www.helius.dev/blog/solana-cu-optimization)
