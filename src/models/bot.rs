use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bot {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub strategy: String,
}

impl Bot {
    pub fn new(user_id: Uuid, name: String, strategy: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            name,
            strategy,
        }
    }
}
