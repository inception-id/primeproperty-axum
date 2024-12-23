use super::services::Language;
use crate::db::DbPool;
use crate::middleware::ApiResponse;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;

type LanguageResponse = (StatusCode, Json<ApiResponse<Language>>);

#[derive(Deserialize)]
struct CreateLanguagePayload {
    title: String,
    iso_639_1: String,
}

async fn create_language_route(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateLanguagePayload>,
) -> LanguageResponse {
    let language_createion = Language::create_language(&pool, &payload.title, &payload.iso_639_1);

    match language_createion {
        Ok(language) => ApiResponse::new(StatusCode::CREATED, Some(language), "Created").send(),
        Err(error) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &error.to_string()).send()
        }
    }
}

pub fn language_routes() -> Router<DbPool> {
    Router::new().route("/create", post(create_language_route))
}
