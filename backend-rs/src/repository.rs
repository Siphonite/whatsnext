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
    let open_bd = BigDecimal::from_f64(open_price)
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

    let pnl_bd = BigDecimal::from_f64(pnl_delta).unwrap();

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

//
// Get Latest Market
//
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

    Ok(Market {
        id: row.id,
        market_id: row.market_id,
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

    Ok(Market {
        id: row.id,
        market_id: row.market_id,
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
// Get Expired Unsettled Markets
//
pub async fn get_expired_unsettled_markets(pool: &Pool<Postgres>) -> Result<Vec<Market>> {

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

    Ok(rows.into_iter().map(|row| Market {
        id: row.id,
        market_id: row.market_id,
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
// Active Markets (for dashboard grid)
//
pub async fn get_active_markets(pool: &Pool<Postgres>) -> Result<Vec<Market>> {

    let rows = sqlx::query!(
        r#"
        SELECT 
            id, market_id, asset, start_time, end_time, lock_time,
            open_price, close_price, green_pool_weighted, red_pool_weighted,
            virtual_liquidity, settled, created_at
        FROM markets
        WHERE settled = false
        ORDER BY market_id ASC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|row| Market {
        id: row.id,
        market_id: row.market_id,
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
// PnL
//
#[derive(Debug, Serialize, Deserialize)]
pub struct PnlStats {
    pub wallet: String,
    pub total_pnl: f64,
    pub total_bets: i64,
}

pub async fn get_user_pnl(pool: &Pool<Postgres>, wallet: &str) -> Result<PnlStats> {

    let row = sqlx::query!(
        r#"
        SELECT wallet, total_pnl, total_bets
        FROM pnl
        WHERE wallet = $1
        "#,
        wallet
    )
    .fetch_optional(pool)
    .await?;

    if let Some(r) = row {
        return Ok(PnlStats {
            wallet: r.wallet,
            total_pnl: r.total_pnl.and_then(|v| v.to_f64()).unwrap_or(0.0),
            total_bets: r.total_bets.unwrap_or(0),
        });
    }

    Ok(PnlStats {
        wallet: wallet.to_string(),
        total_pnl: 0.0,
        total_bets: 0,
    })
}

//
// Positions
//
#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub market_id: i64,
    pub side: String,
    pub amount: f64,
    pub weight: f64,
    pub effective_stake: f64,
    pub payout: Option<f64>,
    pub timestamp: i64,
    pub status: String,
}

pub async fn get_user_positions(pool: &Pool<Postgres>, wallet: &str) -> Result<(Vec<Position>, Vec<Position>)> {

    let rows = sqlx::query!(
        r#"
        SELECT 
            b.market_id,
            b.side,
            b.amount,
            b.weight,
            b.effective_stake,
            b.payout,
            EXTRACT(EPOCH FROM b.created_at)::BIGINT as timestamp,
            COALESCE(m.settled, false) as settled
        FROM bets b
        LEFT JOIN markets m ON b.market_id = m.market_id
        WHERE b.wallet = $1
        ORDER BY b.created_at DESC
        "#,
        wallet
    )
    .fetch_all(pool)
    .await?;

    let mut open_positions = Vec::new();
    let mut settled_positions = Vec::new();

    for row in rows {
        let pos = Position {
            market_id: row.market_id,
            side: row.side,
            amount: row.amount.to_f64().unwrap_or(0.0),
            weight: row.weight.to_f64().unwrap_or(0.0),
            effective_stake: row.effective_stake.to_f64().unwrap_or(0.0),
            payout: row.payout.and_then(|v| v.to_f64()),
            timestamp: row.timestamp.unwrap_or(0),
            status: if row.settled.unwrap_or(false) { "SETTLED".into() } else { "OPEN".into() }
        };

        if row.settled.unwrap_or(false) {
            settled_positions.push(pos);
        } else {
            open_positions.push(pos);
        }
    }

    Ok((open_positions, settled_positions))
}

//
// Enhanced PnL
//
#[derive(Debug, Serialize, Deserialize)]
pub struct EnhancedPnlStats {
    pub total_pnl: f64,
    pub win_rate: f64,
    pub streak: i64,
}

pub async fn get_enhanced_pnl(pool: &Pool<Postgres>, wallet: &str) -> Result<EnhancedPnlStats> {

    // Total PnL
    let pnl_row = sqlx::query!(
        r#"
        SELECT 
            COALESCE(SUM(b.payout - b.effective_stake), 0) as total_pnl
        FROM bets b
        INNER JOIN markets m ON b.market_id = m.market_id
        WHERE b.wallet = $1 AND m.settled = true AND b.payout IS NOT NULL
        "#,
        wallet
    )
    .fetch_one(pool)
    .await?;

    let total_pnl = pnl_row.total_pnl.and_then(|v| v.to_f64()).unwrap_or(0.0);

    // Win Rate
    let win_rate_row = sqlx::query!(
        r#"
        SELECT 
            COUNT(*) as total,
            SUM(CASE WHEN b.payout > b.effective_stake THEN 1 ELSE 0 END) as wins
        FROM bets b
        INNER JOIN markets m ON b.market_id = m.market_id
        WHERE b.wallet = $1 AND m.settled = true AND b.payout IS NOT NULL
        "#,
        wallet
    )
    .fetch_one(pool)
    .await?;

    let total_settled = win_rate_row.total.unwrap_or(0) as f64;
    let wins = win_rate_row.wins.unwrap_or(0) as f64;
    let win_rate = if total_settled > 0.0 { (wins / total_settled) * 100.0 } else { 0.0 };

    // Streak
    let streak_rows = sqlx::query!(
        r#"
        SELECT 
            b.payout,
            b.effective_stake,
            b.created_at
        FROM bets b
        INNER JOIN markets m ON b.market_id = m.market_id
        WHERE b.wallet = $1 AND m.settled = true AND b.payout IS NOT NULL
        ORDER BY b.created_at DESC
        LIMIT 100
        "#,
        wallet
    )
    .fetch_all(pool)
    .await?;

    let mut streak = 0;
    for row in streak_rows {
        let payout = row.payout.and_then(|v| v.to_f64()).unwrap_or(0.0);
        let stake = row.effective_stake.to_f64().unwrap_or(0.0);
        if payout > stake {
            streak += 1;
        } else {
            break;
        }
    }

    Ok(EnhancedPnlStats {
        total_pnl,
        win_rate,
        streak,
    })
}
