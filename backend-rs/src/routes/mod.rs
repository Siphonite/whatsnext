use axum::{Router, routing::get};
use std::sync::Arc;
use crate::solana_client::SolanaClient;

pub mod health;
pub mod oracle;
pub mod market;

pub fn create_router(sol: Arc<SolanaClient>) -> Router {
    Router::new()
        // Add homepage route
        .route("/", get(homepage))

        // existing routes
        .nest("/health", health::routes())
        .nest("/oracle", oracle::routes())
        .nest("/market", market::routes(sol))
}

async fn homepage() -> &'static str {
    "Welcome to What's Next? Backend.\n\nAvailable endpoints:\n  GET /health\n  GET /oracle/:symbol\n  GET /market/current\n  GET /market/:id\n"
}
