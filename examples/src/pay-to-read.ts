/**
 * Example: x402 Payment flow — pay USDC to read curve state.
 *
 * This simulates the full x402 flow:
 *   1. Client requests /curve/:mint without payment
 *   2. Server responds 402 + PaymentRequirements
 *   3. Client constructs a USDC transfer tx
 *   4. Client retries with X-PAYMENT header
 *   5. Server verifies and settles via the facilitator
 *
 * Run:
 *   npm run start:pay-to-read
 *
 * This assumes the launchpad is running on localhost:4400 and
 * the facilitator on localhost:4402.
 */
import { PublicKey } from "@solana/web3.js";
import { PaymentRequirements } from "@x402pt/shared";

const LAUNCHPAD_URL = process.env.LAUNCHPAD_URL ?? "http://localhost:4400";
const FACILITATOR_URL = process.env.FACILITATOR_URL ?? "http://localhost:4402";

async function main() {
  console.log("=== x402 Payment Flow Example ===");
  console.log("");

  // Use a dummy mint for the example.
  const mint = PublicKey.unique().toBase58();

  // Step 1: Request without payment.
  console.log("1. Requesting curve state without payment...");
  let res = await fetch(`${LAUNCHPAD_URL}/curve/${mint}`);

  if (res.status !== 402) {
    console.log(`   Unexpected status: ${res.status}`);
    process.exit(1);
  }

  const body = await res.json();
  console.log(`   Got 402 Payment Required!`);
  const requirements: PaymentRequirements = body.accepts[0];
  console.log(`   Resource: ${requirements.resource}`);
  console.log(`   Pay: ${requirements.maxAmountRequired} USDC to ${requirements.payTo}`);
  console.log(`   Network: ${requirements.network}, Asset: ${requirements.asset}`);
  console.log("");

  // Step 2: Verify with the facilitator.
  console.log("2. Verifying payment with facilitator...");
  const dummySig = "5VERv8NM1iFVMWkJ1e8mPbGdsRWNd4mMx5y7tVJmZ8RqKpBuYRqQpBuYRqQpBuYRqQpBuYRqQpBuYRqQpBuYRq";
  const verifyResp = await fetch(`${FACILITATOR_URL}/verify`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({
      payment: dummySig,
      requirements,
    }),
  });
  const verify = await verifyResp.json();
  console.log(`   Valid: ${verify.valid}`);
  if (!verify.valid) {
    console.log(`   Reason: ${verify.invalidReason}`);
  }
  console.log("");

  // Step 3: Settle with the facilitator.
  console.log("3. Settling payment with facilitator...");
  const settleResp = await fetch(`${FACILITATOR_URL}/settle`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({
      payment: dummySig,
      requirements,
    }),
  });
  const settle = await settleResp.json();
  console.log(`   Success: ${settle.success}`);
  console.log(`   Already settled: ${settle.alreadySettled ?? false}`);
  console.log("");

  // Step 4: Retry with payment header.
  console.log("4. Retrying with X-PAYMENT header...");
  res = await fetch(`${LAUNCHPAD_URL}/curve/${mint}`, {
    headers: { "X-PAYMENT": dummySig },
  });
  console.log(`   Status: ${res.status}`);
  if (res.ok) {
    const data = await res.json();
    console.log(`   Data: ${JSON.stringify(data, null, 2)}`);
  } else {
    const err = await res.json().catch(() => ({}));
    console.log(`   Error: ${JSON.stringify(err)}`);
  }
  console.log("");

  console.log("=== Done ===");
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
