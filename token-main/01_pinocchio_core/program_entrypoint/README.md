# Program Entrypoint

Custom Solana program entrypoint using Pinocchio's lightweight process_instruction
pattern. No Anchor macros — pure Rust function dispatch.

## Planned Pattern
```rust
pinocchio_sdk::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult { ... }
```

> 🚧 Coming Soon
