use axum::{
    extract::{Path, State},
    Json,
    response::IntoResponse,
};
use serde_json::{json, Value};
use crate::models::user_model::User;
use crate::models::trade_model::Trade;
use crate::services::AppState;
use std::sync::Arc;

// GET /api/users/:id
pub async fn get_user(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let users = state.users.lock().unwrap();
    match users.get(&id) {
        Some(user) => Json(json!(user)),
        None => Json(json!({"error": "User not found"})),
    }
}

// POST /api/users
// Notice: State extractor is placed first
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(user): Json<User>,
) -> impl IntoResponse {
    let mut users = state.users.lock().unwrap();
    users.insert(user.id, user.clone());
    Json(json!({"message": "User created successfully", "user": user}))
}

// POST /api/users/:id/trades
pub async fn create_trade(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
    Json(trade): Json<Trade>,
) -> impl IntoResponse {
    let mut users = state.users.lock().unwrap();
    if let Some(user) = users.get_mut(&id) {
        user.add_trade(trade.clone());
        Json(json!({"message": "Trade added", "trade": trade}))
    } else {
        Json(json!({"error": "User not found"}))
    }
}

// GET /api/users/:id/trades
pub async fn get_trades(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let users = state.users.lock().unwrap();
    match users.get(&id) {
        Some(user) => Json(json!(user.trades)),
        None => Json(json!({"error": "User not found"})),
    }
}
