use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Trade {
    pub id: i32,
    pub symbol: String,
    pub amount: f64,
    pub price: f64,
}

impl Trade {
    pub fn new(id: i32, symbol: String, amount: f64, price: f64) -> Self {
        Trade { id, symbol, amount, price }
    }
}
