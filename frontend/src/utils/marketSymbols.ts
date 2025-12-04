export const mapToTVSymbol = (symbol: string) => {
  switch (symbol) {
    case "GOLD":
      return "XAUUSD";     // Binance futures symbol
    case "SILVER":
      return "XAGUSD";     // Binance futures symbol
    case "OIL":
      return "OILUSDT";    // Synthetic oil token
    default:
      return symbol.replace("/", ""); // e.g. SOL/USDT → SOLUSDT
  }
};
