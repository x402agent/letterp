# PDA Validation

Assert that a given account address matches the expected PDA derived from
known seeds and program ID. Prevents seed substitution attacks.

## Planned API
```rust
pub fn assert_pda(
    account: &AccountInfo,
    seeds: &[&[u8]],
    program_id: &Pubkey,
) -> PTokenResult<u8>  // returns bump
```

> 🚧 Coming Soon
