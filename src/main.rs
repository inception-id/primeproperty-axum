mod db;

use crate::db::build_db_pool;
use axum::{middleware::from_fn, Router};
use std::env;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = build_db_pool();
    let cors = CorsLayer::permissive();

    let tracing_filter = tracing_subscriber::EnvFilter::new("tower_http::trace::make_span=debug,tower_http::trace::on_response=debug,tower_http::trace::on_request=debug");
    tracing_subscriber::fmt()
        .with_env_filter(tracing_filter)
        .init();

    // build our application with a route
    let app = Router::new()
        .with_state(pool)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on env port
    let host_addr = env::var("HOST_ADDRESS").expect("Missing HOST_ADDRESS");
    let listener = tokio::net::TcpListener::bind(&host_addr).await.unwrap();

    match axum::serve(listener, app).await {
        Ok(_) => println!("Server started at {}", host_addr),
        Err(err) => println!("Server failed to start: {}", &err.to_string()),
    }
}
