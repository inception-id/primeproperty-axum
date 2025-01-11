use super::services::TranslationStorage;
use crate::db::DbPool;
use crate::middleware::{extract_header_user_id, ApiResponse};
use crate::translation::services::Translation;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use serde::Deserialize;

type TranslationStorageResponse = (StatusCode, Json<ApiResponse<TranslationStorage>>);

#[derive(Deserialize)]
pub(crate) struct CreateTranslationStoragePayload {
    translation_id: i32,
    updated_completion: String,
}

pub(crate) async fn create_translation_storage_route(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateTranslationStoragePayload>,
) -> TranslationStorageResponse {
    match Translation::find_translation(&pool, &payload.translation_id) {
        Ok(translation) => {
            match TranslationStorage::create_translation_storage(
                &pool,
                &translation,
                &payload.updated_completion,
            ) {
                Ok(translation_storage) => {
                    ApiResponse::new(StatusCode::CREATED, Some(translation_storage), "Created")
                        .send()
                }
                Err(err) => {
                    ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string())
                        .send()
                }
            }
        }
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

pub(crate) async fn find_translation_storage_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
) -> (StatusCode, Json<ApiResponse<Vec<TranslationStorage>>>) {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");

    match TranslationStorage::find_user_translation_storage(&pool, &user_id) {
        Ok(translation_storage) => {
            ApiResponse::new(StatusCode::OK, Some(translation_storage), "success").send()
        }
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

pub(crate) async fn delete_translation_storage_route(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> TranslationStorageResponse {
    match TranslationStorage::delete_translation_storage(&pool, &id) { 
        Ok(translation_storage) => ApiResponse::new(StatusCode::OK, Some(translation_storage), "success").send(),
        Err(err) => ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
    }
}