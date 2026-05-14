# pToken SDK — Architecture

## Philosophy
pToken is built on Pinocchio — a zero-dependency Solana program framework that gives you
direct, low-level access to account data, instruction parsing, and CPI invocation without
the macro overhead of Anchor.

## Pinocchio vs Anchor
| Feature | Pinocchio | Anchor |
|---------|-----------|--------|
| Macro magic | None | Heavy (derive macros) |
| Binary size | Minimal | Larger |
| Serialization | Manual / Borsh | Auto via IDL |
| Account parsing | Zero-copy, explicit | Auto-deserialized |
| Learning curve | Higher | Lower |
| Performance | Maximum | Good |

## Token Programs
### SPL Token (Classic)
Original Solana token program. Fixed feature set, widely supported.

### Token-2022
New token program with extension system. Each extension is opt-in at mint creation time
and stored directly in the mint or token account's extra space.

## Extension Architecture (Token-2022)
Extensions are appended after the fixed-size base data in the account:
```
[Base Mint Data — 82 bytes][Extension Type: 2 bytes][Extension Length: 2 bytes][Extension Data]
```
Multiple extensions can be chained. Each one is identified by its `ExtensionType` discriminant.
