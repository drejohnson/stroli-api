use crate::domain::models::kafka::KafkaMessage;

#[tracing::instrument]
pub async fn create_message(
    message: KafkaMessage,
    db: &surrealdb::Surreal<surrealdb::engine::any::Any>,
) {
    let _ = db
        .query(
            "INSERT INTO MESSAGES (message_id, name, message) VALUES ($message_id, $name, $message)",
        )
        .bind(("message_id", &message.message_id))
        .bind(("name", &message.data().name()))
        .bind(("message", &message.data().message()))
        .await
        .inspect_err(|e| tracing::error!("Error while inserting message: {e}"));
}

#[tracing::instrument]
pub async fn update_message(
    message: KafkaMessage,
    db: &surrealdb::Surreal<surrealdb::engine::any::Any>,
) {
    let _ = db
        .query(
            "UPDATE MESSAGES
                SET
                name = $message_id,
                message = $name
                where message_id = $message",
        )
        .bind(("message_id", &message.message_id))
        .bind(("name", &message.data().name()))
        .bind(("message", &message.data().message()))
        .await
        .inspect_err(|e| tracing::error!("Error while updating message: {e}"));
}

#[tracing::instrument]
pub async fn delete_message(
    message: KafkaMessage,
    db: &surrealdb::Surreal<surrealdb::engine::any::Any>,
) {
    let _ = db
        .query("DELETE from messages where message_id = $message_id")
        .bind(("message_id", &message.message_id))
        .await
        .inspect_err(|e| tracing::error!("Error while deleting message: {e}"));
}
