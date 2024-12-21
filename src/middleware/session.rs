use serde::Deserialize;
use serde_json::{json, Value};
use std::env;

#[derive(Deserialize)]
pub(super) struct VerifySessionResponse {
    pub(super) status: String,
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
