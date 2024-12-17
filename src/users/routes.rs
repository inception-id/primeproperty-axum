use crate::db::DbPool;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use diesel::{prelude::*, select, sql_types::Text};
use serde::{Deserialize, Serialize};

// #[axum::]
async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    State(pool): State<DbPool>,
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let conn = &mut pool.get().unwrap();
    let query = select("Hello world!".into_sql::<Text>());
    let result = query.get_result::<String>(conn);
    println!("{:?}", result);
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

pub fn user_routes() -> Router<DbPool> {
    Router::new().route("/users", post(create_user))
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
