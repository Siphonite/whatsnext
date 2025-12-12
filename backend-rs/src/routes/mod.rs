use axum::{Router, routing::get};
use std::sync::Arc;
use crate::state::AppState;

// Declare ALL route modules here
pub mod health;
pub mod oracle;
pub mod market;
pub mod positions;
pub mod pnl;
pub mod claim;
pub mod treasury;
pub mod prices;

// Build router (but we no longer use this — main.rs merges manually)
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(homepage))
        .nest("/health", health::routes())
        .nest("/oracle", oracle::routes())
        .nest("/market", market::routes())
        .nest("/positions", positions::routes())
        .nest("/pnl", pnl::routes())
        .nest("/claim", claim::routes())
        .nest("/treasury", treasury::treasury_routes())
        .nest("/prices", prices::routes())
        .with_state(state)
}

async fn homepage() -> &'static str {
    "Welcome to What's Next? Backend"
}
