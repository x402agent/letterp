import { PublicKey } from "@solana/web3.js";
import BN from "bn.js";

declare const process: { env: Record<string, string | undefined> };

// ---------------------------------------------------------------------------
// Program IDs
// ---------------------------------------------------------------------------

function publicKeyFromEnv(name: string, fallback: string): PublicKey {
  const value = process.env[name] ?? fallback;
  try {
    return new PublicKey(value);
  } catch {
    return new PublicKey(fallback);
  }
}

export const SPL_TOKEN_PROGRAM_ID = new PublicKey(
  "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
);

/** Pinocchio p-token program id. Override for a custom deployment. */
export const P_TOKEN_PROGRAM_ID = publicKeyFromEnv(
  "P_TOKEN_PROGRAM_ID",
  "ptok6rngomXrDbWf5v5Mkmu5CEbB51hzSCPDoj9DrvF",
);

export function isPTokenPreferred(): boolean {
  const value = process.env.USE_P_TOKEN;
  return value !== "0" && value !== "false";
}

/** Token program used by the launchpad. Defaults to p-token, with SPL fallback. */
export const TOKEN_PROGRAM_ID = isPTokenPreferred()
  ? P_TOKEN_PROGRAM_ID
  : SPL_TOKEN_PROGRAM_ID;

export const ASSOCIATED_TOKEN_PROGRAM_ID = new PublicKey(
  "ATokenGPvbdGVxr1b2hvZbsiqW5xr25ix9fFhNoxLAUVK",
);

/** The self-hosted p-token launchpad program. Override after deployment. */
export const P_TOKEN_LAUNCHPAD_PROGRAM_ID = publicKeyFromEnv(
  "P_TOKEN_LAUNCHPAD_PROGRAM_ID",
  // Valid placeholder only. Set P_TOKEN_LAUNCHPAD_PROGRAM_ID to your deployed id.
  "11111111111111111111111111111111",
);

/** Backwards-compatible alias used by the older curve-only SDK. */
export const BONDING_CURVE_PROGRAM_ID = P_TOKEN_LAUNCHPAD_PROGRAM_ID;

export const P_TOKEN_FEATURE_GATE_PROGRAM_ID = publicKeyFromEnv(
  "P_TOKEN_FEATURE_GATE_PROGRAM_ID",
  "ptokFjwyJtrwCa9Kgo9xoDS59V4QccBGEaRFnRPnSdP",
);

/** Metaplex Core program ID. */
export const MPL_CORE_PROGRAM_ID = publicKeyFromEnv(
  "MPL_CORE_PROGRAM_ID",
  "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d",
);

/** Metaplex Agent Registry program ID. */
export const MPL_AGENT_REGISTRY_PROGRAM_ID = publicKeyFromEnv(
  "MPL_AGENT_REGISTRY_PROGRAM_ID",
  "11111111111111111111111111111111",
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

export interface AgentState {
  owner: PublicKey;
  uri: string;
  createdAt: BN;
  executiveDelegate: PublicKey | null;
  isActive: boolean;
}

export interface AgentTokenState {
  agent: PublicKey;
  mint: PublicKey;
  name: string;
  symbol: string;
  uri: string;
  createdAt: BN;
  isBound: boolean;
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
