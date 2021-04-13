use anyhow::Result;
use async_trait::async_trait;
use realworld_shared::structs::Transfer;

#[async_trait]
pub trait TransferenceStore {
    async fn add_transaction_update_balance(&self, transfer: &Transfer) -> Result<()>;
    async fn get_account(&self, account_id: i64) -> Result<f64>;
}
