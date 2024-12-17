mod middleware;
mod schema;
mod users;
mod connection;

use axum::{
    routing::{get},
    Router,
};
use serde::{Deserialize, Serialize};
use std::env;
use deadpool_diesel::Pool;
use crate::connection::{establish_connection};

#[derive(Clone)]
pub struct AppState {
    pool: deadpool_diesel::postgres::Pool
}
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let host_addr = match env::var("HOST_ADDRESS") {
        Ok(url) => url,
        Err(_) => panic!("Missing HOST_ADDRESS"),
    };

    let listener = tokio::net::TcpListener::bind(host_addr).await.unwrap();
    let pool= establish_connection();
    // let ap = AppState { pool: pg_connection };

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .nest("/users", users::user_routes())
        .layer(axum::middleware::from_fn(middleware::api_key_middleware))
        .with_state(pool);

    // run our app with hyper, listening globally on env port
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

