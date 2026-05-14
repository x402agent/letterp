# 04 — Token-2022 Extensions

All supported Token-2022 extensions, each as an isolated module.
Extensions are configured at mint creation time and embedded in account data.

## Extension Index
| Extension | Account | Description |
|-----------|---------|-------------|
| `transfer_fee` | Mint | Charge a fee on every transfer |
| `confidential_transfer` | Mint + Account | Hide transfer amounts with ZK proofs |
| `confidential_transfer_fee` | Mint | Fee on confidential transfers |
| `interest_bearing` | Mint | Accumulate interest on token supply |
| `non_transferable` | Mint | Make tokens permanently non-transferable (soul-bound) |
| `permanent_delegate` | Mint | Assign an irrevocable global delegate |
| `transfer_hook` | Mint | Call a custom program on every transfer |
| `metadata_pointer` | Mint | Point to a metadata account |
| `token_metadata` | Mint | Embed metadata directly in the mint |
| `group_pointer` | Mint | Point to a token group account |
| `group_member_pointer` | Mint | Point to a group member account |
| `mint_close_authority` | Mint | Allow closing the mint account |
| `default_account_state` | Mint | New accounts start frozen by default |
| `immutable_owner` | Account | Prevent owner changes on token accounts |
| `required_memo` | Account | Require a memo on every transfer |
| `cpi_guard` | Account | Prevent certain CPI operations |

> 🚧 Coming Soon
