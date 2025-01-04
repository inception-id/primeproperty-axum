use super::services::TextToSpeech;
use crate::db::DbPool;
use crate::middleware::{extract_header_user_id, ApiResponse};
use crate::schema::text_to_speech;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use diesel::Insertable;
use serde::Deserialize;

type TtsResponse = (StatusCode, Json<ApiResponse<TextToSpeech>>);

#[derive(Deserialize, Insertable)]
#[diesel(table_name = text_to_speech)]
pub(super) struct CreateTtsPayload {
    input_content: String,
    audio_url: String,
    voice: String,
}

async fn create_tts_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreateTtsPayload>,
) -> TtsResponse {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");
    match TextToSpeech::create_tts(&pool, &user_id, &payload) {
        Ok(tts) => ApiResponse::new(StatusCode::CREATED, Some(tts), "Created").send(),
        Err(e) => ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &e.to_string()).send(),
    }
}

async fn find_tts_history_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
) -> (StatusCode, Json<ApiResponse<Vec<TextToSpeech>>>) {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");
    match TextToSpeech::find_tts_history(&pool, &user_id) {
        Ok(tts) => ApiResponse::new(StatusCode::OK, Some(tts), "Found").send(),
        Err(e) => ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &e.to_string()).send(),
    }
}

pub fn tts_routes() -> Router<DbPool> {
    Router::new()
        .route("/create", post(create_tts_route))
        .route("/history", get(find_tts_history_route))
}
