import {
  AccountMeta,
  PublicKey,
  TransactionInstruction,
} from "@solana/web3.js";
import { P_MEMO_PROGRAM_ID } from "./types";

export interface MemoInstructionParams {
  memo: string;
  signers?: PublicKey[];
  programId?: PublicKey;
}

export function encodeMemo(memo: string): Buffer {
  const data = Buffer.from(memo, "utf8");
  if (data.toString("utf8") !== memo) {
    throw new Error("memo must be valid UTF-8");
  }
  return data;
}

export function buildMemoInstruction({
  memo,
  signers = [],
  programId = P_MEMO_PROGRAM_ID,
}: MemoInstructionParams): TransactionInstruction {
  const keys: AccountMeta[] = signers.map((pubkey) => ({
    pubkey,
    isSigner: true,
    isWritable: false,
  }));

  return new TransactionInstruction({
    programId,
    keys,
    data: encodeMemo(memo),
  });
}

export function memoByteLength(memo: string): number {
  return encodeMemo(memo).length;
}
