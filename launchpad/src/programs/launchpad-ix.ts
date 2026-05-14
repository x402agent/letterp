/**
 * Transaction builders for launching a token on the bonding curve.
 *
 * After SIMD-0266 activates, every call into TOKEN_PROGRAM_ID hits the new
 * p-token implementation transparently — the instruction layouts are 100%
 * backward compatible. We use `@solana-program/token@^0.13` because it
 * includes the IDL updates for the three *new* p-token instructions
 * (batch, withdraw_excess_lamports, unwrap_lamports) — even though we don't
 * call them here, picking up the new IDL means indexers see the right shape.
 */
import {
  PublicKey,
  SystemProgram,
  TransactionInstruction,
  Connection,
  Keypair,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  BONDING_CURVE_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  findCurvePda,
  findCurveVaultPda,
  findAssociatedTokenAddress,
} from "@x402pt/shared";
import BN from "bn.js";
import { DEFAULT_CURVE } from "../curves/constant-product";

// ---------------------------------------------------------------------------
// Instruction discriminators for the bonding-curve program.
// (Match the Rust enum order in programs/src/instruction.rs.)
// ---------------------------------------------------------------------------
const IX_INITIALIZE_CURVE = 0;
const IX_BUY = 1;
const IX_SELL = 2;
const IX_GRADUATE = 3;
const IX_CLAIM_CREATOR_FEES = 4;

/**
 * Build the instructions to create a brand-new p-token mint and a bonding
 * curve initialised against it. Returns the unsigned instructions plus the
 * mint keypair (caller signs with it).
 *
 * The mint authority is set to the curve PDA so only the program can mint
 * — guaranteeing supply can never exceed the curve cap.
 */
export interface CreateLaunchParams {
  payer: PublicKey;
  /** Where creator fees accrue. For agents, this is the Asset Signer PDA. */
  creatorFeeWallet: PublicKey;
  decimals?: number;
  creatorFeeBps?: number;
  protocolFeeBps?: number;
  totalSupply?: BN;
  virtualSolReserves?: BN;
  virtualTokenReserves?: BN;
}

export interface CreateLaunchResult {
  instructions: TransactionInstruction[];
  mint: Keypair;
  curvePda: PublicKey;
  curveVaultPda: PublicKey;
}

