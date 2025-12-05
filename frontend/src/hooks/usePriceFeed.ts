import { useEffect } from "react";
import { useMarketStore } from "../store/useMarketStore";

export const usePriceFeed = () => {
  const { setPrice, tvSymbol } = useMarketStore();

  useEffect(() => {
    const fetchPrice = async () => {
      try {
        // BTC/USDT → BTCUSDT (already mapped inside store as tvSymbol)
        const res = await fetch(
          `https://api.binance.com/api/v3/ticker/price?symbol=${tvSymbol}`
        );
        const data = await res.json();

        if (data?.price) {
          setPrice(parseFloat(data.price)); // store live price
        }
      } catch (err) {
        console.error("Failed to fetch BTC price:", err);
      }
    };

    // initial load
    fetchPrice();

    // Poll every 3 seconds (smooth without rate limits)
    const interval = setInterval(fetchPrice, 3000);

    return () => clearInterval(interval);
  }, [setPrice, tvSymbol]);
};
