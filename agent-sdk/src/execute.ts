/**
 * Agent Execute wrapper — wraps an instruction inside a Metaplex Core
 * Execute instruction so the asset (agent) can sign for the operation.
 *
 * The agent has no private key. Instead, the Core `Execute` instruction
 * validates that the asset's signer PDA approves the CPI call. This is
 * the same mechanism Metaplex Genesis uses.
 *
 * Usage:
 *   const innerIx = buildBuy({ mint, buyer: assetSigner, solIn, minTokensOut });
 *   const executeIx = wrapAgentExecute(asset, assetSigner, innerIx, [mint]);
 */
import {
  PublicKey,
  TransactionInstruction,
} from "@solana/web3.js";
import {
  MPL_CORE_PROGRAM_ID,
  findAssetSignerPda,
} from "@x402pt/shared";

/**
 * Wrap an instruction inside a Metaplex Core Execute instruction.
 *
 * The `Execute` instruction is a CPI gateway: it forwards the inner
 * instruction data to the target program, but only if the asset approves.
 * The asset signs via its derivation — no private key needed.
 *
 * @param asset       - The Core asset address (the agent).
 * @param signerPda   - The Asset Signer PDA (derived from asset).
 * @param innerIx     - The instruction the agent wants to sign.
 * @param extraKeys   - Any additional accounts the inner ix references.
 * @returns           - The wrapped Execute instruction.
 */
export function wrapAgentExecute(
  asset: PublicKey,
  signerPda: PublicKey,
  innerIx: TransactionInstruction,
  extraKeys: PublicKey[] = [],
): TransactionInstruction {
  const [expectedSignerPda] = findAssetSignerPda(asset);

  // Sanity: the signerPda must match.
  if (!signerPda.equals(expectedSignerPda)) {
    throw new Error(
      `signerPda mismatch: got ${signerPda.toBase58()}, expected ${expectedSignerPda.toBase58()}`,
    );
  }

  // The Execute instruction layout from mpl-core:
  //   accounts:
  //     - asset (writable)
  //     - signer_pda (signer)
  //     - program_id of the inner instruction
  //   data:
  //     - discriminator (0x0c)
  //     - inner instruction data
  const data = Buffer.concat([
    Buffer.from([0x0c]), // Execute discriminator for mpl-core
    innerIx.data,
  ]);

  const keys = [
    { pubkey: asset, isSigner: false, isWritable: true },
    { pubkey: signerPda, isSigner: true, isWritable: false },
    { pubkey: innerIx.programId, isSigner: false, isWritable: false },
    // Include all accounts the inner instruction needs (as remapped by the program).
    ...innerIx.keys.map((k) => ({
      pubkey: k.pubkey,
      isSigner: k.isSigner,
      isWritable: k.isWritable,
    })),
    // Include any extra keys the caller specified.
    ...extraKeys.map((pk) => ({
      pubkey: pk,
      isSigner: false,
      isWritable: false,
    })),
  ];

  return new TransactionInstruction({
    keys,
    programId: MPL_CORE_PROGRAM_ID,
    data,
  });
}
