mod db;
mod middleware;
mod schema;
mod users;
mod ai_system_prompt;

use crate::db::build_db_pool;
use axum::{middleware::from_fn, routing::get, Router};
use std::env;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let host_addr = match env::var("HOST_ADDRESS") {
        Ok(url) => url,
        Err(_) => panic!("Missing HOST_ADDRESS"),
    };

    let listener = tokio::net::TcpListener::bind(host_addr).await.unwrap();
    let pool = build_db_pool();

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .nest("/users", users::user_routes())
        .nest("/ai-system-prompt", ai_system_prompt::ai_system_prompt_routes())
        .with_state(pool)
        .layer(from_fn(middleware::api_key_middleware));

    // run our app with hyper, listening globally on env port
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
