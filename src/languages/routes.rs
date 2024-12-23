use super::services::Language;
use crate::db::DbPool;
use crate::middleware::ApiResponse;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post, delete, put};
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

async fn find_all_language_route(
    State(pool): State<DbPool>,
) -> (StatusCode, Json<ApiResponse<Vec<Language>>>) {
    let languages_search = Language::find_all_languages(&pool);
    match languages_search {
        Ok(languages) => ApiResponse::new(StatusCode::OK, Some(languages), "Found").send(),
        Err(error) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &error.to_string()).send()
        }
    }
}

async fn delete_language_route(
    State(pool): State<DbPool>,
    Path(id): Path<i32>
) -> LanguageResponse {
    let language_removal = Language::delete_language(&pool, &id);
    match language_removal { 
        Ok(language) => ApiResponse::new(StatusCode::OK, Some(language), "Delete success").send(),
        Err(error) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &error.to_string()).send()
        }
    }
}

pub fn language_routes() -> Router<DbPool> {
    Router::new()
        .route("/create", post(create_language_route))
        .route("/find-all", get(find_all_language_route))
        .route("/delete/:id", delete(delete_language_route))
}
