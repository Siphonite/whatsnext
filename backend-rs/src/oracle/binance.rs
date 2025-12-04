use anyhow::{anyhow, Result};
use serde_json::Value;
use reqwest::Client;
use std::time::Duration as StdDuration;

use crate::oracle::types::CandleData;
use crate::constants::BINANCE_SYMBOL;

/// Fetch a single Binance candle for BTC/USDT
/// hours = 1h, 2h, 4h, 6h, 12h, 24h
pub async fn fetch_binance_candle(hours: i64) -> Result<CandleData> {
    // Map hours → Binance interval
    let interval = match hours {
        1 => "1h",
        2 => "2h",
        4 => "4h",
        6 => "6h",
        12 => "12h",
        24 => "1d",
        _ => return Err(anyhow!("Unsupported candle interval: {}h", hours)),
    };

    // We now ALWAYS use "BTCUSDT"
    let url = format!(
        "https://api.binance.com/api/v3/klines?symbol={}&interval={}&limit=1",
        BINANCE_SYMBOL,
        interval
    );

    let client = Client::builder()
        .timeout(StdDuration::from_secs(10))
        .build()?;

    let resp = client.get(&url).send().await?;
    let json: Value = resp.json().await?;

    let arr = json.as_array()
        .and_then(|v| v.get(0))
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow!("Invalid Binance candle format"))?;

    let open  = arr[1].as_str().unwrap().parse::<f64>()?;
    let high  = arr[2].as_str().unwrap().parse::<f64>()?;
    let low   = arr[3].as_str().unwrap().parse::<f64>()?;
    let close = arr[4].as_str().unwrap().parse::<f64>()?;

    // Convert ms → seconds
    let timestamp = arr[0].as_i64().unwrap() / 1000;

    Ok(CandleData {
        open,
        high,
        low,
        close,
        timestamp,
    })
}
