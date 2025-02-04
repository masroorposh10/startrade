use axum::{
    extract::{State, Path},
    Json, Router,
    http::StatusCode,
};
use uuid::Uuid;
use serde_json::json;

use crate::state::AppState;
use crate::models::user::User;
use axum::routing::{get, post};

/// GET /users - List all users
async fn list_users(State(state): State<AppState>) -> (StatusCode, Json<Vec<User>>) {
    let users = state.users.lock().await;
    (StatusCode::OK, Json(users.clone()))
}

/// POST /users - Create a new user
/// Expects JSON payload: { "name": "Alice", "email": "alice@example.com" }
async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<UserCreateRequest>,
) -> (StatusCode, Json<User>) {
    let user = User::new(payload.name, payload.email);
    state.users.lock().await.push(user.clone());
    (StatusCode::CREATED, Json(user))
}

/// GET /users/:id - Get a user by ID
async fn get_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> (StatusCode, Json<serde_json::Value>) {
    let users = state.users.lock().await;
    if let Some(user) = users.iter().find(|u| u.id == id) {
        (StatusCode::OK, Json(json!({ "user": user })))
    } else {
        (StatusCode::NOT_FOUND, Json(json!({ "error": "User not found" })))
    }
}

/// Request payload for creating a user
#[derive(serde::Deserialize)]
pub struct UserCreateRequest {
    pub name: String,
    pub email: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users", get(list_users).post(create_user))
        .route("/users/:id", get(get_user))
}
