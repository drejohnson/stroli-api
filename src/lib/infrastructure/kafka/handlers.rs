use crate::domain::models::kafka::KafkaMessage;
use crate::domain::AppState;
use crate::errors::ApiError;

use axum::extract::State;
use axum::Json;
use rdkafka::producer::FutureRecord;

#[tracing::instrument(skip_all)]
pub async fn send_message(
    State(state): State<AppState>,
    Json(message): Json<KafkaMessage>,
) -> Result<&'static str, ApiError> {
    let msg = serde_json::to_vec(&message)?;
    let record: FutureRecord<str, Vec<u8>> = FutureRecord::to("messages").payload(&msg).key("1");

    state.producer().send_result(record)?.await??;

    tracing::info!("Message sent with data: {message:?}");

    Ok("Message sent!")
}
