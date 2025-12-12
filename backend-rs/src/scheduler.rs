use anyhow::Result;
use chrono::{Utc, TimeZone};
use tokio_cron_scheduler::{Job, JobScheduler};
use std::sync::Arc;
use sqlx::{Pool, Postgres};

// INTERNAL IMPORTS
use crate::oracle::get_latest_candle;
use crate::solana_client::SolanaClient;
use crate::repository::{
    insert_market,
    update_market_settlement,
    get_expired_unsettled_markets,
};

/// ---------------------------------------------------------------------------
/// CREATE MARKET JOB
/// ---------------------------------------------------------------------------
async fn create_market_job(
    sol: Arc<SolanaClient>,
    pool: Pool<Postgres>,
) -> Result<()> 
{
    let asset = "BTC/USDT";

    // 1. Fetch oracle candle (4h interval)
    let candle_res = get_latest_candle(4).await;
    if let Err(e) = candle_res {
        tracing::error!("Oracle error, skipping market creation: {:?}", e);
        return Ok(());
    }

    let candle = candle_res.unwrap();
    let open_price = candle.open;

    // 2. Compute times
    let start_time = candle.timestamp as i64;
    let end_time = start_time + 4 * 3600;
    let lock_time = end_time - 10 * 60;

    // ------------------------------------------------------------
    // 3. Compute the REAL market_id for Solana (UNIQUE candle ID)
    // ------------------------------------------------------------
    let market_id = start_time; // USE timestamp as unique market identifier

    // 4. Insert into DB → returns DB primary key ID (unused for blockchain)
    let db_id = insert_market(
        &pool,
        market_id,                                   // FIXED
        asset,
        Utc.timestamp_opt(start_time, 0).unwrap(),
        Utc.timestamp_opt(end_time, 0).unwrap(),
        Utc.timestamp_opt(lock_time, 0).unwrap(),
        open_price,
    )
    .await?;

    tracing::info!(
        "[MARKET CREATE] DB Market Created: db_id={} | market_id={} | open={}",
        db_id,
        market_id,
        open_price
    );

    // 5. Call Solana create_market
    let sol_clone = sol.clone();
    let on_chain_price = (open_price * 100.0) as u64;

    let sig_res = tokio::task::spawn_blocking(move || {
        sol_clone.create_market_and_send(
            on_chain_price,
            start_time,
            end_time,
            market_id as u64,          // FIXED - use REAL market_id
        )
    })
    .await;

    match sig_res {
        Ok(Ok(sig)) => {
            tracing::info!(
                "[MARKET CREATE] On-chain success: market_id={} tx={}",
                market_id,
                sig
            );
        }
        Ok(Err(e)) => {
            tracing::error!(
                "[MARKET CREATE] On-chain failure: market_id={} err={:?}",
                market_id,
                e
            );
        }
        Err(e) => tracing::error!("spawn_blocking error: {:?}", e),
    }

    Ok(())
}

/// ---------------------------------------------------------------------------
/// SETTLE MARKET JOB
/// ---------------------------------------------------------------------------
async fn settle_market_job(
    sol: Arc<SolanaClient>,
    pool: Pool<Postgres>,
) -> Result<()> 
{
    let markets = get_expired_unsettled_markets(&pool).await?;

    if markets.is_empty() {
        tracing::info!("[SETTLEMENT] No expired markets.");
        return Ok(());
    }

    tracing::info!("[SETTLEMENT] Found {} markets to settle.", markets.len());

    for market in markets {
        let market_id = market.market_id; // FIXED — not market.id

        tracing::info!(
            "[SETTLEMENT] Processing market_id={}",
            market_id
        );

        // 1. Fetch new close candle
        let candle_res = get_latest_candle(4).await;
        if let Err(e) = candle_res {
            tracing::error!(
                "[SETTLEMENT] Oracle error for market {}: {:?}",
                market_id, e
            );
            continue;
        }

        let close_price = candle_res.unwrap().close;
        let on_chain_price = (close_price * 100.0) as u64;

        // 2. Call Solana settle_market
        let sol_clone = sol.clone();
        let sig_res = tokio::task::spawn_blocking(move || {
            sol_clone.settle_market_and_send(market_id as u64, on_chain_price)
        })
        .await;

        match sig_res {
            Ok(Ok(sig)) => {
                tracing::info!(
                    "[SETTLEMENT] On-chain settlement complete: market_id={} tx={}",
                    market_id, sig
                );

                // 3. Update DB
                update_market_settlement(
                    &pool,
                    market_id,
                    close_price,
                    true,
                )
                .await?;

                tracing::info!(
                    "[SETTLEMENT] Database updated for market_id={}",
                    market_id
                );
            }
            Ok(Err(e)) => {
                tracing::error!(
                    "[SETTLEMENT] On-chain settlement failed: market_id={} err={:?}",
                    market_id, e
                );
            }
            Err(e) => tracing::error!("spawn_blocking error: {:?}", e),
        }
    }

    Ok(())
}

/// ---------------------------------------------------------------------------
/// START SCHEDULER
/// ---------------------------------------------------------------------------
pub async fn start_scheduler(
    sol: Arc<SolanaClient>,
    pool: Pool<Postgres>,
) -> Result<()> 
{
    let sched = JobScheduler::new().await?;

    // Every 4 hours → create new market
    let sol_clone = sol.clone();
    let pool_clone = pool.clone();
    let create_job = Job::new_async("0 0 */4 * * *", move |_uuid, _l| {
        let sol = sol_clone.clone();
        let pool = pool_clone.clone();
        Box::pin(async move {
            if let Err(e) = create_market_job(sol, pool).await {
                tracing::error!("[SCHEDULER] Create job error: {:?}", e);
            }
        })
    })?;
    sched.add(create_job).await?;

    // Every 10 minutes → settle expired markets
    let sol_clone = sol.clone();
    let pool_clone = pool.clone();
    let settle_job = Job::new_async("0 */10 * * * *", move |_uuid, _l| {
        let sol = sol_clone.clone();
        let pool = pool_clone.clone();
        Box::pin(async move {
            if let Err(e) = settle_market_job(sol, pool).await {
                tracing::error!("[SCHEDULER] Settle job error: {:?}", e);
            }
        })
    })?;
    sched.add(settle_job).await?;

    sched.start().await?;
    tracing::info!("[SCHEDULER] BTC Market Scheduler Active.");

    Ok(())
}
