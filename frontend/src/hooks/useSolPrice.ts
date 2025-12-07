import { useEffect, useState } from "react";

export const useSolPrice = () => {
  const [solPrice, setSolPrice] = useState<number | null>(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const fetchSolPrice = async () => {
      try {
        setLoading(true);
        const res = await fetch(
          `https://api.binance.com/api/v3/ticker/price?symbol=SOLUSDT`
        );
        const data = await res.json();

        if (data?.price) {
          setSolPrice(parseFloat(data.price));
        }
      } catch (err) {
        console.error("Failed to fetch SOL price:", err);
      } finally {
        setLoading(false);
      }
    };

    // initial load
    fetchSolPrice();

    // Poll every 3 seconds (same as BTC price feed)
    const interval = setInterval(fetchSolPrice, 3000);

    return () => clearInterval(interval);
  }, []);

  return { solPrice, loading };
};

