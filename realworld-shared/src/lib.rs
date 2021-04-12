pub mod kafka_adapter {
    use rdkafka::ClientConfig;
    use std::env;

    const KAFKA_CONFIG: &str = "KAFKA";

    pub fn build_config() -> ClientConfig {
        let mut kafka_config = ClientConfig::new();

        for (config_key, config_value) in env::vars_os() {
            if let (Ok(key), Ok(value)) = (config_key.into_string(), config_value.into_string()) {
                match key.contains(KAFKA_CONFIG) {
                    true => {
                        kafka_config.set(
                            &key.replace("_", ".").replace("KAFKA.", "").to_lowercase(),
                            &value,
                        );
                    }
                    _ => {}
                }
            }
        }
        kafka_config
    }
}

pub mod cockroach_adapter {
    use sqlx::postgres::PgPoolOptions;
    use sqlx::{Pool, Postgres};
    use std::env;

    const DB_HOST_ENV: &str = "DATABASE_URL";
    const DB_MAX_CONNECTIONS_ENV: &str = "DATABASE_MAX_CONNECTIONS";
    const DB_DEFAULT_MAX_CONNECTIONS: u32 = 5;

    pub async fn build_pool() -> Pool<Postgres> {
        let db_host = env::var(DB_HOST_ENV).expect("DATABASE_URL must be defined.");

        let max_connections = match env::var(DB_MAX_CONNECTIONS_ENV) {
            Ok(v) => v.parse::<u32>().unwrap(),
            Err(_) => DB_DEFAULT_MAX_CONNECTIONS,
        };

        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(&db_host)
            .await
            .expect("Failed to connect to cockroachdb.");

        pool
    }
}

pub mod structs {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Serialize, Deserialize, strum_macros::Display)]
    pub enum TransactionType {
        Withdraw,
        Deposit,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Transaction {
        pub transaction_type: TransactionType,
        pub parent_id: Option<Uuid>,
        pub transaction_id: Uuid,
        pub account: i64,
        pub amount: f64,
    }

    #[derive(Serialize, Deserialize, strum_macros::Display)]
    pub enum Status {
        Completed,
        Pending,
        Failed(String),
    }

    #[derive(Serialize, Deserialize)]
    pub struct TransactionStatus {
        pub status: Status,
        pub account: i64,
        pub id: Uuid,
    }

    impl TransactionStatus {
        pub fn from_transaction(transaction: Transaction, status: Status) -> TransactionStatus {
            TransactionStatus {
                status,
                account: transaction.account,
                id: transaction.transaction_id,
            }
        }
    }
}
