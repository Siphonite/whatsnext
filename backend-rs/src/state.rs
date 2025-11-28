use std::sync::Arc;
use sqlx::{Pool, Postgres};
use crate::solana_client::SolanaClient;

#[derive(Clone)]
pub struct AppState {
    pub sol: Arc<SolanaClient>,
    pub pool: Pool<Postgres>,
}
