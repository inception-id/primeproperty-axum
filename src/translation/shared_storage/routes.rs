use crate::db::DbPool;
use crate::language_ai::SharedStoragePermission;
use crate::schema::shared_translation_storage;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use diesel::Insertable;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct CreateSharedTranslationPayload {
    email: String,
}

pub async fn create_translation_shared_storage_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreateSharedTranslationPayload>,
) -> (StatusCode, String) {
    (StatusCode::CREATED, payload.email)
}
