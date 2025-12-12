use axum::{
    extract::State,
    Json,
};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use crate::state::AppState;

//
// ----------------------------------------------------------
//  POST /treasury/init
// ----------------------------------------------------------
//  Initializes the treasury PDA ONCE per deployment.
// ----------------------------------------------------------
//
pub async fn init_treasury_handler(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let sol = state.sol.clone();

    let result = tokio::task::spawn_blocking(move || {
        sol.initialize_treasury_and_send()
    }).await;

    match result {
        Ok(Ok(sig)) => Json(json!({ "ok": true, "tx": sig })),
        Ok(Err(e))  => Json(json!({ "ok": false, "error": e.to_string() })),
        Err(e)      => Json(json!({ "ok": false, "error": format!("{:?}", e) })),
    }
}

//
// ----------------------------------------------------------
//  POST /treasury/fund
// ----------------------------------------------------------
//  Body:
//  { "lamports": number }
//
//  Admin loads lamports into the treasury PDA.
// ----------------------------------------------------------
//
#[derive(Debug, Deserialize)]
pub struct FundBody {
    pub lamports: u64,
}

pub async fn fund_treasury_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<FundBody>,
) -> Json<serde_json::Value> {
    let sol = state.sol.clone();

    let result = tokio::task::spawn_blocking(move || {
        sol.fund_treasury_and_send(body.lamports)
    }).await;

    match result {
        Ok(Ok(sig)) => Json(json!({ "ok": true, "tx": sig })),
        Ok(Err(e))  => Json(json!({ "ok": false, "error": e.to_string() })),
        Err(e)      => Json(json!({ "ok": false, "error": format!("{:?}", e) })),
    }
}

//
// Router for treasury endpoints
//
pub fn treasury_routes() -> axum::Router<Arc<AppState>> {
    use axum::routing::post;

    axum::Router::new()
        .route("/treasury/init", post(init_treasury_handler))
        .route("/treasury/fund", post(fund_treasury_handler))
}
