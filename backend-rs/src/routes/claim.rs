use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use crate::state::AppState;
use crate::repository::{compute_user_payout, mark_bet_claimed, record_payout};

//
// ----------------------------------------------------------
//  GET /claimable/:market_id/:wallet
// ----------------------------------------------------------
//  Returns:
//  { ok: true, claimable: bool, payout: lamports }
// ----------------------------------------------------------
//
pub async fn get_claimable_handler(
    Path((market_id, wallet)): Path<(i64, String)>,
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    match compute_user_payout(&state.pool, &wallet, market_id).await {
        Ok(payout) => {
            let claimable = payout > 0;
            Json(json!({
                "ok": true,
                "claimable": claimable,
                "payout": payout
            }))
        }
        Err(e) => Json(json!({
            "ok": false,
            "error": e.to_string()
        })),
    }
}

//
// ----------------------------------------------------------
//  POST /claim/record
// ----------------------------------------------------------
//  Body:
//  {
//      "market_id": number,
//      "wallet": "wallet_pubkey",
//      "tx_sig": "transaction_signature"
//  }
//
//  After user signs and submits on-chain claim transaction,
//  backend records payout + marks bet claimed.
// ----------------------------------------------------------
//
#[derive(Debug, Deserialize)]
pub struct ClaimRecordBody {
    pub market_id: i64,
    pub wallet: String,
    pub tx_sig: String,
}

pub async fn post_claim_record_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<ClaimRecordBody>,
) -> Json<serde_json::Value> {
    // Compute payout to store in DB (same formula as on-chain)
    let payout = compute_user_payout(&state.pool, &body.wallet, body.market_id)
        .await
        .unwrap_or(0);

    // Mark bet claimed in DB
    let _ = mark_bet_claimed(&state.pool, &body.wallet, body.market_id, payout).await;

    // Record payout entry (optional but recommended)
    let _ = record_payout(&state.pool, &body.wallet, body.market_id, payout, &body.tx_sig).await;

    Json(json!({
        "ok": true,
        "payout": payout,
        "tx_sig": body.tx_sig
    }))
}

//
// Router for claim endpoints
//
pub fn routes() -> axum::Router<Arc<AppState>> {
    use axum::routing::{get, post};

    axum::Router::new()
        .route("/claimable/:market_id/:wallet", get(get_claimable_handler))
        .route("/claim/record", post(post_claim_record_handler))
}
