use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::{Json, Router};
use axum::routing::post;
use diesel::Insertable;
use serde::Deserialize;
use crate::db::DbPool;
use crate::middleware::{extract_header_user_id, ApiResponse};
use crate::schema::speech_to_text;
use super::services::SpeechToText;

type TranscriptionResponse = (StatusCode, Json<ApiResponse<SpeechToText>>);

#[derive(Deserialize, Insertable)]
#[diesel(table_name = speech_to_text)]
pub(super) struct CreateTranscriptionPayload {
    audio_url: Option<String>,
    transcription_text: String,
}

async fn create_transcription_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreateTranscriptionPayload>,
) -> TranscriptionResponse {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");
    match SpeechToText::create_transcription(&pool, &user_id, &payload) {
        Ok(transcription) => ApiResponse::new(StatusCode::CREATED, Some(transcription), "Created").send(),
        Err(e) => ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &e.to_string()).send(),
    }
}

pub fn transcription_routes() -> Router<DbPool> {
    Router::new().route("/create", post(create_transcription_route))
}
