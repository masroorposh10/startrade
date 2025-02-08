use axum::{Router, routing::get, Json};
use crate::services::{market_data::fetch_binance_price, ai_service::get_ai_recommendation};
use serde_json::json;

pub async fn get_ai_trade_decision() -> Json<String> {
    if let Ok(price_data) = fetch_binance_price("BTCUSDT").await {
        if let Ok(ai_response) = get_ai_recommendation(&format!("BTC price is ${}", price_data.price)).await {
            if let Some(choice) = ai_response["choices"][0]["message"]["content"].as_str() {
                return Json(choice.to_string());
            }
        }
    }
    Json("Failed to get AI recommendation".to_string())
}

pub fn ai_routes() -> Router {
    Router::new().route("/ai/trade", get(get_ai_trade_decision))
}
