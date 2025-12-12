use serde::{Serialize, Deserialize};
use sqlx::{Pool, Postgres};
use chrono::{DateTime, Utc};
use anyhow::Result;
use bigdecimal::BigDecimal;
use bigdecimal::ToPrimitive;
use bigdecimal::FromPrimitive;

//
// Data Models
//

#[derive(Debug, Serialize, Deserialize)]
pub struct Market {
    pub id: i64,
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
// Insert Market — DB assigns ID automatically
//
pub async fn insert_market(
    pool: &Pool<Postgres>,
    asset: &str,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    lock_time: DateTime<Utc>,
    open_price: f64,
) -> Result<i64> {

    let open_bd = BigDecimal::from_f64(open_price)
        .ok_or_else(|| anyhow::anyhow!("Failed to convert open_price to BigDecimal"))?;

    let row = sqlx::query!(
        r#"
        INSERT INTO markets (
            asset, start_time, end_time, lock_time,
            open_price, green_pool_weighted, red_pool_weighted, virtual_liquidity
        )
        VALUES ($1, $2, $3, $4, $5, 100, 100, 100)
        RETURNING id
        "#,
        asset,
        start_time,
        end_time,
        lock_time,
        open_bd
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
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

    let amount_bd = BigDecimal::from_f64(amount).unwrap();
    let weight_bd = BigDecimal::from_f64(weight).unwrap();
    let stake_bd = BigDecimal::from_f64(effective_stake).unwrap();

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
// Update Market Settlement
//
pub async fn update_market_settlement(
    pool: &Pool<Postgres>,
    market_id: i64,
    close_price: f64,
    settled: bool,
) -> Result<()> {

    let close_bd = BigDecimal::from_f64(close_price).unwrap();

    sqlx::query!(
        r#"
        UPDATE markets
        SET close_price = $2,
            settled = $3
        WHERE id = $1
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
// Fetch Latest Market (by highest id)
//
pub async fn get_latest_market(pool: &Pool<Postgres>) -> Result<Market> {

    let row = sqlx::query!(
        r#"
        SELECT 
            id, asset, start_time, end_time, lock_time,
            open_price, close_price, green_pool_weighted,
            red_pool_weighted, virtual_liquidity, settled, created_at
        FROM markets
        ORDER BY id DESC
        LIMIT 1
        "#
    )
    .fetch_one(pool)
    .await?;

    Ok(Market {
        id: row.id,
        asset: row.asset,
        start_time: row.start_time,
        end_time: row.end_time,
        lock_time: row.lock_time,
        open_price: row.open_price.and_then(|v| v.to_f64()),
        close_price: row.close_price.and_then(|v| v.to_f64()),
        green_pool_weighted: row.green_pool_weighted.and_then(|v| v.to_f64()),
        red_pool_weighted: row.red_pool_weighted.and_then(|v| v.to_f64()),
        virtual_liquidity: row.virtual_liquidity.and_then(|v| v.to_f64()),
        settled: row.settled,
        created_at: row.created_at,
    })
}

//
// Get Market by ID
//
pub async fn get_market_from_db(pool: &Pool<Postgres>, id: i64) -> Result<Market> {

    let row = sqlx::query!(
        r#"
        SELECT 
            id, asset, start_time, end_time, lock_time,
            open_price, close_price, green_pool_weighted,
            red_pool_weighted, virtual_liquidity, settled, created_at
        FROM markets
        WHERE id = $1
        LIMIT 1
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(Market {
        id: row.id,
        asset: row.asset,
        start_time: row.start_time,
        end_time: row.end_time,
        lock_time: row.lock_time,
        open_price: row.open_price.and_then(|v| v.to_f64()),
        close_price: row.close_price.and_then(|v| v.to_f64()),
        green_pool_weighted: row.green_pool_weighted.and_then(|v| v.to_f64()),
        red_pool_weighted: row.red_pool_weighted.and_then(|v| v.to_f64()),
        virtual_liquidity: row.virtual_liquidity.and_then(|v| v.to_f64()),
        settled: row.settled,
        created_at: row.created_at,
    })
}

//
// Expired Unsatisfied Markets
//
pub async fn get_expired_unsettled_markets(pool: &Pool<Postgres>) -> Result<Vec<Market>> {

    let rows = sqlx::query!(
        r#"
        SELECT 
            id, asset, start_time, end_time, lock_time,
            open_price, close_price, green_pool_weighted,
            red_pool_weighted, virtual_liquidity, settled, created_at
        FROM markets
        WHERE settled = false 
        AND end_time <= NOW()
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|row| Market {
        id: row.id,
        asset: row.asset,
        start_time: row.start_time,
        end_time: row.end_time,
        lock_time: row.lock_time,
        open_price: row.open_price.and_then(|v| v.to_f64()),
        close_price: row.close_price.and_then(|v| v.to_f64()),
        green_pool_weighted: row.green_pool_weighted.and_then(|v| v.to_f64()),
        red_pool_weighted: row.red_pool_weighted.and_then(|v| v.to_f64()),
        virtual_liquidity: row.virtual_liquidity.and_then(|v| v.to_f64()),
        settled: row.settled,
        created_at: row.created_at,
    }).collect())
}

//
// Active Markets list
//
pub async fn get_active_markets(pool: &Pool<Postgres>) -> Result<Vec<Market>> {

    let rows = sqlx::query!(
        r#"
        SELECT 
            id, asset, start_time, end_time, lock_time,
            open_price, close_price, green_pool_weighted,
            red_pool_weighted, virtual_liquidity, settled, created_at
        FROM markets
        WHERE settled = false
        ORDER BY id ASC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|row| Market {
        id: row.id,
        asset: row.asset,
        start_time: row.start_time,
        end_time: row.end_time,
        lock_time: row.lock_time,
        open_price: row.open_price.and_then(|v| v.to_f64()),
        close_price: row.close_price.and_then(|v| v.to_f64()),
        green_pool_weighted: row.green_pool_weighted.and_then(|v| v.to_f64()),
        red_pool_weighted: row.red_pool_weighted.and_then(|v| v.to_f64()),
        virtual_liquidity: row.virtual_liquidity.and_then(|v| v.to_f64()),
        settled: row.settled,
        created_at: row.created_at,
    }).collect())
}
