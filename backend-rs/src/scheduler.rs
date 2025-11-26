use anyhow::Result;
use chrono::{DateTime, Utc, Timelike};
use tokio_cron_scheduler::{Job, JobScheduler};
use serde::Deserialize;

#[derive(Deserialize)]
struct PriceApiResponse {
    price: f64,
}

async fn fetch_price_from_api(symbol: &str) -> Result<f64> {
    // dummy price for now (so no error)
    Ok(100.0)
}

// placeholder candle logic
fn compute_candle_bounds(now: DateTime<Utc>, hours: i64) -> (DateTime<Utc>, DateTime<Utc>) {
    let start_hour = now.hour() - (now.hour() % (hours as u32));
    let start = now
        .with_hour(start_hour).unwrap()
        .with_minute(0).unwrap()
        .with_second(0).unwrap()
        .with_nanosecond(0).unwrap();

    let end = start + chrono::Duration::hours(hours);
    (start, end)
}

async fn create_market_job() -> Result<()> {
    tracing::info!("Running create_market_job");

    let now = Utc::now();
    let (start, end) = compute_candle_bounds(now, 4);

    let price = fetch_price_from_api("BTCUSD").await?;
    tracing::info!("Fetched price: {}", price);

    // TODO: integrate SolanaClient here
    tracing::info!("Would call create_market here: start={}, end={}", start, end);

    Ok(())
}

async fn settle_market_job() -> Result<()> {
    tracing::info!("Running settle_market_job");

    let now = Utc::now();
    let (start, end) = compute_candle_bounds(now, 4);

    let price = fetch_price_from_api("BTCUSD").await?;
    tracing::info!("Fetched close price: {}", price);

    // TODO: integrate SolanaClient here
    tracing::info!("Would call settle_market here: start={}, end={}", start, end);

    Ok(())
}

pub async fn start_scheduler() -> Result<()> {
    tracing::info!("Starting scheduler...");

    let sched = JobScheduler::new().await?;

    let create_job = Job::new_async("0 0 */4 * * *", |_, _| {
        Box::pin(async {
            if let Err(e) = create_market_job().await {
                tracing::error!("create_market_job error: {:?}", e);
            }
        })
    })?;

    let settle_job = Job::new_async("0 10 */4 * * *", |_, _| {
        Box::pin(async {
            if let Err(e) = settle_market_job().await {
                tracing::error!("settle_market_job error: {:?}", e);
            }
        })
    })?;

    sched.add(create_job).await?;
    sched.add(settle_job).await?;

    sched.start().await?;

    tracing::info!("Scheduler started successfully!");

    Ok(())
}
