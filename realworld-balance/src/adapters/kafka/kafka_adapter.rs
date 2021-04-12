use rdkafka::consumer::StreamConsumer;
use rdkafka::producer::FutureProducer;
use rdkafka::ClientConfig;
use std::sync::Arc;

const KAFKA_RESPONSE_TOPIC: &str = "enge-corp-transac-response";
const KAFKA_COMPENSATION_TOPIC: &str = "enge-corp-transac-compensation";
const KAFKA_REQUEST_TOPIC: &str = "enge-corp-transac-request";

#[derive(Clone)]
pub(crate) struct KafkaAdapter {
    pub response_topic: String,
    pub request_topic: String,
    pub compensation_topic: String,
    pub future_producer: Arc<FutureProducer>,
    pub stream_consumer: Arc<StreamConsumer>,
}

impl KafkaAdapter {
    pub async fn new(kafka_config: ClientConfig) -> KafkaAdapter {
        let future_producer: Arc<FutureProducer> = Arc::new(kafka_config.create().unwrap());
        let stream_consumer: Arc<StreamConsumer> = Arc::new(kafka_config.create().unwrap());
        KafkaAdapter {
            response_topic: KAFKA_RESPONSE_TOPIC.to_string(),
            request_topic: KAFKA_REQUEST_TOPIC.to_string(),
            compensation_topic: KAFKA_COMPENSATION_TOPIC.to_string(),
            future_producer,
            stream_consumer,
        }
    }
}
