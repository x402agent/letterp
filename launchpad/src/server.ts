/**
 * Launchpad HTTP API.
 *
 * Endpoints:
 *   POST /launch                Create a new mint + bonding curve (free for now)
 *   GET  /curve/:mint           Read curve state              [PAID via x402]
 *   GET  /curve/:mint/quote     Quote a buy or sell           [PAID via x402]
 *   POST /buy                   Returns an unsigned buy tx    [PAID via x402]
 *   POST /sell                  Returns an unsigned sell tx   [PAID via x402]
 *   GET  /agents/:asset/tokens  List tokens an agent has launched
 *
 * The PAID endpoints respond with 402 + PaymentRequirements when no
 * X-PAYMENT header is present. The facilitator service handles
 * verification & settlement — we never touch the buyer's wallet directly.
 */
import express, { Request, Response, NextFunction } from "express";
import { PublicKey } from "@solana/web3.js";
import BN from "bn.js";
import {
  PaymentRequirements,
  findAssetSignerPda,
} from "@x402pt/shared";
import { z } from "zod";
import { makeConnection, fetchCurveState } from "./state-reader";
import {
  buildCreateLaunch,
  buildCreateAgentToken,
  buildBuy,
  buildSell,
} from "./programs/launchpad-ix";
import { quoteBuy, quoteSell } from "./curves/constant-product";

const app = express();
app.use(express.json());

const connection = makeConnection();

// USDC mint — switch by network. Devnet uses 4zMM...; mainnet uses EPjF...
const USDC_MINT =
  process.env.SOLANA_NETWORK === "solana-devnet"
    ? "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU"
    : "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

const NETWORK =
  (process.env.SOLANA_NETWORK as "solana" | "solana-devnet") ?? "solana";

const PAYTO_FALLBACK = new PublicKey(
  process.env.PROTOCOL_TREASURY ??
    "11111111111111111111111111111111",
);

// ---------------------------------------------------------------------------
// x402 middleware
// ---------------------------------------------------------------------------

/**
 * Wraps a route so it returns 402 unless a valid X-PAYMENT header is present.
 * Behaviour:
 *   - if no header: emit PaymentRequirements (status 402),
 *   - if header present: forward to the facilitator's /verify and /settle.
 *
 * The `payTo` resolver is a callback because for agent-owned endpoints,
 * the destination is the agent's Asset Signer PDA (not the protocol).
 */
type PayToResolver = (req: Request) => PublicKey;

function require402(
  amountUsdc: string,
  description: string,
  payTo: PayToResolver = () => PAYTO_FALLBACK,
) {
  return async (req: Request, res: Response, next: NextFunction) => {
    const header = req.header("X-PAYMENT");
    const requirements: PaymentRequirements = {
      scheme: "exact",
      network: NETWORK,
      asset: USDC_MINT,
      maxAmountRequired: amountUsdc,
      payTo: payTo(req).toBase58(),
      resource: `${req.protocol}://${req.get("host")}${req.originalUrl}`,
      description,
      maxTimeoutSeconds: 60,
      mimeType: "application/json",
    };

    if (!header) {
      res
        .status(402)
        .header("PAYMENT-REQUIRED", Buffer.from(JSON.stringify(requirements)).toString("base64"))
        .json({ error: "Payment Required", accepts: [requirements] });
      return;
    }

    // Delegate verify+settle to the facilitator.
    try {
      const facilitator = process.env.FACILITATOR_URL ?? "http://localhost:4402";
      const verifyResp = await fetch(`${facilitator}/verify`, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ payment: header, requirements }),
      });
      const verify = await verifyResp.json();
      if (!verify.valid) {
        res.status(402).json({ error: "Payment invalid", reason: verify.invalidReason });
        return;
      }
      const settleResp = await fetch(`${facilitator}/settle`, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ payment: header, requirements }),
      });
      const settle = await settleResp.json();
      if (!settle.success) {
        res.status(402).json({ error: "Settlement failed", reason: settle.error });
        return;
      }
      res.setHeader(
        "X-PAYMENT-RESPONSE",
        Buffer.from(JSON.stringify(settle)).toString("base64"),
      );
      next();
    } catch (e: any) {
      res.status(502).json({ error: "Facilitator unreachable", detail: e.message });
    }
  };
}

