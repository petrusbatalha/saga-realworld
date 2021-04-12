use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TransactionStore {
    fn add_transaction() -> Result<>;
    fn update_balance() -> Result<>;
}
