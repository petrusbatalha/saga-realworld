#![feature(async_closure)]

use crate::adapters::cockroachdb::cockroach_adapter::CockroachAdapter;
use crate::adapters::kafka::kafka_adapter::KafkaAdapter;
use crate::application::ports::port_in::transference_consumer::TransferenceEvent;
use crate::application::transference_service::TransactionService;
use dotenv::dotenv;
use realworld_shared::cockroach_adapter::build_pool;
use realworld_shared::kafka_adapter::build_config;
use std::sync::Arc;

pub mod adapters;
pub mod application;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let kafka_config = build_config();
    let pool = Arc::new(build_pool().await);

    let kafka_adapter = KafkaAdapter::new(kafka_config).await;
    let cockroach_adapter = CockroachAdapter::new(pool);

    let transaction_service = TransactionService {
        persist: cockroach_adapter.await,
    };
    let kafka_adapter_consumer = kafka_adapter.clone();

    let (transaction_sender, transaction_receiver) = flume::unbounded();
    let (status_sender, status_receiver) = flume::unbounded();

    tokio::spawn(async move {
        kafka_adapter.consume(transaction_sender.clone()).await;
    });
    tokio::spawn(async move {
        kafka_adapter_consumer.notify(status_receiver.clone()).await;
    });
    tokio::spawn(async move {
        transaction_service
            .start_transaction_handler(transaction_receiver.clone(), status_sender.clone())
            .await;
    });
    loop {}
}
