// --- Single Asset Configuration ---
export const ACTIVE_ASSET = "BTC/USDT";
export const ACTIVE_TV_SYMBOL = "BTCUSDT";

// Mapping table for special cases
const SPECIAL_SYMBOLS: Record<string, string> = {
  "GOLD": "XAUUSD",
  "SILVER": "XAGUSD",
  "OIL": "OILUSDT",
  "BTC/USDT": "BTCUSDT",   // Explicitly added for single-asset mode
};

// Convert UI/Backend symbol → TradingView symbol
export const mapToTVSymbol = (symbol: string): string => {
  // If exists in special cases, return mapped
  if (SPECIAL_SYMBOLS[symbol]) {
    return SPECIAL_SYMBOLS[symbol];
  }

  // Default rule (e.g., SOL/USDT → SOLUSDT)
  return symbol.replace("/", "");
};

// Convenience helper (optional)
export const getActiveSymbol = () => ({
  asset: ACTIVE_ASSET,
  tvSymbol: ACTIVE_TV_SYMBOL,
});
