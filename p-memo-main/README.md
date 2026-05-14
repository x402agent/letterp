# LetterP p-memo

`p-memo` is the LetterP memo program used to attach low-compute UTF-8 receipts to p-agent, x402, launchpad, and token flows.

## Behavior

- Instruction data must be valid UTF-8.
- Every supplied account must be a signer.
- Signer public keys are logged in base58.
- The memo body is logged after account validation.
- No heap allocator is used.

## Build

From this directory:

```bash
cargo build
```

From the repository root:

```bash
npm run memo:build
```

For an SBF artifact, use:

```bash
npm run memo:build-sbf
```

## TypeScript Integration

The shared SDK exports:

```ts
import { buildMemoInstruction, P_MEMO_PROGRAM_ID } from "@x402pt/shared";
```

The launchpad accepts an optional `memo` field on launch requests and inserts a p-memo instruction before the launch instructions. The facilitator returns a serialized settlement memo instruction from `/settle`.

## Program ID

Default placeholder:

```text
PMemo11111111111111111111111111111111111111
```

Set `P_MEMO_PROGRAM_ID` after deploying your own build.

## Tests

The integration tests use Mollusk and expect an SBF artifact named `p_memo`.

```bash
cargo test-sbf
```
