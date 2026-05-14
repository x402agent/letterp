/**
 * Off-chain state reader for the bonding-curve program.
 *
 * Deserialises the curve's on-chain account from raw bytes into a typed
 * `BondingCurveState` struct. Works with both v0 and legacy transactions.
 */
import { Connection, PublicKey, AccountInfo } from "@solana/web3.js";
import BN from "bn.js";
import { BondingCurveState, findCurvePda } from "@x402pt/shared";

// ---------------------------------------------------------------------------
// On-chain account layout (must match programs/src/state.rs)
// ---------------------------------------------------------------------------
//
// Byte offsets:
//   0   - u8   discriminant       (0 = Uninitialized, 1 = Initialized)
//   1   - bool graduated          (1 byte)
//   2   - Pubkey authority        (32 bytes)
//   34  - Pubkey creatorFeeWallet (32 bytes)
//   66  - Pubkey mint             (32 bytes)
//   98  - u64 realSolReserves     (8 bytes, LE)
//   106 - u64 realTokenReserves   (8 bytes, LE)
//   114 - u64 virtualSolReserves  (8 bytes, LE)
//   122 - u64 virtualTokenReserves(8 bytes, LE)
//   130 - u64 totalSupply         (8 bytes, LE)
//   138 - u16 creatorFeeBps       (2 bytes, LE)
//   140 - u16 protocolFeeBps      (2 bytes, LE)
//   142 - [u8; 10] padding        (10 bytes)
// Total: 152 bytes

const CURVE_STATE_LEN = 152;

/** Parse a raw buffer into a BondingCurveState. */
export function decodeCurveState(data: Buffer): BondingCurveState | null {
  if (data.length < CURVE_STATE_LEN) return null;
  if (data.readUInt8(0) !== 1) return null; // not Initialized

  return {
    graduated: data.readUInt8(1) === 1,
    authority: new PublicKey(data.subarray(2, 34)),
    creatorFeeWallet: new PublicKey(data.subarray(34, 66)),
    mint: new PublicKey(data.subarray(66, 98)),
    realSolReserves: new BN(data.subarray(98, 106), "le"),
    realTokenReserves: new BN(data.subarray(106, 114), "le"),
    virtualSolReserves: new BN(data.subarray(114, 122), "le"),
    virtualTokenReserves: new BN(data.subarray(122, 130), "le"),
    totalSupply: new BN(data.subarray(130, 138), "le"),
    creatorFeeBps: data.readUInt16LE(138),
    protocolFeeBps: data.readUInt16LE(140),
  };
}

/** Fetch and decode the curve state for a given mint. */
export async function fetchCurveState(
  connection: Connection,
  mint: PublicKey,
): Promise<BondingCurveState | null> {
  const [curvePda] = findCurvePda(mint);
  const accountInfo: AccountInfo<Buffer> | null =
    await connection.getAccountInfo(curvePda);

  if (!accountInfo) return null;
  return decodeCurveState(accountInfo.data);
}

/**
 * Create a Solana connection, using Helius if the env var is set,
 * otherwise falling back to the cluster default.
 */
export function makeConnection(): Connection {
  const heliusApiKey = process.env.HELIUS_API_KEY;
  const network = process.env.SOLANA_NETWORK ?? "mainnet";
  const heliusCluster =
    network === "solana-devnet" || network === "devnet" ? "devnet" : "mainnet";
  const endpoint =
    process.env.HELIUS_RPC_URL ??
    process.env.SOLANA_RPC_URL ??
    (heliusApiKey
      ? `https://${heliusCluster}.helius-rpc.com/?api-key=${heliusApiKey}`
      : undefined) ??
    "https://api.mainnet-beta.solana.com";
  return new Connection(endpoint, "confirmed");
}
