// ---------------------------------------------------------
// constants.rs
// ---------------------------------------------------------
// This file now defines constants for the single-asset MVP.
// The entire backend operates only on BTC/USDT markets.
//
// All previous multi-asset arrays are removed to simplify
// the backend and match our single-market architecture.
// ---------------------------------------------------------

/// The single supported asset in the MVP.
pub const MARKET_ASSET: &str = "BTC/USDT";

/// The Binance-compatible symbol for BTC/USDT.
/// Example: "BTCUSDT" for klines, price feeds, etc.
pub const BINANCE_SYMBOL: &str = "BTCUSDT";
