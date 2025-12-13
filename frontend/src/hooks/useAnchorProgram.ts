import { useMemo } from "react";
import * as anchor from "@coral-xyz/anchor";
import type { Idl } from "@coral-xyz/anchor";
import { Connection, PublicKey } from "@solana/web3.js";
import { useWallet } from "@solana/wallet-adapter-react";
import idl from "../candle_markets.json";

/**
 * Hook that initializes the Anchor Program + Provider
 * and exposes helper PDA derivations for:
 *  - market PDA
 *  - user_bet PDA
 *  - treasury PDA
 */
export function useAnchorProgram() {
  const wallet = useWallet();

  // ------------------------------
  // 1. Solana RPC Connection
  // ------------------------------
  const connection = useMemo(() => {
    const rpc = import.meta.env.VITE_SOLANA_RPC || "https://api.devnet.solana.com";
    return new Connection(rpc, "confirmed");
  }, []);

  // ------------------------------
  // 2. Anchor Provider
  // ------------------------------
  const provider = useMemo(() => {
    if (
      !wallet.publicKey ||
      !wallet.signTransaction ||
      !wallet.signAllTransactions
    ) {
      return null;
    }

    const anchorWallet = {
      publicKey: wallet.publicKey,
      signTransaction: wallet.signTransaction,
      signAllTransactions: wallet.signAllTransactions,
    };

    return new anchor.AnchorProvider(connection, anchorWallet, {
      preflightCommitment: "confirmed",
      commitment: "confirmed",
    });
  }, [
    wallet.publicKey,
    wallet.signTransaction,
    wallet.signAllTransactions,
    connection,
  ]);

  // ------------------------------
  // 3. Program Instance
  // ------------------------------
  const program = useMemo(() => {
    if (!provider) return null;

    try {
      return new anchor.Program(idl as Idl, provider);
    } catch (err) {
      console.error("Failed to create Anchor Program:", err);
      return null;
    }
  }, [provider]);

  const programId = program?.programId ?? null;

  // ------------------------------
  // 4. PDA Derivation Helpers
  // ------------------------------
  const deriveMarketPda = (marketId: number) => {
    if (!programId) return null;

    const seed = new anchor.BN(marketId).toArrayLike(Buffer, "le", 8);

    return PublicKey.findProgramAddressSync(
      [Buffer.from("market"), seed],
      programId
    )[0];
  };

  const deriveUserBetPda = (wallet: PublicKey, marketPda: PublicKey) => {
    if (!programId) return null;

    return PublicKey.findProgramAddressSync(
      [Buffer.from("bet"), wallet.toBytes(), marketPda.toBytes()],
      programId
    )[0];
  };

  const deriveTreasuryPda = () => {
    if (!programId) return null;

    return PublicKey.findProgramAddressSync(
      [Buffer.from("treasury")],
      programId
    )[0];
  };

  // ------------------------------
  // 5. Return everything needed
  // ------------------------------
  return {
    program,
    programId,
    provider,
    connection,
    walletPubkey: wallet.publicKey ?? null,
    connected: Boolean(wallet.publicKey),
    deriveMarketPda,
    deriveUserBetPda,
    deriveTreasuryPda,
  };
}
