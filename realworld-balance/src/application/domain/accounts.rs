use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Account {
    id: i64,
    balance: f64,
}

impl Account {
    pub fn has_balance(&self, amount: f64) -> bool {
        self.balance >= amount
    }
}
