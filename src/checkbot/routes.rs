use super::services::Checkbot;
use crate::checkbot::storage::{
    create_checkbot_storage_route, delete_checkbot_storage_route, find_many_checkbot_storage_route,
    update_checkbot_storage_route,
};
use crate::db::DbPool;
use crate::middleware::{extract_header_user_id, ApiResponse};
use crate::schema::checkbot;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use diesel::Insertable;
use serde::Deserialize;
use crate::languageai_subscriptions::SubcriptionLimit;

type CheckbotResponse = (StatusCode, Json<ApiResponse<Checkbot>>);

#[derive(Deserialize, Insertable)]
#[diesel(table_name = checkbot)]
pub(super) struct CreateCheckbotPayload {
    instruction: String,
    ai_system_prompt: String,
    content: String,
    completion: String,
}

async fn create_checkbot_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreateCheckbotPayload>,
) -> CheckbotResponse {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");
    let checkbot_creation = Checkbot::create_checkbot(&pool, &user_id, &payload);
    match checkbot_creation {
        Ok(checkbot) => ApiResponse::new(StatusCode::CREATED, Some(checkbot), "Created").send(),
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

async fn find_checkbot_history_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
) -> (StatusCode, Json<ApiResponse<Vec<Checkbot>>>) {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");
    let history_limit = SubcriptionLimit::find_user_subscription_limit_count(&pool, &user_id, &SubcriptionLimit::History);
    match Checkbot::find_checkbot_history(&pool, &user_id, &history_limit) {
        Ok(checkbot) => ApiResponse::new(StatusCode::OK, Some(checkbot), "Created").send(),
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

pub fn checkbot_routes() -> Router<DbPool> {
    Router::new()
        .route("/create", post(create_checkbot_route))
        .route("/history", get(find_checkbot_history_route))
        .route("/create-storage", post(create_checkbot_storage_route))
        .route("/find-storage", get(find_many_checkbot_storage_route))
        .route("/delete-storage/:id", delete(delete_checkbot_storage_route))
        .route("/update-storage/:id", put(update_checkbot_storage_route))
}
