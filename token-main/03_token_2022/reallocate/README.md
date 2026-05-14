# Reallocate

Add new extensions to an existing Token-2022 token account after creation.
Uses the `Reallocate` instruction to resize account data and append extension space.

## Planned API
```rust
pub fn reallocate(
    account: &AccountInfo,
    payer: &AccountInfo,
    owner: &AccountInfo,
    new_extensions: &[ExtensionType],
) -> ProgramResult
```

> 🚧 Coming Soon
