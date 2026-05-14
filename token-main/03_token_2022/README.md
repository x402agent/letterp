# 03 — Token-2022

Base operations for the Token-2022 program (Token Extensions Program).
Program ID: `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb`

Token-2022 is backward-compatible with SPL Token but adds an extension system
that embeds extra functionality directly into mint and token account data.

## Sub-modules
- `mint_with_extensions/` — Initialize mints with one or more extensions
- `reallocate/` — Add extensions to existing token accounts post-creation
- `token_account_2022/` — Token-2022 specific account initialization
- `close_account_2022/` — Close Token-2022 accounts with extension cleanup

> 🚧 Coming Soon
