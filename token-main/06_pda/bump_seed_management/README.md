# Bump Seed Management

Store canonical bump seeds in account data to avoid expensive re-derivation
on every instruction. Retrieve and verify stored bumps efficiently.

## Pattern
```rust
// Store bump at account creation
account_data[BUMP_OFFSET] = bump;

// Retrieve and use for signing
let bump = account_data[BUMP_OFFSET];
let signer_seeds = &[seed1, seed2, &[bump]];
invoke_signed(ix, accounts, &[signer_seeds])?;
```

> 🚧 Coming Soon
