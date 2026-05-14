import {
  PublicKey,
  Connection,
  Keypair,
  TransactionInstruction,
  SystemProgram,
} from "@solana/web3.js";
import BN from "bn.js";
import {
  MPL_CORE_PROGRAM_ID,
  P_TOKEN_LAUNCHPAD_PROGRAM_ID,
  findAssetSignerPda,
  findAgentPda,
  findExecutionDelegationPda,
  findGlobalPda,
} from "@x402pt/shared";
import { buildBuy, buildSell, buildCreateAgentToken } from "@x402pt/launchpad";
import { wrapAgentExecute } from "./execute";

export interface PAgentOptions {
  asset: PublicKey;
  connection: Connection;
  payer?: Keypair;
}

export class PAgent {
  readonly asset: PublicKey;
  readonly connection: Connection;
  readonly payer?: Keypair;

  constructor(opts: PAgentOptions) {
    this.asset = opts.asset;
    this.connection = opts.connection;
    this.payer = opts.payer;
  }

  get signerPda(): PublicKey {
    return findAssetSignerPda(this.asset)[0];
  }

  async register(uri: string): Promise<TransactionInstruction> {
    const [global] = findGlobalPda();
    const [agentPda] = findAgentPda(this.signerPda);
    const discriminator = Buffer.from([214, 136, 93, 144, 83, 112, 192, 124]);
    const uriBytes = Buffer.from(uri, "utf8");
    const uriLen = Buffer.alloc(4);
    uriLen.writeUInt32LE(uriBytes.length, 0);

    const data = Buffer.concat([discriminator, uriLen, uriBytes]);

    return new TransactionInstruction({
      keys: [
        { pubkey: global, isSigner: false, isWritable: true },
        { pubkey: agentPda, isSigner: false, isWritable: true },
        { pubkey: this.signerPda, isSigner: true, isWritable: true },
        {
          pubkey: SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        },
      ],
      programId: P_TOKEN_LAUNCHPAD_PROGRAM_ID,
      data,
    });
  }

  async launchToken(opts: {
    name: string;
    symbol: string;
    uri: string;
    decimals?: number;
    creatorFeeBps?: number;
  }): Promise<TransactionInstruction[]> {
    const signerPda = this.signerPda;

    const built = buildCreateAgentToken({
      payer: signerPda,
      name: opts.name,
      symbol: opts.symbol,
      uri: opts.uri,
      agentUri: opts.uri,
      decimals: opts.decimals,
      creatorFeeBps: opts.creatorFeeBps,
    });

    return built.instructions.map((ix) =>
      wrapAgentExecute(this.asset, signerPda, ix),
    );
  }

  async buy(
    mint: PublicKey,
    solIn: BN,
    minTokensOut: BN,
  ): Promise<TransactionInstruction> {
    const signerPda = this.signerPda;
    const innerIx = buildBuy({
      mint,
      buyer: signerPda,
      solIn,
      minTokensOut,
    });
    return wrapAgentExecute(this.asset, signerPda, innerIx, [mint]);
  }

  async sell(
    mint: PublicKey,
    tokensIn: BN,
    minSolOut: BN,
  ): Promise<TransactionInstruction> {
    const signerPda = this.signerPda;
    const innerIx = buildSell({
      mint,
      seller: signerPda,
      tokensIn,
      minSolOut,
    });
    return wrapAgentExecute(this.asset, signerPda, innerIx, [mint]);
  }

  async delegateTo(
    delegate: PublicKey,
    expiresAtSlot: number,
  ): Promise<TransactionInstruction> {
    const [agentPda] = findAgentPda(this.signerPda);
    const [delegationPda] = findExecutionDelegationPda(agentPda, delegate);

    const expiresSlotBuf = Buffer.alloc(8);
    new BN(expiresAtSlot).toArrayLike(Buffer, "le", 8).copy(expiresSlotBuf);

    const data = Buffer.concat([
      Buffer.from([8]), // IX_DELEGATE_EXECUTION
      expiresSlotBuf,
    ]);

    return new TransactionInstruction({
      keys: [
        { pubkey: delegationPda, isSigner: false, isWritable: true },
        { pubkey: agentPda, isSigner: false, isWritable: false },
        { pubkey: delegate, isSigner: false, isWritable: false },
        { pubkey: this.signerPda, isSigner: true, isWritable: true },
        {
          pubkey: SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        },
      ],
      programId: P_TOKEN_LAUNCHPAD_PROGRAM_ID,
      data,
    });
  }

  static fromAsset(asset: PublicKey, connection: Connection): PAgent {
    return new PAgent({ asset, connection });
  }
}
