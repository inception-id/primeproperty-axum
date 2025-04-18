use super::axum_response::{AxumResponse, JsonResponse};
use axum::{extract::Request, middleware::Next, response::Response};
use std::env;

pub struct ApiKey;

impl ApiKey {
    pub async fn middleware(req: Request, next: Next) -> Result<Response, AxumResponse<String>> {
        let path = req.uri().path();

        let api_key_paths = ["/agent/login"];

        if api_key_paths.contains(&path) {
            let env_api_key = env::var("API_KEY").expect("Missing API_KEY");
            let header_api_key = req
                .headers()
                .get("x-api-key")
                .and_then(|header| header.to_str().ok());

            match header_api_key {
                Some(api_key) if env_api_key == api_key => Ok(next.run(req).await),
                _ => {
                    let response = JsonResponse::send(401, None, None);
                    Err(response)
                }
            }
        } else {
            Ok(next.run(req).await)
        }
    }
}
