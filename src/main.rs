mod agents;
mod db;
mod leads;
mod middleware;
mod properties;
mod schema;
mod traits;

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
        .nest("/agents", agents::agent_routes(pool.clone()))
        .nest("/leads", leads::lead_routes())
        .nest("/properties", properties::property_routes())
        .with_state(pool)
        .layer(from_fn(middleware::Session::middleware))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(NewSentryLayer::new_from_top())
        .layer(SentryHttpLayer::with_transaction());

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
