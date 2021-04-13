use anyhow::Result;
use async_trait::async_trait;
use realworld_shared::structs::Transaction;

#[async_trait]
pub trait TransactionStore {
    async fn add_transaction_update_balance(&self, transaction: &Transaction) -> Result<()>;
    async fn get_account(&self, account_id: i64) -> Result<f64>;
}
