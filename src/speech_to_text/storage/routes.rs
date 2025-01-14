use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use crate::db::DbPool;
use crate::middleware::ApiResponse;
use crate::speech_to_text::services::SpeechToText;
use crate::speech_to_text::storage::services::SpeechToTextStorage;

type TranscriptionStorageResponse = (StatusCode, Json<ApiResponse<SpeechToTextStorage>>);

#[derive(Deserialize)]
pub(crate) struct CreateTranscriptionStoragePayload {
    speech_to_text_id: i32,
    updated_transcription_text: String,
}

pub(crate) async fn create_transcription_storage_route(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateTranscriptionStoragePayload>,
) -> TranscriptionStorageResponse {
    match SpeechToText::find_transcription_by_id(&pool, &payload.speech_to_text_id) {
        Ok(speech_to_text) => {
            match SpeechToTextStorage::create_storage(&pool, &speech_to_text, &payload.updated_transcription_text) {
                Ok(transcription_storage) => ApiResponse::new(StatusCode::CREATED, Some(transcription_storage), "success").send(),
                Err(storage_err) => ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &storage_err.to_string()).send()
            }
        }
        Err(e) => ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &e.to_string()).send()
    }
} 