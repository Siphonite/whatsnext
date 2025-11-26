pub mod types;
pub mod tradingview;
pub mod binance;

pub use types::CandleData;

pub async fn get_latest_candle(symbol: &str, hours: i64) -> anyhow::Result<CandleData> {
    // 1) try Binance (always available on your network)
    if let Ok(cndl) = crate::oracle::binance::fetch_binance_candle(symbol, hours).await {
        return Ok(cndl);
    }

    // 2) fallback to TradingView proxy
    let tv_symbol = format!("BINANCE:{}", symbol);
    Ok(crate::oracle::tradingview::fetch_tradingview_candle(&tv_symbol, hours).await?)
}
