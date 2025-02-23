use super::services::SharedTranslationStorage;
use crate::db::DbPool;
use crate::language_ai::{LanguageaiStorage, LanguageaiStorageSharing, SharedStoragePermission};
use crate::middleware::{extract_header_user_id, ApiResponse};
use crate::schema;
use crate::translation::TranslationStorage;
use crate::users::User;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use diesel::Insertable;
use serde::Deserialize;
use crate::translation::shared_storage::join_structs::SharedTranslationStorageJoinTranslationStorage;

type SharedTranslationStorageResponse = (StatusCode, Json<ApiResponse<SharedTranslationStorage>>);

#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::shared_translation_storage)]
pub struct CreateSharedTranslationPayload {
    shared_user_email: String,
    translation_storage_id: i32,
}

pub async fn create_translation_shared_storage_route(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateSharedTranslationPayload>,
) -> SharedTranslationStorageResponse {
    // Check if user is already invited
    if SharedTranslationStorage::check_shared_storage_and_shared_email(
        &pool,
        &payload.translation_storage_id,
        &payload.shared_user_email,
    )
    .is_ok()
    {
        return ApiResponse::new(
            StatusCode::BAD_REQUEST,
            None,
            "Can not invite the same user twice",
        )
        .send();
    }

    // Find the storage and its owner
    let translation_storage =
        match TranslationStorage::find_storage_by_id(&pool, &payload.translation_storage_id) {
            Ok(storage) => storage,
            Err(err) => {
                return ApiResponse::new(StatusCode::BAD_REQUEST, None, &err.to_string()).send()
            }
        };

    let owner_data = match User::find_user_by_id(&pool, &translation_storage.user_id) {
        Ok(user) => user,
        Err(err) => {
            return ApiResponse::new(StatusCode::BAD_REQUEST, None, &err.to_string()).send()
        }
    };

    // Prevent self-invitation
    if owner_data.email == payload.shared_user_email {
        return ApiResponse::new(StatusCode::BAD_REQUEST, None, "Can not share to owner!").send();
    }

    // Find shared user, if they exist
    let shared_user_id = match User::find_user_by_email(&pool, &payload.shared_user_email) {
        Ok(shared_user) => Some(shared_user.id),
        Err(_) => None,
    };

    // Create shared storage
    match SharedTranslationStorage::create_shared_storage(
        &pool,
        &payload,
        &owner_data,
        &shared_user_id,
    ) {
        Ok(shared_translation_storage) => ApiResponse::new(
            StatusCode::CREATED,
            Some(shared_translation_storage),
            "Created",
        )
        .send(),
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
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
    match SharedTranslationStorage::update_storage_permission(&pool, &id, &payload.permission) {
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

pub async fn find_shared_users(
    State(pool): State<DbPool>,
    Path(storage_id): Path<i32>,
    headers: HeaderMap,
) -> (StatusCode, Json<ApiResponse<Vec<SharedTranslationStorage>>>) {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");
    let user = match User::find_user_by_id(&pool, &user_id) {
        Ok(user) => user,
        Err(err) => return ApiResponse::new(StatusCode::UNAUTHORIZED, None, &err.to_string()).send(),
    };
    match SharedTranslationStorage::find_shared_users(&pool, &storage_id, &user.email) {
        Ok(shared_translation_users) => {
            ApiResponse::new(StatusCode::OK, Some(shared_translation_users), "Ok").send()
        }
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

pub async fn find_user_shared_storage(
   State(pool): State<DbPool>,
   headers: HeaderMap,
) -> (StatusCode, Json<ApiResponse<Vec<SharedTranslationStorageJoinTranslationStorage>>>) {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");
    match SharedTranslationStorage::find_shared_join_storage(&pool, &user_id) { 
        Ok(shared_join_storage) => ApiResponse::new(StatusCode::OK, Some(shared_join_storage), "Ok").send(),
        Err(err) => ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send(),
    }
}
