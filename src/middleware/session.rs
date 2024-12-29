use axum::http::HeaderMap;
use serde::Deserialize;
use serde_json::{json, Value};
use std::env;

#[derive(Deserialize, Debug)]
pub(super) struct UserDataInJwt {
    pub(super) id: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub(super) struct VerifySessionData {
    pub(super) userDataInJWT: UserDataInJwt,
}

#[derive(Deserialize, Debug)]
pub(super) struct VerifySessionResponse {
    pub(super) status: String,
    pub(super) session: VerifySessionData,
}

pub fn create_verify_session_payload(access_token: &str) -> Value {
    json!({
        "accessToken": access_token,
        "enableAntiCsrf": false,
        "doAntiCsrfCheck": false,
        "checkDatabase": true
    })
}
pub(super) async fn verify_session(
    access_token: &str,
) -> Result<VerifySessionResponse, reqwest::Error> {
    let connection_uri =
        env::var("SUPERTOKENS_CONNECTION_URI").expect("Missing SUPERTOKENS_CONNECTION_URI");
    let supertokens_api_key = env::var("SUPERTOKENS_API_KEY").expect("Missing SUPERTOKENS_API_KEY");
    let url = format!("{}{}", connection_uri, "/recipe/session/verify");

    let payload = create_verify_session_payload(access_token);

    reqwest::Client::new()
        .post(url)
        .header("Authorization", &supertokens_api_key)
        .json(&payload)
        .send()
        .await?
        .json()
        .await
}

pub fn extract_header_user_id(header_map: HeaderMap) -> Result<uuid::Uuid, uuid::Error> {
    let header_user_id = header_map.get("user-id").expect("Missing user id");
    let user_id = header_user_id.to_str().expect("Invalid header user id");
    uuid::Uuid::parse_str(user_id)
}
