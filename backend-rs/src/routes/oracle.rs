use axum::{Router, routing::get, extract::Path, Json};
use serde_json::json;
use std::sync::Arc;

use crate::state::AppState;
use crate::oracle::get_latest_candle;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/:symbol", get(oracle_handler))
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
