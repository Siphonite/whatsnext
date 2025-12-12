use axum::{
    Router,
    routing::{get, post},
    extract::{Path, State},
    Json,
};
use serde_json::json;
    use std::sync::Arc;
use chrono::{Utc, TimeZone};

use crate::state::AppState;
use crate::repository::{
    get_market_from_db,
    get_active_markets,
    get_user_pnl,
    insert_market,
};
use crate::oracle::get_latest_candle;


/// ---------------------------------------------------------------------------
/// MARKET ROUTES
/// ---------------------------------------------------------------------------
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/active", get(get_active_markets_handler))
        .route("/:id", get(get_market_handler))
        .route("/pnl/:wallet", get(get_pnl_handler))
        .route("/force-create", post(force_create_market_handler))   // DEV ONLY
}


/// ---------------------------------------------------------------------------
/// GET /market/active
/// ---------------------------------------------------------------------------
async fn get_active_markets_handler(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    match get_active_markets(&state.pool).await {
        Ok(markets) => Json(json!(markets)),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

/// ---------------------------------------------------------------------------
/// GET /market/:id
/// ---------------------------------------------------------------------------
async fn get_market_handler(
    Path(id): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    match get_market_from_db(&state.pool, id).await {
        Ok(market) => Json(json!(market)),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

/// ---------------------------------------------------------------------------
/// GET /market/pnl/:wallet
/// ---------------------------------------------------------------------------
async fn get_pnl_handler(
    Path(wallet): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    match get_user_pnl(&state.pool, &wallet).await {
        Ok(stats) => Json(json!(stats)),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}


/// ---------------------------------------------------------------------------
/// POST /market/force-create (DEV ONLY)
/// ---------------------------------------------------------------------------
async fn force_create_market_handler(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    
    let asset = "BTC/USDT";
    tracing::info!("[FORCE CREATE] Starting forced market creation...");

    // 1. Oracle fetch
    let candle_res = get_latest_candle(4).await;
    if let Err(e) = candle_res {
        tracing::error!("[FORCE CREATE] Oracle error: {:?}", e);
        return Json(json!({ "ok": false, "error": e.to_string() }));
    }
    let candle = candle_res.unwrap();
    let open_price = candle.open;

    // 2. Compute times
    let start_time = candle.timestamp as i64;
    let end_time = start_time + 4 * 3600;
    let lock_time = end_time - 600;

    // ------------------------------------------------------------
    // 3. Compute REAL market_id used on-chain (match scheduler)
    // ------------------------------------------------------------
    let market_id = start_time;   // UNIQUE candle-based ID

    // 4. Insert into DB (returns DB row id)
    let db_row_id = match insert_market(
        &state.pool,
        market_id,                                    // FIXED
        asset,
        Utc.timestamp_opt(start_time, 0).unwrap(),
        Utc.timestamp_opt(end_time, 0).unwrap(),
        Utc.timestamp_opt(lock_time, 0).unwrap(),
        open_price,
    )
    .await {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("[FORCE CREATE] DB insert error: {:?}", e);
            return Json(json!({ "ok": false, "error": e.to_string() }));
        }
    };

    tracing::info!(
        "[FORCE CREATE] DB Row ID = {}, market_id = {}",
        db_row_id,
        market_id
    );

    // 5. Call Solana create_market
    let sol = state.sol.clone();
    let on_chain_price = (open_price * 100.0) as u64;

    let sol_call = tokio::task::spawn_blocking(move || {
        sol.create_market_and_send(
            on_chain_price,
            start_time,
            end_time,
            market_id as u64,    // Correct value for program
        )
    })
    .await;

    match sol_call {
        Ok(Ok(tx)) => {
            tracing::info!(
                "[FORCE CREATE] On-chain create success: market_id={} tx={}",
                market_id,
                tx
            );

            Json(json!({
                "ok": true,
                "tx": tx,
                "market_id": market_id,
                "db_row_id": db_row_id
            }))
        }
        Ok(Err(e)) => {
            tracing::error!("[FORCE CREATE] On-chain error: {:?}", e);
            Json(json!({ "ok": false, "error": e.to_string() }))   // FIXED
        }
        Err(e) => {
            tracing::error!("[FORCE CREATE] spawn_blocking error: {:?}", e);
            Json(json!({ "ok": false, "error": e.to_string() }))
        }
    }
}
