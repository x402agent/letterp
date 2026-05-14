/**
 * x402 Facilitator Service
 *
 * Verifies HTTP 402 payment challenges and settles them on-chain via
 * p-token transfers using Helius RPC. Implements these endpoints:
 *
 *   POST /supported   — list the tokens/networks this facilitator accepts
 *   POST /verify      — check whether an X-PAYMENT header is valid
 *   POST /settle      — settle a verified payment (initiate on-chain transfer)
 *
 * The facilitator never holds private keys. It validates payment proofs and
 * hands off settlement instructions to the caller or a relayer.
 */
import express from "express";
import { PublicKey } from "@solana/web3.js";
import { z } from "zod";
import { buildMemoInstruction } from "@x402pt/shared";

const app = express();
app.use(express.json({ limit: "1mb" }));

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

const PORT = Number(process.env.FACILITATOR_PORT ?? 4402);
const NETWORK = process.env.SOLANA_NETWORK ?? "solana";

const USDC_MINT = new PublicKey(
  NETWORK === "solana-devnet"
    ? "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU"
    : "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
);

// In-memory set of settled payment IDs (idempotency).
// In production this would be a DB or Redis.
const settledPayments = new Set<string>();

// ---------------------------------------------------------------------------
// Schemas
// ---------------------------------------------------------------------------

/** What the launchpad sends us to verify. */
const VerifyBody = z.object({
  payment: z.string(), // the X-PAYMENT header value (opaque string)
  requirements: z.object({
    scheme: z.literal("exact"),
    network: z.string(),
    asset: z.string(),
    maxAmountRequired: z.string(),
    payTo: z.string(),
    resource: z.string(),
    description: z.string(),
    maxTimeoutSeconds: z.number(),
    mimeType: z.string(),
  }),
});

const SettleBody = z.object({
  payment: z.string(),
  requirements: z.object({
    scheme: z.literal("exact"),
    network: z.string(),
    asset: z.string(),
    maxAmountRequired: z.string(),
    payTo: z.string(),
    resource: z.string(),
    description: z.string(),
    maxTimeoutSeconds: z.number(),
    mimeType: z.string(),
  }),
});

// ---------------------------------------------------------------------------
// Routes
// ---------------------------------------------------------------------------

/**
 * GET /supported — lists what this facilitator accepts.
 */
app.get("/supported", (_req, res) => {
  res.json({
    network: NETWORK,
    assets: [USDC_MINT.toBase58()],
    scheme: "exact",
    minAmount: "1",       // 0.000001 USDC
    maxAmount: "1000000000", // 1000 USDC
  });
});

/**
 * POST /verify — checks whether a payment proof is valid.
 *
 * The `payment` field is an opaque string the client received from
 * their wallet after authorising a transfer. For this reference impl,
 * we accept any non-empty string as a "valid" proof (in production,
 * the facilitator would verify a signed off-chain message or a
 * transaction signature).
 */
app.post("/verify", async (req, res) => {
  const parsed = VerifyBody.safeParse(req.body);
  if (!parsed.success) {
    res.status(400).json({ valid: false, invalidReason: "bad input" });
    return;
  }

  const { payment, requirements } = parsed.data;

  // Basic sanity: payment must be non-empty.
  if (!payment || payment.length < 8) {
    res.status(200).json({ valid: false, invalidReason: "payment too short" });
    return;
  }

  // Idempotency: already settled?
  const paymentId = `${requirements.resource}:${payment.slice(0, 16)}`;
  if (settledPayments.has(paymentId)) {
    res.status(200).json({ valid: false, invalidReason: "already settled" });
    return;
  }

  // In production, here we would:
  //   1. Decode `payment` as a Solana tx signature.
  //   2. Fetch the tx to confirm it's a USDC transfer to `payTo` for at least `maxAmountRequired`.
  //   3. Check the tx is recent enough (< maxTimeoutSeconds).

  // For now, accept any payment string that looks like a base58 tx sig (87–88 chars).
  const looksLikeSig = /^[1-9A-HJ-NP-Za-km-z]{87,88}$/.test(payment);
  if (!looksLikeSig) {
    res.status(200).json({ valid: false, invalidReason: "not a valid tx signature" });
    return;
  }

  res.status(200).json({ valid: true });
});

/**
 * POST /settle — confirms a payment has been settled on chain.
 *
 * For the reference implementation, settlement is a no-op: we trust that
 * the caller has already submitted the transfer tx. In production, the
 * facilitator would either:
 *   - Submit the transfer itself (if using a relayer key),
 *   - Wait for confirmation of the tx referenced in `payment`.
 */
app.post("/settle", async (req, res) => {
  const parsed = SettleBody.safeParse(req.body);
  if (!parsed.success) {
    res.status(400).json({ success: false, error: "bad input" });
    return;
  }

  const { payment, requirements } = parsed.data;
  const paymentId = `${requirements.resource}:${payment.slice(0, 16)}`;

  if (settledPayments.has(paymentId)) {
    res.status(200).json({ success: true, alreadySettled: true });
    return;
  }

  // Mark as settled for idempotency.
  settledPayments.add(paymentId);

  // In production, we would submit the transfer tx here or wait for
  // confirmation of the referenced tx.

  res.status(200).json({
    success: true,
    settlementTx: null, // no tx submitted in reference impl
    memo: serializeInstruction(
      buildMemoInstruction({
        memo: `x402:settled:${paymentId}`,
      }),
    ),
    network: NETWORK,
  });
});

/**
 * GET /health — simple health check.
 */
app.get("/health", (_req, res) => {
  res.json({ ok: true, network: NETWORK });
});

function serializeInstruction(ix: ReturnType<typeof buildMemoInstruction>) {
  return {
    programId: ix.programId.toBase58(),
    keys: ix.keys.map((key) => ({
      pubkey: key.pubkey.toBase58(),
      isSigner: key.isSigner,
      isWritable: key.isWritable,
    })),
    data: ix.data.toString("base64"),
  };
}

// ---------------------------------------------------------------------------
// Start
// ---------------------------------------------------------------------------

export const server = app.listen(PORT, () => {
  console.log(`x402 Facilitator listening on http://localhost:${PORT}`);
  console.log(`Network: ${NETWORK}, USDC: ${USDC_MINT.toBase58()}`);
});
