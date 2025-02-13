use crate::db::DbPool;
use crate::languageai_subscriptions::{SubcriptionLimit, SubcriptionStorageLimit};
use crate::middleware::{extract_header_user_id, ApiResponse};
use crate::schema;
use crate::speech_to_text::services::SpeechToText;
use crate::speech_to_text::storage::services::SpeechToTextStorage;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use diesel::{AsChangeset, Insertable};
use serde::Deserialize;

type TranscriptionStorageResponse = (StatusCode, Json<ApiResponse<SpeechToTextStorage>>);

#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::speech_to_text_storage)]
pub(crate) struct CreateTranscriptionStoragePayload {
    speech_to_text_id: i32,
    title: Option<String>,
    updated_transcription_text: String,
}

pub(crate) async fn create_transcription_storage_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreateTranscriptionStoragePayload>,
) -> TranscriptionStorageResponse {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");
    match SubcriptionLimit::check_user_exceed_limit(
        &pool,
        &user_id,
        &SubcriptionLimit::Storage,
        &Some(SubcriptionStorageLimit::SpeechToText),
    ) {
        true => ApiResponse::new(
            StatusCode::PAYMENT_REQUIRED,
            None,
            &StatusCode::PAYMENT_REQUIRED.to_string(),
        )
        .send(),
        false => match SpeechToText::find_transcription_by_id(&pool, &payload.speech_to_text_id) {
            Ok(speech_to_text) => {
                match SpeechToTextStorage::create_storage(&pool, &speech_to_text, &payload) {
                    Ok(transcription_storage) => ApiResponse::new(
                        StatusCode::CREATED,
                        Some(transcription_storage),
                        "success",
                    )
                    .send(),
                    Err(storage_err) => ApiResponse::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        None,
                        &storage_err.to_string(),
                    )
                    .send(),
                }
            }
            Err(e) => {
                ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &e.to_string()).send()
            }
        },
    }
}

pub(crate) async fn find_transcription_storage_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
) -> (StatusCode, Json<ApiResponse<Vec<SpeechToTextStorage>>>) {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");
    let storage_limit = SubcriptionLimit::find_user_subscription_limit_count(
        &pool,
        &user_id,
        &SubcriptionLimit::Storage,
    );
    match SpeechToTextStorage::find_storage(&pool, &user_id, &storage_limit) {
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

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = schema::speech_to_text_storage)]
pub(crate) struct UpdateTranscriptionStoragePayload {
    title: Option<String>,
    updated_transcription_text: String,
}

pub(crate) async fn update_transcription_storage_route(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTranscriptionStoragePayload>,
) -> TranscriptionStorageResponse {
    match SpeechToTextStorage::update_storage(&pool, &id, &payload) {
        Ok(storage) => ApiResponse::new(StatusCode::OK, Some(storage), "success").send(),
        Err(e) => ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &e.to_string()).send(),
    }
}
