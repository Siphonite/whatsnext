use axum::{
    Router,
    routing::get,
    extract::{Path, State},
    Json,
};
use serde_json::json;
use std::sync::Arc;

use crate::state::AppState;
use crate::repository::get_user_positions;

/// Routes for positions-related data
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/:wallet", get(get_positions_handler))
}

/// GET /positions/:wallet
/// Returns open and settled positions for a wallet
async fn get_positions_handler(
    Path(wallet): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    match get_user_positions(&state.pool, &wallet).await {
        Ok((open, settled)) => {
            Json(json!({
                "open": open.iter().map(|p| json!({
                    "marketId": p.market_id,
                    "side": p.side,
                    "amount": p.amount,
                    "weight": p.weight,
                    "effectiveStake": p.effective_stake,
                    "timestamp": p.timestamp,
                    "status": "OPEN"
                })).collect::<Vec<_>>(),
                "settled": settled.iter().map(|p| json!({
                    "marketId": p.market_id,
                    "side": p.side,
                    "amount": p.amount,
                    "weight": p.weight,
                    "effectiveStake": p.effective_stake,
                    "payout": p.payout,
                    "timestamp": p.timestamp,
                    "status": "SETTLED"
                })).collect::<Vec<_>>()
            }))
        }
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

