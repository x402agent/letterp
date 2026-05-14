import {
  PublicKey,
  TransactionInstruction,
  Keypair,
  SystemProgram,
} from "@solana/web3.js";
import { MPL_CORE_PROGRAM_ID } from "@x402pt/shared";

export interface CoreAssetConfig {
  name: string;
  uri: string;
  collection?: PublicKey;
  owner?: PublicKey;
}

export interface MintCoreAssetResult {
  asset: PublicKey;
  assetSigner: PublicKey;
  instruction: TransactionInstruction;
  signers: Keypair[];
}

export function buildMintCoreAsset(
  payer: PublicKey,
  config: CoreAssetConfig,
): MintCoreAssetResult {
  const assetKeypair = Keypair.generate();
  const asset = assetKeypair.publicKey;

  const [assetSigner] = PublicKey.findProgramAddressSync(
    [Buffer.from("mpl-core-execute"), asset.toBuffer()],
    MPL_CORE_PROGRAM_ID,
  );

  const owner = config.owner ?? payer;

  const discriminator = Buffer.from([165, 186, 150, 180, 207, 183, 140, 123]);

  const nameBytes = Buffer.from(config.name, "utf8");
  const nameLenBuf = Buffer.alloc(4);
  nameLenBuf.writeUInt32LE(nameBytes.length, 0);

  const uriBytes = Buffer.from(config.uri, "utf8");
  const uriLenBuf = Buffer.alloc(4);
  uriLenBuf.writeUInt32LE(uriBytes.length, 0);

  const pluginsCountBuf = Buffer.alloc(4);
  pluginsCountBuf.writeUInt32LE(0, 0);

  const data = Buffer.concat([
    discriminator,
    nameLenBuf,
    nameBytes,
    uriLenBuf,
    uriBytes,
    pluginsCountBuf,
  ]);

  const keys: Array<{
    pubkey: PublicKey;
    isSigner: boolean;
    isWritable: boolean;
  }> = [
    { pubkey: asset, isSigner: true, isWritable: true },
  ];

  if (config.collection) {
    keys.push({
      pubkey: config.collection,
      isSigner: false,
      isWritable: true,
    });
  }

  keys.push(
    { pubkey: payer, isSigner: true, isWritable: false },
    { pubkey: payer, isSigner: true, isWritable: true },
    { pubkey: owner, isSigner: false, isWritable: false },
    { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
  );

  const instruction = new TransactionInstruction({
    keys,
    programId: MPL_CORE_PROGRAM_ID,
    data,
  });

  return {
    asset,
    assetSigner,
    instruction,
    signers: [assetKeypair],
  };
}

export function deriveAgentCollection(
  authority: PublicKey,
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("p-agent-collection"), authority.toBuffer()],
    MPL_CORE_PROGRAM_ID,
  );
}
