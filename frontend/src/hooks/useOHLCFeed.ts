import { useEffect } from "react";
import { useOHLCStore } from "../store/useOHLCStore";
import { useMarketStore } from "../store/useMarketStore";
import { useMarketTimerStore } from "../store/useMarketTimerStore";
import type { OHLCBar } from "../store/useOHLCStore";
import type { UTCTimestamp } from "lightweight-charts";
import { apiClient } from "../api/client"; // Import from axios client

/**
 * Hook to fetch and sync OHLC candle data from backend.
 */
export const useOHLCFeed = () => {
  const { asset } = useMarketStore();
  const { timeLeft } = useMarketTimerStore();
  const { setCandles, updateCurrentCandle, reset, getAllCandles } = useOHLCStore();

  // Transform backend candle format to TradingView format
  const transformCandle = (c: any): OHLCBar => {
    return {
      time: (c.timestamp || c.time) as UTCTimestamp,
      open: c.open,
      high: c.high,
      low: c.low,
      close: c.close,
    };
  };

  // Fetch historical OHLC data from backend
  const fetchOHLCData = async () => {
    try {
      // FIX: Use apiClient (axios) instead of fetch
      // FIX: Remove '/api' prefix to match backend-rs/src/main.rs routes
      const response = await apiClient.get(`/oracle/${encodeURIComponent(asset)}/historical?limit=200`);
      
      const data = response.data; // Axios stores body in .data
      
      if (data.error) {
        throw new Error(data.error);
      }

      if (!data.candles || !Array.isArray(data.candles)) {
        throw new Error("Invalid OHLC response format");
      }

      // Transform and set candles
      const transformed = data.candles.map(transformCandle);
      setCandles(transformed, asset);
    } catch (error) {
      console.error("Failed to fetch OHLC data:", error);
    }
  };

  // Fetch current candle update
  const fetchCurrentCandle = async () => {
    try {
      // FIX: Use apiClient and remove '/api' prefix
      const response = await apiClient.get(`/oracle/${encodeURIComponent(asset)}`);
      
      const data = response.data;
      
      if (data.error || !data.timestamp) {
        return;
      }

      // Transform and update current candle
      const currentCandle = transformCandle(data);
      updateCurrentCandle(currentCandle);
    } catch (error) {
      // Silently fail for current candle updates
      console.debug("Current candle update failed:", error);
    }
  };

  // Initial load and on asset change
  useEffect(() => {
    reset(); // Clear old data when asset changes
    fetchOHLCData();
  }, [asset]);

  // Reload when timer hits 0 (candle closes)
  useEffect(() => {
    if (timeLeft <= 0) {
      fetchOHLCData();
    }
  }, [timeLeft]);

  // Poll for current candle updates every 10 seconds
  useEffect(() => {
    const interval = setInterval(() => {
      if (timeLeft > 0) {
        fetchCurrentCandle();
      }
    }, 10000); 

    return () => clearInterval(interval);
  }, [asset, timeLeft]);

  return {
    candles: getAllCandles(),
    refresh: fetchOHLCData,
  };
};