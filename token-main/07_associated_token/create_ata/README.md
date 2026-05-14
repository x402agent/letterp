# Create ATA

Create an Associated Token Account for a given wallet and mint.
The ATA address is deterministically derived from these two keys.

## Planned API
```rust
pub fn create_associated_token_account(
    payer: &AccountInfo,
    wallet: &Pubkey,
    mint: &AccountInfo,
    token_program_id: &Pubkey,
) -> ProgramResult
```

> 🚧 Coming Soon
