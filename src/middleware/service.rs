use axum::{http::StatusCode, middleware::Next, response::Response, extract::Request};
use std::env;

pub async fn api_key_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let env_api_key = match env::var("API_KEY") {
        Ok(key_result) => key_result,
        Err(key_error) => panic!("{}", key_error.to_string()),
    };

    let header_api_key_option = req
        .headers()
        .get("api_key")
        .and_then(|header| header.to_str().ok());

    match header_api_key_option {
        Some(header_api_key) if header_api_key == env_api_key => Ok(next.run(req).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}
