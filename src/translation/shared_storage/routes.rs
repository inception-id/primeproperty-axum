use super::services::SharedTranslationStorage;
use crate::db::DbPool;
use crate::language_ai::{LanguageaiStorageSharing, SharedStoragePermission};
use crate::middleware::{extract_header_user_id, ApiResponse};
use crate::schema;
use crate::users::User;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use diesel::Insertable;
use serde::Deserialize;

type SharedTranslationStorageResponse = (StatusCode, Json<ApiResponse<SharedTranslationStorage>>);

#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::shared_translation_storage)]
pub(crate) struct CreateSharedTranslationPayload {
    shared_user_email: String,
    translation_storage_id: i32,
}

pub async fn create_translation_shared_storage_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreateSharedTranslationPayload>,
) -> SharedTranslationStorageResponse {
    match extract_header_user_id(headers) {
        Ok(user_id) => {
            let owner_data = User::find_user_by_id(&pool, &user_id);
            let shared_user_data = User::find_user_by_email(&pool, &payload.shared_user_email);
            match (owner_data, shared_user_data) {
                (Ok(owner_data), Ok(shared_user_data)) => {
                    match SharedTranslationStorage::create_shared_storage(
                        &pool,
                        &payload,
                        &owner_data,
                        &Some(shared_user_data.id),
                    ) {
                        Ok(shared_translation_storage) => ApiResponse::new(
                            StatusCode::CREATED,
                            Some(shared_translation_storage),
                            "created",
                        )
                        .send(),
                        Err(err) => ApiResponse::new(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            None,
                            &err.to_string(),
                        )
                        .send(),
                    }
                }
                (Ok(owner_data), Err(_)) => {
                    match SharedTranslationStorage::create_shared_storage(
                        &pool,
                        &payload,
                        &owner_data,
                        &None,
                    ) {
                        Ok(shared_translation_storage) => ApiResponse::new(
                            StatusCode::CREATED,
                            Some(shared_translation_storage),
                            "created",
                        )
                        .send(),
                        Err(err) => ApiResponse::new(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            None,
                            &err.to_string(),
                        )
                        .send(),
                    }
                }
                _ => ApiResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    None,
                    "Internal server error",
                )
                .send(),
            }
        }
        Err(err) => ApiResponse::new(StatusCode::UNAUTHORIZED, None, &err.to_string()).send(),
    }
}

#[derive(Deserialize)]
pub(crate) struct UpdateSharedTranslationPermissionPayload {
    permission: SharedStoragePermission,
}

pub async fn update_shared_translation_permission(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateSharedTranslationPermissionPayload>,
) -> SharedTranslationStorageResponse {
    match SharedTranslationStorage::update_permission(&pool, &id, &payload.permission) {
        Ok(shared_translation) => {
            ApiResponse::new(StatusCode::OK, Some(shared_translation), "Updated").send()
        }
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

pub async fn delete_shared_translation_storage(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> SharedTranslationStorageResponse {
    match SharedTranslationStorage::delete_shared_storage(&pool, &id) {
        Ok(shared_translation) => {
            ApiResponse::new(StatusCode::OK, Some(shared_translation), "Deleted").send()
        }
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}
