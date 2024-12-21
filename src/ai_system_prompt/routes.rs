use axum::extract::State;
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::{post};
use serde::Deserialize;
use crate::db::DbPool;
use crate::middleware::ApiResponse;
use super::services::AiSystemPrompt;

#[derive(Debug, Deserialize)]
struct CreateAiSystemPromptPayload {
   product_name: String,
    prompt: String,
}

async fn create_ai_system_prompt_route(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateAiSystemPromptPayload>
) -> (StatusCode, Json<ApiResponse<AiSystemPrompt>>){
    let ai_system_prompt_creation = AiSystemPrompt::create_ai_system_prompt(&pool, &payload.product_name, &payload.prompt);

    match ai_system_prompt_creation {
        Ok(ai_system_prompt) => {
            ApiResponse::new(StatusCode::CREATED, Some(ai_system_prompt), "Created ai system prompt").send()
        },
        Err(error) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &error.to_string()).send()
        }
    }
}

pub fn ai_system_prompt_routes() -> Router<DbPool> {
    Router::new().route("/create", post(create_ai_system_prompt_route))
}
