/**
 * Agent launchpad client — a thin SDK an agent (or its owner) uses to
 * interact with the launchpad HTTP API from within a Core asset.
 *
 * All paid endpoints go through the x402 flow:
 *   1. Agent requests an endpoint without X-PAYMENT
 *   2. Server returns 402 + PaymentRequirements
 *   3. Agent creates a USDC transfer tx (via its owner's wallet)
 *   4. Agent retries with X-PAYMENT set to the tx signature
 *   5. Server verifies + settles via the facilitator
 */
import { PublicKey, Connection, Keypair, Transaction } from "@solana/web3.js";
import BN from "bn.js";
import {
  PaymentRequirements,
  findAssetSignerPda,
} from "@x402pt/shared";

export interface LaunchpadClientOptions {
  baseUrl: string;
  /** If set, the client will auto-pay 402 challenges using this wallet.
   *  The wallet must have USDC to cover the fee. */
  payer?: Keypair;
  connection?: Connection;
}

export interface LaunchResult {
  mint: string;
  curve: string;
  vault: string;
  mintSecret: string;
  instructions: Array<{
    programId: string;
    keys: Array<{ pubkey: string; isSigner: boolean; isWritable: boolean }>;
    data: string;
  }>;
}

export interface CurveStateResult {
  mint: string;
  authority: string;
  creatorFeeWallet: string;
  realSolReserves: string;
  realTokenReserves: string;
  virtualSolReserves: string;
  virtualTokenReserves: string;
  totalSupply: string;
  graduated: boolean;
  creatorFeeBps: number;
  protocolFeeBps: number;
}

export interface SwapResult {
  programId: string;
  keys: Array<{ pubkey: string; isSigner: boolean; isWritable: boolean }>;
  data: string;
}

export class LaunchpadClient {
  private baseUrl: string;
  private payer?: Keypair;
  private connection?: Connection;

  constructor(opts: LaunchpadClientOptions) {
    this.baseUrl = opts.baseUrl.replace(/\/$/, "");
    this.payer = opts.payer;
    this.connection = opts.connection;
  }

  // -------------------------------------------------------------------
  // Paid request helper — handles the 402 flow automatically if a payer
  // is configured.
  // -------------------------------------------------------------------

  private async paidFetch<T>(
    path: string,
    options: RequestInit = {},
    payDescription?: string,
  ): Promise<T> {
    const url = `${this.baseUrl}${path}`;

    // Try without payment first.
    let res = await fetch(url, { ...options, headers: { ...options.headers as Record<string, string> } });

    if (res.status === 402 && this.payer) {
      const body = await res.json();
      const requirements: PaymentRequirements = body.accepts?.[0];
      if (!requirements) throw new Error("No payment requirements in 402 response");

      // The payer must send `maxAmountRequired` USDC to `payTo`.
      // This is a simplified flow — in production the payer would
      // construct a USDC transfer tx, sign it, and submit.
      console.log(`Paying ${requirements.maxAmountRequired} USDC to ${requirements.payTo} for ${payDescription ?? requirements.resource}`);

      // Build a dummy tx signature for the reference implementation.
      const dummySig = Keypair.generate().publicKey.toBase58();

      // Retry with payment header.
      const paidHeaders: Record<string, string> = {
        ...(options.headers as Record<string, string>),
        "X-PAYMENT": dummySig,
      };
      res = await fetch(url, { ...options, headers: paidHeaders });
    }

    if (!res.ok) {
      const err = await res.json().catch(() => ({}));
      throw new Error(`HTTP ${res.status}: ${(err as any).error ?? res.statusText}`);
    }

    return res.json();
  }

  // -------------------------------------------------------------------
  // API methods
  // -------------------------------------------------------------------

  async launchToken(params: {
    payer: string;
    agentAsset?: string;
    decimals?: number;
    creatorFeeBps?: number;
  }): Promise<LaunchResult> {
    return this.paidFetch<LaunchResult>("/launch", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify(params),
    });
  }

  async getCurveState(mint: string): Promise<CurveStateResult> {
    return this.paidFetch<CurveStateResult>(`/curve/${mint}`);
  }

  async getQuote(
    mint: string,
    side: "buy" | "sell",
    amount: BN,
  ): Promise<{
    side: string;
    amountIn: string;
    amountOut: string;
    amountOutGross: string;
    creatorFee: string;
    protocolFee: string;
    priceImpactBps: number;
  }> {
    return this.paidFetch(
      `/curve/${mint}/quote?side=${side}&amount=${amount.toString()}`,
    );
  }

  async buildBuyTx(params: {
    mint: string;
    buyer: string;
    amount: string;
    minOut: string;
  }): Promise<SwapResult> {
    return this.paidFetch<SwapResult>("/buy", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify(params),
    }, "Build a buy tx");
  }

  async buildSellTx(params: {
    mint: string;
    buyer: string;
    amount: string;
    minOut: string;
  }): Promise<SwapResult> {
    return this.paidFetch<SwapResult>("/sell", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify(params),
    }, "Build a sell tx");
  }
}
