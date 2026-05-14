# Get ATA Address

Derive the Associated Token Account address for a wallet/mint pair
without requiring an on-chain call.

## Derivation
```rust
Pubkey::find_program_address(
    &[wallet.as_ref(), token_program_id.as_ref(), mint.as_ref()],
    &spl_associated_token_account::ID,
)
```

> 🚧 Coming Soon
