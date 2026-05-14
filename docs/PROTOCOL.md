# Protocol Specification

## Bonding Curve Math

The curve uses a constant-product invariant with virtual reserves:

```
Invariant: (virtualSol + realSol) * (virtualTokens + realTokens) = k
```

### Buy

```
protocolFee = solIn * protocolFeeBps / 10000
creatorFee  = solIn * creatorFeeBps / 10000
solInNet    = solIn - protocolFee - creatorFee

newSol   = virtualSol + realSol + solInNet
newTok   = k / newSol                (rounded down)
tokensOut = (virtualTokens + realTokens) - newTok
```

### Sell

```
newTok       = virtualTokens + realTokens + tokensIn
newSol       = k / newTok              (rounded down)
solOutGross  = (virtualSol + realSol) - newSol

protocolFee  = solOutGross * protocolFeeBps / 10000
creatorFee   = solOutGross * creatorFeeBps / 10000
solOut       = solOutGross - protocolFee - creatorFee
```

### Graduation

When `sold >= totalSupply * graduationThresholdBps / 10000`, the curve graduates.
Once graduated, trades against the curve are disabled and liquidity can migrate to a
Raydium pool or a perp DEX market.

## Default Parameters

| Parameter               | Value       |
|-------------------------|-------------|
| Total supply            | 1,000,000,000,000,000 (1e15, 1B tokens at 6 decimals) |
| Virtual SOL reserves    | 30 SOL (30_000_000_000 lamports) |
| Virtual token reserves  | Equal to total supply |
| Graduation threshold    | 80% of tokens sold |
| Creator fee             | 100 bps (1%) |
| Protocol fee            | 50 bps (0.5%) |

## On-Chain Account Layout

### Curve State Account (152 bytes)

| Offset | Size | Field            | Type    |
|--------|------|------------------|---------|
| 0      | 1    | discriminant     | u8      |
| 1      | 1    | graduated        | bool    |
| 2      | 32   | authority        | Pubkey  |
| 34     | 32   | creatorFeeWallet | Pubkey  |
| 66     | 32   | mint             | Pubkey  |
| 98     | 8    | realSolReserves  | u64 LE  |
| 106    | 8    | realTokenReserves| u64 LE  |
| 114    | 8    | virtualSolReserves| u64 LE |
| 122    | 8    | virtualTokenReserves| u64 LE|
| 130    | 8    | totalSupply      | u64 LE  |
| 138    | 2    | creatorFeeBps    | u16 LE  |
| 140    | 2    | protocolFeeBps   | u16 LE  |
| 142    | 10   | padding          | [u8;10] |

### Curve Vault Account

A system-owned account (PDA) that holds the curve's SOL reserves and
accumulated fees.

## Instruction Discriminators

| Disc | Instruction      | Data Layout                                     |
|------|------------------|-------------------------------------------------|
| 0    | InitializeCurve  | u8 + u64 + u64 + u64 + u16 + u16                |
| 1    | Buy              | u8 + u64 + u64                                  |
| 2    | Sell             | u8 + u64 + u64                                  |
| 3    | Graduate         | u8                                              |
| 4    | ClaimCreatorFees | u8                                              |

## PDA Derivation

```
curve = PDA(["curve", mint], BONDING_CURVE_PROGRAM_ID)
vault = PDA(["vault", mint], BONDING_CURVE_PROGRAM_ID)
assetSigner = PDA(["mpl-core-execute", asset], MPL_CORE_PROGRAM_ID)
```

## x402 Payment Flow

1. Client sends request without `X-PAYMENT` header.
2. Server responds with `402 Payment Required` and `PAYMENT-REQUIRED` header
   containing base64-encoded `PaymentRequirements` JSON.
3. Client constructs and signs a USDC transfer tx for `maxAmountRequired` to `payTo`.
4. Client retries the request with `X-PAYMENT` set to the tx signature.
5. Launchpad calls `/verify` on the facilitator to check the payment.
6. Launchpad calls `/settle` on the facilitator to mark payment as used.
7. Request proceeds to the handler.

## Agent Integration

Agents are Metaplex Core assets registered via `mpl-agent-registry`.
They have no private key — their Asset Signer PDA signs via Core's `Execute`
instruction. The launchpad routes creator fees to the agent's Asset Signer PDA
when `agentAsset` is provided in the launch request.
