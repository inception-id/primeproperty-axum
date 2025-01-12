use super::services::CheckbotStorage;
use crate::checkbot::services::Checkbot;
use crate::db::DbPool;
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
    Json(payload): Json<CreateCheckbotStoragePayload>,
) -> CheckbotStorageResponse {
    match Checkbot::find_checkbot_by_id(&pool, &payload.checkbot_id) {
        Ok(checkbot) => {
            match CheckbotStorage::create_checkbot_storage(
                &pool,
                &checkbot,
                &payload.updated_completion,
            ) {
                Ok(checkbot_storage) => {
                    ApiResponse::new(StatusCode::CREATED, Some(checkbot_storage), "Created").send()
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
    }
}

pub(crate) async fn find_many_checkbot_storage_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
) -> (StatusCode, Json<ApiResponse<Vec<CheckbotStorage>>>) {

    let user_id = extract_header_user_id(headers).expect("Could not extract user id");
    
    match CheckbotStorage::find_many_checkbot_storage(&pool, &user_id) {
        Ok(checkbot_storage) => ApiResponse::new(StatusCode::OK, Some(checkbot_storage), "success").send(),
        Err(err) => ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send(),
    }
}

pub(crate) async fn delete_checkbot_storage_route(
    State(pool): State<DbPool>,

    Path(id): Path<i32>,
) -> CheckbotStorageResponse {
    match CheckbotStorage::delete_checkbot_storage(&pool, &id) {
        Ok(checkbot_storage) => ApiResponse::new(StatusCode::OK, Some(checkbot_storage), "success").send(),
        Err(err) => ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send(),
    }
}