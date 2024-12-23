use super::services::AiSystemPrompt;
use crate::db::DbPool;
use crate::middleware::ApiResponse;
use crate::schema::ai_system_prompts;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use diesel::AsChangeset;
use serde::Deserialize;

type AiSystemPromptResponse = (StatusCode, Json<ApiResponse<AiSystemPrompt>>);

#[derive(Debug, Deserialize)]
struct CreateAiSystemPromptPayload {
    product_name: String,
    prompt: String,
    name: String,
}

async fn create_ai_system_prompt_route(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateAiSystemPromptPayload>,
) -> AiSystemPromptResponse {
    let ai_system_prompt_creation = AiSystemPrompt::create_ai_system_prompt(
        &pool,
        &payload.product_name,
        &payload.prompt,
        &payload.name,
    );

    match ai_system_prompt_creation {
        Ok(ai_system_prompt) => {
            ApiResponse::new(StatusCode::CREATED, Some(ai_system_prompt), "Created").send()
        }
        Err(error) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &error.to_string()).send()
        }
    }
}

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

async fn delete_ai_system_prompt_route(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> AiSystemPromptResponse {
    let deleted_ai_system_prompt = AiSystemPrompt::delete_ai_system_prompts(&pool, &id);

    match deleted_ai_system_prompt {
        Ok(ai_system_prompt) => ApiResponse::new(
            StatusCode::OK,
            Some(ai_system_prompt),
            "System prompt deleted",
        )
        .send(),
        Err(error) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &error.to_string()).send()
        }
    }
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = ai_system_prompts)]
pub(super) struct UpdateAiSystemPromptPayload {
    product_name: Option<String>,
    name: Option<String>,
    prompt: Option<String>,
}

async fn update_ai_system_prompt_route(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(update_payload): Json<UpdateAiSystemPromptPayload>,
) -> AiSystemPromptResponse {
    let updated_ai_system_prompt =
        AiSystemPrompt::update_ai_system_prompt(&pool, &id, &update_payload);

    match updated_ai_system_prompt {
        Ok(ai_system_prompt) => ApiResponse::new(
            StatusCode::OK,
            Some(ai_system_prompt),
            "System prompt updated",
        )
        .send(),
        Err(error) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &error.to_string()).send()
        }
    }
}

pub fn ai_system_prompt_routes() -> Router<DbPool> {
    Router::new()
        .route("/create", post(create_ai_system_prompt_route))
        .route("/find-all", get(find_ai_system_prompts_route))
        .route("/delete/:id", delete(delete_ai_system_prompt_route))
        .route("/update/:id", put(update_ai_system_prompt_route))
}
