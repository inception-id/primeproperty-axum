use super::session::verify_session;
use axum::http::HeaderValue;
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
    let path = req.uri().path();

    let no_auth_path = ["/languageai/subscriptions/payment/notification/doku"];

    if no_auth_path.contains(&path) {
        Ok(next.run(req).await)
    } else {
        let env_api_key = env::var("API_KEY").expect("Missing API_KEY");
        let header_api_key_option = req
            .headers()
            .get("x-api-key")
            .and_then(|header| header.to_str().ok());

        match header_api_key_option {
            Some(header_api_key) if header_api_key == env_api_key => Ok(next.run(req).await),
            _ => Err(StatusCode::UNAUTHORIZED),
        }
    }
}

pub async fn session_middleware(req: Request, next: Next) -> Result<Response, (StatusCode, Json<ApiResponse<String>>)> {
    let path = req.uri().path();

    let no_auth_path = [
        "/users/create-user",
        "/users/find-user",
        "/languages/find-all",
        "/ai-system-prompts/find-all",
        "/languageai/subscriptions/plans",
        "/languageai/subscriptions/payment/notification/doku",
    ];

    if no_auth_path.contains(&path) {
        Ok(next.run(req).await)
    } else {
        let header_access_token_option = req
            .headers()
            .get("x-access-token")
            .and_then(|header| header.to_str().ok());

        match header_access_token_option {
            Some(header_api_key) => {
                let session_verification = verify_session(header_api_key).await;
                match session_verification {
                    Ok(session) if session.status == "OK" => {
                        let mut new_req = req;
                        let userid_header =
                            HeaderValue::from_str(&session.session.userDataInJWT.id)
                                .expect("Fail to convert userID header");
                        new_req.headers_mut().insert("user-id", userid_header);
                        Ok(next.run(new_req).await)
                    }
                    _ => Err((StatusCode::UNAUTHORIZED, Json(ApiResponse::new(StatusCode::UNAUTHORIZED, None, "Unauthorized")))),
                }
            }
            _ => Err((StatusCode::UNAUTHORIZED, Json(ApiResponse::new(StatusCode::UNAUTHORIZED, None, "Unauthorized")))),
        }
    }
}
