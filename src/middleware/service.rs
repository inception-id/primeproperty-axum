use super::session::verify_session;
use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response, Json};
use serde::Serialize;
use std::env;

#[derive(Debug, Serialize, Default)]
pub struct ApiResponse<T> {
    status: u16,
    data: Option<T>,
    message: String,
}

impl<T> ApiResponse<T> {
    pub fn new(status: StatusCode, data: Option<T>, message: &str) -> Self {
        Self {
            status: status.as_u16(),
            data,
            message: message.to_string(),
        }
    }

    pub fn send(self) -> (StatusCode, Json<ApiResponse<T>>) {
        (StatusCode::from_u16(self.status).unwrap(), Json(self))
    }
}

pub async fn api_key_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    let env_api_key = env::var("API_KEY").expect("Missing API_KEY");

    let header_api_key_option = req
        .headers()
        .get("ApiKey")
        .and_then(|header| header.to_str().ok());

    match header_api_key_option {
        Some(header_api_key) if header_api_key == env_api_key => Ok(next.run(req).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn session_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    let path = req.uri().path();

    let auth_path = ["/users/find-user", "/users/create-user"];

    if auth_path.contains(&path) {
        Ok(next.run(req).await)
    } else {
        let header_access_token_option = req
            .headers()
            .get("AccessToken")
            .and_then(|header| header.to_str().ok());

        match header_access_token_option {
            Some(header_api_key) => {
                let session_verification = verify_session(header_api_key).await;
                match session_verification {
                    Ok(session) if session.status == "OK" => Ok(next.run(req).await),
                    _ => Err(StatusCode::UNAUTHORIZED),
                }
            }
            _ => Err(StatusCode::UNAUTHORIZED),
        }
    }
}
