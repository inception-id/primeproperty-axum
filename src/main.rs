mod ai_system_prompt;
mod checkbot;
mod db;
mod languageai_subscriptions;
mod languages;
mod middleware;
mod schema;
mod speech_to_text;
mod text_to_speech;
mod translation;
mod users;
mod utils;

use crate::db::build_db_pool;
use axum::{middleware::from_fn, routing::get, Router};
use std::env;
use axum::http::Request;
use reqwest::Body;
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::DEBUG).init();
    dotenvy::dotenv().ok();

    let host_addr = env::var("HOST_ADDRESS").expect("Missing HOST_ADDRESS");

    let listener = tokio::net::TcpListener::bind(&host_addr).await.unwrap();
    let pool = build_db_pool();

    let cors = CorsLayer::permissive();

    let trace_layer = TraceLayer::new_for_http()
        .on_request(
            DefaultOnRequest::new().level(Level::DEBUG)
        )
        .on_response(
            DefaultOnResponse::new()
                .level(Level::INFO)
        );



    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .nest("/users", users::user_routes())
        .nest(
            "/ai-system-prompts",
            ai_system_prompt::ai_system_prompt_routes(),
        )
        .nest("/languages", languages::language_routes())
        .nest("/translation", translation::translation_routes())
        .nest("/checkbot", checkbot::checkbot_routes())
        .nest("/tts", text_to_speech::tts_routes())
        .nest("/transcription", speech_to_text::transcription_routes())
        .nest(
            "/languageai/subscriptions",
            languageai_subscriptions::languageai_subscription_routes(),
        )
        .with_state(pool)
        .layer(from_fn(middleware::session_middleware))
        .layer(from_fn(middleware::api_key_middleware))
        .layer(cors)
        .layer(trace_layer);

    // run our app with hyper, listening globally on env port
    println!("Server started at http://{}", host_addr);
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