// ---------------------------------------------------------------------------
// Routes
// ---------------------------------------------------------------------------

const LaunchBody = z.object({
  payer: z.string(), // base58 pubkey
  agentAsset: z.string().optional(), // Core asset address if the launcher is an agent
  name: z.string().max(32).optional(),
  symbol: z.string().max(10).optional(),
  uri: z.string().max(256).optional(),
  agentUri: z.string().max(256).optional(),
  memo: z.string().max(566).optional(),
  decimals: z.number().int().min(0).max(9).optional(),
  creatorFeeBps: z.number().int().min(0).max(1000).optional(),
  protocolFeeBps: z.number().int().min(0).max(1000).optional(),
  computeUnitLimit: z.number().int().positive().optional(),
  priorityFeeMicroLamports: z.number().int().nonnegative().optional(),
});

app.post("/launch", async (req, res) => {
  const parsed = LaunchBody.safeParse(req.body);
  if (!parsed.success) {
    res.status(400).json({ error: "bad input", issues: parsed.error.issues });
    return;
  }
  const payer = new PublicKey(parsed.data.payer);

  // If the launcher is an agent, route creator fees to its Asset Signer PDA
  // instead of the payer wallet. This is the same pattern Genesis uses.
  let creatorFeeWallet = payer;
  if (parsed.data.agentAsset) {
    const [pda] = findAssetSignerPda(new PublicKey(parsed.data.agentAsset));
    creatorFeeWallet = pda;
  }

  if (
    parsed.data.name &&
    parsed.data.symbol &&
    parsed.data.uri &&
    parsed.data.agentUri
  ) {
    const result = buildCreateAgentToken({
      payer,
      name: parsed.data.name,
      symbol: parsed.data.symbol,
      uri: parsed.data.uri,
      agentUri: parsed.data.agentUri,
      memo: parsed.data.memo,
      decimals: parsed.data.decimals,
      creatorFeeBps: parsed.data.creatorFeeBps,
      protocolFeeBps: parsed.data.protocolFeeBps,
      computeUnitLimit: parsed.data.computeUnitLimit,
      priorityFeeMicroLamports: parsed.data.priorityFeeMicroLamports,
    });

    res.json({
      mint: result.mint.publicKey.toBase58(),
      agent: result.agentPda.toBase58(),
      agentToken: result.agentTokenPda.toBase58(),
      curve: result.bondingCurvePda.toBase58(),
      vault: result.bondingCurveVaultPda.toBase58(),
      creatorVault: result.creatorVaultPda.toBase58(),
      mintSecret: Buffer.from(result.mint.secretKey).toString("base64"),
      instructions: result.instructions.map((ix) => ({
        programId: ix.programId.toBase58(),
        keys: ix.keys.map((k) => ({
          pubkey: k.pubkey.toBase58(),
          isSigner: k.isSigner,
          isWritable: k.isWritable,
        })),
        data: ix.data.toString("base64"),
      })),
    });
    return;
  }

  const result = await buildCreateLaunch(connection, {
    payer,
    creatorFeeWallet,
    memo: parsed.data.memo,
    decimals: parsed.data.decimals,
    creatorFeeBps: parsed.data.creatorFeeBps,
    protocolFeeBps: parsed.data.protocolFeeBps,
  });

  // Return the unsigned tx for the client to sign — we never see their key.
  res.json({
    mint: result.mint.publicKey.toBase58(),
    curve: result.curvePda.toBase58(),
    vault: result.curveVaultPda.toBase58(),
    // The mint Keypair must sign too; we expose its secret so the caller
    // can finalize the tx. In production this would be a partial-sign
    // pattern instead of leaking the key — for clarity here we just emit it.
    mintSecret: Buffer.from(result.mint.secretKey).toString("base64"),
    instructions: result.instructions.map((ix) => ({
      programId: ix.programId.toBase58(),
      keys: ix.keys.map((k) => ({
        pubkey: k.pubkey.toBase58(),
        isSigner: k.isSigner,
        isWritable: k.isWritable,
      })),
      data: ix.data.toString("base64"),
    })),
  });
});

