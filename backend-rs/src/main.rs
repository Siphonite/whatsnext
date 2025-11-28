use std::sync::Arc;
use sqlx::{Pool, Postgres};

use backend_rs::config::AppConfig;
use backend_rs::scheduler;
use backend_rs::solana_client::SolanaClient;
use backend_rs::routes;
use backend_rs::db;
use backend_rs::state::AppState;

// CHANGE 2: Remove IncomingStream from the import
use axum::serve; 

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ... (Your existing setup code remains exactly the same up to here) ...
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    tracing::info!("Starting backend...");

    let pool: Pool<Postgres> = db::create_db_pool().await;
    tracing::info!("Connected to PostgreSQL");

    let cfg = AppConfig::load();
    let sol = Arc::new(SolanaClient::new(&cfg)?);

    let state = Arc::new(AppState {
        sol: sol.clone(),
        pool: pool.clone(),
    });

    tokio::spawn({
        let sol = sol.clone();
        let pool = pool.clone();
        async move {
            if let Err(e) = scheduler::start_scheduler(sol, pool).await {
                tracing::error!("Scheduler failed: {:?}", e);
            }
        }
    });

    let app = routes::create_router(state);

    // --- Axum 0.7 server startup ---
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("HTTP server running on http://127.0.0.1:8080");

    // CHANGE 3: Remove `IncomingStream::new`. 
    // Pass the `listener` directly to `serve`.
    serve(listener, app).await?;

    Ok(())
}