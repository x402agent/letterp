# SPL Token Classic Reference

Complete instruction reference for the original SPL Token program.

## Instructions
| Instruction | Accounts Required | Description |
|-------------|-------------------|-------------|
| InitializeMint | mint, rent | Create a new token mint |
| InitializeAccount | account, mint, owner, rent | Create a token account |
| Transfer | source, dest, owner | Move tokens |
| Approve | source, delegate, owner | Approve delegate |
| Revoke | source, owner | Revoke delegate |
| MintTo | mint, destination, authority | Mint new tokens |
| Burn | account, mint, authority | Burn tokens |
| CloseAccount | account, destination, owner | Reclaim rent |
| FreezeAccount | account, mint, authority | Freeze account |
| ThawAccount | account, mint, authority | Thaw account |

> 🚧 Coming Soon
