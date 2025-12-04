use axum::{
    Router,
    routing::get,
    extract::{Path, State},
    Json,
};
use serde_json::json;
use std::sync::Arc;

use crate::state::AppState;
use crate::repository::{get_market_from_db, get_active_markets, get_user_pnl};

/// Routes for market-related data
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/active", get(get_active_markets_handler))   // Returns active BTC markets
        .route("/:id", get(get_market_handler))              // Returns details for a specific BTC market
        .route("/pnl/:wallet", get(get_pnl_handler))         // Returns user PnL stats
}

/// GET /market/active
/// Returns a list of active BTC/USDT markets.
/// (Scheduler only creates BTC markets now, so this list is always BTC-only)
async fn get_active_markets_handler(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    match get_active_markets(&state.pool).await {
        Ok(markets) => Json(json!(markets)),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

/// GET /market/:id
/// Returns details for a specific market ID
async fn get_market_handler(
    Path(id): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    match get_market_from_db(&state.pool, id).await {
        Ok(market) => Json(json!(market)),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

/// GET /market/pnl/:wallet
/// Returns user-level aggregated PnL statistics
async fn get_pnl_handler(
    Path(wallet): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    match get_user_pnl(&state.pool, &wallet).await {
        Ok(stats) => Json(json!(stats)),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}
