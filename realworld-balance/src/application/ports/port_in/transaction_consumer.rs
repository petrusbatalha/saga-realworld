use anyhow::Result;
use async_trait::async_trait;
use flume::{Receiver, Sender};
use realworld_shared::structs::{Transaction, TransactionStatus};

#[async_trait]
pub trait TransactionEvent {
    async fn consume(&self, sender: Sender<Transaction>);
    async fn notify(&self, receiver: Receiver<TransactionStatus>);
}
