use axum::{Router, routing::get, Json};
use serde_json::json;
use std::sync::Arc;

use crate::state::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(health_check))
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().timestamp()
    }))
}
