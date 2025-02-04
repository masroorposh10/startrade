use std::sync::Arc;
use tokio::sync::Mutex;

use crate::models::user::User;
use crate::models::bot::Bot;
use crate::models::trade::Trade;

#[derive(Clone, Default)]
pub struct AppState {
    pub users: Arc<Mutex<Vec<User>>>,
    pub bots: Arc<Mutex<Vec<Bot>>>,
    pub trades: Arc<Mutex<Vec<Trade>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(Vec::new())),
            bots: Arc::new(Mutex::new(Vec::new())),
            trades: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
