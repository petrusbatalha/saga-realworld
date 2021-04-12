use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct Account {
    id: Uuid,
    balance: f64,
    last_update: u64,
}

impl Account {
    pub fn has_balance(&self, amount: f64) -> bool {
        self.balance >= amount
    }
}