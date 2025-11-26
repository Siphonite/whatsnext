mod config;
mod scheduler;
mod solana_client;
mod oracle;

use std::sync::Arc;

use config::AppConfig;
use solana_client::SolanaClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    tracing::info!("Starting backend...");

    // -----------------------------------
    // ORACLE TEST (optional)
    // -----------------------------------
    match oracle::get_latest_candle("BTCUSDT", 4).await {
        Ok(cndl) => println!("Oracle Test Candle: {:?}", cndl),
        Err(e) => tracing::error!("Oracle test failed: {:?}", e),
    }

    // -----------------------------------
    // LOAD CONFIG + INIT SOL CLIENT
    // -----------------------------------
    let cfg = AppConfig::load();
    let sol = Arc::new(SolanaClient::new(&cfg)?);

    tracing::info!("Program ID: {}", sol.program_id);

    // -----------------------------------
    // START SCHEDULER WITH SOL CLIENT
    // -----------------------------------
    scheduler::start_scheduler(sol.clone()).await?;

    tracing::info!("Backend started. Scheduler running.");

    // Keep running forever
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}
