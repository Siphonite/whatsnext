import { create } from "zustand";

type MarketState = {
  // Fixed single asset
  asset: string;        // "BTC/USDT"
  tvSymbol: string;     // "BTCUSDT"

  // Live price for the single asset
  price: number | null;

  // Setter for updating price
  setPrice: (newPrice: number) => void;
};

export const useMarketStore = create<MarketState>((set) => ({
  // We now support ONLY one asset
  asset: "BTC/USDT",
  tvSymbol: "BTCUSDT",  // required by TradingView / Lightweight Charts

  // Live price (initially unknown)
  price: null,

  // Update the live price
  setPrice: (newPrice) =>
    set(() => ({
      price: newPrice,
    })),
}));
