/**
 * Drift Perps Adapter
 *
 * Once a bonding curve graduates, the token can be listed as a perp market
 * on Drift. This adapter provides the necessary market-creation instructions
 * and config generation.
 *
 * Drift uses a governance vote to add new perp markets, so this adapter
 * mainly prepares the market parameters rather than submitting the on-chain
 * instruction directly.
 */
import { PublicKey } from "@solana/web3.js";
import BN from "bn.js";
import { BondingCurveState } from "@x402pt/shared";

export interface PerpMarketConfig {
  marketIndex: number;
  oracle: PublicKey;
  baseAssetReserve: BN;
  quoteAssetReserve: BN;
  initialMarginRatio: number; // e.g. 1000 = 10%
  maintenanceMarginRatio: number; // e.g. 500 = 5%
  contractTier: number; // Drift contract tier enum
}

/**
 * Generate a Drift perp market config from a graduated curve's token.
 *
 * The `marketIndex` must be assigned by the Drift program (usually the next
 * available index). The oracle is the pyth / switchboard feed for the token.
 * If no oracle exists, the adapter falls back to a stub oracle pointing at
 * the curve itself (off-chain pricing via the bonding curve quote).
 */
export function generatePerpMarketConfig(
  state: BondingCurveState,
  marketIndex: number,
  oracle?: PublicKey,
): PerpMarketConfig {
  if (!state.graduated) {
    throw new Error("Cannot create perp market: curve has not graduated yet");
  }

  // Use the curve vault's spot price as the initial oracle if none provided.
  const spotPriceLamports = state.realSolReserves
    .isZero()
    ? new BN(1)
    : state.realSolReserves.mul(new BN(1_000_000)).div(state.totalSupply);

  // If no oracle is available, we use the curve PDA as a "stub oracle"
  // — in production this would be a proper pyth/switchboard feed.
  const oraclePubkey = oracle ?? new PublicKey("11111111111111111111111111111111");

  // Base asset reserve: use the total supply scaled to 6 decimals.
  const baseAssetReserve = state.totalSupply.div(new BN(1_000_000));

  // Quote asset reserve: value of total supply in USDC (simplified).
  const quoteAssetReserve = spotPriceLamports
    .mul(baseAssetReserve)
    .div(new BN(1_000_000));

  return {
    marketIndex,
    oracle: oraclePubkey,
    baseAssetReserve,
    quoteAssetReserve,
    initialMarginRatio: 1000, // 10%
    maintenanceMarginRatio: 500, // 5%
    contractTier: 0, // A (speculative)
  };
}

/**
 * Adrena Perps Adapter (stub).
 *
 * Adrena has a different market-creation interface. This adapter follows
 * the same pattern but emits Adrena-compatible config.
 *
 * TODO: fill in Adrena-specific instruction building once the Adrena SDK
 * is published.
 */
export function generateAdrenaMarketConfig(
  state: BondingCurveState,
): Record<string, unknown> {
  if (!state.graduated) {
    throw new Error("Cannot create perp market: curve has not graduated yet");
  }

  return {
    tokenMint: state.mint.toBase58(),
    initialPrice: state.realSolReserves.gt(new BN(0))
      ? state.realSolReserves.toNumber() / state.realTokenReserves.toNumber()
      : 0.001,
    maxLeverage: 10,
    liquidationFeeRatio: 0.05,
    protocol: "adrena",
  };
}
