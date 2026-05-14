# Token Account 2022

Initialize token accounts under the Token-2022 program.
Supports extensions like ImmutableOwner and RequiredMemo at account creation time.

## Planned API
```rust
pub fn initialize_account_2022(
    account: &AccountInfo,
    mint: &AccountInfo,
    owner: &Pubkey,
) -> ProgramResult
```

> 🚧 Coming Soon
