# Invoke System Program

CPI into the Solana System Program. Used for creating new accounts,
transferring SOL, and allocating space before token account initialization.

## Planned API
```rust
pub fn cpi_create_account(payer: &AccountInfo, new_account: &AccountInfo, space: u64, owner: &Pubkey) -> ProgramResult
pub fn cpi_transfer_sol(from: &AccountInfo, to: &AccountInfo, lamports: u64) -> ProgramResult
```

> 🚧 Coming Soon
