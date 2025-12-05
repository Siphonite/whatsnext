import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { useEffect, useState } from "react";

export const useBalance = () => {
  const { connection } = useConnection();
  const { publicKey } = useWallet();

  const [balance, setBalance] = useState<number | null>(null);
  const [loading, setLoading] = useState(false);

  const fetchBalance = async () => {
    if (!publicKey) {
      setBalance(null);
      return;
    }

    try {
      setLoading(true);
      const lamports = await connection.getBalance(publicKey);
      setBalance(lamports / 1_000_000_000); // Convert lamports → SOL
    } catch (err) {
      console.error("Failed to fetch wallet balance:", err);
      setBalance(null);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchBalance(); // initial load

    // Refresh balance automatically
    const interval = setInterval(fetchBalance, 10_000);

    return () => clearInterval(interval);
  }, [publicKey, connection]);

  return { balance, loading, refresh: fetchBalance };
};
