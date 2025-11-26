use crate::oracle::types::CandleData;
use anyhow::{anyhow, Result};
use chrono::Utc;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::OnceLock;
use tokio::sync::RwLock;
use std::time::Duration as StdDuration;

static CACHE: OnceLock<RwLock<HashMap<String, (CandleData, i64)>>> = OnceLock::new();

fn cache() -> &'static RwLock<HashMap<String, (CandleData, i64)>> {
    CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

fn hours_to_resolution_minutes(hours: i64) -> i64 {
    hours * 60
}

fn build_tradingview_symbol(symbol: &str) -> String {
    if symbol.contains(':') {
        symbol.to_string()
    } else {
        format!("BINANCE:{}", symbol)
    }
}

pub async fn fetch_tradingview_candle(symbol: &str, candle_hours: i64) -> Result<CandleData> {
    let tv_symbol = build_tradingview_symbol(symbol);
    let resolution = hours_to_resolution_minutes(candle_hours);

    let key = format!("{}:{}", tv_symbol, resolution);

    {
        let map = cache().read().await;
        if let Some((cndl, ts)) = map.get(&key) {
            let now = Utc::now().timestamp();
            if now - *ts <= 5 {
                return Ok(cndl.clone());
            }
        }
    }

    let url = format!(
        "https://query1.finance.yahoo.com/v8/finance/chart/{}?interval={}m&range=1d",
        tv_symbol.replace("BINANCE:", ""),
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

    // Yahoo structure: chart.result[0]
    let result = json["chart"]["result"]
        .as_array()
        .and_then(|arr| arr.get(0))
        .ok_or_else(|| anyhow!("Missing chart.result[0] in Yahoo response"))?;

    let t = result["timestamp"]
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
        return Err(anyhow!("Yahoo arrays are empty"));
    }

    let idx = o.len() - 1;

    let candle = CandleData {
        open: o[idx].as_f64().unwrap_or(0.0),
        high: h[idx].as_f64().unwrap_or(0.0),
        low:  l[idx].as_f64().unwrap_or(0.0),
        close: c[idx].as_f64().unwrap_or(0.0),
        timestamp: t[idx].as_i64().unwrap_or(0),
    };

    {
        let mut map = cache().write().await;
        map.insert(key, (candle.clone(), Utc::now().timestamp()));
    }

    Ok(candle)
}
