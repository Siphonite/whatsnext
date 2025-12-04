// -----------------------------------------------------------------------------
// oracle/mod.rs
// Simplified Oracle Module for SINGLE-ASSET MVP (BTC/USDT only)
// -----------------------------------------------------------------------------

pub mod types;
pub mod tradingview;  // kept as a fallback option
pub mod binance;

pub use types::CandleData;

use anyhow::{Result, anyhow};

/// Fetch latest BTC/USDT candle.
/// Primary source: Binance
/// Fallback: TradingView (Yahoo Finance)
pub async fn get_latest_candle(hours: i64) -> Result<CandleData> {
    // 1. Primary oracle: Binance BTCUSDT
    match crate::oracle::binance::fetch_binance_candle(hours).await {
        Ok(cndl) => return Ok(cndl),
        Err(e) => {
            tracing::error!("Binance oracle failed: {:?}", e);
        }
    }

    // 2. Fallback oracle: TradingView/Yahoo BTCUSD
    match crate::oracle::tradingview::fetch_tradingview_candle(hours).await {
        Ok(cndl) => Ok(cndl),
        Err(_) => Err(anyhow!(
            "All oracle sources failed for BTC/USDT ({}h).",
            hours
        )),
    }
}
