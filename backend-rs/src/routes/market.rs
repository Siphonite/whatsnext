use axum::{Router, routing::get, extract::Path, Json};
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::sync::Arc;

use crate::solana_client::SolanaClient;
use crate::oracle::get_latest_candle;

// Import the on-chain MarketAccount struct
use candle_markets::state::MarketAccount;

// -------------------------
// JSON RESPONSE STRUCT
// -------------------------
#[derive(Serialize, Deserialize, Debug)]
pub struct MarketResponse {
    pub asset: String,
    pub market_id: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub lock_time: i64,
    pub open_price: u64,
    pub close_price: u64,
    pub green_pool_weighted: u64,
    pub red_pool_weighted: u64,
    pub virtual_liquidity: u64,
    pub settled: bool,
}

// -------------------------
// ROUTER ENTRYPOINT
// -------------------------
pub fn routes(sol: Arc<SolanaClient>) -> Router {
    Router::new()
        .route("/current", {
            let sol = sol.clone();
            get(move || current_market(sol.clone()))
        })
        .route("/:id", {
            let sol = sol.clone();
            get(move |Path(id): Path<u64>| get_market_by_id(sol.clone(), id))
        })
}

// -------------------------
// GET /market/current
// -------------------------
async fn current_market(sol: Arc<SolanaClient>) -> Json<serde_json::Value> {
    let id = match crate::scheduler::load_market_id() {
        Ok(id) if id > 0 => id,
        _ => return Json(json!({"error": "No markets created yet"})),
    };

    get_market_by_id(sol, id).await
}

// -------------------------
// GET /market/:id
// -------------------------
async fn get_market_by_id(sol: Arc<SolanaClient>, id: u64) -> Json<serde_json::Value> {
    let (pda, _bump) = sol.derive_market_pda(id);
    let program = sol.program();

    // attempt to fetch on-chain account
    let account_data: Result<MarketAccount, _> = program.account(pda);

    match account_data {
        Ok(acc) => {
            // Found the on-chain account — return it properly formatted
            Json(json!(MarketResponse {
                asset: acc.asset,
                market_id: acc.market_id,
                start_time: acc.start_time,
                end_time: acc.end_time,
                lock_time: acc.lock_time,
                open_price: acc.open_price,
                close_price: acc.close_price,
                green_pool_weighted: acc.green_pool_weighted,
                red_pool_weighted: acc.red_pool_weighted,
                virtual_liquidity: acc.virtual_liquidity,
                settled: acc.settled,
            }))
        }
        Err(_) => {
            // Fallback to oracle data if account doesn't exist
            match get_latest_candle("BTCUSDT", 4).await {
                Ok(cndl) => Json(json!({
                    "asset": "BTCUSDT",
                    "market_id": id,
                    "start_time": cndl.timestamp,
                    "end_time": cndl.timestamp + 14400,
                    "open_price": (cndl.open * 100.0).round() as u64,
                    "close_price": null,
                    "fallback": true,
                    "message": "On-chain account not found"
                })),
                Err(e) => Json(json!({
                    "error": e.to_string()
                })),
            }
        }
    }
}
