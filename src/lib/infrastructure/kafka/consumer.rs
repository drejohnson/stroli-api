use crate::domain::{Action, AppSecrets, KafkaMessage};
use crate::infrastructure::kafka::queries;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message;
use tokio::sync::broadcast;

pub fn create_kafka_consumer_upstash(secrets: &AppSecrets) -> StreamConsumer {
    let url = secrets.kafka_url.clone();
    let user = secrets.kafka_sasl_user.clone();
    let pw = secrets.kafka_sasl_pass.clone();

    ClientConfig::new()
        .set("bootstrap.servers", url)
        .set("sasl.mechanism", "SCRAM-SHA-256")
        .set("security.protocol", "SASL_SSL")
        .set("sasl.username", user)
        .set("sasl.password", pw)
        .set("group.id", "hello")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Consumer creation failed")
}

pub fn create_kafka_consumer(secrets: &AppSecrets) -> StreamConsumer {
    let url = secrets.kafka_url.clone();

    ClientConfig::new()
        .set("group.id", "test")
        .set("bootstrap.servers", url)
        .set("enable.partition.eof", "false")
        .set("allow.auto.create.topics", "true")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("enable.auto.offset.store", "false")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Consumer creation failed")
}

#[tracing::instrument(skip(con))]
pub async fn kafka_consumer_task(
    con: &StreamConsumer,
    db: &surrealdb::Surreal<surrealdb::engine::any::Any>,
    mut shutdown: broadcast::Receiver<()>,
) {
    con.subscribe(&["messages"])
        .expect("Failed to subscribe to topics");

    tracing::warn!("Starting the consumer loop...");

    // Consumer loop
    loop {
        tokio::select! {
            msg = con.recv() => {
                match msg {
                    Ok(m) => {
                        // Process the message
                        if let Some(payload) = m.payload() {
                            let message: KafkaMessage = match serde_json::from_slice(payload) {
                                Ok(msg) => msg,
                                Err(e) => {
                                    tracing::error!("Deserialization error: {}", e);
                                    continue;
                                }
                            };

                            tracing::info!("Received message: {:?}", message);
                            // Here, handle the actions like Create, Update, Delete
                            match message.action {
                                Action::Create => queries::create_message(message, &db).await,
                                Action::Update => queries::update_message(message, &db).await,
                                Action::Delete => queries::delete_message(message, &db).await,
                            }
                        } else {
                            tracing::warn!("Received message without payload");
                        }
                    },
                    Err(e) => tracing::warn!("Kafka error: {}", e),
                }
            },
            _ = shutdown.recv() => {
                tracing::info!("Shutdown signal received, terminating consumer loop...");
                break;
            }
        }
    }
}
