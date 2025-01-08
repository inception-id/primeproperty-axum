use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use crate::db::DbPool;
use crate::middleware::ApiResponse;
use crate::translation::services::Translation;
use super::services::TranslationStorage;

type TranslationStorageResponse = (StatusCode, Json<ApiResponse<TranslationStorage>>);

#[derive(Deserialize)]
pub(crate) struct CreateTranslationStoragePayload {
    translation_id: i32,
    updated_completion: String,
}

pub(crate) async fn create_translation_storage(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateTranslationStoragePayload>,
) -> TranslationStorageResponse {
    match Translation::find_translation(&pool, &payload.translation_id) {
        Ok(translation) => {
            match TranslationStorage::create_translation_storage(&pool, &translation, &payload.updated_completion) {
                Ok(translation_storage) => ApiResponse::new(StatusCode::CREATED, Some(translation_storage), "Created").send(),
                Err(err) => ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
            }
        }
        Err(err) => ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
    }
}