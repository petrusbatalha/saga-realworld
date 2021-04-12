use anyhow::Result;
use async_trait::async_trait;
use realworld_shared::structs::Transaction;

#[async_trait]
pub trait TransactionEvent {
    async fn consume(&self) -> Transaction;
    async fn notify(&self) -> Result<()>;
}
