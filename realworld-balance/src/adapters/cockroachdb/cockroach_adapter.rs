use crate::application::ports::port_out::transaction_store::TransactionStore;
use anyhow::Result;
use async_trait::async_trait;
use realworld_shared::structs::Transaction;
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
}

#[async_trait]
impl TransactionStore for CockroachAdapter {
    async fn add_transaction_update_balance(&self, transaction: &Transaction) -> Result<()> {
        println!("Add transaction balance");

        self.pool.begin().await?;

        sqlx::query!(
            r#"INSERT INTO transactions(id, amount, account, parent_id) VALUES ($1, $2, $3, $4);"#,
            transaction.transaction_id.to_string(),
            transaction.amount,
            transaction.account.to_string(),
            transaction.parent_id.unwrap().to_string(),
        )
        .execute(&*self.pool)
        .await?;

        sqlx::query(r#"UPDATE balance SET balance = balance + $1 WHERE account_id = $2;"#)
            .bind(transaction.amount)
            .bind(transaction.account)
            .execute(&*self.pool)
            .await?;

        Ok(())
    }

    async fn get_account(&self, account_id: i64) -> Result<f64> {
        println!("Get account.");
        let balance = sqlx::query(r#"SELECT balance FROM balance WHERE account_id=$1;"#)
            .bind(account_id)
            .fetch_one(&*self.pool)
            .await?;
        Ok(balance.get(0))
    }
}
