import { useEffect } from "react";
import { useMarketTimerStore } from "../store/useMarketTimerStore";
import { useMarketStore } from "../store/useMarketStore";
import { get4HWindow } from "../utils/get4HWindow";

/**
 * Hook to fetch current market data and sync with timer store.
 * Fetches from /market/active and updates the timer store.
 */
export const useMarketTimer = () => {
  const { setMarketTimes, tick } = useMarketTimerStore();
  const { asset } = useMarketStore();

  useEffect(() => {
    const fetchMarketData = async () => {
      try {
        // Fetch active markets from backend
        const response = await fetch("/api/market/active");
        
        if (!response.ok) {
          throw new Error(`Failed to fetch market data: ${response.statusText}`);
        }
        
        const markets = await response.json();
        
        // Find the current BTC market (or first active market)
        const currentMarket = Array.isArray(markets) 
          ? markets.find((m: any) => m.asset === asset || m.asset === "BTC/USDT") || markets[0]
          : markets;
        
        if (!currentMarket) {
          // Fallback to TradingView 4H window if no market found
          const now = Date.now();
          const window = get4HWindow(now);
          setMarketTimes({
            startTime: window.start,
            endTime: window.end,
            serverTime: now,
          });
          return;
        }
        
        // Parse backend timestamps (DateTime<Utc> from Rust serializes as ISO strings)
        const startTime = currentMarket.start_time 
          ? new Date(currentMarket.start_time).getTime()
          : undefined;
        const endTime = currentMarket.end_time
          ? new Date(currentMarket.end_time).getTime()
          : undefined;
        
        // Use current time as server time if not provided
        const serverTime = Date.now();
        
        setMarketTimes({
          startTime,
          endTime,
          serverTime,
        });
      } catch (error) {
        console.error("Failed to fetch market data, using TradingView 4H window:", error);
        
        // Fallback to TradingView 4H window
        const now = Date.now();
        const window = get4HWindow(now);
        setMarketTimes({
          startTime: window.start,
          endTime: window.end,
          serverTime: now,
        });
      }
    };

    // Initial fetch
    fetchMarketData();

    // Refresh every 30 seconds to stay in sync
    const interval = setInterval(fetchMarketData, 30000);

    return () => clearInterval(interval);
  }, [asset, setMarketTimes]);

  // Tick every second to update timeLeft
  useEffect(() => {
    const interval = setInterval(() => {
      tick();
    }, 1000);

    return () => clearInterval(interval);
  }, [tick]);
};

