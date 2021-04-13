use crate::application::error_msgs;
use crate::application::ports::port_out::transaction_store::TransactionStore;
use flume::{Receiver, Sender, TryRecvError};
use realworld_shared::structs::{Status, Transaction, TransactionStatus};

#[derive(Clone, Debug)]
pub struct TransactionService<T> {
    pub(crate) persist: T,
}

impl<T: 'static + TransactionStore + std::marker::Sync + Send> TransactionService<T> {
    pub async fn start_transaction_handler(
        &self,
        receiver: Receiver<Transaction>,
        sender: Sender<TransactionStatus>,
    ) {
        println!("Start transaction handler.");
        loop {
            while let Some(transaction) = receiver.iter().next() {
                match &self.persist.get_account(transaction.account).await {
                    Ok(account) => match account >= &transaction.amount {
                        true => {
                            println!("WTF account {}", account);
                            match self
                                .persist
                                .add_transaction_update_balance(&transaction)
                                .await
                            {
                                Ok(_) => {
                                    sender
                                        .send(TransactionStatus::from_transaction(
                                            transaction,
                                            Status::Completed,
                                        ))
                                        .unwrap();
                                }
                                Err(e) => {
                                    sender
                                        .send(TransactionStatus::from_transaction(
                                            transaction,
                                            Status::Failed(e.to_string()),
                                        ))
                                        .unwrap();
                                }
                            };
                        }
                        false => {
                            sender
                                .send(TransactionStatus::from_transaction(
                                    transaction,
                                    Status::Failed(error_msgs::INSUFFICIENT_BALANCE.to_string()),
                                ))
                                .unwrap();
                        }
                    },
                    Err(_) => {
                        sender
                            .send(TransactionStatus::from_transaction(
                                transaction,
                                Status::Failed(error_msgs::ACCOUNT_DOESNT_EXIST.to_string()),
                            ))
                            .unwrap();
                    }
                }
            }
        }
    }
}
