use axum::{
    extract::{State, Path, Json},
    Router,
    http::StatusCode,
};
use uuid::Uuid;
use serde_json::json;

use crate::state::AppState;
use crate::models::bot::Bot;
use axum::routing::post;

/// POST /users/:id/bots - Create a new bot for a given user.
/// Expects JSON payload: { "name": "Bot Name", "strategy": "Mean Reversion" }
async fn create_bot(
    Path(user_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<BotCreateRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    // Check if user exists
    let users = state.users.lock().await;
    if users.iter().find(|u| u.id == user_id).is_none() {
        return (StatusCode::NOT_FOUND, Json(json!({"error": "User not found"})));
    }
    drop(users); // release the lock before locking bots

    let bot = Bot::new(user_id, payload.name, payload.strategy);
    state.bots.lock().await.push(bot.clone());
    (StatusCode::CREATED, Json(json!({ "bot": bot })))
}

/// Request payload for creating a bot.
#[derive(serde::Deserialize)]
pub struct BotCreateRequest {
    pub name: String,
    pub strategy: String,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/users/:id/bots", post(create_bot))
}
