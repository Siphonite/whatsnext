import { useMemo } from "react";
import { Program, AnchorProvider } from "@coral-xyz/anchor";
import type { Idl } from "@coral-xyz/anchor";
import { Connection } from "@solana/web3.js";
import { useWallet } from "@solana/wallet-adapter-react";
import idl from "../../public/candle_markets.json"; 

// Your program ID from declare_id!(...) in lib.rs
export const PROGRAM_ID = "Bo4HrqyUDZtFwBwqrLZ4GnFqnvg2wzoKRb1hXgf41Aco";

export function useAnchorProgram() {
  const { publicKey, signTransaction, signAllTransactions } = useWallet();

  // -------------------------------------------------------
  // Connection
  // -------------------------------------------------------
  const connection = useMemo(() => {
    const rpc = import.meta.env.VITE_SOLANA_RPC || "https://api.devnet.solana.com";
    return new Connection(rpc, "confirmed");
  }, []);

  // -------------------------------------------------------
  // Provider (wallet + connection)
  // -------------------------------------------------------
  const provider = useMemo(() => {
    if (!publicKey || !signTransaction || !signAllTransactions) return null;

    const anchorWallet = {
      publicKey,
      signTransaction: signTransaction!,
      signAllTransactions: signAllTransactions!,
    };

    return new AnchorProvider(connection, anchorWallet, {
      preflightCommitment: "confirmed",
      commitment: "confirmed",
    });
  }, [publicKey, signTransaction, signAllTransactions, connection]);

  // -------------------------------------------------------
  // Program (idl + provider)
  // NOTE: new Program(idl, provider) is correct for @coral-xyz/anchor
  // -------------------------------------------------------
  const program = useMemo(() => {
    if (!provider) return null;

    try {
      return new Program(idl as Idl, provider);
    } catch (e) {
      console.error("Failed to initialize Anchor Program:", e);
      return null;
    }
  }, [provider]);

  // -------------------------------------------------------
  // Hook return values
  // -------------------------------------------------------
  return {
    connection,
    provider,
    program,
    walletPubkey: publicKey ?? null,
    connected: Boolean(publicKey),
  };
}
