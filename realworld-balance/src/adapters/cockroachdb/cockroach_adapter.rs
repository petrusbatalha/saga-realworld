use crate::application::ports::port_out::transference_store::TransferenceStore;
use anyhow::Result;
use async_trait::async_trait;
use realworld_shared::structs::{TransactionType, Transfer, Transaction};
use sqlx::{Pool, Postgres, Row};
use std::sync::Arc;

#[derive(Clone)]
pub struct CockroachAdapter {
    pool: Arc<Pool<Postgres>>,
}

impl CockroachAdapter {
    pub async fn new(pool: Arc<Pool<Postgres>>) -> CockroachAdapter {
        CockroachAdapter { pool }
    }
    pub async fn persist_transaction(&self, transaction: &Transaction, id: String) -> Result<()> {
        match transaction.transaction_type {
            TransactionType::Withdraw => -amount,
            TransactionType::Deposit =>
        };
        sqlx::query( r#"INSERT INTO transactions(id, amount, account, transfer_id) VALUES ($1, $2, $3, $4);"#)
            .bind(transaction.transaction_id.to_string())
            .bind(transaction.amount)
            .bind(transaction.account.to_string())
            .bind(id)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }
    pub async fn update_balance(&self, transaction: &Transaction) -> Result<()> {
        let update_query = match transaction.transaction_type {
            TransactionType::Withdraw => r#"UPDATE balance SET balance = balance - $1 WHERE account_id = $2;"#,
            TransactionType::Deposit => r#"UPDATE balance SET balance = balance + $1 WHERE account_id = $2;"#,
        };
        sqlx::query(update_query)
            .bind(transaction.amount)
            .bind(transaction.account)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl TransferenceStore for CockroachAdapter {
    async fn add_transaction_update_balance(&self, transfer: &Transfer) -> Result<()> {
        self.pool.begin().await?;
        let id = transfer.transfer_id;
        let deposit_transaction= &transfer.deposit_transaction;
        let withdraw_transaction = &transfer.withdraw_transaction;

        let deposit_future = self.persist_transaction(deposit_transaction, id.to_string());
        let withdraw_future = self.persist_transaction(withdraw_transaction, id.to_string());
        let source_balance = self.update_balance(withdraw_transaction);
        let target_balance = self.update_balance(deposit_transaction);

        tokio::try_join!(deposit_future, withdraw_future, source_balance, target_balance)?;
        Ok(())
    }

    async fn get_account(&self, account_id: i64) -> Result<f64> {
        let balance = sqlx::query(r#"SELECT balance FROM balance WHERE account_id=$1;"#)
            .bind(account_id)
            .fetch_one(&*self.pool)
            .await?;
        Ok(balance.get(0))
    }
}
