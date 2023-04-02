use crate::IngestData;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde_json::json;

pub async fn send_to_kafka(ingest_data: IngestData) -> Result<(), rdkafka::error::KafkaError> {
    let bootstrap_servers = std::env::var("KAFKA_BOOTSTRAP_SERVERS")
        .expect("KAFKA_BOOTSTRAP_SERVERS must be set");

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &bootstrap_servers)
        .set("message.timeout.ms", "5000")
        .create()?;

    let message = json!({
        "organization_id": ingest_data.organization_id,
        "metadata": ingest_data.metadata
    });

    producer.send(
        FutureRecord::to("customer-transactions")
            .payload(&message.to_string())
            .key(""),
        rdkafka::util::Timeout::Never,
    ).await.map(|_| ()).map_err(|(err, _)| err)
    
}
