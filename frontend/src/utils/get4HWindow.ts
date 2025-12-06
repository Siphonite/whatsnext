/**
 * Computes the TradingView 4H candle window for a given timestamp.
 * 
 * TradingView 4H candles align to UTC hour boundaries:
 * 00:00 → 04:00
 * 04:00 → 08:00
 * 08:00 → 12:00
 * 12:00 → 16:00
 * 16:00 → 20:00
 * 20:00 → 00:00
 * 
 * @param timestamp - Unix timestamp in milliseconds
 * @returns Object with start and end timestamps (in milliseconds)
 */
export function get4HWindow(timestamp: number): { start: number; end: number } {
  const d = new Date(timestamp);
  const hourBlock = Math.floor(d.getUTCHours() / 4) * 4;
  const start = Date.UTC(
    d.getUTCFullYear(),
    d.getUTCMonth(),
    d.getUTCDate(),
    hourBlock,
    0,
    0
  );
  const end = start + 4 * 60 * 60 * 1000; // 4 hours in milliseconds
  return { start, end };
}

