use crate::oracle::types::CandleData;
use anyhow::{anyhow, Result};
use chrono::Utc;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::OnceLock;
use tokio::sync::RwLock;
use std::time::Duration as StdDuration;

// Simple in-memory cache
static CACHE: OnceLock<RwLock<HashMap<String, (CandleData, i64)>>> = OnceLock::new();

fn cache() -> &'static RwLock<HashMap<String, (CandleData, i64)>> {
    CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

fn hours_to_resolution_minutes(hours: i64) -> i64 {
    hours * 60
}

/// ----------------------------------------------------------------------------
/// fetch_tradingview_candle()
/// Fallback oracle for BTC-only
/// ----------------------------------------------------------------------------
/// Notes:
///  - Only supports BTCUSD
///  - No multi-asset mapping
///  - Only called if Binance fails
///  - Uses Yahoo Finance chart API
/// ----------------------------------------------------------------------------
pub async fn fetch_tradingview_candle(candle_hours: i64) -> Result<CandleData> {
    let tv_symbol = "BTCUSD"; // Only fallback symbol we support now.
    let resolution = hours_to_resolution_minutes(candle_hours);

    let key = format!("{}:{}", tv_symbol, resolution);

    // -------- Cache Check (5 second TTL) --------
    {
        let map = cache().read().await;
        if let Some((cndl, ts)) = map.get(&key) {
            let now = Utc::now().timestamp();
            if now - *ts <= 5 {
                return Ok(cndl.clone());
            }
        }
    }

    // Yahoo URL: BTC-USD
    let url = format!(
        "https://query1.finance.yahoo.com/v8/finance/chart/{}?interval={}m&range=1d",
        "BTC-USD",
        resolution
    );

    let client = Client::builder()
        .timeout(StdDuration::from_secs(10))
        .build()?;

    let resp = client.get(&url).send().await?;
    if !resp.status().is_success() {
        return Err(anyhow!("Yahoo Finance returned HTTP {}", resp.status()));
    }

    let json: Value = resp.json().await?;

    // Yahoo format: chart.result[0]
    let result = json["chart"]["result"]
        .as_array()
        .and_then(|arr| arr.get(0))
        .ok_or_else(|| anyhow!("Missing chart.result[0] in Yahoo response"))?;

    let timestamps = result["timestamp"]
        .as_array()
        .ok_or_else(|| anyhow!("Missing timestamp"))?;

    let indicators = result["indicators"]["quote"]
        .as_array()
        .and_then(|arr| arr.get(0))
        .ok_or_else(|| anyhow!("Missing indicators.quote[0]"))?;

    let o = indicators["open"].as_array().ok_or_else(|| anyhow!("Missing open"))?;
    let h = indicators["high"].as_array().ok_or_else(|| anyhow!("Missing high"))?;
    let l = indicators["low"].as_array().ok_or_else(|| anyhow!("Missing low"))?;
    let c = indicators["close"].as_array().ok_or_else(|| anyhow!("Missing close"))?;

    if o.is_empty() {
        return Err(anyhow!("Yahoo price arrays are empty"));
    }

    let idx = o.len() - 1;

    let candle = CandleData {
        open: o[idx].as_f64().unwrap_or(0.0),
        high: h[idx].as_f64().unwrap_or(0.0),
        low:  l[idx].as_f64().unwrap_or(0.0),
        close: c[idx].as_f64().unwrap_or(0.0),
        timestamp: timestamps[idx].as_i64().unwrap_or(0),
    };

    // Cache insert
    {
        let mut map = cache().write().await;
        map.insert(key, (candle.clone(), Utc::now().timestamp()));
    }

    Ok(candle)
}
