use super::storage;
use crate::db::DbPool;
use crate::language_ai::LanguageAiCrud;
use crate::languageai_subscriptions::SubcriptionLimit;
use crate::middleware::{extract_header_user_id, ApiResponse};
use crate::schema::translation;
use crate::translation::services::Translation;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use diesel::Insertable;
use serde::Deserialize;

type TranslationResponse = (StatusCode, Json<ApiResponse<Translation>>);

#[derive(Deserialize, Insertable)]
#[diesel(table_name = translation)]
pub struct CreateTranslationPayload {
    ai_system_prompt: String,
    content_language: Option<String>,
    target_language: String,
    content: String,
    completion: String,
    input_tokens: i32,
    output_tokens: i32,
    total_tokens: i32,
    temperature: f64,
}

async fn create_translation_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreateTranslationPayload>,
) -> TranslationResponse {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");
    match Translation::create(&pool, &user_id, &payload) {
        Ok(translation) => ApiResponse::reply(StatusCode::CREATED, Some(translation), "Created"),
        Err(err) => ApiResponse::reply(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()),
    }
}

async fn find_translation_history_route(
    State(pool): State<DbPool>,
    headers: HeaderMap,
) -> (StatusCode, Json<ApiResponse<Vec<Translation>>>) {
    let user_id = extract_header_user_id(headers).expect("Could not extract user id");
    let history_limit = SubcriptionLimit::find_user_subscription_limit_count(
        &pool,
        &user_id,
        &SubcriptionLimit::History,
    );
    let translation_history =
        Translation::find_translation_history(&pool, &user_id, &history_limit);
    match translation_history {
        Ok(translations) => ApiResponse::new(StatusCode::OK, Some(translations), "success").send(),
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

pub fn translation_routes() -> Router<DbPool> {
    Router::new()
        .route("/create", post(create_translation_route))
        .route("/history", get(find_translation_history_route))
        .route(
            "/create-storage",
            post(storage::create_translation_storage_route),
        )
        .route(
            "/find-storage",
            get(storage::find_translation_storage_route),
        )
        .route(
            "/delete-storage/:id",
            delete(storage::delete_translation_storage_route),
        )
        .route(
            "/update-storage/:id",
            put(storage::update_translation_storage_route),
        )
}
