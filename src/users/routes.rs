use std::fmt::Debug;
use crate::db::DbPool;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use axum::response::{IntoResponse, Response};
use crate::users::request::CreateUserPayload;
use super::services::User;

async fn create_user_route(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    State(pool): State<DbPool>,
    Json(payload): Json<CreateUserPayload>,
) -> Response {
    let check_user = User::find_user_by_email(&pool, &payload.email);
    match check_user {
        Ok(_) => (StatusCode::BAD_REQUEST, "User already exist".to_string()).into_response(),
        Err(_) => {
            let create_user = User::create_user(&pool, "1", "wawa");
            match create_user {
                Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
}


pub fn user_routes() -> Router<DbPool> {
    Router::new().route("/users", post(create_user_route))
}

