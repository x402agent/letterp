# Token Metadata Extension

Embed metadata (name, symbol, URI, additional fields) directly inside the mint account.
Eliminates the need for a separate Metaplex metadata account.

## Key Fields
- `name` — Token name (e.g. "USD Coin")
- `symbol` — Ticker symbol (e.g. "USDC")
- `uri` — URI to off-chain JSON metadata
- `additional_metadata` — Vec of arbitrary key-value string pairs

## Planned API
```rust
pub fn initialize_token_metadata(
    mint: &AccountInfo,
    name: &str,
    symbol: &str,
    uri: &str,
) -> ProgramResult
```

> 🚧 Coming Soon
