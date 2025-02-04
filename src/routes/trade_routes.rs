use axum::{
    extract::{State, Json},
    Router,
    http::StatusCode,
};
use serde_json::json;
use uuid::Uuid;

use crate::state::AppState;
use crate::models::trade::{Trade, TradeType};
use axum::routing::{get, post};

/// GET /trades - List all trades
async fn list_trades(State(state): State<AppState>) -> (StatusCode, Json<Vec<Trade>>) {
    let trades = state.trades.lock().await;
    (StatusCode::OK, Json(trades.clone()))
}

/// POST /trades - Create a new trade.
/// Expects JSON payload:
/// {
///    "bot_id": "uuid-string",
///    "symbol": "AAPL",
///    "quantity": 10.0,
///    "price": 150.0,
///    "trade_type": "Buy" // or "Sell"
/// }
async fn create_trade(
    State(state): State<AppState>,
    Json(payload): Json<TradeCreateRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    // Validate that the bot exists
    let bot_id = payload.bot_id;
    let bots = state.bots.lock().await;
    if bots.iter().find(|b| b.id == bot_id).is_none() {
        return (StatusCode::NOT_FOUND, Json(json!({"error": "Bot not found"})));
    }
    drop(bots);

    // Parse trade type from string (case insensitive)
    let trade_type = match payload.trade_type.to_lowercase().as_str() {
        "buy" => TradeType::Buy,
        "sell" => TradeType::Sell,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid trade type"})),
            )
        }
    };

    let trade = Trade::new(bot_id, payload.symbol, payload.quantity, payload.price, trade_type);
    state.trades.lock().await.push(trade.clone());
    (StatusCode::CREATED, Json(json!({ "trade": trade })))
}

/// Request payload for creating a trade.
#[derive(serde::Deserialize)]
pub struct TradeCreateRequest {
    pub bot_id: Uuid,
    pub symbol: String,
    pub quantity: f64,
    pub price: f64,
    pub trade_type: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/trades", get(list_trades).post(create_trade))
}
