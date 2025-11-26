mod config;
mod scheduler;
mod solana_client;

use config::AppConfig;
use solana_client::SolanaClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    tracing::info!("Starting backend...");

    let cfg = AppConfig::load();
    let sol = SolanaClient::new(&cfg)?;

    tracing::info!("Program ID: {}", sol.program_id);

    // start cron scheduler (no Arc for now)
    scheduler::start_scheduler().await?;

    // TEST CALL
    let now = chrono::Utc::now().timestamp();
    let sig = sol.create_market_and_send(
        "SOL/USDT".to_string(),
        100_000,
        now,
        now + 3600,
        1,
    )?;

    tracing::info!("Test transaction: {}", sig);

    Ok(())
}
