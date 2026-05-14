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
  Transaction,
  sendAndConfirmTransaction,
  ComputeBudgetProgram,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  P_TOKEN_PROGRAM_ID,
  P_TOKEN_LAUNCHPAD_PROGRAM_ID,
  BONDING_CURVE_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  findCurvePda,
  findCurveVaultPda,
  findGlobalPda,
  findAgentPda,
  findAgentTokenPda,
  findCreatorVaultPda,
  findExecutionDelegationPda,
  findAssociatedTokenAddress,
  buildMemoInstruction,
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
const IX_REGISTER_AGENT = 5;
const IX_CREATE_AGENT_TOKEN = 6;
const IX_REGISTER_EXECUTIVE = 7;
const IX_DELEGATE_EXECUTION = 8;
const IX_WITHDRAW_FEES = 9;

export const LAUNCHPAD_PROGRAM_ID = P_TOKEN_LAUNCHPAD_PROGRAM_ID;

function pushString(parts: Buffer[], value: string): void {
  const bytes = Buffer.from(value, "utf8");
  if (bytes.length > 0xffff) {
    throw new Error("String is too long for launchpad instruction encoding");
  }
  const len = Buffer.alloc(2);
  len.writeUInt16LE(bytes.length, 0);
  parts.push(len, bytes);
}

function u64(value: BN | bigint | number): Buffer {
  const bn =
    typeof value === "bigint"
      ? new BN(value.toString())
      : value instanceof BN
        ? value
        : new BN(value);
  return bn.toArrayLike(Buffer, "le", 8);
}

function u16(value: number): Buffer {
  const data = Buffer.alloc(2);
  data.writeUInt16LE(value, 0);
  return data;
}

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
  memo?: string;
}

export interface CreateLaunchResult {
  instructions: TransactionInstruction[];
  mint: Keypair;
  curvePda: PublicKey;
  curveVaultPda: PublicKey;
}

export interface CreateAgentTokenParams {
  connection?: Connection;
  payer: Keypair | PublicKey;
  name: string;
  symbol: string;
  uri: string;
  agentUri: string;
  decimals?: number;
  creatorFeeBps?: number;
  protocolFeeBps?: number;
  totalSupply?: BN;
  virtualSolReserves?: BN;
  virtualTokenReserves?: BN;
  mint?: Keypair;
  computeUnitLimit?: number;
  priorityFeeMicroLamports?: number;
  memo?: string;
}

export interface CreateAgentTokenResult {
  instructions: TransactionInstruction[];
  mint: Keypair;
  agentPda: PublicKey;
  agentTokenPda: PublicKey;
  bondingCurvePda: PublicKey;
  bondingCurveVaultPda: PublicKey;
  creatorVaultPda: PublicKey;
}

