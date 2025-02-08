use axum::{Router, routing::get, Json};
use crate::services::market_data::fetch_binance_price;
use serde_json::json;

pub async fn get_market_price() -> Json<String> {
    match fetch_binance_price("BTCUSDT").await {
        Ok(price) => Json(format!("Current price for {}: ${}", price.symbol, price.price)),
        Err(_) => Json("Failed to fetch price".to_string()),
    }
}

pub fn market_routes() -> Router {
    Router::new().route("/market/price", get(get_market_price))
}
