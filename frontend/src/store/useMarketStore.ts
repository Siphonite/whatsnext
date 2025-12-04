import { create } from "zustand";

type Market = {
  symbol: string;
};

type State = {
  markets: Market[];
  activeAsset: string;

  // NEW — Live price store
  prices: Record<string, number>;
  setPrice: (symbol: string, price: number) => void;

  setActiveAsset: (asset: string) => void;
};

export const useMarketStore = create<State>((set) => ({
  markets: [
    { symbol: "SOL/USDT" },
    { symbol: "BTC/USDT" },
    { symbol: "ETH/USDT" },

    { symbol: "EUR/USD" },
    { symbol: "GBP/USD" },
    { symbol: "USD/JPY" },

    { symbol: "GOLD" },
    { symbol: "SILVER" },
    { symbol: "OIL" },
  ],

  activeAsset: "SOL/USDT",

  // NEW — initially empty price map
  prices: {},

  // NEW — setter that updates just one asset price
  setPrice: (symbol, price) =>
    set((state) => ({
      prices: {
        ...state.prices,
        [symbol]: price,
      },
    })),

  setActiveAsset: (asset) => set({ activeAsset: asset }),
}));
