use serde::{Serialize, Deserialize};
use crate::models::trade_model::Trade;
use crate::models::bot_model::Bot;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub trades: Vec<Trade>,
    pub bots: Vec<Bot>,
}

impl User {
    pub fn new(id: i32, username: String, email: String) -> Self {
        User {
            id,
            username,
            email,
            trades: Vec::new(),
            bots: Vec::new(),
        }
    }

    pub fn add_trade(&mut self, trade: Trade) {
        self.trades.push(trade);
    }

    pub fn add_bot(&mut self, bot: Bot) {
        self.bots.push(bot);
    }
}
