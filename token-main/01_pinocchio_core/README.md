# 01 — Pinocchio Core

Foundational Pinocchio runtime primitives used across the entire pToken SDK.
Pinocchio is a zero-dependency Solana program library that operates directly
on raw account bytes and instruction data without any macro overhead.

## Sub-modules
- `account_info/` — Raw AccountInfo access and lifetime management
- `instruction_data/` — Parsing instruction discriminants and arguments
- `program_entrypoint/` — Custom entrypoint without Anchor boilerplate
- `syscalls/` — Direct Solana syscall wrappers (sol_log, sol_invoke, etc.)
- `zero_copy_layout/` — Zero-copy account deserialization via byte slices

> 🚧 Coming Soon
