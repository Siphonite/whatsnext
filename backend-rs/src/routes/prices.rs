use axum::{Router, routing::get, Json};
use serde_json::json;
use std::sync::Arc;

use crate::state::AppState;
use crate::oracle::binance::fetch_binance_price;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/BTCUSDT", get(get_btc_price))
}

/// Unified price endpoint for frontend TopBar
async fn get_btc_price() -> Json<serde_json::Value> {
    match fetch_binance_price().await {
        Ok(price) => Json(json!({
            "asset": "BTCUSDT",
            "price": price
        })),
        Err(e) => Json(json!({
            "error": e.to_string()
        })),
    }
}
