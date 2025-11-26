mod config;
mod scheduler;
mod solana_client;
mod oracle;
mod routes;

use std::sync::Arc;

use axum::Router;
use config::AppConfig;
use solana_client::SolanaClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    tracing::info!("Starting backend...");

    // Load config + solana client
    let cfg = AppConfig::load();
    let sol = Arc::new(SolanaClient::new(&cfg)?);

    // Start scheduler in background
    let sol_clone = sol.clone();
    tokio::spawn(async move {
        if let Err(e) = scheduler::start_scheduler(sol_clone).await {
            tracing::error!("Scheduler failed: {:?}", e);
        }
    });

    // Build API router
    let app = routes::create_router(sol.clone());

    // Start Axum 0.7 server using TcpListener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;

    tracing::info!("HTTP server running on http://127.0.0.1:8080");

    axum::serve(listener, app).await?;

    Ok(())
}
