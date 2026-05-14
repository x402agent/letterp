/**
 * PDA derivation helpers.
 *
 * The bonding-curve program owns two PDAs per curve:
 *   - the curve state account ("curve" + mint)
 *   - the vault that holds the curve's SOL ("vault" + mint)
 *
 * Metaplex Core asset signer is derived as ['mpl-core-execute', asset_pubkey].
 */
import { PublicKey } from "@solana/web3.js";
import {
  BONDING_CURVE_PROGRAM_ID,
  MPL_CORE_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "./types";

export function findCurvePda(mint: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("curve"), mint.toBuffer()],
    BONDING_CURVE_PROGRAM_ID,
  );
}

export function findCurveVaultPda(mint: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), mint.toBuffer()],
    BONDING_CURVE_PROGRAM_ID,
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
