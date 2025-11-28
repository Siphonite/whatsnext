use std::sync::Arc;
use axum::Router;
use sqlx::{Pool, Postgres};

use backend_rs::config::AppConfig;
use backend_rs::scheduler;
use backend_rs::solana_client::SolanaClient;
use backend_rs::routes;
use backend_rs::db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Load .env and initialize logger
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    tracing::info!("Starting backend...");

    // 2. Create the DB connection pool
    let pool: Pool<Postgres> = db::create_db_pool().await;
    tracing::info!("Connected to PostgreSQL");

    // 3. Load config and create Solana client
    let cfg = AppConfig::load();
    let sol = Arc::new(SolanaClient::new(&cfg)?);

    // 4. Start scheduler (with Solana client + DB pool)
    let sol_clone = sol.clone();
    let pool_clone = pool.clone();

    tokio::spawn(async move {
        if let Err(e) = scheduler::start_scheduler(sol_clone, pool_clone).await {
            tracing::error!("Scheduler failed: {:?}", e);
        }
    });

    // 5. Build API router
    let app: Router = routes::create_router(sol.clone());

    // 6. Start Axum server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("HTTP server running on http://127.0.0.1:8080");

    axum::serve(listener, app).await?;

    Ok(())
}
