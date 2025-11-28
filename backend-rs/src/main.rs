use std::sync::Arc;
use sqlx::{Pool, Postgres};

use backend_rs::config::AppConfig;
use backend_rs::scheduler;
use backend_rs::solana_client::SolanaClient;
use backend_rs::routes;
use backend_rs::db;
use backend_rs::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    tracing::info!("Starting backend...");

    // Database
    let pool: Pool<Postgres> = db::create_db_pool().await;
    tracing::info!("Connected to PostgreSQL");

    // Solana client
    let cfg = AppConfig::load();
    let sol = Arc::new(SolanaClient::new(&cfg)?);

    // Shared application state
    let state = Arc::new(AppState {
        sol: sol.clone(),
        pool: pool.clone(),
    });

    // Scheduler (background)
    tokio::spawn({
        let sol = sol.clone();
        let pool = pool.clone();
        async move {
            if let Err(e) = scheduler::start_scheduler(sol, pool).await {
                tracing::error!("Scheduler failed: {:?}", e);
            }
        }
    });

    // App router
    let app = routes::create_router(state);

    // HTTP Server
    use hyper::server::Server;

    let addr = "0.0.0.0:8080".parse().unwrap();
    tracing::info!("HTTP server running on http://127.0.0.1:8080");

    Server::bind(&addr)
    .serve(app.into_service())
    .await?;


    Ok(())
}
