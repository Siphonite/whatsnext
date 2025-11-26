pub mod health;
pub mod oracle;
// pub mod market;   // DISABLED FOR NOW 

use axum::Router;
use crate::solana_client::SolanaClient;
use std::sync::Arc;

pub fn create_router(_sol: Arc<SolanaClient>) -> Router {
    Router::new()
        .nest("/health", health::routes())
        .nest("/oracle", oracle::routes())

        // .nest("/market", market::routes(sol.clone()))
        // commented until we create market.rs next
}
