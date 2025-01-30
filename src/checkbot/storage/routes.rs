use super::services::CheckbotStorage;
use crate::checkbot::services::Checkbot;
use crate::db::DbPool;
use crate::languageai_subscriptions::{SubcriptionLimit, SubcriptionStorageLimit};
use crate::middleware::{extract_header_user_id, ApiResponse};
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use serde::Deserialize;

type CheckbotStorageResponse = (StatusCode, Json<ApiResponse<CheckbotStorage>>);

#[derive(Deserialize)]
pub(crate) struct CreateCheckbotStoragePayload {
    checkbot_id: i32,
    updated_completion: String,
}

pub(crate) async fn create_checkbot_storage_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreateCheckbotStoragePayload>,
) -> CheckbotStorageResponse {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");

    match SubcriptionLimit::check_user_exceed_limit(
        &pool,
        &user_id,
        &SubcriptionLimit::Storage,
        &Some(SubcriptionStorageLimit::Checkbot),
    ) {
        true => ApiResponse::new(
            StatusCode::PAYMENT_REQUIRED,
            None,
            &StatusCode::PAYMENT_REQUIRED.to_string(),
        )
        .send(),
        false => match Checkbot::find_checkbot_by_id(&pool, &payload.checkbot_id) {
            Ok(checkbot) => {
                match CheckbotStorage::create_checkbot_storage(
                    &pool,
                    &checkbot,
                    &payload.updated_completion,
                ) {
                    Ok(checkbot_storage) => {
                        ApiResponse::new(StatusCode::CREATED, Some(checkbot_storage), "Created")
                            .send()
                    }
                    Err(storage_err) => ApiResponse::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        None,
                        &storage_err.to_string(),
                    )
                    .send(),
                }
            }
            Err(err) => {
                ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
            }
        },
    }
}

pub(crate) async fn find_many_checkbot_storage_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
) -> (StatusCode, Json<ApiResponse<Vec<CheckbotStorage>>>) {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");

    match CheckbotStorage::find_many_checkbot_storage(&pool, &user_id) {
        Ok(checkbot_storage) => {
            ApiResponse::new(StatusCode::OK, Some(checkbot_storage), "success").send()
        }
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

pub(crate) async fn delete_checkbot_storage_route(
    State(pool): State<DbPool>,

    Path(id): Path<i32>,
) -> CheckbotStorageResponse {
    match CheckbotStorage::delete_checkbot_storage(&pool, &id) {
        Ok(checkbot_storage) => {
            ApiResponse::new(StatusCode::OK, Some(checkbot_storage), "success").send()
        }
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct UpdateCheckbotPayload {
    updated_completion: String,
}

pub(crate) async fn update_checkbot_storage_route(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateCheckbotPayload>,
) -> CheckbotStorageResponse {
    match CheckbotStorage::update_checkbot_storage(&pool, &id, &payload.updated_completion) {
        Ok(checkbot_storage) => {
            ApiResponse::new(StatusCode::OK, Some(checkbot_storage), "success").send()
        }
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}
