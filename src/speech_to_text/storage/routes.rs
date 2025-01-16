use crate::db::DbPool;
use crate::middleware::{extract_header_user_id, ApiResponse};
use crate::speech_to_text::services::SpeechToText;
use crate::speech_to_text::storage::services::SpeechToTextStorage;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use serde::Deserialize;

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
            match SpeechToTextStorage::create_storage(
                &pool,
                &speech_to_text,
                &payload.updated_transcription_text,
            ) {
                Ok(transcription_storage) => {
                    ApiResponse::new(StatusCode::CREATED, Some(transcription_storage), "success")
                        .send()
                }
                Err(storage_err) => ApiResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    None,
                    &storage_err.to_string(),
                )
                .send(),
            }
        }
        Err(e) => ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &e.to_string()).send(),
    }
}

pub(crate) async fn find_transcription_storage_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
) -> (StatusCode, Json<ApiResponse<Vec<SpeechToTextStorage>>>) {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");
    match SpeechToTextStorage::find_storage(&pool, &user_id) {
        Ok(storage) => ApiResponse::new(StatusCode::OK, Some(storage), "success").send(),
        Err(e) => ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &e.to_string()).send(),
    }
}

pub(crate) async fn delete_transcription_storage_route(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> TranscriptionStorageResponse {
    match SpeechToTextStorage::delete_storage(&pool, &id) {
        Ok(storage) => ApiResponse::new(StatusCode::OK, Some(storage), "success").send(),
        Err(e) => ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &e.to_string()).send(),
    }
}

#[derive(Deserialize)]
pub(crate) struct UpdateTranscriptionStoragePayload {
    updated_transcription_text: String,
}

pub(crate) async fn update_transcription_storage_route(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTranscriptionStoragePayload>
) -> TranscriptionStorageResponse {
    match SpeechToTextStorage::update_storage(&pool, &id, &payload.updated_transcription_text) { 
        Ok(storage) => ApiResponse::new(StatusCode::OK, Some(storage), "success").send(),
        Err(e) => ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &e.to_string()).send(),
    }
}
