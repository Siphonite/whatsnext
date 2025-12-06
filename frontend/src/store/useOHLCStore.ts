import { create } from "zustand";
import type { UTCTimestamp } from "lightweight-charts";

export interface OHLCBar {
  time: UTCTimestamp;
  open: number;
  high: number;
  low: number;
  close: number;
}

interface OHLCState {
  // Historical candles (all completed candles)
  historicalCandles: OHLCBar[];
  
  // Current live candle (being formed)
  currentCandle: OHLCBar | null;
  
  // Asset this data is for
  asset: string;
  
  // Last update timestamp
  lastUpdated: number;
  
  // Actions
  setCandles: (candles: OHLCBar[], asset: string) => void;
  updateCurrentCandle: (candle: OHLCBar) => void;
  reset: () => void;
  getAllCandles: () => OHLCBar[];
}

export const useOHLCStore = create<OHLCState>((set, get) => ({
  historicalCandles: [],
  currentCandle: null,
  asset: "",
  lastUpdated: 0,

  setCandles: (candles, asset) => {
    // Separate historical from current candle
    // Current candle is the last one if it's still in the current 4H window
    const now = Date.now() / 1000;
    const fourHours = 4 * 60 * 60;
    
    const historical: OHLCBar[] = [];
    let current: OHLCBar | null = null;
    
    for (let i = 0; i < candles.length; i++) {
      const candle = candles[i];
      const candleEnd = candle.time + fourHours;
      
      // If candle hasn't ended yet, it's the current one
      if (candleEnd > now) {
        current = candle;
        // All previous candles are historical
        historical.push(...candles.slice(0, i));
        break;
      }
    }
    
    // If no current candle found, all are historical
    if (!current && candles.length > 0) {
      historical.push(...candles);
    }
    
    set({
      historicalCandles: historical,
      currentCandle: current,
      asset,
      lastUpdated: Date.now(),
    });
  },

  updateCurrentCandle: (candle) => {
    set({
      currentCandle: candle,
      lastUpdated: Date.now(),
    });
  },

  reset: () => {
    set({
      historicalCandles: [],
      currentCandle: null,
      asset: "",
      lastUpdated: 0,
    });
  },

  getAllCandles: () => {
    const state = get();
    if (state.currentCandle) {
      return [...state.historicalCandles, state.currentCandle];
    }
    return state.historicalCandles;
  },
}));
