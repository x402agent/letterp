# Transfer Hook Extension

Execute a custom program on every token transfer. The hook program receives
the transfer accounts and can enforce arbitrary logic (allowlists, royalties, etc).

## Key Parameters
- `authority` — Can update the hook program
- `program_id` — The hook program to invoke on transfer

## Hook Interface
The hook program must implement the `Execute` instruction defined by the
`spl-transfer-hook-interface` crate.

## Use Cases
- NFT royalties
- KYC/AML enforcement
- Dynamic transfer taxes

> 🚧 Coming Soon
