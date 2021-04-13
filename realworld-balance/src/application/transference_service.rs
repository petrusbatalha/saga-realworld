use crate::application::error_msgs;
use crate::application::ports::port_out::transference_store::TransferenceStore;
use flume::{Receiver, Sender};
use realworld_shared::structs::{Status, Transfer, TransferenceStatus};

#[derive(Clone, Debug)]
pub struct TransactionService<T> {
    pub(crate) persist: T,
}

impl<T: 'static + TransferenceStore + std::marker::Sync + Send> TransactionService<T> {
    pub async fn start_transaction_handler(
        &self,
        receiver: Receiver<Transfer>,
        sender: Sender<TransferenceStatus>,
    ) {
        println!("Start transaction handler.");
        loop {
            while let Some(transfer) = receiver.iter().next() {
                match &self.persist.get_account(transfer.withdraw_transaction.account).await {
                    Ok(account) => match account >= &transfer.withdraw_transaction.amount {
                        true => {
                            println!("WTF account {}", account);
                            match self
                                .persist
                                .add_transaction_update_balance(&transfer)
                                .await
                            {
                                Ok(_) => {
                                    sender
                                        .send(TransferenceStatus::from_transference(
                                            transfer,
                                            Status::Completed,
                                        ))
                                        .unwrap();
                                }
                                Err(e) => {
                                    sender
                                        .send(TransferenceStatus::from_transference(
                                            transfer,
                                            Status::Failed(e.to_string()),
                                        ))
                                        .unwrap();
                                }
                            };
                        }
                        false => {
                            sender
                                .send(TransferenceStatus::from_transference(
                                    transfer,
                                    Status::Failed(error_msgs::INSUFFICIENT_BALANCE.to_string()),
                                ))
                                .unwrap();
                        }
                    },
                    Err(_) => {
                        sender
                            .send(TransferenceStatus::from_transference(
                                transfer,
                                Status::Failed(error_msgs::ACCOUNT_DOESNT_EXIST.to_string()),
                            ))
                            .unwrap();
                    }
                }
            }
        }
    }
}