app.get(
  "/curve/:mint",
  require402("1000", "Read curve state"), // 0.001 USDC (6 decimals)
  async (req, res) => {
    const mint = new PublicKey(req.params.mint);
    const state = await fetchCurveState(connection, mint);
    if (!state) {
      res.status(404).json({ error: "curve not found" });
      return;
    }
    res.json({
      mint: state.mint.toBase58(),
      authority: state.authority.toBase58(),
      creatorFeeWallet: state.creatorFeeWallet.toBase58(),
      realSolReserves: state.realSolReserves.toString(),
      realTokenReserves: state.realTokenReserves.toString(),
      virtualSolReserves: state.virtualSolReserves.toString(),
      virtualTokenReserves: state.virtualTokenReserves.toString(),
      totalSupply: state.totalSupply.toString(),
      graduated: state.graduated,
      creatorFeeBps: state.creatorFeeBps,
      protocolFeeBps: state.protocolFeeBps,
    });
  },
);

app.get(
  "/curve/:mint/quote",
  require402("1000", "Quote a swap"),
  async (req, res) => {
    const mint = new PublicKey(req.params.mint);
    const side = (req.query.side as string) ?? "buy";
    const amount = new BN(String(req.query.amount ?? "0"));
    const state = await fetchCurveState(connection, mint);
    if (!state) return res.status(404).json({ error: "curve not found" });
    try {
      const q = side === "sell" ? quoteSell(state, amount) : quoteBuy(state, amount);
      res.json({
        side,
        amountIn: q.amountIn.toString(),
        amountOut: q.amountOut.toString(),
        amountOutGross: q.amountOutGross.toString(),
        creatorFee: q.creatorFee.toString(),
        protocolFee: q.protocolFee.toString(),
        priceImpactBps: q.priceImpactBps,
      });
    } catch (e: any) {
      res.status(400).json({ error: e.message });
    }
  },
);

const SwapBody = z.object({
  mint: z.string(),
  buyer: z.string(),
  amount: z.string(),
  minOut: z.string(),
});

app.post(
  "/buy",
  require402("2000", "Build a buy tx"),
  async (req, res) => {
    const p = SwapBody.parse(req.body);
    const ix = buildBuy({
      mint: new PublicKey(p.mint),
      buyer: new PublicKey(p.buyer),
      solIn: new BN(p.amount),
      minTokensOut: new BN(p.minOut),
    });
    res.json({
      programId: ix.programId.toBase58(),
      keys: ix.keys.map((k) => ({
        pubkey: k.pubkey.toBase58(),
        isSigner: k.isSigner,
        isWritable: k.isWritable,
      })),
      data: ix.data.toString("base64"),
    });
  },
);

app.post(
  "/sell",
  require402("2000", "Build a sell tx"),
  async (req, res) => {
    const p = SwapBody.parse(req.body);
    const ix = buildSell({
      mint: new PublicKey(p.mint),
      seller: new PublicKey(p.buyer),
      tokensIn: new BN(p.amount),
      minSolOut: new BN(p.minOut),
    });
    res.json({
      programId: ix.programId.toBase58(),
      keys: ix.keys.map((k) => ({
        pubkey: k.pubkey.toBase58(),
        isSigner: k.isSigner,
        isWritable: k.isWritable,
      })),
      data: ix.data.toString("base64"),
    });
  },
);

const port = Number(process.env.PORT ?? 4400);
app.listen(port, () => {
  console.log(`Launchpad listening on http://localhost:${port}`);
  console.log(`Network: ${NETWORK}`);
});
