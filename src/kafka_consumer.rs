use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use std::env;
use futures::TryStreamExt;

pub async fn consume_kafka_messages() {
    let bootstrap_servers = env::var("KAFKA_BOOTSTRAP_SERVERS").unwrap_or_else(|_| "localhost:9092".to_string());
    let topic = "customer-transactions";

    tokio::spawn(async move {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", "rust-kafka-consumer")
            .set("bootstrap.servers", &bootstrap_servers)
            .set("enable.auto.commit", "false")
            .set("auto.offset.reset", "earliest")
            .create()
            .expect("Consumer creation failed");

        consumer
            .subscribe(&[&topic])
            .expect("Failed to subscribe to topic");

        let message_stream = consumer.stream();
        
        message_stream.try_for_each(|message| async move {
            let payload = match message.payload_view::<str>() {
                Some(Ok(payload)) => payload,
                Some(Err(_)) => "[INVALID UTF-8]",
                None => "[EMPTY MESSAGE]",
            };

            println!(
                "Received message from Kafka: topic={}, partition={}, offset={}, payload={}",
                message.topic(),
                message.partition(),
                message.offset(),
                payload
            );

            Ok(())
        }).await.unwrap_or_else(|_| {
            eprintln!("Error while processing Kafka messages");
        });
    });
}
