use super::{model::TarsChatMessage, role::TarsChatMessagesRole};
use crate::{db::DbPool, middleware::ApiResponse, schema::tars_chat_messages};
use axum::routing::post;
use axum::{extract::State, Json, Router};
use diesel::prelude::Insertable;
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = tars_chat_messages)]
pub struct CreateTarsChatMessagePayload {
    tars_chat_room_id: Option<i32>,
    role: TarsChatMessagesRole,
    content: String,
    input_tokens: Option<i32>,
    output_tokens: Option<i32>,
    total_tokens: Option<i32>,
}

impl CreateTarsChatMessagePayload {
    pub fn assign_room_id_to_messages(messages: Vec<Self>, room_id: i32) -> Vec<Self> {
        messages
            .into_iter()
            .map(|mut message| {
                message.tars_chat_room_id = Some(room_id);
                message
            })
            .collect()
    }
}

async fn create_tars_chat_message_route(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateTarsChatMessagePayload>,
) -> (StatusCode, Json<ApiResponse<TarsChatMessage>>) {
    match TarsChatMessage::create(&pool, &payload) {
        Ok(message) => ApiResponse::new(StatusCode::CREATED, Some(message), "created").send(),
        Err(err) => {
            ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, None, &err.to_string()).send()
        }
    }
}

pub fn tars_chat_messages_routes() -> Router<DbPool> {
    Router::new().route("/create", post(create_tars_chat_message_route))
}
