use serde::{Serialize, Deserialize};
use sqlx::{Pool, Postgres};
use chrono::{DateTime, Utc};
use anyhow::Result;
use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use bigdecimal::ToPrimitive;


//
// Data Models
//

#[derive(Debug, Serialize, Deserialize)]
pub struct Market {
    pub id: i64,
    pub market_id: i64,
    pub asset: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub lock_time: DateTime<Utc>,
    pub open_price: Option<f64>,
    pub close_price: Option<f64>,
    pub green_pool_weighted: Option<f64>,
    pub red_pool_weighted: Option<f64>,
    pub virtual_liquidity: Option<f64>,
    pub settled: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bet {
    pub id: i64,
    pub wallet: String,
    pub market_id: i64,
    pub side: String,
    pub amount: f64,
    pub weight: f64,
    pub effective_stake: f64,
    pub payout: Option<f64>,
    pub claimed: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
}

//
// Insert Market
//
pub async fn insert_market(
    pool: &Pool<Postgres>,
    market_id: i64,
    asset: &str,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    lock_time: DateTime<Utc>,
    open_price: f64,
) -> Result<()> {
    // Convert f64 -> BigDecimal because DB column is NUMERIC
    let open_bd: BigDecimal = BigDecimal::from_f64(open_price)
        .ok_or_else(|| anyhow::anyhow!("Failed to convert open_price to BigDecimal"))?;

    sqlx::query!(
        r#"
        INSERT INTO markets (
            market_id, asset, start_time, end_time, lock_time,
            open_price, green_pool_weighted, red_pool_weighted, virtual_liquidity
        )
        VALUES ($1, $2, $3, $4, $5, $6, 100, 100, 100)
        "#,
        market_id,
        asset,
        start_time,
        end_time,
        lock_time,
        open_bd
    )
    .execute(pool)
    .await?;

    Ok(())
}

//
// Insert Bet
//
pub async fn insert_bet(
    pool: &Pool<Postgres>,
    wallet: &str,
    market_id: i64,
    side: &str,
    amount: f64,
    weight: f64,
    effective_stake: f64,
) -> Result<()> {
    let amount_bd = BigDecimal::from_f64(amount)
        .ok_or_else(|| anyhow::anyhow!("Failed to convert amount to BigDecimal"))?;
    let weight_bd = BigDecimal::from_f64(weight)
        .ok_or_else(|| anyhow::anyhow!("Failed to convert weight to BigDecimal"))?;
    let stake_bd = BigDecimal::from_f64(effective_stake)
        .ok_or_else(|| anyhow::anyhow!("Failed to convert effective_stake to BigDecimal"))?;

    sqlx::query!(
        r#"
        INSERT INTO bets (
            wallet, market_id, side, amount, weight, effective_stake
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        wallet,
        market_id,
        side,
        amount_bd,
        weight_bd,
        stake_bd
    )
    .execute(pool)
    .await?;

    Ok(())
}

//
// Update Settlement
//
pub async fn update_market_settlement(
    pool: &Pool<Postgres>,
    market_id: i64,
    close_price: f64,
    settled: bool,
) -> Result<()> {
    let close_bd = BigDecimal::from_f64(close_price)
        .ok_or_else(|| anyhow::anyhow!("Failed to convert close_price to BigDecimal"))?;

    sqlx::query!(
        r#"
        UPDATE markets
        SET close_price = $2,
            settled = $3
        WHERE market_id = $1
        "#,
        market_id,
        close_bd,
        settled
    )
    .execute(pool)
    .await?;

    Ok(())
}

//
// Update PnL
//
pub async fn update_pnl(
    pool: &Pool<Postgres>,
    wallet: &str,
    pnl_delta: f64,
) -> Result<()> {
    let pnl_bd = BigDecimal::from_f64(pnl_delta)
        .ok_or_else(|| anyhow::anyhow!("Failed to convert pnl_delta to BigDecimal"))?;

    sqlx::query!(
        r#"
        INSERT INTO pnl (wallet, total_pnl, total_bets)
        VALUES ($1, $2, 1)
        ON CONFLICT (wallet)
        DO UPDATE SET 
            total_pnl = pnl.total_pnl + $2,
            total_bets = pnl.total_bets + 1,
            last_updated = NOW()
        "#,
        wallet,
        pnl_bd
    )
    .execute(pool)
    .await?;

    Ok(())
}
pub async fn get_latest_market(pool: &Pool<Postgres>) -> Result<Market> {
    let row = sqlx::query!(
        r#"
        SELECT 
            id, market_id, asset, start_time, end_time, lock_time,
            open_price, close_price, green_pool_weighted, red_pool_weighted,
            virtual_liquidity, settled, created_at
        FROM markets
        ORDER BY id DESC
        LIMIT 1
        "#
    )
    .fetch_one(pool)
    .await?;

    let mkt = Market {
        id: row.id,
        market_id: row.market_id,
        asset: row.asset,
        start_time: row.start_time,
        end_time: row.end_time,
        lock_time: row.lock_time,
        open_price: row.open_price.map(|v| v.to_f64().unwrap()),
        close_price: row.close_price.map(|v| v.to_f64().unwrap()),
        green_pool_weighted: row.green_pool_weighted.map(|v| v.to_f64().unwrap()),
        red_pool_weighted: row.red_pool_weighted.map(|v| v.to_f64().unwrap()),
        virtual_liquidity: row.virtual_liquidity.map(|v| v.to_f64().unwrap()),
        settled: row.settled,
        created_at: row.created_at,
    };

    Ok(mkt)
}

pub async fn get_market_from_db(pool: &Pool<Postgres>, id: i64) -> Result<Market> {
    let row = sqlx::query!(
        r#"
        SELECT 
            id, market_id, asset, start_time, end_time, lock_time,
            open_price, close_price, green_pool_weighted, red_pool_weighted,
            virtual_liquidity, settled, created_at
        FROM markets
        WHERE market_id = $1
        LIMIT 1
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    let mkt = Market {
        id: row.id,
        market_id: row.market_id,
        asset: row.asset,
        start_time: row.start_time,
        end_time: row.end_time,
        lock_time: row.lock_time,
        open_price: row.open_price.map(|v| v.to_f64().unwrap()),
        close_price: row.close_price.map(|v| v.to_f64().unwrap()),
        green_pool_weighted: row.green_pool_weighted.map(|v| v.to_f64().unwrap()),
        red_pool_weighted: row.red_pool_weighted.map(|v| v.to_f64().unwrap()),
        virtual_liquidity: row.virtual_liquidity.map(|v| v.to_f64().unwrap()),
        settled: row.settled,
        created_at: row.created_at,
    };

    Ok(mkt)
}

// NEW FUNCTION: Fetch all markets that are ready to be settled
pub async fn get_expired_unsettled_markets(pool: &Pool<Postgres>) -> Result<Vec<Market>> {
    // We query for markets where:
    // 1. 'settled' is false (we haven't finished it yet)
    // 2. 'end_time' is in the past (time is up!)
    let rows = sqlx::query!(
        r#"
        SELECT 
            id, market_id, asset, start_time, end_time, lock_time,
            open_price, close_price, green_pool_weighted, red_pool_weighted,
            virtual_liquidity, settled, created_at
        FROM markets
        WHERE settled = false 
        AND end_time <= NOW()
        "#
    )
    .fetch_all(pool)
    .await?;

    // Map the raw SQL rows into our clean 'Market' struct
    let markets = rows.into_iter().map(|row| Market {
        id: row.id,
        market_id: row.market_id,
        asset: row.asset,
        start_time: row.start_time,
        end_time: row.end_time,
        lock_time: row.lock_time,
        // Convert BigDecimals/Options to Rust types
        open_price: row.open_price.map(|v| v.to_f64().unwrap()),
        close_price: row.close_price.map(|v| v.to_f64().unwrap()),
        green_pool_weighted: row.green_pool_weighted.map(|v| v.to_f64().unwrap()),
        red_pool_weighted: row.red_pool_weighted.map(|v| v.to_f64().unwrap()),
        virtual_liquidity: row.virtual_liquidity.map(|v| v.to_f64().unwrap()),
        settled: row.settled,
        created_at: row.created_at,
    }).collect();

    Ok(markets)
}
