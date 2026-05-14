import { PublicKey } from "@solana/web3.js";
import BN from "bn.js";

// ---------------------------------------------------------------------------
// Program IDs
// ---------------------------------------------------------------------------

/** The mainnet p-token program (TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA).
 *  SIMD-0266 (p-token) is a runtime swap behind a feature gate — same ID,
 *  95–98% lower CU per instruction. */
export const TOKEN_PROGRAM_ID = new PublicKey(
  "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
);

export const ASSOCIATED_TOKEN_PROGRAM_ID = new PublicKey(
  "ATokenGPvbdGVxr1b2hvZbsiqW5xr25ix9fFhNoxLAUVK",
);

/** The x402 launchpad's on-chain bonding-curve program. Deployed by us. */
export const BONDING_CURVE_PROGRAM_ID = new PublicKey(
  "x402XXXXXXXXXXXXXXbondingXXXXXXXcurve",
);

/** Metaplex Core program ID. */
export const MPL_CORE_PROGRAM_ID = new PublicKey(
  "MplCore111111111111111111111111111111111111",
);

/** Metaplex Agent Registry program ID. */
export const MPL_AGENT_REGISTRY_PROGRAM_ID = new PublicKey(
  "MplAgentReg1111111111111111111111111111111",
);

// ---------------------------------------------------------------------------
// On-chain account layouts (mirrors programs/src/state.rs)
// ---------------------------------------------------------------------------

/**
 * Bonding-curve state account.
 * Exactly 152 bytes on chain (2 + 32 + 32 + 8 + 8 + 8 + 8 + 8 + 1 + 2 + 2 + 1 + padding).
 */
export interface BondingCurveState {
  /** Whether the curve has graduated and is no longer accepting trades. */
  graduated: boolean;
  /** The authority / creator who launched the curve. */
  authority: PublicKey;
  /** Wallet that accrues creator fees — for agents this is the Asset Signer PDA. */
  creatorFeeWallet: PublicKey;
  /** Real SOL reserves in the vault (lamports). */
  realSolReserves: BN;
  /** Real token reserves held in the vault (base units). */
  realTokenReserves: BN;
  /** Virtual SOL reserves added to invariant computation. */
  virtualSolReserves: BN;
  /** Virtual token reserves added to invariant computation. */
  virtualTokenReserves: BN;
  /** Total supply of the token (base units). */
  totalSupply: BN;
  /** Creator fee in basis points (0–1000). */
  creatorFeeBps: number;
  /** Protocol fee in basis points (0–1000). */
  protocolFeeBps: number;
  /** The associated mint. */
  mint: PublicKey;
}

// ---------------------------------------------------------------------------
// Swap quote / payment types
// ---------------------------------------------------------------------------

export interface SwapQuote {
  amountIn: BN;
  amountOut: BN;
  amountOutGross: BN;
  creatorFee: BN;
  protocolFee: BN;
  priceImpactBps: number;
}

/**
 * Payment requirements as defined in the x402 draft spec.
 * Returns as 402 + PAYMENT-REQUIRED header base64-encoded JSON.
 */
export interface PaymentRequirements {
  scheme: "exact";
  network: string;
  asset: string; // USDC mint
  maxAmountRequired: string; // USDC base units (6 decimals)
  payTo: string; // recipient address (base58)
  resource: string; // the URL being paid for
  description: string;
  maxTimeoutSeconds: number;
  mimeType: string;
}

// ---------------------------------------------------------------------------
// Agent context
// ---------------------------------------------------------------------------

/** Minimal agent info the SDK sends when acting on behalf of a Core asset. */
export interface AgentContext {
  asset: PublicKey; // Core asset address
  signer: PublicKey; // Asset Signer PDA (derived from asset)
}
