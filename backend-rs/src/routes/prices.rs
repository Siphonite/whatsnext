use axum::{Router, routing::get, Json};
use serde_json::json;
use std::sync::Arc;

use crate::state::AppState;
use crate::oracle::binance::fetch_binance_candle;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/BTCUSDT", get(get_btc_price))
}

/// Unified price endpoint for frontend TopBar
async fn get_btc_price() -> Json<serde_json::Value> {
    // Use the same 4-hour interval your backend uses everywhere
    match fetch_binance_candle(4).await {
        Ok(candle) => Json(json!({
            "asset": "BTCUSDT",
            "price": candle.close,
            "timestamp": candle.timestamp
        })),
        Err(e) => Json(json!({
            "error": e.to_string()
        })),
    }
}
