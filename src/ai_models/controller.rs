use super::model::AiModel;
use crate::db::DbPool;
use crate::{middleware::ApiResponse, schema::ai_models};
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use diesel::prelude::Insertable;
use reqwest::StatusCode;
use serde::Deserialize;

type AiModelResponse = (StatusCode, Json<ApiResponse<AiModel>>);

#[derive(Deserialize, Insertable)]
#[diesel(table_name = ai_models)]
pub(crate) struct CreateAiModelPayload {
    parent: String,
    label: String,
    value: String,
}

async fn create_ai_model_route(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateAiModelPayload>,
) -> AiModelResponse {
    match AiModel::create(&pool, &payload) {
        Ok(ai_model) => ApiResponse::new(StatusCode::CREATED, Some(ai_model), "Created").send(),
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

async fn find_all_ai_models_route(
    State(pool): State<DbPool>,
) -> (StatusCode, Json<ApiResponse<Vec<AiModel>>>) {
    match AiModel::find_all(&pool) {
        Ok(ai_models) => ApiResponse::new(StatusCode::OK, Some(ai_models), "Ok").send(),
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

pub fn ai_model_routes() -> Router<DbPool> {
    Router::new()
        .route("/create", post(create_ai_model_route))
        .route("/find-all", get(find_all_ai_models_route))
}
