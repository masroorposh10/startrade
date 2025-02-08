mod db;         // Optional, for future database integration
mod handlers;
mod models;
mod routes;
mod services;

use axum::Router;
use routes::{user_routes, bot_routes, market_routes, ai_routes};
use services::AppState;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // Initialize shared application state
    let state = Arc::new(AppState {
        users: Mutex::new(HashMap::new()),
        bots: Mutex::new(HashMap::new()),
    });

    // Merge all the sub-routers into a single router
    let api = Router::new()
        .merge(user_routes::user_routes(state.clone()))
        .merge(bot_routes::bot_routes(state.clone()))
        .merge(market_routes::market_routes())
        .merge(ai_routes::ai_routes());

    // Nest the merged router under the "/api" prefix
    let app = Router::new().nest("/api", api);

    println!("Server running at http://0.0.0.0:3000");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
