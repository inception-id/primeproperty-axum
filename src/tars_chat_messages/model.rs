use super::{controller::CreateTarsChatMessagePayload, role::TarsChatMessagesRole};
use crate::db::DbPool;
use crate::schema::tars_chat_messages;
use chrono::NaiveDateTime;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct TarsChatMessage {
    id: i32,
    tars_chat_room_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    role: TarsChatMessagesRole,
    content: String,
    input_tokens: i32,
    output_tokens: i32,
    total_tokens: i32,
}

impl TarsChatMessage {
    pub fn create_multiple(
        pool: &DbPool,
        payload: &Vec<CreateTarsChatMessagePayload>,
    ) -> QueryResult<Vec<Self>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::insert_into(tars_chat_messages::table)
            .values(payload)
            .get_results(conn)
    }

    pub fn create(pool: &DbPool, payload: &CreateTarsChatMessagePayload) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::insert_into(tars_chat_messages::table)
            .values(payload)
            .get_result(conn)
    }
}
