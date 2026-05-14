# Instruction Data

Utilities for parsing raw instruction byte arrays into typed structs.
Handles discriminant extraction and argument deserialization without Anchor IDL.

## Planned API
```rust
pub fn parse_instruction(data: &[u8]) -> PTokenResult<PTokenInstruction>
pub fn read_u64(data: &[u8], offset: usize) -> u64
pub fn read_pubkey(data: &[u8], offset: usize) -> Pubkey
```

> 🚧 Coming Soon
