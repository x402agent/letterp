/**
 * Example: Launch a token via the launchpad HTTP API.
 *
 * Run:
 *   npm run start:launch-token
 *
 * This assumes the launchpad server is running on localhost:4400.
 */
import BN from "bn.js";
import { PublicKey } from "@solana/web3.js";
import {
  findCurvePda,
  findCurveVaultPda,
} from "@x402pt/shared";
import { makeConnection } from "@x402pt/launchpad";

const LAUNCHPAD_URL = process.env.LAUNCHPAD_URL ?? "http://localhost:4400";
const connection = makeConnection();

async function main() {
  console.log("=== Launch Token Example ===");
  console.log(`Launchpad URL: ${LAUNCHPAD_URL}`);
  console.log("");

  // In production, the payer would be your wallet's public key.
  const payer = "11111111111111111111111111111111"; // placeholder

  // Step 1: Request a new token launch.
  console.log("1. Requesting token launch...");
  const launchResp = await fetch(`${LAUNCHPAD_URL}/launch`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({
      payer,
      name: "Example p-token Agent",
      symbol: "PAGT",
      uri: "https://example.com/pagt-token.json",
      agentUri: "https://example.com/pagt-agent.json",
      decimals: 6,
      creatorFeeBps: 100, // 1%
    }),
  });

  if (!launchResp.ok) {
    console.error(`Launch failed: ${launchResp.status} ${await launchResp.text()}`);
    process.exit(1);
  }

  const launch = await launchResp.json();
  console.log(`   Mint:        ${launch.mint}`);
  console.log(`   Curve PDA:   ${launch.curve}`);
  console.log(`   Vault PDA:   ${launch.vault}`);
  console.log(`   Instructions: ${launch.instructions.length}`);
  console.log("");

  // Step 2: Derive PDAs locally to verify they match.
  const mint = new PublicKey(launch.mint);
  const [curvePda] = findCurvePda(mint);
  const [vaultPda] = findCurveVaultPda(mint);
  console.log("2. PDA verification:");
  console.log(`   Curve PDA (derived): ${curvePda.toBase58()}`);
  console.log(`   Server says:         ${launch.curve}`);
  console.log(`   Match: ${curvePda.toBase58() === launch.curve}`);
  console.log(`   Vault PDA (derived): ${vaultPda.toBase58()}`);
  console.log(`   Server says:         ${launch.vault}`);
  console.log(`   Match: ${vaultPda.toBase58() === launch.vault}`);
  console.log("");

  // Step 3: Try to read curve state (will get 402 unless paid).
  console.log("3. Trying to read curve state (expects 402)...");
  const curveResp = await fetch(`${LAUNCHPAD_URL}/curve/${launch.mint}`);
  if (curveResp.status === 402) {
    const body = await curveResp.json();
    console.log(`   402 Payment Required`);
    console.log(`   Accepts: ${JSON.stringify(body.accepts, null, 2)}`);
  } else {
    console.log(`   Status: ${curveResp.status}`);
  }
  console.log("");

  // Step 4: Get a buy quote (also requires payment).
  console.log("4. Trying to get a buy quote (expects 402)...");
  const quoteResp = await fetch(
    `${LAUNCHPAD_URL}/curve/${launch.mint}/quote?side=buy&amount=${new BN(100_000_000).toString()}`, // 0.1 SOL
  );
  if (quoteResp.status === 402) {
    const body = await quoteResp.json();
    console.log(`   402 Payment Required`);
  } else {
    console.log(`   Status: ${quoteResp.status}`);
  }
  console.log("");

  console.log("=== Done ===");
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
