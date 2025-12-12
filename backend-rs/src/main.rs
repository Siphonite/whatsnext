use std::sync::Arc;
use sqlx::{Pool, Postgres};

use backend_rs::config::AppConfig;
use backend_rs::scheduler;
use backend_rs::solana_client::SolanaClient;
use backend_rs::routes;
use backend_rs::db;
use backend_rs::state::AppState;

// Axum + CORS
use axum::serve;
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    tracing::info!("Starting backend...");

    // -------------------------------
    // DATABASE
    // -------------------------------
    let pool: Pool<Postgres> = db::create_db_pool().await;
    tracing::info!("Connected to PostgreSQL");

    // -------------------------------
    // CONFIG + SOLANA CLIENT
    // -------------------------------
    let cfg = AppConfig::load();
    let sol = Arc::new(SolanaClient::new(&cfg)?);

    // -------------------------------
    // TREASURY INITIALIZATION
    // -------------------------------
    tracing::info!("Checking treasury PDA...");
    sol.initialize_treasury_if_needed()?;        
    tracing::info!("Treasury ready.");

    // -------------------------------
    // APPLICATION STATE
    // -------------------------------
    let state = Arc::new(AppState {
        sol: sol.clone(),
        pool: pool.clone(),
    });

    // -------------------------------
    // START SCHEDULER
    // -------------------------------
    tokio::spawn({
        let sol = sol.clone();
        let pool = pool.clone();
        async move {
            tracing::info!("Starting scheduler...");
            if let Err(e) = scheduler::start_scheduler(sol, pool).await {
                tracing::error!("Scheduler failed: {:?}", e);
            }
        }
    });

    // -------------------------------
    // BUILD ROUTER + CORS
    // -------------------------------
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    
    let app = routes::routes().with_state(state).layer(cors);

    // -------------------------------
    // START HTTP SERVER (AXUM 0.7)
    // -------------------------------
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("HTTP server running at http://127.0.0.1:8080");

    serve(listener, app).await?;

    Ok(())
}
