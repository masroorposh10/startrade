use axum::{Router, routing::post};
use crate::handlers::bot_handlers::create_bot;
use crate::services::AppState;
use std::sync::Arc;

pub fn bot_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/users/:id/bots", post(create_bot))
        .with_state(state)
}
