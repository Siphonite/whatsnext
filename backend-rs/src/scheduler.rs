use anyhow::Result;
use chrono::{Utc, TimeZone};
use tokio_cron_scheduler::{Job, JobScheduler};
use std::{fs, path::Path, sync::Arc};
use sqlx::{Pool, Postgres};

// INTERNAL IMPORTS
use crate::oracle::get_latest_candle;
use crate::solana_client::SolanaClient;
use crate::repository::{insert_market, update_market_settlement, get_expired_unsettled_markets};
use crate::config::MARKET_ASSET;

// path to store current market_id (local file-based sequencer)
const MARKET_ID_PATH: &str = "market_id.txt"; 

// Load current market_id from file (default 0)
pub fn load_market_id() -> Result<u64> {
    if !Path::new(MARKET_ID_PATH).exists() {
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
// CREATE MARKET JOB (Single Asset - BTC/USDT)
// -----------------------------------------------------------------------------
async fn create_market_job(
    sol: Arc<SolanaClient>,
    pool: Pool<Postgres>,
) -> Result<()> {
    
    let asset = MARKET_ASSET;

    // 1. Fetch Oracle Data
    let candle_res = get_latest_candle(4).await;
    if let Err(e) = candle_res {
        tracing::error!("Skipping {} due to oracle error: {:?}", asset, e);
        return Ok(());
    }

    let candle = candle_res.unwrap();
    let open_price = candle.open;

    // 2. Compute Times
    let start_time = candle.timestamp as i64;
    let end_time   = start_time + (4 * 3600); // 4 hours later
    let lock_time  = end_time - (10 * 60);    // 10 mins before close

    // 3. Generate Unique ID (local file-based counter)
    let mut id = load_market_id()?;
    id += 1;
    save_market_id(id)?;

    tracing::info!(
        "Creating BTC market {} | Open: {} | Start: {}",
        id, open_price, start_time
    );

    // 4. Submit to Solana
    let sol_clone = sol.clone();

    // Scale price for on-chain units (float → integer)
    let on_chain_price = (open_price * 100.0) as u64;

    let sig_res = tokio::task::spawn_blocking(move || {
        sol_clone.create_market_and_send(
            on_chain_price,
            start_time,
            end_time,
            id,
        )
    })
    .await;

    // 5. Save to Database
    match sig_res {
        Ok(Ok(sig)) => {
            tracing::info!("Market {} confirmed on-chain. Tx: {}", id, sig);

            match insert_market(
                &pool,
                id as i64,
                asset,
                Utc.timestamp_opt(start_time, 0).unwrap(),
                Utc.timestamp_opt(end_time, 0).unwrap(),
                Utc.timestamp_opt(lock_time, 0).unwrap(),
                open_price,
            ).await {
                Ok(_) => tracing::info!("Market {} saved to DB", id),
                Err(e) => tracing::error!("DB Insert failed for {}: {:?}", id, e),
            }
        }
        Ok(Err(e)) => tracing::error!("On-chain creation failed for {}: {:?}", id, e),
        Err(e) => tracing::error!("Spawn blocking error: {:?}", e),
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// SETTLE MARKET JOB (Database Driven)
// -----------------------------------------------------------------------------
async fn settle_market_job(
    sol: Arc<SolanaClient>,
    pool: Pool<Postgres>,
) -> Result<()> {
    
    // 1. Ask Database: "Which BTC markets have expired but not settled?"
    let markets_to_settle = get_expired_unsettled_markets(&pool).await?;

    if markets_to_settle.is_empty() {
        tracing::info!("No markets need settling right now.");
        return Ok(());
    }

    tracing::info!("Found {} BTC markets to settle.", markets_to_settle.len());

    for market in markets_to_settle {
        tracing::info!(
            "Settling Market {} (BTC/USDT)",
            market.market_id
        );

        // 2. Fetch Closing Price from Oracle
        let candle_res = get_latest_candle(4).await;

        if let Err(e) = candle_res {
            tracing::error!("Settlement oracle failed for BTC: {:?}", e);
            continue;
        }
        let close_price = candle_res.unwrap().close;

        // 3. Submit to Solana
        let sol_clone = sol.clone();
        let m_id = market.market_id as u64;
        let on_chain_close_price = (close_price * 100.0) as u64;

        let sig_res = tokio::task::spawn_blocking(move || {
            sol_clone.settle_market_and_send(m_id, on_chain_close_price)
        })
        .await;

        // 4. Update Database
        match sig_res {
            Ok(Ok(sig)) => {
                tracing::info!("Market {} settled on-chain. Tx: {}", m_id, sig);

                match update_market_settlement(
                    &pool,
                    market.market_id,
                    close_price,
                    true
                ).await {
                    Ok(_)  => tracing::info!("DB updated: Market {} SETTLED", m_id),
                    Err(e) => tracing::error!("DB update failed: {:?}", e),
                }
            }
            Ok(Err(e)) => tracing::error!("On-chain settlement failed: {:?}", e),
            Err(e)     => tracing::error!("Spawn blocking error: {:?}", e),
        }
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

    //  Create one BTC market every 4 hours
    let sol_clone = sol.clone();
    let pool_clone = pool.clone();
    let create_job = Job::new_async("0 0 */4 * * *", move |_uuid, _l| {
        let sol = sol_clone.clone();
        let pool = pool_clone.clone();
        Box::pin(async move {
            if let Err(e) = create_market_job(sol, pool).await {
                tracing::error!("Create BTC job crashed: {:?}", e);
            }
        })
    })?;
    sched.add(create_job).await?;

    //  Settle expired BTC markets every 10 minutes
    let sol_clone = sol.clone();
    let pool_clone = pool.clone();
    let settle_job = Job::new_async("0 */10 * * * *", move |_uuid, _l| {
        let sol = sol_clone.clone();
        let pool = pool_clone.clone();
        Box::pin(async move {
            if let Err(e) = settle_market_job(sol, pool).await {
                tracing::error!("Settle BTC job crashed: {:?}", e);
            }
        })
    })?;
    sched.add(settle_job).await?;

    sched.start().await?;

    tracing::info!("BTC-Only Scheduler Active: Creating a single BTC market every 4h, checking settlement every 10m.");

    Ok(())
}
