/**
 * Constant-product bonding curve math.
 *
 * The curve uses virtual reserves so the initial price is non-zero even before
 * any real liquidity is added. This matches the design used by pump.fun and
 * Metaplex Genesis bonding curves.
 *
 * Invariant:    (virtualSol + realSol) * (virtualTokens + realTokens) = k
 *
 * On a buy with `solIn`:
 *   tokensOut = realTokens - k / (virtualSol + realSol + solIn - virtualTokens)
 *   ...simplifying to keep virtual reserves separate:
 *   tokensOut = (virtualTokens + realTokens) -
 *               k / (virtualSol + realSol + solIn)
 *
 * All math is u64-safe via bn.js. We round in the protocol's favour
 * (down on outputs, up on inputs) to prevent rounding-attack drains.
 */
import BN from "bn.js";
import { BondingCurveState, SwapQuote } from "@x402pt/shared";

const BPS = new BN(10_000);

/** Default curve params used when an agent doesn't override them. */
export const DEFAULT_CURVE = {
  /** 1B tokens (with 6 decimals = 1e15 base units). */
  totalSupply: new BN("1000000000000000"),
  /** Virtual SOL reserves = 30 SOL. */
  virtualSolReserves: new BN(30).mul(new BN(1e9)),
  /** Virtual token reserves = total supply (gives a nice starting price). */
  virtualTokenReserves: new BN("1000000000000000"),
  /** Curve graduates when 80% of tokens have been sold. */
  graduationThresholdBps: 8000,
  creatorFeeBps: 100, // 1%
  protocolFeeBps: 50, // 0.5%
};

/** k = (vSol + rSol) * (vTok + rTok). */
function k(state: BondingCurveState): BN {
  const sol = state.virtualSolReserves.add(state.realSolReserves);
  const tok = state.virtualTokenReserves.add(state.realTokenReserves);
  return sol.mul(tok);
}

/**
 * Quote a buy: spend `solIn` lamports, receive tokens.
 * Returns gross output (no fee deducted) and the fee breakdown.
 */
export function quoteBuy(state: BondingCurveState, solIn: BN): SwapQuote {
  if (state.graduated) {
    throw new Error("Curve graduated — trade on Raydium / perps instead");
  }

  // Fees on a buy come off the SOL input — they don't enter the curve.
  const protocolFee = solIn.mul(new BN(state.protocolFeeBps)).div(BPS);
  const creatorFee = solIn.mul(new BN(state.creatorFeeBps)).div(BPS);
  const solInNet = solIn.sub(protocolFee).sub(creatorFee);

  const invariant = k(state);
  const newSol = state.virtualSolReserves
    .add(state.realSolReserves)
    .add(solInNet);
  // Round token-output *down*: protocol keeps the dust.
  const newTok = invariant.div(newSol);
  const currentTok = state.virtualTokenReserves.add(state.realTokenReserves);
  const tokensOut = currentTok.sub(newTok);

  if (tokensOut.gt(state.realTokenReserves)) {
    throw new Error("Buy exceeds remaining curve supply");
  }

  const priceImpactBps = solIn.isZero()
    ? 0
    : tokensOut.mul(BPS).div(currentTok).toNumber();

  return {
    amountIn: solIn,
    amountOutGross: tokensOut,
    amountOut: tokensOut,
    creatorFee,
    protocolFee,
    priceImpactBps,
  };
}

/**
 * Quote a sell: provide `tokensIn`, receive SOL.
 */
export function quoteSell(state: BondingCurveState, tokensIn: BN): SwapQuote {
  if (state.graduated) {
    throw new Error("Curve graduated — trade on Raydium / perps instead");
  }
  if (tokensIn.gt(state.totalSupply.sub(state.realTokenReserves))) {
    throw new Error("Sell exceeds tokens that have been bought");
  }

  const invariant = k(state);
  const newTok = state.virtualTokenReserves
    .add(state.realTokenReserves)
    .add(tokensIn);
  // Round SOL output *down*.
  const newSol = invariant.div(newTok);
  const currentSol = state.virtualSolReserves.add(state.realSolReserves);
  const solOutGross = currentSol.sub(newSol);

  // Fees come out of the SOL the user receives.
  const protocolFee = solOutGross.mul(new BN(state.protocolFeeBps)).div(BPS);
  const creatorFee = solOutGross.mul(new BN(state.creatorFeeBps)).div(BPS);
  const solOut = solOutGross.sub(protocolFee).sub(creatorFee);

  const priceImpactBps = currentSol.isZero()
    ? 0
    : solOutGross.mul(BPS).div(currentSol).toNumber();

  return {
    amountIn: tokensIn,
    amountOutGross: solOutGross,
    amountOut: solOut,
    creatorFee,
    protocolFee,
    priceImpactBps,
  };
}

/**
 * Apply a buy to the in-memory state (used by tests and simulations).
 * The on-chain program does the equivalent in Rust.
 */
export function applyBuy(
  state: BondingCurveState,
  quote: SwapQuote,
): BondingCurveState {
  return {
    ...state,
    realSolReserves: state.realSolReserves.add(
      quote.amountIn.sub(quote.creatorFee).sub(quote.protocolFee),
    ),
    realTokenReserves: state.realTokenReserves.sub(quote.amountOut),
  };
}

export function applySell(
  state: BondingCurveState,
  quote: SwapQuote,
): BondingCurveState {
  // The SOL coming out includes fees that get split off to creator/protocol.
  return {
    ...state,
    realSolReserves: state.realSolReserves.sub(quote.amountOutGross),
    realTokenReserves: state.realTokenReserves.add(quote.amountIn),
  };
}

/** Has the curve crossed its graduation threshold? */
export function isReadyToGraduate(
  state: BondingCurveState,
  thresholdBps: number = DEFAULT_CURVE.graduationThresholdBps,
): boolean {
  const sold = state.totalSupply.sub(state.realTokenReserves);
  return sold.mul(BPS).gte(state.totalSupply.mul(new BN(thresholdBps)));
}

/** Current spot price in lamports per token base unit. */
export function spotPrice(state: BondingCurveState): number {
  const sol = state.virtualSolReserves.add(state.realSolReserves);
  const tok = state.virtualTokenReserves.add(state.realTokenReserves);
  // Returns a float — fine for display, never for on-chain math.
  return sol.toNumber() / tok.toNumber();
}
