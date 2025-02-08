use axum::{Router, routing::{get, post}};
use crate::handlers::user_handlers::{get_user, create_user, create_trade, get_trades};
use crate::services::AppState;
use std::sync::Arc;

pub fn user_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/users/:id", get(get_user))
        .route("/users", post(create_user))
        .route("/users/:id/trades", post(create_trade))
        .route("/users/:id/trades", get(get_trades))
        .with_state(state)
}
