# 08 - Serialization

Instruction and account-data encoding helpers used by the SDK and examples.

## Code Map
- Primary source: `ptoken-sdk/src/serialization`
- Crate entry: `ptoken-sdk/src/lib.rs`

## Local Rules
- Prefer fixed-width little-endian encoders for discriminants and token amounts.
- Use Borsh only for payloads whose schema is part of the program contract.
- All decoders must return structured errors on short buffers.

## Review Checklist
- Keep account ordering explicit in docs and code.
- Prefer project errors over generic `ProgramError` until the Solana boundary.
- Update the matching example or test when behavior changes.
