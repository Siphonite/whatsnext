use anyhow::{Result};
use chrono::Utc;
use tokio_cron_scheduler::{Job, JobScheduler};
use std::{
    fs,
    path::Path,
    sync::Arc,
};

use crate::oracle::get_latest_candle;
use crate::solana_client::SolanaClient;

// path to store current market_id (absolute fine; you can change to relative if you prefer)
const MARKET_ID_PATH: &str = "/home/siphonite/whatsnext/backend-rs/src/data/market_id.txt";

// Load current market_id from file (default 0)
pub fn load_market_id() -> Result<u64> {
    if !Path::new(MARKET_ID_PATH).exists() {
        // ensure directory exists
        if let Some(parent) = Path::new(MARKET_ID_PATH).parent() {
            fs::create_dir_all(parent)?;
        }
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

    // Fetch candle (async)
    let candle = get_latest_candle(asset, 4).await?;
    let open_price = (candle.open * 100.0) as u64; // scale

    // Compute times
    let start_time = candle.timestamp;
    let end_time = start_time + (4 * 3600);

    // Determine next market_id (sync file read)
    let mut id = load_market_id()?;
    id += 1;

    tracing::info!(
        "Creating market {} for {} | open={} start={} end={}",
        id, asset, open_price, start_time, end_time
    );

    // Call Solana program in blocking thread pool
    let sol_clone = sol.clone();
    let asset_string = asset.to_string();
    let sig_res = tokio::task::spawn_blocking(move || {
        // This closure runs on the blocking thread pool.
        sol_clone.create_market_and_send(
            asset_string,
            open_price,
            start_time,
            end_time,
            id,
        )
    })
    .await; // await JoinHandle

    match sig_res {
        Ok(Ok(sig)) => {
            tracing::info!("Market {} created. Tx: {}", id, sig);
            // persist id
            save_market_id(id)?;
        }
        Ok(Err(e)) => {
            tracing::error!("Failed to create market {}: {:?}", id, e);
        }
        Err(join_err) => {
            tracing::error!("spawn_blocking join error: {:?}", join_err);
        }
    }

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

    let sol_clone = sol.clone();
    let sig_res = tokio::task::spawn_blocking(move || {
        sol_clone.settle_market_and_send(id, close_price)
    })
    .await;

    match sig_res {
        Ok(Ok(sig)) => {
            tracing::info!("Market {} settled. Tx: {}", id, sig);
        }
        Ok(Err(e)) => {
            tracing::error!("Failed to settle market {}: {:?}", id, e);
        }
        Err(join_err) => {
            tracing::error!("spawn_blocking join error: {:?}", join_err);
        }
    }

    Ok(())
}

// START SCHEDULER
pub async fn start_scheduler(sol: Arc<SolanaClient>) -> Result<()> {
    let sched = JobScheduler::new().await?;

    // Create Market every 4 hours (at 0 minutes of the 4-hour block)
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

    // Settle Market every 4 hours + 10 minutes
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

    // Start scheduler (runs within existing runtime)
    sched.start().await?;
    tracing::info!("Scheduler started: create every 4h, settle every 4h+10m");

    Ok(())
}
