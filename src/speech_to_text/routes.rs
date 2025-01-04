use super::services::SpeechToText;
use crate::db::DbPool;
use crate::middleware::{extract_header_user_id, ApiResponse};
use crate::schema::speech_to_text;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use diesel::Insertable;
use serde::Deserialize;

type TranscriptionResponse = (StatusCode, Json<ApiResponse<SpeechToText>>);

#[derive(Deserialize, Insertable)]
#[diesel(table_name = speech_to_text)]
pub(super) struct CreateTranscriptionPayload {
    audio_url: String,
    transcription_text: String,
    language: String,
}

async fn create_transcription_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreateTranscriptionPayload>,
) -> TranscriptionResponse {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");
    match SpeechToText::create_transcription(&pool, &user_id, &payload) {
        Ok(transcription) => {
            ApiResponse::new(StatusCode::CREATED, Some(transcription), "Created").send()
        }
        Err(e) => ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &e.to_string()).send(),
    }
}

async fn find_transcription_history_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
) -> (StatusCode, Json<ApiResponse<Vec<SpeechToText>>>) {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");
    match SpeechToText::find_transcription_history(&pool, &user_id) {
        Ok(transcription_history) => {
            ApiResponse::new(StatusCode::OK, Some(transcription_history), "OK").send()
        }
        Err(e) => ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &e.to_string()).send(),
    }
}

pub fn transcription_routes() -> Router<DbPool> {
    Router::new()
        .route("/create", post(create_transcription_route))
        .route("/history", get(find_transcription_history_route))
}
