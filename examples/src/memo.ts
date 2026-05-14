/**
 * Example: build a LetterP memo instruction.
 */
import { PublicKey } from "@solana/web3.js";
import {
  buildMemoInstruction,
  memoByteLength,
  P_MEMO_PROGRAM_ID,
} from "@x402pt/shared";

const signer = PublicKey.unique();
const memo = "letterp:agent-launch:demo";

const ix = buildMemoInstruction({
  memo,
  signers: [signer],
});

console.log("=== Memo Instruction Example ===");
console.log(`Program: ${P_MEMO_PROGRAM_ID.toBase58()}`);
console.log(`Memo bytes: ${memoByteLength(memo)}`);
console.log(`Signer: ${signer.toBase58()}`);
console.log(`Instruction accounts: ${ix.keys.length}`);
console.log(`Instruction data: ${ix.data.toString("base64")}`);
