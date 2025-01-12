use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use crate::checkbot::services::Checkbot;
use super::services::CheckbotStorage;
use crate::db::DbPool;
use crate::middleware::ApiResponse;

type CheckbotStorageResponse = (StatusCode, Json<ApiResponse<CheckbotStorage>>);

#[derive(Deserialize)]
pub(crate) struct CreateCheckbotStoragePayload {
    checkbot_id: i32,
    updated_completion: String,
}

pub(crate) async fn create_checkbot_storage_route(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateCheckbotStoragePayload>
) -> CheckbotStorageResponse {
    match Checkbot::find_checkbot_by_id(&pool, &payload.checkbot_id) { 
        Ok(checkbot) => {
           match CheckbotStorage::create_checkbot_storage(&pool, &checkbot, &payload.updated_completion) { 
               Ok(checkbot_storage) => ApiResponse::new(StatusCode::CREATED, Some(checkbot_storage), "Created").send(),
               Err(storage_err) => ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &storage_err.to_string()).send()
           } 
        },
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}