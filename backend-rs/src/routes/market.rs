use axum::{
    Router,
    routing::{get, post},
    extract::{Path, State},
    Json,
};
use serde_json::json;
use std::sync::Arc;

use chrono::TimeZone;
use chrono::Utc;
use std::fs;
use std::path::Path as FsPath;

use crate::state::AppState;
use crate::repository::{get_market_from_db, get_active_markets, get_user_pnl, insert_market};
use crate::oracle::get_latest_candle;
use crate::config::MARKET_ASSET;

/// local file path for market id (same as scheduler)
const MARKET_ID_PATH: &str = "market_id.txt";

/// Routes for market-related data
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/active", get(get_active_markets_handler))   // GET active BTC/USDT markets
        .route("/:id", get(get_market_handler))              // GET details for a specific market
        .route("/pnl/:wallet", get(get_pnl_handler))         // GET user PnL
        .route("/force-create", post(force_create_market_handler)) // TEMP: Force-create a market
}

/// small helper: read market id file (default 0)
fn load_market_id_from_file() -> Result<u64, anyhow::Error> {
    if !FsPath::new(MARKET_ID_PATH).exists() {
        fs::write(MARKET_ID_PATH, "0")?;
    }
    let txt = fs::read_to_string(MARKET_ID_PATH)?;
    let id = txt.trim().parse::<u64>().unwrap_or(0);
    Ok(id)
}

/// small helper: save market id file
fn save_market_id_to_file(id: u64) -> Result<(), anyhow::Error> {
    fs::write(MARKET_ID_PATH, id.to_string())?;
    Ok(())
}

/// GET /market/active
async fn get_active_markets_handler(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    match get_active_markets(&state.pool).await {
        Ok(markets) => Json(json!(markets)),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

/// GET /market/:id
async fn get_market_handler(
    Path(id): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    match get_market_from_db(&state.pool, id).await {
        Ok(market) => Json(json!(market)),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

/// GET /market/pnl/:wallet
async fn get_pnl_handler(
    Path(wallet): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    match get_user_pnl(&state.pool, &wallet).await {
        Ok(stats) => Json(json!(stats)),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

/// POST /market/force-create
/// TEMPORARY DEV ENDPOINT — creates a market immediately so frontend can test bets.
/// This replicates the scheduler's create_market_job behavior.
async fn force_create_market_handler(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let asset = MARKET_ASSET; // should be "BTC/USDT"
    tracing::info!("Force-create market called for asset {}", asset);

    // 1) Fetch oracle data (4H candle)
    let candle_res = get_latest_candle(4).await;
    if let Err(e) = candle_res {
        tracing::error!("Oracle fetch failed: {:?}", e);
        return Json(json!({ "ok": false, "error": format!("Oracle error: {:?}", e) }));
    }
    let candle = candle_res.unwrap();
    let open_price = candle.open;

    // 2) Compute times (same as scheduler)
    let start_time = candle.timestamp as i64;
    let end_time = start_time + (4 * 3600); // 4 hours
    let lock_time = end_time - (10 * 60);    // 10 minutes before close

    // 3) Generate unique ID (file-based counter)
    let mut id = match load_market_id_from_file() {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("Failed to load market_id file: {:?}", e);
            return Json(json!({ "ok": false, "error": format!("market_id load error: {:?}", e) }));
        }
    };
    id += 1;
    if let Err(e) = save_market_id_to_file(id) {
        tracing::error!("Failed to save market_id file: {:?}", e);
        return Json(json!({ "ok": false, "error": format!("market_id save error: {:?}", e) }));
    }

    tracing::info!(
        "Force-creating BTC market {} | Open: {} | Start: {}",
        id,
        open_price,
        start_time
    );

    // 4) Submit to Solana in blocking task (create_market_and_send is sync)
    let sol_clone = state.sol.clone();
    let pool_clone = state.pool.clone();
    let asset_static = asset; // copy for closure

    // scale price for on-chain units (float -> integer)
    let on_chain_price = (open_price * 100.0) as u64;
    let start_time_local = start_time;
    let end_time_local = end_time;
    let id_local = id;

    let spawn_res = tokio::task::spawn_blocking(move || {
        sol_clone.create_market_and_send(on_chain_price, start_time_local, end_time_local, id_local)
    })
    .await;

    match spawn_res {
        Ok(Ok(sig)) => {
            tracing::info!("Market {} confirmed on-chain. Tx: {}", id, sig);

            // 5) Save to Database (reuse insert_market)
            match insert_market(
                &pool_clone,
                id as i64,
                asset_static,
                Utc.timestamp_opt(start_time_local, 0).unwrap(),
                Utc.timestamp_opt(end_time_local, 0).unwrap(),
                Utc.timestamp_opt(lock_time, 0).unwrap(),
                open_price,
            ).await {
                Ok(_) => {
                    tracing::info!("Market {} saved to DB", id);
                    Json(json!({ "ok": true, "tx": sig, "market_id": id }))
                }
                Err(e) => {
                    tracing::error!("DB Insert failed for {}: {:?}", id, e);
                    Json(json!({ "ok": false, "error": format!("DB insert failed: {:?}", e) }))
                }
            }
        }
        Ok(Err(e)) => {
            tracing::error!("On-chain creation failed for {}: {:?}", id, e);
            Json(json!({ "ok": false, "error": format!("On-chain creation failed: {:?}", e) }))
        }
        Err(e) => {
            tracing::error!("Spawn blocking error: {:?}", e);
            Json(json!({ "ok": false, "error": format!("Spawn error: {:?}", e) }))
        }
    }
}
