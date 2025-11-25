mod config;
mod solana_client;

use config::AppConfig;
use solana_client::SolanaClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting backend test...");

    let cfg = AppConfig::load();
    let sol = SolanaClient::new(&cfg)?;

    println!("Program ID: {}", sol.program_id);

    let now = chrono::Utc::now().timestamp();
    let sig = sol.create_market_and_send(
        "SOL/USDT".to_string(),
        100_000,
        now,
        now + 3600,
        1,
    )?;

    println!("Transaction: {}", sig);

    Ok(())
}
