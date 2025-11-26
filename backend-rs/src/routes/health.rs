use axum::{Router, routing::get, Json};
use serde_json::json;

pub fn routes() -> Router {
    Router::new().route("/", get(health_check))
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().timestamp()
    }))
}
