import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { useEffect, useState } from "react";

export const useBalance = () => {
  const { connection } = useConnection();
  const { publicKey } = useWallet();
  const [balance, setBalance] = useState<number | null>(null);

  useEffect(() => {
    const fetchBalance = async () => {
      if (!publicKey) return setBalance(null);

      const lamports = await connection.getBalance(publicKey);
      setBalance(lamports / 1_000_000_000); // Convert to SOL
    };

    fetchBalance();

    // Auto-refresh every 10 seconds
    const interval = setInterval(fetchBalance, 10000);

    return () => clearInterval(interval);
  }, [connection, publicKey]);

  return balance;
};
