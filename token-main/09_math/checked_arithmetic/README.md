# Checked Arithmetic

Wraps Rust's checked_add, checked_sub, checked_mul, checked_div with
PToken error types for clean error propagation.

## Planned API
```rust
pub fn safe_add(a: u64, b: u64) -> PTokenResult<u64>
pub fn safe_sub(a: u64, b: u64) -> PTokenResult<u64>
pub fn safe_mul(a: u64, b: u64) -> PTokenResult<u64>
pub fn safe_div(a: u64, b: u64) -> PTokenResult<u64>
```

> 🚧 Coming Soon
