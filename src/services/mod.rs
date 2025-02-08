pub mod ai_service;
pub mod market_data;

use std::sync::Mutex;
use std::collections::HashMap;
use crate::models::user_model::User;
use crate::models::bot_model::Bot;

pub struct AppState {
    pub users: Mutex<HashMap<i32, User>>,
    pub bots: Mutex<HashMap<i32, Bot>>,
}
