use axum::{Router, routing::get};
use std::sync::Arc;
use crate::state::AppState;

pub mod health;
pub mod oracle;
pub mod market;

// CHANGE 1: The return type should just be `Router`.
// `Router` is an alias for `Router<()>`. 
// The router no longer "needs" generic state because you provide it at the end.
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(homepage))
        .nest("/health", health::routes())
        .nest("/oracle", oracle::routes())
        .nest("/market", market::routes())
        .with_state(state) 
}

async fn homepage() -> &'static str {
    "Welcome to What's Next? Backend.\n\nAvailable endpoints:\n  GET /health\n  GET /oracle/:symbol\n  GET /market/current\n  GET /market/:id\n"
}