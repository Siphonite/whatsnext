import { useMemo } from "react";
import { Program, AnchorProvider } from "@coral-xyz/anchor";
import type { Idl } from "@coral-xyz/anchor";
import { Connection } from "@solana/web3.js";
import { useWallet } from "@solana/wallet-adapter-react";
import idl from "../candle_markets.json";


// Not used in constructor for your Anchor version
export const PROGRAM_ID = "HDMbkC4Dzg4YhpuJnEdy4KEAQbbuAaYymiEcwLsLYHaH";

export function useAnchorProgram() {
  const { publicKey, signTransaction, signAllTransactions } = useWallet();

  const connection = useMemo(() => {
    const rpc = import.meta.env.VITE_SOLANA_RPC || "https://api.devnet.solana.com";
    return new Connection(rpc, "confirmed");
  }, []);

  const provider = useMemo(() => {
    if (!publicKey || !signTransaction || !signAllTransactions) return null;

    const anchorWallet = {
      publicKey,
      signTransaction,
      signAllTransactions,
    };

    return new AnchorProvider(connection, anchorWallet, {
      preflightCommitment: "confirmed",
      commitment: "confirmed",
    });
  }, [publicKey, signTransaction, signAllTransactions, connection]);

  const program = useMemo(() => {
    if (!provider) return null;

    try {
      // Your Anchor version only supports: Program(idl, provider)
      return new Program(idl as Idl, provider);
    } catch (e) {
      console.error("Failed to initialize Anchor Program:", e);
      return null;
    }
  }, [provider]);

  return {
    connection,
    provider,
    program,
    walletPubkey: publicKey ?? null,
    connected: Boolean(publicKey),
  };
}
