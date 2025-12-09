use std::sync::Arc;
use sqlx::{Pool, Postgres};

use backend_rs::config::AppConfig;
use backend_rs::scheduler;
use backend_rs::solana_client::SolanaClient;
use backend_rs::routes;
use backend_rs::db;
use backend_rs::state::AppState;

// Axum
use axum::serve;

// ADD THIS:
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    // ADD CORS LAYER HERE
    let cors = CorsLayer::new()
        .allow_origin(Any)       // allow all origins (frontend :5173)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router with CORS applied
    let app = routes::create_router(state).layer(cors);

    // Axum 0.7 server startup
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("HTTP server running on http://127.0.0.1:8080");

    serve(listener, app).await?;

    Ok(())
}
