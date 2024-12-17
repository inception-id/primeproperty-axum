mod middleware;
mod schema;
mod users;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::env;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let host_addr = match env::var("HOST_ADDRESS") {
        Ok(url) => url,
        Err(_) => panic!("Please set HOST_ADDRESS"),
    };

    let listener = tokio::net::TcpListener::bind(host_addr).await.unwrap();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .nest("/users", users::user_routes())
        // .route("/users", post(create_user))
        .layer(axum::middleware::from_fn(middleware::api_key_middleware));

    // run our app with hyper, listening globally on env port
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

