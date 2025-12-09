import { PublicKey } from "@solana/web3.js";

/**
 * Derive the Market PDA from market_id (u64 little-endian)
 * Seeds on-chain: ["market", market_id.to_le_bytes()]
 */
export function deriveMarketPDA(programId: PublicKey, marketId: number) {
  const idBuf = Buffer.alloc(8);
  idBuf.writeBigUInt64LE(BigInt(marketId), 0);
  return PublicKey.findProgramAddressSync(
    [Buffer.from("market"), idBuf],
    programId
  );
}

/**
 * Derive User Bet PDA
 * Seeds on-chain: ["bet", user_pubkey, market_pda]
 */
export function deriveUserBetPDA(programId: PublicKey, userPubkey: PublicKey, marketPda: PublicKey) {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("bet"),
      userPubkey.toBuffer(),
      marketPda.toBuffer(),
    ],
    programId
  );
}
