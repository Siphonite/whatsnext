use axum::{
    Router,
    routing::get,
    extract::{Path, State},
    Json,
};
use serde_json::json;
use std::sync::Arc;

use crate::state::AppState;
// Import the new functions
use crate::repository::{get_market_from_db, get_active_markets, get_user_pnl};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/active", get(get_active_markets_handler)) // Returns list of 9 markets
        .route("/:id", get(get_market_handler))            // Returns specific market details
        .route("/pnl/:wallet", get(get_pnl_handler))       // Returns user stats
}

// GET /market/active
async fn get_active_markets_handler(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    match get_active_markets(&state.pool).await {
        Ok(markets) => Json(json!(markets)),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

// GET /market/:id
async fn get_market_handler(
    Path(id): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    match get_market_from_db(&state.pool, id).await {
        Ok(m) => Json(json!(m)),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

// GET /market/pnl/:wallet
async fn get_pnl_handler(
    Path(wallet): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    match get_user_pnl(&state.pool, &wallet).await {
        Ok(stats) => Json(json!(stats)),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}