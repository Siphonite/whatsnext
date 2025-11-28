use axum::{
    Router,
    routing::get,
    extract::{Path, State},
    Json,
};
use serde_json::json;
use std::sync::Arc;

use crate::state::AppState;
use crate::repository::{get_latest_market, get_market_from_db};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/current", get(get_current_market))
        .route("/:id", get(get_market))
}

async fn get_current_market(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    match get_latest_market(&state.pool).await {
        Ok(m) => Json(json!(m)),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn get_market(
    Path(id): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    match get_market_from_db(&state.pool, id).await {
        Ok(m) => Json(json!(m)),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}
