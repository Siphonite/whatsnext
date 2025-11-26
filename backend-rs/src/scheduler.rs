use anyhow::{anyhow, Result};
use chrono::{Utc, Duration};
use tokio_cron_scheduler::{Job, JobScheduler};
use std::{
    fs,
    path::Path,
    sync::Arc,
};

use crate::oracle::get_latest_candle;
use crate::solana_client::SolanaClient;

// path to store current market_id
const MARKET_ID_PATH: &str = "/home/siphonite/whatsnext/backend-rs/src/data/market_id.txt";

// Load current market_id from file (default 0)
fn load_market_id() -> Result<u64> {
    if !Path::new(MARKET_ID_PATH).exists() {
        fs::create_dir_all("data")?;
        fs::write(MARKET_ID_PATH, "0")?;
    }

    let txt = fs::read_to_string(MARKET_ID_PATH)?;
    Ok(txt.trim().parse::<u64>().unwrap_or(0))
}

// Save updated market_id
fn save_market_id(id: u64) -> Result<()> {
    fs::write(MARKET_ID_PATH, id.to_string())?;
    Ok(())
}

// CREATE MARKET JOB
async fn create_market_job(sol: Arc<SolanaClient>) -> Result<()> {
    let asset = "BTCUSDT";

    // Fetch candle
    let candle = get_latest_candle(asset, 4).await?;
    let open_price = (candle.open * 100.0) as u64; // scale

    // Compute times
    let start_time = candle.timestamp;
    let end_time = start_time + (4 * 3600);

    // Determine next market_id
    let mut id = load_market_id()?;
    id += 1;

    tracing::info!(
        "Creating market {} for {} | open={} start={} end={}",
        id, asset, open_price, start_time, end_time
    );

    // Call Solana program
    let sig = sol.create_market_and_send(
        asset.to_string(),
        open_price,
        start_time,
        end_time,
        id,
    )?;

    tracing::info!("Market {} created. Tx: {}", id, sig);

    save_market_id(id)?;

    Ok(())
}

// SETTLE MARKET JOB
async fn settle_market_job(sol: Arc<SolanaClient>) -> Result<()> {
    let asset = "BTCUSDT";

    let id = load_market_id()?;
    if id == 0 {
        tracing::warn!("No market exists to settle yet.");
        return Ok(());
    }

    let candle = get_latest_candle(asset, 4).await?;
    let close_price = (candle.close * 100.0) as u64;

    tracing::info!(
        "Settling market {} | close={} timestamp={}",
        id, close_price, candle.timestamp
    );

    let sig = sol.settle_market_and_send(id, close_price)?;

    tracing::info!("Market {} settled. Tx: {}", id, sig);

    Ok(())
}

// START SCHEDULER
pub async fn start_scheduler(sol: Arc<SolanaClient>) -> Result<()> {
    let sched = JobScheduler::new().await?;

    // ─────────────────────────────
    // Create Market every 4 hours
    // ─────────────────────────────
    let sol_clone = sol.clone();
    let create_job = Job::new_async("0 0 */4 * * *", move |_uuid, _l| {
        let sol = sol_clone.clone();
        Box::pin(async move {
            if let Err(e) = create_market_job(sol).await {
                tracing::error!("Create market job failed: {:?}", e);
            }
        })
    })?;
    sched.add(create_job).await?;

    // ─────────────────────────────
    // Settle Market every 4 hours + 10 min
    // ─────────────────────────────
    let sol_clone = sol.clone();
    let settle_job = Job::new_async("0 10 */4 * * *", move |_uuid, _l| {
        let sol = sol_clone.clone();
        Box::pin(async move {
            if let Err(e) = settle_market_job(sol).await {
                tracing::error!("Settle market job failed: {:?}", e);
            }
        })
    })?;
    sched.add(settle_job).await?;

    // Start scheduler
    sched.start().await?;
    tracing::info!("Scheduler started: create every 4h, settle every 4h+10m");

    Ok(())
}
