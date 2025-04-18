mod db;
mod middleware;

use crate::db::build_db_pool;
use axum::http::HeaderValue;
use axum::{middleware::from_fn, routing::get, Router};
use std::env;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = build_db_pool();
    let allow_origin = env::var("ALLOW_ORIGIN").expect("Missing ALLOW_ORIGIN");
    let cors = CorsLayer::new()
        .allow_origin(allow_origin.parse::<HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers(Any);

    let tracing_filter = tracing_subscriber::EnvFilter::new("tower_http::trace::make_span=debug,tower_http::trace::on_response=debug,tower_http::trace::on_request=debug");
    tracing_subscriber::fmt()
        .with_env_filter(tracing_filter)
        .init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(pool)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on env port
    let host_addr = env::var("HOST_ADDRESS").expect("Missing HOST_ADDRESS");
    let listener = match tokio::net::TcpListener::bind(&host_addr).await {
        Ok(tcp) => tcp,
        Err(err) => {
            println!("Failed to listen to {}: {}", host_addr, err);
            return;
        }
    };

    match axum::serve(listener, app).await {
        Ok(_) => println!("Server started at {}", host_addr),
        Err(err) => println!("Server failed to start: {}", err),
    }
}
