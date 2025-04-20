use super::{axum_response::AxumResponse, JsonResponse};
use axum::{
    extract::Request,
    http::{header, HeaderValue, Method},
    middleware::Next,
    response::Response,
};
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct UserDataInJwt {
    id: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct VerifySessionData {
    userDataInJWT: UserDataInJwt,
}

#[derive(Deserialize, Debug)]
struct VerifySessionResponse {
    status: String,
    session: VerifySessionData,
}

pub struct Session;

impl Session {
    fn create_verify_session_payload(access_token: &str) -> serde_json::Value {
        serde_json::json!({
            "accessToken": access_token,
            "enableAntiCsrf": false,
            "doAntiCsrfCheck": false,
            "checkDatabase": true
        })
    }

    async fn verify_session(access_token: &str) -> Result<VerifySessionResponse, reqwest::Error> {
        let connection_uri =
            env::var("SUPERTOKENS_CONNECTION_URI").expect("Missing SUPERTOKENS_CONNECTION_URI");
        let supertokens_api_key =
            env::var("SUPERTOKENS_API_KEY").expect("Missing SUPERTOKENS_API_KEY");
        let url = format!("{}{}", connection_uri, "/recipe/session/verify");

        let payload = Self::create_verify_session_payload(access_token);

        reqwest::Client::new()
            .post(url)
            .header("Authorization", &supertokens_api_key)
            .json(&payload)
            .send()
            .await?
            .json()
            .await
    }

    pub async fn middleware(req: Request, next: Next) -> Result<Response, AxumResponse<String>> {
        let method = req.method();

        match method {
            &Method::GET => Ok(next.run(req).await),
            _ => {
                let authorization_header = req.headers().get(header::AUTHORIZATION);
                let access_token = match authorization_header {
                    Some(header_value) => header_value.to_str().unwrap_or(""),
                    None => {
                        let response = JsonResponse::send(401, None, None);
                        return Err(response);
                    }
                };
                let session_verification = Self::verify_session(access_token).await;

                match session_verification {
                    Ok(session) if session.status == "OK" => {
                        let mut new_req = req;
                        let x_user_id = HeaderValue::from_str(&session.session.userDataInJWT.id)
                            .expect("Fail to convert x-user-id");
                        new_req.headers_mut().insert("x-user-id", x_user_id);
                        Ok(next.run(new_req).await)
                    }
                    _ => {
                        let response = JsonResponse::send(401, None, None);
                        Err(response)
                    }
                }
            }
        }
    }
}
