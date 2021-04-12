use crate::application::domain::accounts::Account;
use crate::application::error_msgs;
use crate::application::ports::port_out::transaction_store::TransactionStore;
use anyhow::{Error, Result};
use flume::{Receiver, Sender};
use realworld_shared::structs::{Status, Transaction, TransactionStatus};
use std::thread;

#[derive(Clone, Debug)]
pub struct TransactionService<T> {
    pub receiver: Receiver<Transaction>,
    pub request_receiver: Receiver<Transaction>,
    pub sender: Sender<TransactionStatus>,
    pub persist: T,
}

impl<T: 'static + TransactionStore + std::marker::Sync + Send> TransactionService<T> {
    pub async fn compensation_receiver(&self) -> Result<()> {
        unimplemented!();
    }
    pub async fn start_receiver_thread(self) -> Result<()> {
        thread::spawn(async move || {
            for transaction in self.request_receiver.recv() {
                match &self.persist.get_account(transaction.account).await {
                    Ok(account) => match account.has_balance(transaction.amount) {
                        true => {
                            match self
                                .persist
                                .add_transaction_update_balance(&transaction)
                                .await
                            {
                                Ok(_) => {
                                    self.sender
                                        .send(TransactionStatus::from_transaction(
                                            transaction,
                                            Status::Completed,
                                        ))
                                        .unwrap();
                                }
                                Err(e) => {
                                    self.sender
                                        .send(TransactionStatus::from_transaction(
                                            transaction,
                                            Status::Failed(e.to_string()),
                                        ))
                                        .unwrap();
                                }
                            };
                        }
                        false => {
                            self.sender
                                .send(TransactionStatus::from_transaction(
                                    transaction,
                                    Status::Failed(error_msgs::INSUFFICIENT_BALANCE.to_string()),
                                ))
                                .unwrap();
                        }
                    },
                    Err(_) => {
                        self.sender
                            .send(TransactionStatus::from_transaction(
                                transaction,
                                Status::Failed(error_msgs::ACCOUNT_DOESNT_EXIST.to_string()),
                            ))
                            .unwrap();
                    }
                }
            }
        });
        Ok(())
    }
}
