export interface Asset {
  pair: string;
  color: string;
  price?: string;
}

export const SUPPORTED_ASSETS: Asset[] = [
  { pair: "SOL/USDT", color: "#14F195", price: "62.22" },
  { pair: "BTC/USDT", color: "#67e8f9", price: "64,250" },
  { pair: "ETH/USDT", color: "#627eea", price: "3450" },

  { pair: "EUR/USD", color: "#67e8f9" },
  { pair: "GBP/USD", color: "#67e8f9", price: "0.50" },
  { pair: "USD/JPY", color: "#67e8f9" },

  { pair: "GOLD", color: "#f3ba2f" },
  { pair: "SILVER", color: "#c0c0c0" },
  { pair: "OIL", color: "#2a5ada" }
];
