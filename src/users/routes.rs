use super::services::User;
use crate::db::DbPool;
use crate::middleware::ApiResponse;
use crate::translation::SharedTranslationStorage;
use crate::users::request::CreateUserPayload;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;

async fn create_user_route(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    State(pool): State<DbPool>,
    Json(payload): Json<CreateUserPayload>,
) -> (StatusCode, Json<ApiResponse<User>>) {
    match User::find_user_by_email(&pool, &payload.email) {
        Ok(_) => ApiResponse::new(StatusCode::BAD_REQUEST, None, "User already exist").send(),
        Err(_) => {
            let new_user =
                match User::create_user(&pool, &payload.supertokens_user_id, &payload.email) {
                    Ok(user) => user,
                    Err(e) => {
                        return ApiResponse::new(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            None,
                            &e.to_string(),
                        )
                        .send()
                    }
                };

            let _shared_translation_storage =
                SharedTranslationStorage::upsert_new_id_to_invited_email(
                    &pool,
                    &new_user.id,
                    &new_user.email,
                );

            ApiResponse::new(StatusCode::CREATED, Some(new_user), "Created").send()
        }
    }
}

#[derive(Deserialize)]
struct FindUserQuery {
    email: String,
}

async fn find_user_route(
    State(pool): State<DbPool>,
    query: Query<FindUserQuery>,
) -> (StatusCode, Json<ApiResponse<User>>) {
    let check_user = User::find_user_by_email(&pool, &query.email);

    match check_user {
        Ok(user) => ApiResponse::new(StatusCode::OK, Some(user), "User found").send(),
        Err(e) => ApiResponse::new(StatusCode::BAD_REQUEST, None, &e.to_string()).send(),
    }
}

pub fn user_routes() -> Router<DbPool> {
    Router::new()
        .route("/create-user", post(create_user_route))
        .route("/find-user", get(find_user_route))
}
