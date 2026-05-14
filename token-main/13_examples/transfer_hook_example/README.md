# Example: Transfer Hook

Demonstrates Token-2022 Transfer Hook extension:
1. Write a hook program implementing the `Execute` interface
2. Create a mint with the hook program configured
3. Every transfer invokes the hook — this example logs the transfer amount
4. Update the hook program address using the hook authority

## Hook Program Template
```rust
pub fn execute(ctx: Context<Execute>, amount: u64) -> Result<()> {
    msg!("Transfer hook fired: {} tokens", amount);
    Ok(())
}
```

> 🚧 Coming Soon
