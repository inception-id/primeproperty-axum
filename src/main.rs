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
use axum::{middleware::from_fn, Router};
use sentry_tower::{NewSentryLayer, SentryHttpLayer};
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

    let sentry_url = env::var("SENTRY_URL").expect("Missing SENTRY_URL");
    let _guard = sentry::init((
        sentry_url,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            traces_sample_rate: 1.0,
            ..Default::default()
        },
    ));

    // build our application with a route
    let app = Router::new()
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
        .layer(TraceLayer::new_for_http())
        .layer(NewSentryLayer::new_from_top())
        .layer(SentryHttpLayer::with_transaction());

    // run our app with hyper, listening globally on env port
    let host_addr = env::var("HOST_ADDRESS").expect("Missing HOST_ADDRESS");
    let listener = tokio::net::TcpListener::bind(&host_addr).await.unwrap();

    println!("Server started at {}", host_addr);
    axum::serve(listener, app).await.unwrap();
}
