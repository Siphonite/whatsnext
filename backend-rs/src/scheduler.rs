use anyhow::Result;
use chrono::{Utc, TimeZone};
use tokio_cron_scheduler::{Job, JobScheduler};
use std::{fs, path::Path, sync::Arc};

use crate::oracle::get_latest_candle;
use crate::solana_client::SolanaClient;
use crate::repository::{insert_market, update_market_settlement};
use sqlx::{Pool, Postgres};

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

// -----------------------------------------------------------------------------
// CREATE MARKET JOB
// -----------------------------------------------------------------------------
async fn create_market_job(
    sol: Arc<SolanaClient>,
    pool: Pool<Postgres>,
) -> Result<()> {
    let asset = "BTCUSDT";

    // Fetch candle (async)
    let candle = get_latest_candle(asset, 4).await?;
    // candle.open is a float (f64), representing the open price in normal units
    // We keep it as f64 here; repository will convert to BigDecimal
    // If your oracle gives scaled ints, adapt accordingly.
    let open_price = candle.open;

    // Compute times (ensure conversion to i64 for chrono)
    let start_time = candle.timestamp as i64;
    let end_time = start_time + (4 * 3600);
    let lock_time = end_time - (10 * 60);

    // Determine next market_id
    let mut id = load_market_id()?;
    id += 1;

    tracing::info!(
        "Creating market {} for {} | open={} start={} end={}",
        id, asset, open_price, start_time, end_time
    );

    // -----------------------------
    // 1) CALL SOLANA PROGRAM
    // -----------------------------
    let sol_clone = sol.clone();
    let asset_string = asset.to_string();
    let sig_res = tokio::task::spawn_blocking(move || {
        // Note: your create_market_and_send currently expects scaled ints in earlier code.
        // If your on-chain program expects a scaled integer, scale here (e.g. (open_price*100.0) as u64).
        sol_clone.create_market_and_send(
            asset_string,
            (open_price * 100.0) as u64,
            start_time,
            end_time,
            id,
        )
    })
    .await;

    match sig_res {
        Ok(Ok(sig)) => {
            tracing::info!("Market {} created on-chain. Tx: {}", id, sig);
            save_market_id(id)?;
        }
        Ok(Err(e)) => {
            tracing::error!("Failed to create market {}: {:?}", id, e);
            return Ok(()); // stop DB write on failure
        }
        Err(join_err) => {
            tracing::error!("spawn_blocking join error: {:?}", join_err);
            return Ok(()); // stop DB write on failure
        }
    }

    // -----------------------------
    // 2) INSERT MARKET INTO DATABASE
    // -----------------------------
    match insert_market(
        &pool,
        id as i64,
        asset,
        Utc.timestamp_opt(start_time, 0).unwrap(),
        Utc.timestamp_opt(end_time, 0).unwrap(),
        Utc.timestamp_opt(lock_time, 0).unwrap(),
        open_price,
    )
    .await {
        Ok(_) => tracing::info!("🟢 Market {} saved to DB", id),
        Err(e) => tracing::error!("DB insert_market failed: {:?}", e),
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// SETTLE MARKET JOB
// -----------------------------------------------------------------------------
async fn settle_market_job(
    sol: Arc<SolanaClient>,
    pool: Pool<Postgres>,
) -> Result<()> {
    let asset = "BTCUSDT";

    let id = load_market_id()?;
    if id == 0 {
        tracing::warn!("No market exists to settle yet.");
        return Ok(());
    }

    // 1) Oracle final close
    let candle = get_latest_candle(asset, 4).await?;
    let close_price = candle.close;

    tracing::info!(
        "Settling market {} | close={} timestamp={}",
        id, close_price, candle.timestamp
    );

    // -----------------------------
    // 2) CALL SOLANA SETTLE
    // -----------------------------
    let sol_clone = sol.clone();
    let sig_res = tokio::task::spawn_blocking(move || {
        // If on-chain expects scaled integer, scale here
        sol_clone.settle_market_and_send(id, (close_price * 100.0) as u64)
    })
    .await;

    match sig_res {
        Ok(Ok(sig)) => {
            tracing::info!("Market {} settled on-chain. Tx: {}", id, sig);
        }
        Ok(Err(e)) => {
            tracing::error!("Failed to settle market {}: {:?}", id, e);
            return Ok(()); // stop DB write on failure
        }
        Err(join_err) => {
            tracing::error!("spawn_blocking join error: {:?}", join_err);
            return Ok(()); // stop DB write on failure
        }
    }

    // -----------------------------
    // 3) UPDATE MARKET IN DATABASE
    // -----------------------------
    match update_market_settlement(
        &pool,
        id as i64,
        close_price,
        true,
    )
    .await {
        Ok(_) => tracing::info!("🟣 Market {} updated in DB", id),
        Err(e) => tracing::error!("DB update_market_settlement failed: {:?}", e),
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// START SCHEDULER
// -----------------------------------------------------------------------------
pub async fn start_scheduler(
    sol: Arc<SolanaClient>,
    pool: Pool<Postgres>,
) -> Result<()> {
    let sched = JobScheduler::new().await?;

    // -----------------------------
    // CREATE MARKET (Every 4 hours)
    // -----------------------------
    let sol_clone = sol.clone();
    let pool_clone = pool.clone();
    let create_job = Job::new_async("0 0 */4 * * *", move |_uuid, _l| {
        let sol = sol_clone.clone();
        let pool = pool_clone.clone();
        Box::pin(async move {
            if let Err(e) = create_market_job(sol, pool).await {
                tracing::error!("Create market job failed: {:?}", e);
            }
        })
    })?;
    sched.add(create_job).await?;

    // -----------------------------
    // SETTLE MARKET (4 hours + 10m)
    // -----------------------------
    let sol_clone = sol.clone();
    let pool_clone = pool.clone();
    let settle_job = Job::new_async("0 10 */4 * * *", move |_uuid, _l| {
        let sol = sol_clone.clone();
        let pool = pool_clone.clone();
        Box::pin(async move {
            if let Err(e) = settle_market_job(sol, pool).await {
                tracing::error!("Settle market job failed: {:?}", e);
            }
        })
    })?;
    sched.add(settle_job).await?;

    // Start scheduler
    sched.start().await?;
    tracing::info!("Scheduler started: Create every 4h, Settle after 4h+10m");

    Ok(())
}
