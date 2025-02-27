use super::services::SharedTranslationUser;
use crate::db::DbPool;
use crate::language_ai::{LanguageAiSharedStorageUser, LanguageaiStorage, SharedStoragePermission};
use crate::middleware::{extract_header_user_id, ApiResponse};
use crate::schema;
use crate::translation::shared_storage::join_structs::SharedTranslationStorageJoinTranslationStorage;
use crate::translation::TranslationStorage;
use crate::users::User;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use diesel::Insertable;
use serde::Deserialize;

type SharedTranslationUserResponse = (StatusCode, Json<ApiResponse<SharedTranslationUser>>);

#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::shared_translation_storage)]
pub struct CreateSharedTranslationUserPayload {
    shared_user_email: String,
    translation_storage_id: i32,
}

pub async fn create_shared_translation_user_route(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateSharedTranslationUserPayload>,
) -> SharedTranslationUserResponse {
    // Check if user is already invited
    if SharedTranslationUser::check_shared_user(
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
    match SharedTranslationUser::create_shared_user(&pool, &payload, &owner_data, &shared_user_id) {
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
pub(crate) struct UpdateSharedTranslationUserPermissionPayload {
    permission: SharedStoragePermission,
}

pub async fn update_shared_translation_user_permission_route(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateSharedTranslationUserPermissionPayload>,
) -> SharedTranslationUserResponse {
    match SharedTranslationUser::update_shared_user_permission(&pool, &id, &payload.permission) {
        Ok(shared_translation) => {
            ApiResponse::new(StatusCode::OK, Some(shared_translation), "Updated").send()
        }
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

pub async fn delete_shared_translation_user_route(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> SharedTranslationUserResponse {
    match SharedTranslationUser::delete_shared_user(&pool, &id) {
        Ok(shared_translation) => {
            ApiResponse::new(StatusCode::OK, Some(shared_translation), "Deleted").send()
        }
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

pub async fn find_shared_translation_users_route(
    State(pool): State<DbPool>,
    Path(storage_id): Path<i32>,
    headers: HeaderMap,
) -> (StatusCode, Json<ApiResponse<Vec<SharedTranslationUser>>>) {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");
    let user = match User::find_user_by_id(&pool, &user_id) {
        Ok(user) => user,
        Err(err) => {
            return ApiResponse::new(StatusCode::UNAUTHORIZED, None, &err.to_string()).send()
        }
    };
    match SharedTranslationUser::find_shared_users(&pool, &storage_id, &user.email) {
        Ok(shared_translation_users) => {
            ApiResponse::new(StatusCode::OK, Some(shared_translation_users), "Ok").send()
        }
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

pub async fn find_shared_translation_storage_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
) -> (
    StatusCode,
    Json<ApiResponse<Vec<SharedTranslationStorageJoinTranslationStorage>>>,
) {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");
    match SharedTranslationUser::find_shared_storages(&pool, &user_id) {
        Ok(shared_join_storage) => {
            ApiResponse::new(StatusCode::OK, Some(shared_join_storage), "Ok").send()
        }
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}
