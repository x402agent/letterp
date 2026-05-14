/**
 * Full end-to-end P Agent example.
 *
 * Demonstrates:
 *   1. Mint a Core asset (NFT) as the agent identity
 *   2. Register the agent with a URI
 *   3. Launch a P token from the agent
 *   4. Show the agent's signer PDA / treasury address
 *   5. Wrap a buy instruction via Core Execute
 *
 * Run:
 *   npm run start:p-agent-full
 */
import { PublicKey, Connection } from "@solana/web3.js";
import BN from "bn.js";
import { findAssetSignerPda } from "@x402pt/shared";
import {
  PAgent,
  buildMintCoreAsset,
  buildAgentRegistrationDoc,
} from "@x402pt/agent-sdk";

const DEVNET_RPC = "https://api.devnet.solana.com";

async function main() {
  console.log("=== P Agent Full Example ===");
  console.log("");

  const connection = new Connection(DEVNET_RPC, "confirmed");

  // ---------------------------------------------------------------------------
  // Step 1: Mint a Core asset (NFT) as the agent identity.
  // ---------------------------------------------------------------------------
  console.log("1. Minting Core Asset (agent identity)...");

  const payer = PublicKey.unique(); // In production: use a real Keypair

  const { asset, assetSigner, instruction: mintIx, signers } = buildMintCoreAsset(
    payer,
    {
      name: "My P Agent",
      uri: "https://example.com/agent-metadata.json",
      owner: payer,
    },
  );

  console.log(`   Asset address:     ${asset.toBase58()}`);
  console.log(`   Asset Signer PDA:  ${assetSigner.toBase58()}`);
  console.log(`   Mint signers:      ${signers.map((s) => s.publicKey.toBase58()).join(", ")}`);
  console.log(`   Instruction keys:  ${mintIx.keys.length}`);
  console.log("");

  // ---------------------------------------------------------------------------
  // Step 2: Register the agent with a URI.
  // ---------------------------------------------------------------------------
  console.log("2. Registering agent on-chain...");

  const agent = PAgent.fromAsset(asset, connection);

  const registrationDoc = buildAgentRegistrationDoc(asset.toBase58(), {
    name: "My P Agent",
    description: "An autonomous on-chain agent operating via Core Execute.",
    image: "https://example.com/agent-image.png",
    model: "claude-sonnet-4-6",
    capabilities: ["token-trading", "fee-routing", "delegation"],
    endpoint: "https://example.com/agent",
    services: [
      {
        name: "launchpad",
        endpoint: "https://example.com/launchpad",
        version: "1.0.0",
      },
    ],
    active: true,
    registrations: [
      {
        agentId: asset.toBase58(),
        agentRegistry: "https://example.com/registry",
      },
    ],
    supportedTrust: ["x402", "solana-signed"],
  });

  console.log("   Registration document:");
  console.log(`   ${JSON.stringify(registrationDoc, null, 4).split("\n").join("\n   ")}`);
  console.log("");

  const registerIx = await agent.register(
    "https://example.com/agent-metadata.json",
  );
  console.log(`   Register IX program:  ${registerIx.programId.toBase58()}`);
  console.log(`   Register IX accounts: ${registerIx.keys.length}`);
  console.log("");

  // ---------------------------------------------------------------------------
  // Step 3: Launch a P token from the agent.
  // ---------------------------------------------------------------------------
  console.log("3. Launching P token from agent...");

  const launchIxs = await agent.launchToken({
    name: "Agent Token",
    symbol: "AGNT",
    uri: "https://example.com/token-metadata.json",
    decimals: 6,
    creatorFeeBps: 150,
  });

  console.log(`   Launch instructions count: ${launchIxs.length}`);
  for (let i = 0; i < launchIxs.length; i++) {
    console.log(`   IX[${i}] program: ${launchIxs[i].programId.toBase58()}`);
  }
  console.log("");

  // ---------------------------------------------------------------------------
  // Step 4: Show the agent's signer PDA / treasury address.
  // ---------------------------------------------------------------------------
  console.log("4. Agent signer PDA (treasury / fee wallet)...");

  const [signerPda] = findAssetSignerPda(asset);
  console.log(`   Asset:       ${asset.toBase58()}`);
  console.log(`   Signer PDA:  ${signerPda.toBase58()}`);
  console.log("   (No private key — controlled exclusively via Core Execute)");
  console.log("");

  // ---------------------------------------------------------------------------
  // Step 5: Wrap a buy instruction via Core Execute.
  // ---------------------------------------------------------------------------
  console.log("5. Wrapping a buy instruction via Core Execute...");

  const dummyMint = PublicKey.unique();
  const buyIx = await agent.buy(
    dummyMint,
    new BN(50_000_000), // 0.05 SOL
    new BN(500_000),    // 0.5 tokens (6 decimals)
  );

  console.log(`   Mint:                  ${dummyMint.toBase58()}`);
  console.log(`   Core Execute program:  ${buyIx.programId.toBase58()}`);
  console.log(`   Total accounts:        ${buyIx.keys.length}`);
  console.log("");

  console.log("=== Done ===");
}

main().catch(console.error);
