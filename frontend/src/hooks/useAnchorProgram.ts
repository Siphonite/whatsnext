import { useMemo } from "react";
import * as anchor from "@coral-xyz/anchor";
import type { Idl } from "@coral-xyz/anchor";
import { Connection } from "@solana/web3.js";
import { useWallet } from "@solana/wallet-adapter-react";
import idl from "../candle_markets.json";

export function useAnchorProgram() {
  const wallet = useWallet();

  // Connection
  const connection = useMemo(() => {
    const rpc =
      import.meta.env.VITE_SOLANA_RPC || "https://api.devnet.solana.com";
    return new Connection(rpc, "confirmed");
  }, []);

  // Provider
  const provider = useMemo(() => {
    if (
      !wallet.publicKey ||
      !wallet.signTransaction ||
      !wallet.signAllTransactions
    )
      return null;

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

  // Program instance
  const program = useMemo(() => {
    if (!provider) return null;

    try {
      // Anchor 0.32+ automatically reads programId from the IDL
      return new anchor.Program(idl as Idl, provider);
    } catch (err) {
      console.error("Failed to create Anchor Program:", err);
      return null;
    }
  }, [provider]);

  return {
    program,
    provider,
    connection,
    walletPubkey: wallet.publicKey ?? null,
    connected: Boolean(wallet.publicKey),
  };
}
