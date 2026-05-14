/**
 * Example: An autonomous Metaplex Core agent launches its own token.
 *
 * This simulates the agent flow:
 *   1. Agent's owner registers the agent as a Core asset
 *   2. Agent calls /launch with its asset address
 *   3. Creator fees are routed to the agent's Asset Signer PDA
 *   4. Agent can then trade its own token
 *
 * Run:
 *   npm run start:agent-launch
 */
import { PublicKey } from "@solana/web3.js";
import { findAssetSignerPda } from "@x402pt/shared";
import { wrapAgentExecute } from "@x402pt/agent-sdk";
import { buildBuy } from "@x402pt/launchpad";
import BN from "bn.js";

// Simulated agent Core asset
const AGENT_ASSET = PublicKey.unique(); // In production, this is a real Core asset
const [agentSignerPda] = findAssetSignerPda(AGENT_ASSET);

async function main() {
  console.log("=== Agent Launch Example ===");
  console.log("");

  // Step 1: Show agent identity.
  console.log("1. Agent Identity:");
  console.log(`   Core Asset Address: ${AGENT_ASSET.toBase58()}`);
  console.log(`   Asset Signer PDA:   ${agentSignerPda.toBase58()}`);
  console.log("   (This PDA is the agent's wallet — no private key!)");
  console.log("");

  // Step 2: Route creator fees to the agent.
  console.log("2. Fee Routing:");
  console.log(`   Creator fees would accrue to: ${agentSignerPda.toBase58()}`);
  console.log("   (The agent controls this via Core Execute instructions)");
  console.log("");

  // Step 3: Wrap a Buy instruction in Core Execute.
  console.log("3. Wrapping a Buy instruction in Core Execute:");

  const dummyMint = PublicKey.unique();
  const buyIx = buildBuy({
    mint: dummyMint,
    buyer: agentSignerPda,
    solIn: new BN(100_000_000), // 0.1 SOL
    minTokensOut: new BN(1_000_000), // 1 token
  });

  const executeIx = wrapAgentExecute(
    AGENT_ASSET,
    agentSignerPda,
    buyIx,
    [dummyMint],
  );

  console.log(`   Inner IX program: ${buyIx.programId.toBase58()}`);
  console.log(`   Wrapped IX program: ${executeIx.programId.toBase58()}`);
  console.log(`   Wrapped IX accounts: ${executeIx.keys.length}`);
  console.log("");

  // Step 4: Simulate the launch request.
  console.log("4. Launch Request Payload:");
  const launchPayload = {
    payer: PublicKey.unique().toBase58(), // agent owner
    agentAsset: AGENT_ASSET.toBase58(),
    decimals: 6,
    creatorFeeBps: 100,
  };
  console.log(`   ${JSON.stringify(launchPayload, null, 4)}`);
  console.log("");
  console.log("   The server will route creator fees to the asset signer,");
  console.log("   and the agent can claim them later via a Core Execute.");
  console.log("");

  console.log("=== Done ===");
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
