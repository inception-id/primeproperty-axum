use axum::{http::StatusCode, middleware::Next, response::Response, extract::Request};
use std::env;

pub async fn api_key_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let env_api_key = env::var("API_KEY").expect("Missing API_KEY");

    let header_api_key_option = req
        .headers()
        .get("api_key")
        .and_then(|header| header.to_str().ok());

    match header_api_key_option {
        Some(header_api_key) if header_api_key == env_api_key => Ok(next.run(req).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}
