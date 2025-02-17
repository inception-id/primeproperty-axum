use super::services::SharedTranslationStorage;
use crate::db::DbPool;
use crate::language_ai::LanguageaiSharedUserStorageTrait;
use crate::middleware::{extract_header_user_id, ApiResponse};
use crate::schema::shared_translation_storage;
use crate::users::User;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use diesel::Insertable;
use serde::Deserialize;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = shared_translation_storage)]
pub(crate) struct CreateSharedTranslationPayload {
    shared_user_email: String,
    translation_storage_id: i32,
}

pub async fn create_translation_shared_storage_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreateSharedTranslationPayload>,
) -> (StatusCode, Json<ApiResponse<SharedTranslationStorage>>) {
    match extract_header_user_id(headers) {
        Ok(user_id) => {
            let owner_data = User::find_user_by_id(&pool, &user_id);
            let shared_user_data = User::find_user_by_email(&pool, &payload.shared_user_email);
            match (owner_data, shared_user_data) {
                (Ok(owner_data), Ok(shared_user_data)) => {
                    match SharedTranslationStorage::create(
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
                    match SharedTranslationStorage::create(&pool, &payload, &owner_data, &None) {
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
