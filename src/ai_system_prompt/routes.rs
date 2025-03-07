use super::services::AiSystemPrompt;
use crate::db::DbPool;
use crate::middleware::ApiResponse;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;

#[derive(Deserialize)]
struct FindAiSystemPromptQuery {
    product_name: Option<String>,
}
async fn find_ai_system_prompts_route(
    State(pool): State<DbPool>,
    query: Query<FindAiSystemPromptQuery>,
) -> (StatusCode, Json<ApiResponse<Vec<AiSystemPrompt>>>) {
    let ai_system_prompts = AiSystemPrompt::find_ai_system_prompts(&pool, &query.product_name);

    match ai_system_prompts {
        Ok(ai_system_prompts) => ApiResponse::new(
            StatusCode::CREATED,
            Some(ai_system_prompts),
            "System prompt created",
        )
        .send(),
        Err(error) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &error.to_string()).send()
        }
    }
}

pub fn ai_system_prompt_routes() -> Router<DbPool> {
    Router::new().route("/find-all", get(find_ai_system_prompts_route))
}