export function buildRegisterAgent(args: {
  owner: PublicKey;
  uri: string;
}): TransactionInstruction {
  const [global] = findGlobalPda();
  const [agent] = findAgentPda(args.owner);
  const parts = [Buffer.from([IX_REGISTER_AGENT])];
  pushString(parts, args.uri);

  return new TransactionInstruction({
    keys: [
      { pubkey: global, isSigner: false, isWritable: true },
      { pubkey: agent, isSigner: false, isWritable: true },
      { pubkey: args.owner, isSigner: true, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: LAUNCHPAD_PROGRAM_ID,
    data: Buffer.concat(parts),
  });
}

export function buildCreateAgentToken(
  args: Omit<CreateAgentTokenParams, "connection" | "payer"> & { payer: PublicKey },
): CreateAgentTokenResult {
  const mint = args.mint ?? Keypair.generate();
  const [global] = findGlobalPda();
  const [agentPda] = findAgentPda(args.payer);
  const [agentTokenPda] = findAgentTokenPda(mint.publicKey);
  const [bondingCurvePda] = findCurvePda(mint.publicKey);
  const [bondingCurveVaultPda] = findCurveVaultPda(mint.publicKey);
  const [creatorVaultPda] = findCreatorVaultPda(args.payer);

  const totalSupply = args.totalSupply ?? DEFAULT_CURVE.totalSupply;
  const vSol = args.virtualSolReserves ?? DEFAULT_CURVE.virtualSolReserves;
  const vTok = args.virtualTokenReserves ?? DEFAULT_CURVE.virtualTokenReserves;
  const creatorBps = args.creatorFeeBps ?? DEFAULT_CURVE.creatorFeeBps;
  const protocolBps = args.protocolFeeBps ?? DEFAULT_CURVE.protocolFeeBps;

  const parts = [
    Buffer.from([IX_CREATE_AGENT_TOKEN]),
    Buffer.from([args.decimals ?? 6]),
    u64(totalSupply),
    u64(vSol),
    u64(vTok),
    u16(creatorBps),
    u16(protocolBps),
  ];
  pushString(parts, args.name);
  pushString(parts, args.symbol);
  pushString(parts, args.uri);
  pushString(parts, args.agentUri);

  const instructions: TransactionInstruction[] = [];
  if (args.computeUnitLimit) {
    instructions.push(
      ComputeBudgetProgram.setComputeUnitLimit({
        units: args.computeUnitLimit,
      }),
    );
  }
  if (args.priorityFeeMicroLamports) {
    instructions.push(
      ComputeBudgetProgram.setComputeUnitPrice({
        microLamports: args.priorityFeeMicroLamports,
      }),
    );
  }
  if (args.memo) {
    instructions.push(
      buildMemoInstruction({
        memo: args.memo,
        signers: [args.payer],
      }),
    );
  }
  instructions.push(
    new TransactionInstruction({
      keys: [
        { pubkey: global, isSigner: false, isWritable: true },
        { pubkey: agentPda, isSigner: false, isWritable: true },
        { pubkey: agentTokenPda, isSigner: false, isWritable: true },
        { pubkey: bondingCurvePda, isSigner: false, isWritable: true },
        { pubkey: bondingCurveVaultPda, isSigner: false, isWritable: true },
        { pubkey: creatorVaultPda, isSigner: false, isWritable: true },
        { pubkey: mint.publicKey, isSigner: true, isWritable: true },
        { pubkey: args.payer, isSigner: true, isWritable: true },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
        { pubkey: P_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
        {
          pubkey: ASSOCIATED_TOKEN_PROGRAM_ID,
          isSigner: false,
          isWritable: false,
        },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: LAUNCHPAD_PROGRAM_ID,
      data: Buffer.concat(parts),
    }),
  );

  return {
    instructions,
    mint,
    agentPda,
    agentTokenPda,
    bondingCurvePda,
    bondingCurveVaultPda,
    creatorVaultPda,
  };
}

export async function createAgentToken(
  params: CreateAgentTokenParams & { connection: Connection; payer: Keypair },
): Promise<string> {
  const built = buildCreateAgentToken({
    ...params,
    payer: params.payer.publicKey,
  });
  const tx = new Transaction().add(...built.instructions);
  return sendAndConfirmTransaction(params.connection, tx, [params.payer, built.mint]);
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

  if (params.memo) {
    ixs.push(
      buildMemoInstruction({
        memo: params.memo,
        signers: [params.payer],
      }),
    );
  }

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

export function buildRegisterExecutive(args: {
  owner: PublicKey;
  agent?: PublicKey;
  delegate: PublicKey;
}): TransactionInstruction {
  const agent = args.agent ?? findAgentPda(args.owner)[0];
  const data = Buffer.concat([
    Buffer.from([IX_REGISTER_EXECUTIVE]),
    args.delegate.toBuffer(),
  ]);

  return new TransactionInstruction({
    keys: [
      { pubkey: agent, isSigner: false, isWritable: true },
      { pubkey: args.owner, isSigner: true, isWritable: false },
    ],
    programId: LAUNCHPAD_PROGRAM_ID,
    data,
  });
}

export function buildDelegateExecution(args: {
  owner: PublicKey;
  agent: PublicKey;
  delegate: PublicKey;
  expiresAtSlot: BN | bigint | number;
}): TransactionInstruction {
  const [delegation] = findExecutionDelegationPda(args.agent, args.delegate);
  const data = Buffer.concat([
    Buffer.from([IX_DELEGATE_EXECUTION]),
    u64(args.expiresAtSlot),
  ]);

  return new TransactionInstruction({
    keys: [
      { pubkey: delegation, isSigner: false, isWritable: true },
      { pubkey: args.agent, isSigner: false, isWritable: false },
      { pubkey: args.delegate, isSigner: false, isWritable: false },
      { pubkey: args.owner, isSigner: true, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: LAUNCHPAD_PROGRAM_ID,
    data,
  });
}

export function buildWithdrawFees(args: {
  creator: PublicKey;
  mint: PublicKey;
}): TransactionInstruction {
  const [creatorVault] = findCreatorVaultPda(args.creator);
  return new TransactionInstruction({
    keys: [
      { pubkey: creatorVault, isSigner: false, isWritable: true },
      { pubkey: args.mint, isSigner: false, isWritable: false },
      { pubkey: args.creator, isSigner: true, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: LAUNCHPAD_PROGRAM_ID,
    data: Buffer.from([IX_WITHDRAW_FEES]),
  });
}

export interface FeeDistributionRecipient {
  destinationAta: PublicKey;
  amount: BN | bigint | number;
}

export function buildFeeDistributionIx(
  sourceAta: PublicKey,
  mint: PublicKey,
  owner: PublicKey,
  recipients: FeeDistributionRecipient[],
  decimals = 6,
): TransactionInstruction {
  if (recipients.length > 255) {
    throw new Error("p-token batch supports at most 255 recipients per instruction");
  }

  const parts = [Buffer.from([25, recipients.length])];
  for (const recipient of recipients) {
    parts.push(u64(recipient.amount), Buffer.from([decimals]));
  }

  return new TransactionInstruction({
    keys: [
      { pubkey: sourceAta, isSigner: false, isWritable: true },
      { pubkey: mint, isSigner: false, isWritable: false },
      { pubkey: owner, isSigner: true, isWritable: false },
      ...recipients.map((recipient) => ({
        pubkey: recipient.destinationAta,
        isSigner: false,
        isWritable: true,
      })),
    ],
    programId: TOKEN_PROGRAM_ID,
    data: Buffer.concat(parts),
  });
}

export async function registerAgent(args: {
  connection: Connection;
  payer: Keypair;
  uri: string;
}): Promise<string> {
  const tx = new Transaction().add(
    buildRegisterAgent({ owner: args.payer.publicKey, uri: args.uri }),
  );
  return sendAndConfirmTransaction(args.connection, tx, [args.payer]);
}

export async function registerExecutive(args: {
  connection: Connection;
  payer: Keypair;
  agent?: PublicKey;
  delegate: PublicKey;
}): Promise<string> {
  const tx = new Transaction().add(
    buildRegisterExecutive({
      owner: args.payer.publicKey,
      agent: args.agent,
      delegate: args.delegate,
    }),
  );
  return sendAndConfirmTransaction(args.connection, tx, [args.payer]);
}

export async function delegateExecution(args: {
  connection: Connection;
  payer: Keypair;
  agent: PublicKey;
  delegate: PublicKey;
  expiresAtSlot: BN | bigint | number;
}): Promise<string> {
  const tx = new Transaction().add(
    buildDelegateExecution({
      owner: args.payer.publicKey,
      agent: args.agent,
      delegate: args.delegate,
      expiresAtSlot: args.expiresAtSlot,
    }),
  );
  return sendAndConfirmTransaction(args.connection, tx, [args.payer]);
}

export async function buy(args: {
  connection: Connection;
  payer: Keypair;
  mint: PublicKey;
  amountInLamports: BN | bigint | number;
  minTokensOut?: BN | bigint | number;
}): Promise<string> {
  const tx = new Transaction().add(
    buildBuy({
      mint: args.mint,
      buyer: args.payer.publicKey,
      solIn: new BN(args.amountInLamports.toString()),
      minTokensOut: new BN((args.minTokensOut ?? 1).toString()),
    }),
  );
  return sendAndConfirmTransaction(args.connection, tx, [args.payer]);
}

export async function sell(args: {
  connection: Connection;
  payer: Keypair;
  mint: PublicKey;
  tokensIn: BN | bigint | number;
  minSolOut?: BN | bigint | number;
}): Promise<string> {
  const tx = new Transaction().add(
    buildSell({
      mint: args.mint,
      seller: args.payer.publicKey,
      tokensIn: new BN(args.tokensIn.toString()),
      minSolOut: new BN((args.minSolOut ?? 1).toString()),
    }),
  );
  return sendAndConfirmTransaction(args.connection, tx, [args.payer]);
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
