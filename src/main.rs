use axum::{Router, routing::get, http::StatusCode};
use std::net::SocketAddr;

mod state;
mod routes;
mod models;
use state::AppState;
use routes::{user_routes, bot_routes, trade_routes};

#[tokio::main]
async fn main() {
    // Initialize shared application state (in‑memory storage)
    let state = AppState::new();

    // Build our application with routes.
    let app = Router::new()
        // User routes
        .merge(user_routes::router())
        // Bot routes
        .merge(bot_routes::router())
        // Trade routes
        .merge(trade_routes::router())
        // A health check endpoint
        .route("/health", get(|| async { (StatusCode::OK, "OK") }))
        // Pass state to all routes
        .with_state(state);

    // Run our app
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
