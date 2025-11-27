use axum::{Router, routing::get, extract::Path, Json};
use serde_json::json;

use crate::oracle::get_latest_candle;

pub fn routes() -> Router {
    Router::new().route("/:symbol", get(oracle_handler))
}

async fn oracle_handler(Path(symbol): Path<String>) -> Json<serde_json::Value> {
    match get_latest_candle(&symbol, 4).await {
        Ok(candle) => Json(json!({
            "symbol": symbol,
            "open": candle.open,
            "high": candle.high,
            "low": candle.low,
            "close": candle.close,
            "timestamp": candle.timestamp
        })),
        Err(e) => Json(json!({
            "error": e.to_string()
        })),
    }
}
