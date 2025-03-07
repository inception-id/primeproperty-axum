use super::services::Language;
use crate::db::DbPool;
use crate::middleware::ApiResponse;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};

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

pub fn language_routes() -> Router<DbPool> {
    Router::new().route("/find-all", get(find_all_language_route))
}
