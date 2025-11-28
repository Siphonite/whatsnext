use serde::{Serialize, Deserialize};
use sqlx::{Pool, Postgres};
use chrono::{DateTime, Utc};
use anyhow::Result;

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
        open_price
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
        amount,
        weight,
        effective_stake
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

    sqlx::query!(
        r#"
        UPDATE markets
        SET close_price = $2,
            settled = $3
        WHERE market_id = $1
        "#,
        market_id,
        close_price,
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
        pnl_delta
    )
    .execute(pool)
    .await?;

    Ok(())
}
