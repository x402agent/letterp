/**
 * PDA derivation helpers.
 *
 * The launchpad program owns the curve, agent registry, and vault PDAs.
 * The curve seeds intentionally match the self-hosted p-token launchpad spec:
 *   - ["bonding-curve", mint]
 *   - ["bonding-curve", mint, "vault"]
 *
 * Metaplex Core asset signer is derived as ['mpl-core-execute', asset_pubkey].
 */
import { PublicKey } from "@solana/web3.js";
import {
  BONDING_CURVE_PROGRAM_ID,
  MPL_CORE_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  P_TOKEN_LAUNCHPAD_PROGRAM_ID,
} from "./types";

export function findCurvePda(mint: PublicKey): [PublicKey, number] {
  return findBondingCurvePda(mint);
}

export function findBondingCurvePda(mint: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("bonding-curve"), mint.toBuffer()],
    BONDING_CURVE_PROGRAM_ID,
  );
}

export function findCurveVaultPda(mint: PublicKey): [PublicKey, number] {
  return findBondingCurveVaultPda(mint);
}

export function findBondingCurveVaultPda(mint: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("bonding-curve"), mint.toBuffer(), Buffer.from("vault")],
    BONDING_CURVE_PROGRAM_ID,
  );
}

export function findGlobalPda(): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("global")],
    P_TOKEN_LAUNCHPAD_PROGRAM_ID,
  );
}

export function findAgentPda(owner: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("agent"), owner.toBuffer()],
    P_TOKEN_LAUNCHPAD_PROGRAM_ID,
  );
}

export function findAgentTokenPda(mint: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("agent-token"), mint.toBuffer()],
    P_TOKEN_LAUNCHPAD_PROGRAM_ID,
  );
}

export function findCreatorVaultPda(creator: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("creator-vault"), creator.toBuffer()],
    P_TOKEN_LAUNCHPAD_PROGRAM_ID,
  );
}

export function findExecutionDelegationPda(
  agent: PublicKey,
  delegate: PublicKey,
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("exec-delegation"), agent.toBuffer(), delegate.toBuffer()],
    P_TOKEN_LAUNCHPAD_PROGRAM_ID,
  );
}

/**
 * The Asset Signer is the agent's wallet. No private key — only the asset
 * can sign for it via Core's Execute instruction. Same derivation as
 * `findAssetSignerPda` in @metaplex-foundation/mpl-core.
 */
export function findAssetSignerPda(asset: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("mpl-core-execute"), asset.toBuffer()],
    MPL_CORE_PROGRAM_ID,
  );
}

export function findAssociatedTokenAddress(
  owner: PublicKey,
  mint: PublicKey,
): PublicKey {
  return PublicKey.findProgramAddressSync(
    [owner.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mint.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  )[0];
}
