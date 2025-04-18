use axum::{http::StatusCode, Json};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response<T> {
    status: u16,
    data: Option<T>,
    message: String,
}

impl<T> Response<T> {
    fn new(status: u16, data: Option<T>, message: String) -> Self {
        Response {
            status,
            data,
            message,
        }
    }
    pub fn send(
        status: u16,
        data: Option<T>,
        message: Option<String>,
    ) -> (StatusCode, Json<Response<T>>) {
        let msg = match message {
            Some(msg) => msg,
            None => match StatusCode::from_u16(status) {
                Ok(status_code) => status_code.to_string(),
                Err(_) => "Unknown error".to_string(),
            },
        };
        let response = Response::new(status, data, msg);
        (StatusCode::from_u16(status).unwrap(), Json(response))
    }
}
