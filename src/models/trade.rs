use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Clone, serde::Serialize)]
pub struct Trade {

    pub id: Uuid,

    pub bot_id: Uuid,

    pub symbol: String,

    pub quantity: f64,

    pub price: f64,

    pub trade_type: TradeType,

}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeType {
    Buy,
    Sell,
}

impl Trade {
    pub fn new(bot_id: Uuid, symbol: String, quantity: f64, price: f64, trade_type: TradeType) -> Self {
        Self {
            id: Uuid::new_v4(),
            bot_id,
            symbol,
            quantity,
            price,
            trade_type,
        }
    }
}
