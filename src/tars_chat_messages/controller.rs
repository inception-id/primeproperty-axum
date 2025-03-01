use super::role::TarsChatMessagesRole;
use crate::schema::tars_chat_messages;
use diesel::prelude::Insertable;
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
