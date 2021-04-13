use async_trait::async_trait;
use flume::{Receiver, Sender};
use realworld_shared::structs::{TransferenceStatus, Transfer};

#[async_trait]
pub trait TransferenceEvent {
    async fn consume(&self, sender: Sender<Transfer>);
    async fn notify(&self, receiver: Receiver<TransferenceStatus>);
}
