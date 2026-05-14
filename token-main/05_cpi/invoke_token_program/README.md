# Invoke Token Program

CPI wrappers for every SPL Token instruction callable from another program.
Constructs AccountMeta slices and instruction data manually for minimal overhead.

## Planned API
```rust
pub fn cpi_transfer(accounts: TokenTransferAccounts, amount: u64) -> ProgramResult
pub fn cpi_mint_to(accounts: MintToAccounts, amount: u64) -> ProgramResult
pub fn cpi_burn(accounts: BurnAccounts, amount: u64) -> ProgramResult
```

> 🚧 Coming Soon