export async function buildCreateLaunch(
  connection: Connection,
  params: CreateLaunchParams,
): Promise<CreateLaunchResult> {
  const mint = Keypair.generate();
  const [curvePda] = findCurvePda(mint.publicKey);
  const [curveVaultPda] = findCurveVaultPda(mint.publicKey);

  const decimals = params.decimals ?? 6;
  const mintRent = await connection.getMinimumBalanceForRentExemption(82); // Mint::LEN

  const ixs: TransactionInstruction[] = [];

  // 1. Allocate the mint account, owned by token program.
  ixs.push(
    SystemProgram.createAccount({
      fromPubkey: params.payer,
      newAccountPubkey: mint.publicKey,
      lamports: mintRent,
      space: 82,
      programId: TOKEN_PROGRAM_ID,
    }),
  );

  // 2. InitializeMint2 — mint authority is the curve PDA so only the curve
  //    can mint new supply. Freeze authority disabled.
  //
  //    Layout: [u8 disc=20, u8 decimals, Pubkey mintAuthority, u8 freezeOpt]
  const initMintData = Buffer.alloc(1 + 1 + 32 + 1);
  initMintData.writeUInt8(20, 0); // InitializeMint2
  initMintData.writeUInt8(decimals, 1);
  curvePda.toBuffer().copy(initMintData, 2);
  initMintData.writeUInt8(0, 34); // no freeze authority
  ixs.push(
    new TransactionInstruction({
      keys: [{ pubkey: mint.publicKey, isSigner: false, isWritable: true }],
      programId: TOKEN_PROGRAM_ID,
      data: initMintData,
    }),
  );

  // 3. InitializeCurve on our program.
  //    Layout: [u8 disc, u64 totalSupply, u64 vSol, u64 vTok, u16 creatorBps, u16 protoBps]
  const totalSupply = params.totalSupply ?? DEFAULT_CURVE.totalSupply;
  const vSol = params.virtualSolReserves ?? DEFAULT_CURVE.virtualSolReserves;
  const vTok =
    params.virtualTokenReserves ?? DEFAULT_CURVE.virtualTokenReserves;
  const creatorBps = params.creatorFeeBps ?? DEFAULT_CURVE.creatorFeeBps;
  const protoBps = params.protocolFeeBps ?? DEFAULT_CURVE.protocolFeeBps;

  const initCurveData = Buffer.alloc(1 + 8 + 8 + 8 + 2 + 2);
  let off = 0;
  initCurveData.writeUInt8(IX_INITIALIZE_CURVE, off);
  off += 1;
  initCurveData.set(totalSupply.toArrayLike(Buffer, "le", 8), off);
  off += 8;
  initCurveData.set(vSol.toArrayLike(Buffer, "le", 8), off);
  off += 8;
  initCurveData.set(vTok.toArrayLike(Buffer, "le", 8), off);
  off += 8;
  initCurveData.writeUInt16LE(creatorBps, off);
  off += 2;
  initCurveData.writeUInt16LE(protoBps, off);

  ixs.push(
    new TransactionInstruction({
      keys: [
        { pubkey: params.payer, isSigner: true, isWritable: true },
        { pubkey: curvePda, isSigner: false, isWritable: true },
        { pubkey: curveVaultPda, isSigner: false, isWritable: true },
        { pubkey: mint.publicKey, isSigner: false, isWritable: true },
        { pubkey: params.creatorFeeWallet, isSigner: false, isWritable: false },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: BONDING_CURVE_PROGRAM_ID,
      data: initCurveData,
    }),
  );

  return { instructions: ixs, mint, curvePda, curveVaultPda };
}

/**
 * Build a Buy instruction against an existing curve.
 *
 * Note: if `buyer` is an agent's Asset Signer PDA, the caller must wrap
 * the transaction in a Core Execute instruction so the agent signs. See
 * agent-sdk/src/execute.ts for that wrapper.
 */
export function buildBuy(args: {
  mint: PublicKey;
  buyer: PublicKey;
  solIn: BN;
  minTokensOut: BN;
}): TransactionInstruction {
  const [curvePda] = findCurvePda(args.mint);
  const [curveVaultPda] = findCurveVaultPda(args.mint);
  const buyerAta = findAssociatedTokenAddress(args.buyer, args.mint);

  const data = Buffer.alloc(1 + 8 + 8);
  data.writeUInt8(IX_BUY, 0);
  data.set(args.solIn.toArrayLike(Buffer, "le", 8), 1);
  data.set(args.minTokensOut.toArrayLike(Buffer, "le", 8), 9);

  return new TransactionInstruction({
    keys: [
      { pubkey: args.buyer, isSigner: true, isWritable: true },
      { pubkey: buyerAta, isSigner: false, isWritable: true },
      { pubkey: curvePda, isSigner: false, isWritable: true },
      { pubkey: curveVaultPda, isSigner: false, isWritable: true },
      { pubkey: args.mint, isSigner: false, isWritable: true },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      {
        pubkey: ASSOCIATED_TOKEN_PROGRAM_ID,
        isSigner: false,
        isWritable: false,
      },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: BONDING_CURVE_PROGRAM_ID,
    data,
  });
}

/** Symmetrical to buildBuy. */
export function buildSell(args: {
  mint: PublicKey;
  seller: PublicKey;
  tokensIn: BN;
  minSolOut: BN;
}): TransactionInstruction {
  const [curvePda] = findCurvePda(args.mint);
  const [curveVaultPda] = findCurveVaultPda(args.mint);
  const sellerAta = findAssociatedTokenAddress(args.seller, args.mint);

  const data = Buffer.alloc(1 + 8 + 8);
  data.writeUInt8(IX_SELL, 0);
  data.set(args.tokensIn.toArrayLike(Buffer, "le", 8), 1);
  data.set(args.minSolOut.toArrayLike(Buffer, "le", 8), 9);

  return new TransactionInstruction({
    keys: [
      { pubkey: args.seller, isSigner: true, isWritable: true },
      { pubkey: sellerAta, isSigner: false, isWritable: true },
      { pubkey: curvePda, isSigner: false, isWritable: true },
      { pubkey: curveVaultPda, isSigner: false, isWritable: true },
      { pubkey: args.mint, isSigner: false, isWritable: true },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: BONDING_CURVE_PROGRAM_ID,
    data,
  });
}

/** Trigger graduation once the curve threshold is hit. */
export function buildGraduate(args: {
  mint: PublicKey;
  cranker: PublicKey;
  raydiumPool: PublicKey;
}): TransactionInstruction {
  const [curvePda] = findCurvePda(args.mint);
  const [curveVaultPda] = findCurveVaultPda(args.mint);
  const data = Buffer.from([IX_GRADUATE]);

  return new TransactionInstruction({
    keys: [
      { pubkey: args.cranker, isSigner: true, isWritable: true },
      { pubkey: curvePda, isSigner: false, isWritable: true },
      { pubkey: curveVaultPda, isSigner: false, isWritable: true },
      { pubkey: args.mint, isSigner: false, isWritable: true },
      { pubkey: args.raydiumPool, isSigner: false, isWritable: true },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: BONDING_CURVE_PROGRAM_ID,
    data,
  });
}

/** Claim accumulated creator fees from a curve. */
export function buildClaimCreatorFees(args: {
  mint: PublicKey;
  recipient: PublicKey;
}): TransactionInstruction {
  const [curvePda] = findCurvePda(args.mint);
  const [curveVaultPda] = findCurveVaultPda(args.mint);
  const data = Buffer.from([IX_CLAIM_CREATOR_FEES]);

  return new TransactionInstruction({
    keys: [
      { pubkey: args.recipient, isSigner: true, isWritable: true },
      { pubkey: curvePda, isSigner: false, isWritable: true },
      { pubkey: curveVaultPda, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: BONDING_CURVE_PROGRAM_ID,
    data,
  });
}
