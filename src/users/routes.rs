use super::services::User;
use crate::db::DbPool;
use crate::users::request::CreateUserPayload;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use crate::middleware::ApiResponse;

async fn create_user_route(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    State(pool): State<DbPool>,
    Json(payload): Json<CreateUserPayload>,
) -> (StatusCode, Json<ApiResponse<User>>) {
    let check_user = User::find_user_by_email(&pool, &payload.email);
    match check_user {
        Ok(_) => {
            ApiResponse::new(StatusCode::BAD_REQUEST, None, "User already exist").send()
        }
        Err(_) => {
            let create_user = User::create_user(&pool, &payload.supertokens_user_id, &payload.email);
            match create_user {
                Ok(user) => {
                    ApiResponse::new(StatusCode::CREATED, Some(user), "User created").send()
                }
                Err(e) => ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &e.to_string()).send(),
            }
        }
    }
}

pub fn user_routes() -> Router<DbPool> {
    Router::new().route("/create-user", post(create_user_route))
}
