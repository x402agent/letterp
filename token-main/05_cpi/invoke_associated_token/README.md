# Invoke Associated Token Program

CPI into the Associated Token Account (ATA) program to create ATAs
deterministically from within another program.

## Planned API
```rust
pub fn cpi_create_associated_token_account(
    payer: &AccountInfo,
    wallet: &AccountInfo,
    mint: &AccountInfo,
    token_program: &AccountInfo,
) -> ProgramResult
```

> 🚧 Coming Soon
