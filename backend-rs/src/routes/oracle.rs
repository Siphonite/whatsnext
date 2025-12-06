use axum::{Router, routing::get, extract::{Path, Query}, Json};
use serde_json::json;
use std::sync::Arc;
use serde::Deserialize;

use crate::state::AppState;
use crate::oracle::{get_latest_candle, binance::fetch_binance_historical};

#[derive(Deserialize)]
struct HistoricalParams {
    limit: Option<usize>,
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/:symbol", get(oracle_handler))
        .route("/:symbol/historical", get(historical_handler))
}

async fn oracle_handler(Path(symbol): Path<String>) -> Json<serde_json::Value> {
    match get_latest_candle(4).await {
        Ok(candle) => Json(json!({
            "requested_symbol": symbol,   // what client requested
            "actual_symbol": "BTC/USDT",  // we only support BTC
            "open": candle.open,
            "high": candle.high,
            "low":  candle.low,
            "close": candle.close,
            "timestamp": candle.timestamp
        })),
        Err(e) => Json(json!({
            "error": e.to_string()
        })),
    }
}

/// GET /oracle/:symbol/historical?limit=200
/// Returns historical OHLC candles for the asset
async fn historical_handler(
    Path(symbol): Path<String>,
    Query(params): Query<HistoricalParams>,
) -> Json<serde_json::Value> {
    let limit = params.limit.unwrap_or(200).min(1000);
    
    match fetch_binance_historical(4, limit).await {
        Ok(candles) => {
            let formatted: Vec<serde_json::Value> = candles.iter().map(|c| {
                json!({
                    "timestamp": c.timestamp,
                    "open": c.open,
                    "high": c.high,
                    "low": c.low,
                    "close": c.close,
                })
            }).collect();
            
            Json(json!({
                "requested_symbol": symbol,
                "actual_symbol": "BTC/USDT",
                "interval": "4h",
                "candles": formatted
            }))
        },
        Err(e) => Json(json!({
            "error": e.to_string()
        })),
    }
}
