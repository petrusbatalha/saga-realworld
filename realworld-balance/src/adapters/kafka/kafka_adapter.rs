use crate::application::ports::port_in::transference_consumer::TransferenceEvent;
use async_trait::async_trait;
use flume::{Receiver, Sender};
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::{ClientConfig, Message};
use realworld_shared::structs::{TransferenceStatus, Transfer};
use std::sync::Arc;
use std::time::Duration;

const KAFKA_RESPONSE_TOPIC: &str = "enge-corp-transac-response";
const KAFKA_COMPENSATION_TOPIC: &str = "enge-corp-transac-compensation";
const KAFKA_REQUEST_TOPIC: &str = "enge-corp-transac-request";

#[derive(Clone)]
pub(crate) struct KafkaAdapter {
    pub future_producer: Arc<FutureProducer>,
    pub stream_consumer: Arc<StreamConsumer>,
}

impl KafkaAdapter {
    pub async fn new(kafka_config: ClientConfig) -> KafkaAdapter {
        let future_producer: Arc<FutureProducer> = Arc::new(kafka_config.create().unwrap());
        let stream_consumer: Arc<StreamConsumer> = Arc::new(kafka_config.create().unwrap());
        KafkaAdapter {
            future_producer,
            stream_consumer,
        }
    }
}

#[async_trait]
impl TransferenceEvent for KafkaAdapter {
    async fn consume(&self, sender: Sender<Transfer>) {
        self.stream_consumer
            .subscribe(&[KAFKA_COMPENSATION_TOPIC, KAFKA_REQUEST_TOPIC])
            .expect("Can't subscribe to transactions topics. ERR");
        println!("Start consumer");

        loop {
            match self.stream_consumer.recv().await {
                Err(e) => println!("Kafka error: {}", e),
                Ok(m) => {
                    let payload = match m.payload_view::<str>() {
                        None => "",
                        Some(Ok(t)) => t,
                        Some(Err(_)) => "",
                    };

                    let transfer: Transfer = serde_json::from_str(payload).unwrap();

                    match sender.send(transfer) {
                        Ok(_) => {}
                        Err(e) => println!("Failed to send status: {}", e),
                    };

                    self.stream_consumer
                        .commit_message(&m, CommitMode::Async)
                        .unwrap();
                }
            };
        }
    }

    async fn notify(&self, receiver: Receiver<TransferenceStatus>) {
        loop {
            match receiver.try_recv() {
                Ok(transaction_event) => {
                    let transference = serde_json::to_string(&transaction_event).unwrap();

                    let transference_record: FutureRecord<String, String> =
                        FutureRecord::to(KAFKA_RESPONSE_TOPIC).payload(&transference);

                    let transference_future = self
                        .future_producer
                        .send(transference_record, Duration::from_secs(1));

                    match transference_future.await {
                        Ok(_) => Ok(()),
                        Err(e) => Err(anyhow::Error::from(e.0)),
                    }
                    .unwrap();
                }
                _ => {}
            }
        }
    }
}
