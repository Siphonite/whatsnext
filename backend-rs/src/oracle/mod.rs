pub mod types;
pub mod tradingview;
pub mod binance;

pub use types::CandleData;

// We update this function to accept the generic 'asset' name (e.g., "GOLD")
// and figure out the correct way to fetch it.
pub async fn get_latest_candle(asset: &str, hours: i64) -> anyhow::Result<CandleData> {
    
    // 1. Determine if this is a Crypto asset
    // We check if it contains "USDT". If so, we prioritize Binance.
    if asset.contains("USDT") {
        // Binance expects "BTCUSDT", but our constant is "BTC/USDT".
        // We simply remove the "/" to make it compatible.
        let binance_symbol = asset.replace("/", "");
        
        // Try fetching from Binance first
        if let Ok(cndl) = crate::oracle::binance::fetch_binance_candle(&binance_symbol, hours).await {
            return Ok(cndl);
        }
        // If Binance fails, we don't return error yet; we fall through to Yahoo/TradingView
    }

    // 2. Map Internal Name -> Oracle Symbol (Yahoo Finance Tickers)
    // This 'match' is the translation layer we discussed.
    let oracle_symbol = match asset {
        // Crypto Fallbacks (Yahoo format)
        "SOL/USDT" => "SOL-USD",
        "BTC/USDT" => "BTC-USD",
        "ETH/USDT" => "ETH-USD",
        
        // Forex (Yahoo Tickers often end in =X)
        "EUR/USD" => "EURUSD=X",
        "GBP/USD" => "GBPUSD=X",
        "USD/JPY" => "JPY=X",
        
        // Commodities (Futures Tickers)
        "GOLD"   => "GC=F",   // Gold Futures
        "SILVER" => "SI=F",   // Silver Futures
        "OIL"    => "CL=F",   // Crude Oil Futures
        
        // Default: If we don't know it, just try using the name as-is
        _ => asset,
    };

    // 3. Fetch from TradingView (which wraps Yahoo Finance)
    // We pass the translated 'oracle_symbol' here.
    crate::oracle::tradingview::fetch_tradingview_candle(oracle_symbol, hours).await
}