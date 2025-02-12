use super::services::TranslationStorage;
use crate::db::DbPool;
use crate::languageai_subscriptions::{SubcriptionLimit, SubcriptionStorageLimit};
use crate::middleware::{extract_header_user_id, ApiResponse};
use crate::translation::services::Translation;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use diesel::{AsChangeset, Insertable};
use serde::Deserialize;
use crate::schema;

type TranslationStorageResponse = (StatusCode, Json<ApiResponse<TranslationStorage>>);

#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::translation_storage)]
pub(crate) struct CreateTranslationStoragePayload {
    translation_id: i32,
    title: Option<String>,
    updated_completion: String,
}

pub(crate) async fn create_translation_storage_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreateTranslationStoragePayload>,
) -> TranslationStorageResponse {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");
    match SubcriptionLimit::check_user_exceed_limit(
        &pool,
        &user_id,
        &SubcriptionLimit::Storage,
        &Some(SubcriptionStorageLimit::Translation),
    ) {
        true => ApiResponse::new(
            StatusCode::PAYMENT_REQUIRED,
            None,
            &StatusCode::PAYMENT_REQUIRED.to_string(),
        )
        .send(),
        false => match Translation::find_translation(&pool, &payload.translation_id) {
            Ok(translation) => {
                match TranslationStorage::create_translation_storage(
                    &pool,
                    &translation,
                    &payload,
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
        },
    }
}

pub(crate) async fn find_translation_storage_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
) -> (StatusCode, Json<ApiResponse<Vec<TranslationStorage>>>) {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");
    let storage_limit = SubcriptionLimit::find_user_subscription_limit_count(
        &pool,
        &user_id,
        &SubcriptionLimit::Storage,
    );

    match TranslationStorage::find_user_translation_storage(&pool, &user_id, &storage_limit) {
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
        Ok(translation_storage) => {
            ApiResponse::new(StatusCode::OK, Some(translation_storage), "success").send()
        }
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name= schema::translation_storage)]
pub(crate) struct UpdateTranslationStoragePayload {
    title: Option<String>,
    updated_completion: String,
}

pub(crate) async fn update_translation_storage_route(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTranslationStoragePayload>,
) -> TranslationStorageResponse {
    match TranslationStorage::update_translation_storage(&pool, &id, &payload) {
        Ok(translation_storage) => {
            ApiResponse::new(StatusCode::OK, Some(translation_storage), "success").send()
        }
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}
