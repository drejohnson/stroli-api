use crate::domain::AppSecrets;
use rdkafka::config::ClientConfig;
use rdkafka::producer::FutureProducer;

pub fn create_kafka_producer_upstash(secrets: &AppSecrets) -> FutureProducer {
    let url = secrets.kafka_url.clone();
    let user = secrets.kafka_sasl_user.clone();
    let pw = secrets.kafka_sasl_pass.clone();

    ClientConfig::new()
        .set("bootstrap.servers", url)
        .set("sasl.mechanism", "SCRAM-SHA-256")
        .set("security.protocol", "SASL_SSL")
        .set("sasl.username", user)
        .set("sasl.password", pw)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error")
}

pub fn create_kafka_producer(secrets: &AppSecrets) -> FutureProducer {
    let url = secrets.kafka_url.clone();

    let log_level: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", url)
        .set("message.timeout.ms", "5000")
        .set("queue.buffering.max.ms", "0") // Do not buffer
        .set("allow.auto.create.topics", "true")
        .create()
        .expect("Producer creation error");

    log_level
}
