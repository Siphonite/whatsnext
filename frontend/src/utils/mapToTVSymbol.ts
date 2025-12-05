// src/utils/mapToTVSymbol.ts
export default function mapToTVSymbol(asset: string): string {
  if (!asset) return "BTCUSDT";

  const normalized = asset.toUpperCase().trim();

  switch (normalized) {
    case "GOLD":
    case "XAU":
    case "XAU/USD":
      return "OANDA:XAUUSD"; // or "XAUUSD" / "FX_IDC:XAUUSD" — TradingView offers many exchanges
    case "SILVER":
    case "XAG":
    case "XAG/USD":
      return "OANDA:XAGUSD";
    case "OIL":
    case "BRENT":
      return "OANDA:UKOIL"; // pick one mapping
    // common crypto pairs
    case "BTC/USDT":
    case "BTCUSDT":
      return "BINANCE:BTCUSDT";
    case "SOL/USDT":
    case "SOLUSDT":
      return "BINANCE:SOLUSDT";
    case "ETH/USDT":
    case "ETHUSDT":
      return "BINANCE:ETHUSDT";
    case "EUR/USD":
    case "EURUSD":
      return "OANDA:EUR_USD";
    case "GBP/USD":
    case "GBPUSD":
      return "OANDA:GBP_USD";
    case "USD/JPY":
    case "USDJPY":
      return "OANDA:USD_JPY";
    default:
      // try to convert simple "AAA/BBB" -> "BINANCE:AAABBB"
      const cleaned = normalized.replace("/", "");
      return `BINANCE:${cleaned}`;
  }
}
