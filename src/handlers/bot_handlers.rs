use axum::{
    extract::{Path, State},
    Json,
    response::IntoResponse,
};
use serde_json::{json};
use crate::models::bot_model::Bot;
use crate::services::AppState;
use std::sync::Arc;

// POST /api/users/:id/bots
pub async fn create_bot(
    Path(user_id): Path<i32>,
    State(state): State<Arc<AppState>>,
    Json(bot): Json<Bot>,
) -> impl IntoResponse {
    let mut users = state.users.lock().unwrap();
    if let Some(user) = users.get_mut(&user_id) {
        user.bots.push(bot.clone());
        Json(json!({"message": "Bot added successfully", "bot": bot}))
    } else {
        Json(json!({"error": "User not found"}))
    }
}
