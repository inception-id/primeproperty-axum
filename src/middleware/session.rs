use super::{axum_response::AxumResponse, JsonResponse};
use axum::{extract::Request, http::header, http::Method, middleware::Next, response::Response};

pub struct Session;

impl Session {
    pub async fn middleware(req: Request, next: Next) -> Result<Response, AxumResponse<String>> {
        let method = req.method();

        match method {
            &Method::GET => Ok(next.run(req).await),
            _ => {
                let authorization_header = req.headers().get(header::AUTHORIZATION);
                match authorization_header {
                    Some(header_value) => {
                        // Process the authorization header value
                        Ok(next.run(req).await)
                    }
                    None => {
                        let response = JsonResponse::send(401, None, None);
                        Err(response)
                    }
                }
            }
        }
    }
}
