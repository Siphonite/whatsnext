use axum::{
    Router,
    routing::get,
    extract::{Path, State},
    Json,
};
use serde_json::json;
use std::sync::Arc;

use crate::state::AppState;
use crate::repository::get_enhanced_pnl;

/// Routes for PnL-related data
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/:wallet", get(get_pnl_handler))
}

/// GET /pnl/:wallet
/// Returns enhanced PnL statistics (total PnL, win rate, streak)
async fn get_pnl_handler(
    Path(wallet): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    match get_enhanced_pnl(&state.pool, &wallet).await {
        Ok(stats) => Json(json!({
            "totalPnl": stats.total_pnl,
            "winRate": stats.win_rate,
            "streak": stats.streak
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

