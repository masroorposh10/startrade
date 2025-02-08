use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Bot {
    pub id: i32,
    pub name: String,
    pub strategy: String,
}

impl Bot {
    pub fn new(id: i32, name: String, strategy: String) -> Self {
        Bot { id, name, strategy }
    }
}
