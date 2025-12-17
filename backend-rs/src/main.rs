use std::sync::Arc;
use std::env; // Added for reading env vars directly

use backend_rs::config::AppConfig;
use backend_rs::scheduler;
use backend_rs::solana_client::SolanaClient;
use backend_rs::db;
use backend_rs::state::AppState;

// Route modules
use backend_rs::routes::{market, pnl, oracle, health, claim, treasury, prices};

// Axum + CORS
use axum::{Router, serve};
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    tracing::info!("Starting backend...");

    // -------------------------------
    // DATABASE
    // -------------------------------
    let pool = db::create_db_pool().await;
    tracing::info!("Connected to PostgreSQL");

    // -------------------------------
    // CONFIG + SOLANA CLIENT
    // -------------------------------
    let cfg = AppConfig::load();
    let sol = Arc::new(SolanaClient::new(&cfg)?);

    tracing::info!("Treasury PDA initialization must be done via POST /treasury/init.");

    // -------------------------------
    // APPLICATION STATE
    // -------------------------------
    let state = Arc::new(AppState {
        sol: sol.clone(),
        pool: pool.clone(),
    });

    // -------------------------------
    // CREATE INITIAL MARKET ON STARTUP
    // -------------------------------
    // Ensure a market exists immediately after deployment
    if let Err(e) = scheduler::create_initial_market(sol.clone(), pool.clone()).await {
        tracing::warn!("Initial market creation failed (may already exist): {:?}", e);
    }

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

    let app = Router::new()
        .nest("/market", market::routes())
        .nest("/pnl", pnl::routes())
        .nest("/oracle", oracle::routes())
        .nest("/health", health::routes())
        .nest("/claim", claim::routes())
        .nest("/treasury", treasury::treasury_routes())
        .nest("/prices", prices::routes())
        .with_state(state)
        .layer(cors);

    // -------------------------------
    // START HTTP SERVER
    // -------------------------------
    // FIX: Prioritize Render's PORT env var, fallback to config, then default to 8080
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(cfg.backend_port);

    let addr = format!("0.0.0.0:{}", port);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("HTTP server running at http://{}", addr);

    serve(listener, app).await?;

    Ok(())
}