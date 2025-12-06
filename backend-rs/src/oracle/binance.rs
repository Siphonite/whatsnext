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

/// Fetch historical candles from Binance for BTC/USDT
/// hours = 1h, 2h, 4h, 6h, 12h, 24h
/// limit = number of candles to fetch (max 1000)
pub async fn fetch_binance_historical(hours: i64, limit: usize) -> Result<Vec<CandleData>> {
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

    // Cap limit at 1000 (Binance max)
    let limit = limit.min(1000);

    let url = format!(
        "https://api.binance.com/api/v3/klines?symbol={}&interval={}&limit={}",
        BINANCE_SYMBOL,
        interval,
        limit
    );

    let client = Client::builder()
        .timeout(StdDuration::from_secs(10))
        .build()?;

    let resp = client.get(&url).send().await?;
    let json: Value = resp.json().await?;

    let arr = json.as_array()
        .ok_or_else(|| anyhow!("Invalid Binance response format"))?;

    let mut candles = Vec::new();
    for item in arr {
        let candle_arr = item.as_array()
            .ok_or_else(|| anyhow!("Invalid candle format"))?;

        let open  = candle_arr[1].as_str().unwrap().parse::<f64>()?;
        let high  = candle_arr[2].as_str().unwrap().parse::<f64>()?;
        let low   = candle_arr[3].as_str().unwrap().parse::<f64>()?;
        let close = candle_arr[4].as_str().unwrap().parse::<f64>()?;
        let timestamp = candle_arr[0].as_i64().unwrap() / 1000; // Convert ms → seconds

        candles.push(CandleData {
            open,
            high,
            low,
            close,
            timestamp,
        });
    }

    Ok(candles)
}