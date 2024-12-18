use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateUserPayload {
    pub supertokens_user_id: String,
    pub email: String,
}
