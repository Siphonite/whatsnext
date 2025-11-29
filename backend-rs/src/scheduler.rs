use anyhow::Result;
use chrono::{Utc, TimeZone};
use tokio_cron_scheduler::{Job, JobScheduler};
use std::{fs, path::Path, sync::Arc};
use sqlx::{Pool, Postgres};

// INTERNAL IMPORTS
use crate::oracle::get_latest_candle;
use crate::solana_client::SolanaClient;
// Note: We imported the new function 'get_expired_unsettled_markets' here
use crate::repository::{insert_market, update_market_settlement, get_expired_unsettled_markets};
use crate::constants::SUPPORTED_ASSETS; 

// path to store current market_id (absolute or relative)
// We ONLY use this for generating unique IDs now, not for settlement logic.
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
// CREATE MARKET JOB (Multi-Asset)
// -----------------------------------------------------------------------------
async fn create_market_job(
    sol: Arc<SolanaClient>,
    pool: Pool<Postgres>,
) -> Result<()> {
    
    // LOOP: Iterate through all 9 assets defined in constants.rs
    for asset in SUPPORTED_ASSETS.iter() {
        
        // 1. Fetch Oracle Data
        // We use our smart oracle that knows how to map "GOLD" -> "GC=F"
        let candle_res = get_latest_candle(asset, 4).await;
        
        if let Err(e) = candle_res {
            tracing::error!("Skipping {} due to oracle error: {:?}", asset, e);
            continue; // Skip this asset, try the next one
        }
        let candle = candle_res.unwrap();
        let open_price = candle.open;

        // 2. Compute Times
        let start_time = candle.timestamp as i64;
        let end_time = start_time + (4 * 3600); // 4 hours later
        let lock_time = end_time - (10 * 60);   // 10 mins before close

        // 3. Generate Unique ID
        // Note: In a real prod app, we might use DB sequences, but this is fine for MVP.
        let mut id = load_market_id()?;
        id += 1;
        save_market_id(id)?; // Save immediately so the next asset in the loop gets a fresh ID

        tracing::info!(
            "Creating market {} for {} | Open: {}", 
            id, asset, open_price
        );

        // 4. Submit to Solana
        let sol_clone = sol.clone();
        let asset_string = asset.to_string();
        
        // We scale price by 100 for on-chain integer math (e.g., 50000.50 -> 5000050)
        let on_chain_price = (open_price * 100.0) as u64;

        let sig_res = tokio::task::spawn_blocking(move || {
            sol_clone.create_market_and_send(
                asset_string,
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
                
                // Only insert into DB if on-chain succeeded
                match insert_market(
                    &pool,
                    id as i64,
                    asset,
                    Utc.timestamp_opt(start_time, 0).unwrap(),
                    Utc.timestamp_opt(end_time, 0).unwrap(),
                    Utc.timestamp_opt(lock_time, 0).unwrap(),
                    open_price,
                ).await {
                    Ok(_) => tracing::info!("💾 Market {} saved to DB", id),
                    Err(e) => tracing::error!("DB Insert failed for {}: {:?}", id, e),
                }
            }
            Ok(Err(e)) => tracing::error!("On-chain creation failed for {}: {:?}", id, e),
            Err(e) => tracing::error!("Spawn blocking error: {:?}", e),
        }
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
    
    // 1. Ask Database: "Who is ready to close?"
    let markets_to_settle = get_expired_unsettled_markets(&pool).await?;

    if markets_to_settle.is_empty() {
        tracing::info!("No markets need settling right now.");
        return Ok(());
    }

    tracing::info!("Found {} markets to settle.", markets_to_settle.len());

    for market in markets_to_settle {
        tracing::info!("Settling Market ID {} ({})", market.market_id, market.asset);

        // 2. Fetch Closing Price
        let candle_res = get_latest_candle(&market.asset, 4).await;
        
        // If oracle fails here, we skip. The job will pick it up again in 10 mins.
        if let Err(e) = candle_res {
            tracing::error!("Settlement oracle failed for {}: {:?}", market.asset, e);
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
                tracing::info!("✅ Market {} settled on-chain. Tx: {}", m_id, sig);
                
                match update_market_settlement(&pool, market.market_id, close_price, true).await {
                    Ok(_) => tracing::info!("💾 DB updated: Market {} is SETTLED", m_id),
                    Err(e) => tracing::error!("❌ DB Update failed for {}: {:?}", m_id, e),
                }
            }
            Ok(Err(e)) => tracing::error!("❌ On-chain settlement failed for {}: {:?}", m_id, e),
            Err(e) => tracing::error!("❌ Spawn blocking error: {:?}", e),
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

    // Job 1: Create Markets (Every 4 hours at 00:00, 04:00, etc.)
    let sol_clone = sol.clone();
    let pool_clone = pool.clone();
    let create_job = Job::new_async("0 0 */4 * * *", move |_uuid, _l| {
        let sol = sol_clone.clone();
        let pool = pool_clone.clone();
        Box::pin(async move {
            if let Err(e) = create_market_job(sol, pool).await {
                tracing::error!("Create job crashed: {:?}", e);
            }
        })
    })?;
    sched.add(create_job).await?;

    // Job 2: Settle Markets (Every 10 minutes)
    // We run this frequently now because we check the DB. 
    // If nothing is expired, it just does nothing. Safer than timing it perfectly.
    let sol_clone = sol.clone();
    let pool_clone = pool.clone();
    let settle_job = Job::new_async("0 */10 * * * *", move |_uuid, _l| {
        let sol = sol_clone.clone();
        let pool = pool_clone.clone();
        Box::pin(async move {
            if let Err(e) = settle_market_job(sol, pool).await {
                tracing::error!("Settle job crashed: {:?}", e);
            }
        })
    })?;
    sched.add(settle_job).await?;

    sched.start().await?;
    tracing::info!("Multi-Asset Scheduler Active: Creating 9 assets every 4h, checking settlement every 10m.");

    Ok(())
}